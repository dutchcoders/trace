#![feature(custom_attribute)]

extern crate nix;
extern crate libc;
extern crate capstone;
extern crate byteorder;
extern crate docopt;
extern crate rustc_serialize;
extern crate ansi_term;
extern crate dwarf;
extern crate memmap;
extern crate elftools;

use elftools::*;

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

mod ptrace_util;
use ptrace_util::Process;

mod inferior;
use inferior::*;

mod syscalls64;
use syscalls64::*;

mod addressmap;
use addressmap::*;

mod bytebuf;
use bytebuf::*;

mod process;
use process::*;

// #[macro_use] extern crate text_io;
#[macro_use]
extern crate scan_fmt;

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
pub struct link_map {
    // These first few members are part of the protocol with the debugger.
    // This is the same format used in SVR4.
    pub l_addr: Elf64_Addr, // Base address shared object is loaded at.
    pub l_name: Elf64_Addr, // Absolute file name object was found in.
    pub l_ld: Elf64_Addr, // Dynamic section of the shared object.
    pub l_next: Elf64_Addr,
    pub l_prev: Elf64_Addr, // Chain of loaded objects.
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_pid.is_some() {
    }

    let filename = args.flag_command.unwrap();

    // dwarf::elf::load(&filename.clone()).expect("could not read dwarf data");

    let engine = capstone::Capstone::new(capstone::CsArch::ARCH_X86, capstone::CsMode::MODE_64)
        .expect("could not init engine");

    match fork().expect("fork failed") {
        ForkResult::Parent { child } => {
            println!("{}", Yellow.paint(format!("Waiting for child with pid {:}.", child)));

            let mut wait_status: c_int = Process::wait();

            println!("{}", Yellow.paint("Child ready."));

            let mut process = Process::new(child);

            let map = Maps::load(child).unwrap();

            let base_addr = InferiorPointer(0x400000);

            if (!process.is_elf()) {
                println!("{}", Red.paint("Child is not an elf binary."));
                return;
            }

            // verify elf header
            let ehdr: Elf64_Ehdr = process.ehdr().unwrap();

            println!("Elf header: {:x} {:?}", base_addr, ehdr);

            println!("Program headers");
            println!("********");

            let mut program_headers: HashMap<Elf64_Word, Elf64_Phdr> = process.program_headers().unwrap();

            for ph in program_headers.iter() {
                println!("{:?}", ph);
            }

            println!("");

            let mut dynamic_section = program_headers.get(&(PT_DYNAMIC as Elf64_Word))
                .expect("pt_dynamic not found");

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
                    break;
                } else if (dyn.d_tag == DT_STRTAB as i64) {
                    strtab = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_SYMTAB as i64) {
                    symtab = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTGOT as i64) {
                    got = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_NEEDED as i64) {
                    needed.push(inferior::InferiorPointer(dyn.d_ptr as u64)); // + 0x400000;
                } else if (dyn.d_tag == DT_JMPREL as i64) {
                    dt_jmprel = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTRELSZ as i64) {
                    dt_pltrelsz = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else if (dyn.d_tag == DT_PLTREL as i64) {
                    dt_pltrel = inferior::InferiorPointer(dyn.d_ptr as u64); // + 0x400000;
                } else {
                    println!("{:?}", dyn.d_ptr);
                }

                // process_dynamic_section / read_elf
                // case DT_NULL	:
                // case DT_NEEDED	:
                // case DT_PLTGOT	:
                // case DT_HASH	:
                // case DT_STRTAB	:
                // case DT_SYMTAB	:
                // case DT_RELA	:
                // case DT_INIT	:
                // case DT_FINI	:
                // case DT_SONAME	:
                // case DT_RPATH	:
                // case DT_SYMBOLIC:
                // case DT_REL	:
                // case DT_DEBUG	:
                // case DT_TEXTREL	:
                // case DT_JMPREL	:
                // case DT_RUNPATH	:
                //
                dyn_addr = dyn_addr + mem::size_of::<Elf64_Dyn>() as i64;
            }

            println!("");

            /*

               // section headers are not loaded at runtime
            println!("Section headers");
            println!("********");

            // e_shoff has invalid memory range. Only in file?
            for n in 0..ehdr.e_shnum {

                let base_addr2 = InferiorPointer(0x0061b000);

                println!("offset {:x}",
                         base_addr2 + ehdr.e_shoff as i64 +
                         (n as u64 * ehdr.e_shentsize as u64) as i64);
                let shdr = process.read_struct::<Elf64_External_Shdr>(base_addr2 + ehdr.e_shoff as i64 + (n as u64 * ehdr.e_shentsize as u64) as i64).expect("Could not read ");
                println!("Section header {:?}", shdr);
            }

            println!("");
            */

            println!("Strtab: {:x} Symtab: {:x} Got: {:x} ", strtab, symtab, got);

            // strtab
            let c_str = process.string_at(strtab + 1 as i64).expect("could not read string");
            println!("{:?}", c_str);

            println!("Needed");
            println!("********");

            // needed
            for s in needed.into_iter() {
                let c_str2 = process.string_at(strtab + (s.as_voidptr() as i64))
                    .expect("could not read string");
                println!("Needed: {:?}", c_str2);
            }

            println!("");

            println!("Symbols");
            println!("********");

            let mut offset = symtab;

            // symbols
            let mut sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
            for n in 0..128 {
                // ehdr.e_shnum {

                let c_str2 = process.string_at(strtab + (sym.st_name as i64))
                    .expect("could not read string");
                println!("{:?} {:x} {:?} {:?}", n, offset, sym, c_str2);

                // st_name, st_value, offset?
                offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
            }

            println!("");

            // Relocation
            let mut offset = dt_jmprel;
            let mut rela = process.read_struct::<Elf64_Rela>(offset).expect("rela");
            for n in 0..128 {
                // ehdr.e_shnum {
                // type and offset
                //
                println!("Rela {:?} {:?} {:?} {:?} {:x}",
                         n,
                         rela,
                         (rela.r_info >> 32),
                         (rela.r_info & 0xffffffff),
                         rela.r_offset);

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
            //
            // got = got + 0x8;
            //
            // println!("Link map:");
            //
            // let mut lm = process.read_struct::<link_map>(got).expect("lm");
            // loop {
            // println!("{:?} {:x} {:x} {:x} {:x}", lm, got, lm.l_addr, lm.l_prev, lm.l_next);
            //
            // if lm.l_name != 0 {
            // let c_str2 = process.string_at(InferiorPointer(lm.l_name as u64)).expect("could not read string");
            // println!("Linkmap: {:?}", c_str2);
            // }
            //
            // lm = process.read_struct::<link_map>(InferiorPointer(lm.l_next)).expect("lm");
            // }
            //


            let mut map_addr: u32 = process.read_struct::<u32>(got).expect("could not load got[1]");
            println!("Map addr: Strtab: {:?} Symtab: {:?} Got: {:?} {:x}",
                     strtab.as_voidptr(),
                     symtab.as_voidptr(),
                     got.as_voidptr(),
                     map_addr);

            let str2 = process.string_at(InferiorPointer(map_addr as u64));
            println!("{:?} {:x}", str2, map_addr);

            let mut next = 0;
            while Process::WIFSTOPPED(wait_status) {
                let p = process.rip().ok().expect("");

                if next != p.as_voidptr() as u64 {
                    // println!("");
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
                                    // println!("{:} {:08x} {:<20} {:} {:}", curr_addr_map.map_or(String::from("unknown"), |m|m.path.clone()), p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), i.mnemonic().unwrap(), i.op_str().unwrap());
                                    // println!("{:?}", i);

                                    let address = byteorder::LittleEndian::read_u32(&raw_bytes[2..]) - 2;
                                    // println!("{:x}", address);
                                    let mut ndx: u64 = 0;

                                    let mut offset = dt_jmprel;
                                    let mut rela = process.read_struct::<Elf64_Rela>(offset).expect("rela");
                                    for n in 0..128 { //ehdr.e_shnum {
                                        // type and offset
                                        //
                                        if rela.r_offset == (p + address as i64).as_voidptr() as u64 {
                                            // println!("Rela {:?} {:?} {:?} {:?} {:x}", n, rela, (rela.r_info >> 32), (rela.r_info & 0xffffffff), rela.r_offset);
                                                ndx = (rela.r_info >> 32) + 1;
                                        } else {
                                            // println!("Rela {:?} {:?} {:?} {:?} {:x} {:x}", n, rela, (rela.r_info >> 32), (rela.r_info & 0xffffffff), rela.r_offset, p + address as i64 + 2 as i64);
                                        }

                                        offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                                        rela = process.read_struct::<Elf64_Rela>(offset).expect("rela");
                                    }

                                    if ndx == 0 {
                                        // could not find symbol
                                        continue;
                                    }

                                    let mut offset = symtab;

                                    // symbols
                                    let mut sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
                                    for n in 0..128 { //ehdr.e_shnum {

                                        let c_str2  = process.string_at(strtab + (sym.st_name as i64)).expect("could not read string");
                                        if n == ndx {
                                            // sym.st_shndx -> ref section header

                                            if (sym.st_info >> 4) == elftools::STB_WEAK as u8 {
                                                println!("WEAK");
                                            } else if (sym.st_info >> 4) == elftools::STB_GLOBAL  as u8{
                                                println!("GLOBAL");
                                            } else if (sym.st_info >> 4) == elftools::STB_LOCAL  as u8{
                                                println!("LOCAL");
                                            }

                                            if (sym.st_info & 0xf) == elftools::STT_NOTYPE as u8 {
                                                println!("NOTYPE");
                                            } else if (sym.st_info & 0xf) == elftools::STT_OBJECT  as u8{
                                                println!("OBJECT");
                                            } else if (sym.st_info & 0xf) == elftools::STT_FUNC  as u8{
                                                println!("FUNC");
                                            } else if (sym.st_info & 0xf) == elftools::STT_SECTION  as u8{
                                                println!("SECTIOn");
                                            } else if (sym.st_info & 0xf) == elftools::STT_FILE  as u8{
                                                println!("FILE");
                                            } else if (sym.st_info & 0xf) == elftools::STT_COMMON  as u8{
                                                println!("COMMON");
                                            } else if (sym.st_info & 0xf) == elftools::STT_TLS  as u8{
                                                println!("TLS");
                                            }

                                            println!("Symbol {:}", Red.paint(c_str2));
                                        }

                                        // st_name, st_value, offset?
                                        offset = offset + mem::size_of::<Elf64_Sym>() as i64;
                                        sym = process.read_struct::<Elf64_Sym>(offset).expect("sym");
                                    }
                                } else if raw_bytes[0x0] == 0x0f && raw_bytes[0x1] == 0x05 {
                                    // rax contains syscall number
                                    let rax = ptrace_util::get_rax(child).as_voidptr() as u16;
                                    let text = match syscalls64.iter().find(|&r|r.number == rax) {
                                        Some(v) => v.name,
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
