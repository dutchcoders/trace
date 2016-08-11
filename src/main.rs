#![feature(custom_attribute)]
#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

// http://system.joekain.com/2015/07/15/rust-load-and-ptrace.html
// http://eli.thegreenplace.net/2011/01/23/how-debuggers-work-part-1
// https://github.com/hoelzro/rust-execution-tracer/blob/master/posix.rs
// https://github.com/eliben/pyelftools/blob/5d3b1545df30d4bc3a6cfa8741b1f08cb6bf204b/elftools/elf/elffile.py
// http://man7.org/linux/man-pages/man5/proc.5.html
// http://www.tldp.org/LDP/LG/issue85/sandeep.html
// https://github.com/larsbergstrom/rust-egl/blob/master/test/egl-android-glue/jni/android-dl.cpp
// https://opensource.apple.com/source/gdb/gdb-908/src/binutils/readelf.c
// http://www.sco.com/developers/gabi/1998-04-29/ch4.eheader.html
// https://grugq.github.io/docs/subversiveld.pdf
// http://lxr.free-electrons.com/source/include/uapi/linux/elf.h#L277

// http://www.tldp.org/LDP/LG/issue85/sandeep.html

// add dwarf support
// parse arguments, file get x64
// diff changed register
// check repetitions
// disable colorcoding when piping
// searching
// filtering
// all params
// ptrace_syscall flag?
// PT_DENY_ATTACH
// dl_addr
// /proc/%d/mem
// /proc/%d/maps
// https://github.com/djpnewton/libcorkscrew_ndk/blob/master/ptrace.c
// load_symbol_table
// http://osxr.org:8080/android/source/system/core/libcorkscrew/symbol_table.c
// load_ptrace_context

extern crate nix;
extern crate libc;
extern crate capstone;
extern crate byteorder;
extern crate serde;
extern crate serde_json;
extern crate serde_macros;
extern crate docopt;
extern crate rustc_serialize;
extern crate ansi_term;
extern crate dwarf;
extern crate memmap;

use memmap::{Mmap, Protection};

use ansi_term::Colour::*;

use docopt::Docopt;

use std::process::Command;
use std::str;

use nix::sys::signal::*;
use nix::sys::ptrace::*;
use nix::unistd::*;

use std::fs::{self, File};
use std::io::{self, BufReader};
use std::io::prelude::*;

use serde_json::Map;
use serde::de::Deserializer;

use std::ffi::CStr;

use libc::pid_t;
use libc::wait;
use libc::c_void;
use libc::c_int;
use libc::WIFSTOPPED;
use libc::dladdr;
use libc::Dl_info;

use std::mem;
use std::fmt;
use std::ffi::CString;

use std::collections::HashMap;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian, ByteOrder};

mod elftools;
use elftools::elftools_const::*;

mod ptrace_util;
use ptrace_util::Process;

mod inferior;
use inferior::*;

// #[macro_use] extern crate text_io;
#[macro_use] extern crate scan_fmt;

struct ByteBuf<'a>(&'a[u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            try!(fmtr.write_fmt(format_args!("{:02x}", byte)));
        }

        Ok(())
    }
}

pub struct SyscallParameters {
    name: String,
    sysname: String,
}


#[derive(Debug)]
#[serde(deny_unknown_fields)]
pub struct Syscall {
    number: u16,
    name: String,
    sysname: String,
    file: String,
    // parameters: Option<Vec<SyscallParameters>>,
}

/*
impl serde::Deserialize for Syscall {
    fn deserialize<
        D: Deserializer,
        >(deserializer: &mut D) -> Result<Syscall, D::Error> {
            println!("Deserialize");
            Ok(Syscall{
                name: String::from("test"),
                sysname: String::from("test"),
                file: String::from(""),
                number: 0,
            })
        }
}
*/

const USAGE: &'static str = "
Program.

Usage: program [options]
    tracer --command=<name>
    tracer --pid=<pid>
    tracer --version

Options:
 -p --pid=VALUE
 -c --command=VALUE
 -a --assembly
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_name: Option<String>,
    flag_command: Option<String>,
    flag_pid: Option<i32>,
    flag_version: bool,
    flag_assembly: bool,
}


#[derive(Clone, Debug)]
pub struct AddressMap {
      pub lower_addr      : u64,
      pub upper_addr      : u64,
        pub path : String,
}


#[derive(Clone, Debug)]
pub struct Maps {
    map: Vec<AddressMap>
}

impl Maps {
    // resolve library call

    pub fn resolve(&self, address: u64) -> Option<&AddressMap> {
        self.map.iter().find(|m| m.lower_addr < address && address < m.upper_addr)
    }

    pub fn parse(&self) {
    }

    pub fn load(pid: pid_t) -> Result<Maps, ()> {
            let file = match File::open(format!("/proc/{:}/maps", pid)) {
               Err(why) => panic!("couldn't open proc map: {}", why),
               Ok(file) => file,
            };

            let mut addressmap = vec![];

            let f = BufReader::new(file);
            for line in f.lines() {
                let l = line.unwrap();

                println!("{:}", l);
                let (lower, upper, lib) = scan_fmt!(l.as_str(), "{x}-{x} {*} {*} {*} {*} {}", String, String, String);

                let lower_addr = u64::from_str_radix(&lower.unwrap().as_str(), 16).map_err(| _ | {
                    ()
                });

                let upper_addr = u64::from_str_radix(&upper.unwrap().as_str(), 16).map_err(| _ | {
                    ()
                });

                addressmap.push(AddressMap{
                    lower_addr: lower_addr.unwrap(),
                    upper_addr: upper_addr.unwrap(),
                    path: lib.unwrap_or(String::from("")),
                });
            }

            Ok(Maps{
                map: addressmap,
            })
    }
}

// http://www.mcs.anl.gov/OpenAD/OpenADFortTkExtendedDox/elf_8h.html#a30ce6352cf03c667272698ada477da95
/*
pub type Elf32_Char = u8;
pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf32_Half = u16;
pub type Elf32_Addr = u32;
pub type Elf32_Off = u32;

pub type Elf64_Char = u8;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf64_Half = u16;
pub type Elf64_Addr = u64;
pub type Elf64_Off = u64;
*/

pub const PT_NULL : u32 = 0;
pub const PT_DYNAMIC : u32 = 2;

pub const DT_PLTGOT : i32 = 3;
pub const DT_NEEDED : i32 = 1;
pub const DT_HASH : i32 = 4;
pub const DT_STRTAB : i32 = 5;
pub const DT_SYMTAB : i32 = 6;
pub const DT_JMPREL : i32 = 23;
pub const DT_RELA   : i32 = 7;
pub const DT_REL : i32 = 17;
pub const DT_RELSZ: i32 = 18;
pub const DT_RELENT : i32 = 19;
pub const DT_PLTREL : i32 = 20;
pub const DT_PLTRELSZ : i32 = 2;
pub const DT_TEXTREL : i32 = 22;

// https://fossies.org/dox/binutils-2.26.1/elf_2external_8h_source.html
#[derive(Debug)]
pub struct Elf64_External_Shdr {
       pub sh_name: [u8; 4],     /* Section name, index in string tbl */
       pub sh_type: Elf64_Word,     /* Type of section */
       pub sh_flags: Elf64_Addr,        /* Miscellaneous section attributes */
       pub sh_addr: Elf64_Addr,     /* Section virtual addr at execution */
       pub sh_offset: Elf64_Addr,       /* Section file offset */
       pub sh_size: Elf64_Addr,     /* Size of section in bytes */
       pub sh_link: Elf64_Word,     /* Index of another section */
       pub sh_info: Elf64_Word,     /* Additional section information */
       pub sh_addralign: Elf64_Addr,    /* Section alignment */
       pub sh_entsize: Elf64_Addr,      /* Entry size if section holds table */
}

#[derive(Debug)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
// elftools?

#[derive(Clone, Debug)]
pub struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}

#[derive(Debug)]
pub struct Elf32_Dyn {
    pub d_tag: Elf32_Sword,
    pub d_val: Elf32_Word,
    pub d_ptr: Elf32_Addr,
}

#[derive(Debug)]
pub struct Elf64_Ehdr {
    // ELF signature is missing in memory
    pub e_ident: [u8; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
// elftools?

#[derive(Clone, Debug)]
pub struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Addr,
    pub p_memsz: Elf64_Addr,
    pub p_align: Elf64_Addr,
}

#[derive(Debug)]
pub struct Elf64_Dyn {
    pub d_tag: Elf64_Sword,
    pub d_val: Elf64_Word,
    pub d_ptr: Elf64_Addr,
}
// https://github.com/lattera/glibc/blob/a2f34833b1042d5d8eeb263b4cf4caaea138c4ad/elf/elf.h

#[derive(Clone, Debug)]
pub struct Elf32_Sym {
    pub st_name: Elf32_Word	,		/* Symbol name (string tbl index) */
    pub	st_value: Elf32_Addr,		/* Symbol value */
    pub st_size: Elf32_Word	,		/* Symbol size */
    pub st_info: u8,		/* Symbol type and binding */
    pub st_other: u8,		/* Symbol visibility */
    pub	st_shndx: u16,		/* Section index */
}

#[derive(Clone, Debug)]
pub struct Elf64_Sym {
    pub st_name: Elf64_Word	,		/* Symbol name (string tbl index) */
    pub st_info: u8,		/* Symbol type and binding */
    pub st_other: u8,		/* Symbol visibility */
    pub	st_shndx: u16,		/* Section index */
    pub	st_value: Elf64_Addr,		/* Symbol value */
    pub st_size: Elf64_Xword	,		/* Symbol size */
}


#[derive(Clone, Debug)]
pub struct link_map {
    /* These first few members are part of the protocol with the debugger.
       This is the same format used in SVR4.  */

    pub l_addr: Elf64_Addr,		/* Base address shared object is loaded at.  */
    pub l_name: Elf64_Addr,		/* Absolute file name object was found in.  */
    pub l_ld: Elf64_Addr,		/* Dynamic section of the shared object.  */
    pub l_next: Elf64_Addr,
    pub l_prev: Elf64_Addr, /* Chain of loaded objects.  */
}

#[derive(Clone, Debug)]
 pub struct Elf32_Rela{
  pub r_offset:  Elf32_Addr ,
  pub r_info: Elf32_Word    ,
  pub r_addend: Elf32_Sword ,
}

#[derive(Clone, Debug)]
pub struct Elf64_Rela {
  pub r_offset: Elf64_Addr,  /* Location at which to apply the action */
  pub r_info: Elf64_Xword,   /* index and type of relocation */
  pub r_addend: Elf64_Sxword,        /* Constant addend used to compute value */
}

fn main() {
/*
    let resp_len = self.stream.read_u32::<LittleEndian>().unwrap();
        let mut resp_bytes = Read::by_ref(&mut self.stream).take(resp_len as u64);
            serde_json::de::from_reader(&mut resp_bytes).map_err(Error::from)

*/
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_pid.is_some() {
    }

    println!("{:?}", args);

    let mut file = File::open("TABELLA_64.json").unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let json : serde_json::Value = serde_json::from_str(contents.as_str()).unwrap();

    let obj = json.as_object().unwrap();

    let mut sysmap : HashMap<u16, String>= HashMap::new();

    for (key, value) in obj.iter() {
        let arr = value.as_array().unwrap();

        let nr = match arr[0] {
            serde_json::Value::U64(v) => v as u16,
            _ => 0,
        };

        let s = match arr[1] {
            serde_json::Value::String(ref v) => v.clone(),
            _ => String::from(""),
        };

        let sc : Syscall = Syscall{
            number: nr,
            name: s.to_string(),
            file: String::from(""),
            sysname: String::from(""),
        };

        sysmap.insert(nr, s.clone());
    }

    let filename = args.flag_command.unwrap();
    // dwarf::elf::load(&filename.clone()).expect("could not read dwarf data");

    let engine = capstone::Capstone::new(capstone::CsArch::ARCH_X86,
                                         capstone::CsMode::MODE_64).expect("could not init engine");

    match fork().expect("fork failed") {
        ForkResult::Parent{ child } => {
            println!("Waiting for child with pid {:}.", child);
            // run debugger with child pid

            let mut wait_status: c_int = Process::wait();

            println!("Child ready.");

            let mut process = Process::new(child);

            let map = Maps::load(child).unwrap();

            // now we should parse the proc map, find address ranges and link them to the file

            // elf header is located in first entry of /proc/%d/maps, contains elf header

            let base_addr = InferiorPointer(0x400000);

            let mut elf_header : [u8; 4] = [0, 0, 0, 0];
            process.peek_text_bytes(base_addr, &mut elf_header[0..]);
            println!("Elf header {:x}", ByteBuf(&elf_header));
            // 341 #define ELFMAG          "\177ELF"
            if (elf_header != [b'\x7f', b'E', b'L', b'F']) {
                println!("Could not find elf magic.");
            }

            // verify elf header

            let ehdr : Elf64_Ehdr = process.read_struct::<Elf64_Ehdr>(base_addr).expect("Ehdr");

            println!("Elf header: {:x} {:?}", base_addr, ehdr);

            // e_shoff has invalid memory range. Only in file?
            /*
            for n in 0..ehdr.e_shnum {

                let base_addr2 = InferiorPointer(0x0061b000);

                println!("offset {:x}", base_addr2 + ehdr.e_shoff as i64 + (n as u64 * ehdr.e_shentsize as u64) as i64);
                let shdr = process.read_struct::<Elf64_External_Shdr>(base_addr2 + ehdr.e_shoff as i64 + (n as u64 * ehdr.e_shentsize as u64) as i64).expect("Could not read ");
                println!("Symbol header {:?}", shdr);
            }
            */
            println!("");

            println!("Program headers");
            println!("********");

            let mut program_headers : HashMap<Elf64_Word, Elf64_Phdr>= HashMap::new();

            for n in 0..ehdr.e_phnum {
                let phdr = process.read_struct::<Elf64_Phdr>(base_addr + ehdr.e_phoff as i64 + (ehdr.e_phentsize as i64* n as i64)).expect("Could not read elf phdr");
                program_headers.insert(phdr.p_type, phdr.clone());
            }
            for ph in program_headers.iter() {
                println!("{:?}", ph);
            }

            println!("");

            let mut dynamic_section = program_headers.get(&PT_DYNAMIC).expect("pt_dynamic not found");

            let mut dyn_addr = InferiorPointer(dynamic_section.p_vaddr as u64);

            println!("");
            println!("Dynamic section");
            println!("********");

            // dynamic section
            let mut got = InferiorPointer(0);
            let mut symtab = InferiorPointer(0);
            let mut strtab = InferiorPointer(0);
            let mut dt_jmprel = InferiorPointer(0);
            let mut dt_pltrelsz = InferiorPointer(0);
            let mut dt_pltrel = InferiorPointer(0);
            let mut needed = vec![];

            loop {
                let dyn = process.read_struct::<Elf64_Dyn>(dyn_addr).expect("dyn");
                println!("Elf header dyn {:?}", dyn);

                if (dyn.d_tag == 0) {
                    break
                } else if (dyn.d_tag == DT_STRTAB) {
                    strtab = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_SYMTAB) {
                    symtab = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTGOT) {
                    got = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_NEEDED) {
                    needed.push(inferior::InferiorPointer(dyn.d_ptr as u64)); // + 0x400000;
                } else if (dyn.d_tag == DT_JMPREL) {
                    dt_jmprel = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTRELSZ) {
                    dt_pltrelsz = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTREL) {
                    dt_pltrel = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                }

                /*
                 process_dynamic_section / read_elf
                 case DT_NULL	:
                 case DT_NEEDED	:
                 case DT_PLTGOT	:
                 case DT_HASH	:
                 case DT_STRTAB	:
                 case DT_SYMTAB	:
                 case DT_RELA	:
                 case DT_INIT	:
                 case DT_FINI	:
                 case DT_SONAME	:
                 case DT_RPATH	:
                 case DT_SYMBOLIC:
                 case DT_REL	:
                 case DT_DEBUG	:
                 case DT_TEXTREL	:
                 case DT_JMPREL	:
                 case DT_RUNPATH	:
                 */
                dyn_addr = dyn_addr + mem::size_of::<Elf64_Dyn>() as i64;
            }

            println!("");

            println!("Strtab: {:x} Symtab: {:x} Got: {:x} ", strtab, symtab, got);

            // strtab
            let c_str = process.string_at(strtab + 1 as i64).expect("could not read string");
            println!("{:?}", c_str);

            println!("Needed");
            println!("********");

            // needed
            for s in needed.into_iter() {
                let c_str2  = process.string_at(strtab + (s.as_voidptr() as i64)).expect("could not read string");
                println!("Needed: {:?}", c_str2);
            }

            println!("");

            println!("Symbols");
            println!("********");

            let mut offset = symtab;

            // symbols
            let mut sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
            for n in 0..128 { //ehdr.e_shnum {

                let c_str2  = process.string_at(strtab + (sym.st_name as i64)).expect("could not read string");
                println!("{:?} {:x} {:?} {:?}", n, offset, sym, c_str2);

                // st_name, st_value, offset?
                offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
            }

            println!("");

            // Relocation
            let mut offset = dt_jmprel;
            let mut rela = process.read_struct::<Elf64_Rela>(offset).expect("rela");
            for n in 0..128 { //ehdr.e_shnum {
                // type and offset
                //
                println!("Rela {:?} {:?} {:?} {:?} {:x}", n, rela, (rela.r_info >> 32), (rela.r_info & 0xffffffff), rela.r_offset);

                offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                rela = process.read_struct::<Elf64_Rela>(offset).expect("sym");
            }

            // got
            // got[0] contains address that points to the dynamic segment of the executable, which
            // is used by the dynamic linker for extracting dynamic linking-related information
            // got[1] contains the address of the link_map structure that is used by the dynamic
            // linker to resolve symbols
            // got[2] contains the address to the dynamic links _dl_runtime_resolve() function that
            // resolves the actual symbol address for the shared library function
            /*
            got = got + 0x8;

            println!("Link map:");

            let mut lm = process.read_struct::<link_map>(got).expect("lm");
            loop {
                println!("{:?} {:x} {:x} {:x} {:x}", lm, got, lm.l_addr, lm.l_prev, lm.l_next);

                if lm.l_name != 0 {
                    let c_str2 = process.string_at(InferiorPointer(lm.l_name as u64)).expect("could not read string");
                    println!("Linkmap: {:?}", c_str2);
                }

                lm = process.read_struct::<link_map>(InferiorPointer(lm.l_next)).expect("lm");
            }
            */

            let mut map_addr : u32 = process.read_struct::<u32>(got).expect("could not load got[1]");
            println!("Map addr: Strtab: {:?} Symtab: {:?} Got: {:?} {:x}", strtab.as_voidptr(), symtab.as_voidptr(), got.as_voidptr(), map_addr);

            let str2 = process.string_at(InferiorPointer(map_addr as u64));
            println!("{:?} {:x}", str2, map_addr);

            let mut next = 0;
            while Process::WIFSTOPPED(wait_status) {
                let p = process.rip().ok().expect("");

                if next != p.as_voidptr() as u64 {
                    println!("");
                }

                let curr_addr_map = map.resolve(p.as_voidptr() as u64);

                next = p.as_voidptr() as u64;

                // allow max 16 bits instructions
                let mut raw_bytes : [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

                process.peek_text_bytes(p, &mut raw_bytes[0..]);

/*
                let info = unsafe {
                    let mut info: Dl_info= mem::uninitialized();;

                    dladdr(p.as_voidptr() as *const c_void, &mut info);
                    info
                };

                println!("{:?}", info.dli_fname);
                println!("{:?}", info.dli_sname);

*/
                // check and record changed registers

                match engine
                    .disasm(&raw_bytes, p.as_voidptr() as u64, 0) {
                        Some(insns) => {
                            for i in insns.iter() {
                                next+=i.size as u64;

                                let size = i.size as u8;
                                let bytes : &[u8] = unsafe { std::slice::from_raw_parts(i.bytes.as_ptr(), size as usize) };

                                if raw_bytes[0x0] == 0xff && raw_bytes[0x1] == 0x25 {
                                    println!("{:} {:08x} {:<20} {:} {:}", curr_addr_map.map_or(String::from("unknown"), |m|m.path.clone()), p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), i.mnemonic().unwrap(), i.op_str().unwrap());
                                    println!("{:?}", i);

                                    let address = byteorder::LittleEndian::read_u32(&raw_bytes[2..]);
                                    println!("{:x}", address);
                                    let mut ndx: u64 = 0;

                                    let mut offset = dt_jmprel;
                                    let mut rela = process.read_struct::<Elf64_Rela>(offset).expect("rela");
                                    for n in 0..128 { //ehdr.e_shnum {
                                        // type and offset
                                        //
                                        if rela.r_offset == (p + address as i64 - 2 as i64).as_voidptr() as u64 {
                                            println!("Rela {:?} {:?} {:?} {:?} {:x}", n, rela, (rela.r_info >> 32), (rela.r_info & 0xffffffff), rela.r_offset);
                                                ndx = (rela.r_info >> 32) + 1;
                                        } else {
                                            // println!("Rela {:?} {:?} {:?} {:?} {:x} {:x}", n, rela, (rela.r_info >> 32), (rela.r_info & 0xffffffff), rela.r_offset, p + address as i64 + 2 as i64);
                                        }

                                        offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                                        rela = process.read_struct::<Elf64_Rela>(offset).expect("sym");
                                    }

                                    let mut offset = symtab;

                                    // symbols
                                    let mut sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
                                    for n in 0..128 { //ehdr.e_shnum {

                                        let c_str2  = process.string_at(strtab + (sym.st_name as i64)).expect("could not read string");
                                        if n == ndx {
                                        println!("{:?} {:x} {:?} {:?}", n, offset, sym, c_str2);
                                        }

                                        // st_name, st_value, offset?
                                        offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                                        sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
                                    }
                                } else if raw_bytes[0x0] == 0x0f && raw_bytes[0x1] == 0x05 {
                                    // rax contains syscall number
                                    let rax = ptrace_util::get_rax(child).as_voidptr() as u16;
                                    let text = match sysmap.get(&rax) {
                                        Some(v) => v,
                                        None => "unknown",
                                    };

                                    // dump syscall
                                    if rax == 2 {
                                        let rdi = ptrace_util::get_rdi(child);
                                        let rsi = ptrace_util::get_rsi(child);
                                        let rdx = ptrace_util::get_rdx(child);

                                        println!("RDI: {:2x} RSI: {:2x} RDX: {:2x}", rdi, rsi, rdx);

                                        let s = process.string_at(rdi);
                                        println!("{:?} RDI: {:2x} RSI: {:2x} RDX: {:2x}", s, rdi, rsi, rdx);
                                    }

                                    println!("{:} {:08x} {:<20} {:} {:}{:}({:2x})", curr_addr_map.map_or(String::from("unknown"), |m|m.path.clone()), p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), Yellow.paint(i.mnemonic().unwrap()), i.op_str().unwrap(), text, rax);
                                } else {
                                    if args.flag_assembly {
                                    // println!("{:} {:08x} {:<20} {:} {:}{:}({:2x})", curr_addr_map.map_or("unknown", |m|m.path), p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), Yellow.paint(i.mnemonic().unwrap()), i.op_str().unwrap(), text, rax);

                                        println!("{:} {:08x} {:<20} {:} {:}", curr_addr_map.map_or(String::from("unknown"), |m|m.path.clone()), p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), i.mnemonic().unwrap(), i.op_str().unwrap());
                                    }
                                }
                                break;
                            }
                        }
                        None => {
                            println!("{:08x} {:x} FAILED", p.as_voidptr() as u64, ByteBuf(&raw_bytes));
                        }
                    }

                process.single_step();
                wait_status = Process::wait();
            }
        }
        ForkResult::Child => {
            ptrace_util::trace_me();

            let c_filename = CString::new(filename).unwrap(); //.to_str().unwrap()).unwrap();


            let mut args = &[c_filename];

            execve(&args[0], args, &[])
                .ok()
                .expect("Failed execve");
        }
    }
}

