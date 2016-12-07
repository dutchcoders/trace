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

fn dump_program_headers(process: &mut Process) {
    let program_headers: HashMap<Elf64_Word, Elf64_Phdr> = process.program_headers().unwrap();

    println!("Program headers");
    println!("********");

    for ph in program_headers.iter() {
        println!("{:?}", ph);
    }

    println!("");
    println!("");
}

fn dump_symbols(process: &mut Process) {
    let symtab = *(process.sections(DT_SYMTAB).unwrap().first().unwrap());
    let strtab = *(process.sections(DT_STRTAB).unwrap().first().unwrap());

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
}

fn dump_relocations(process: &mut Process) {
    let dt_jmprel = *(process.sections(DT_JMPREL).unwrap().first().unwrap());

    // Relocation
    println!("Relocations");
    println!("********");

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

    println!("");
}

fn dump_needed(process: &mut Process) {
    println!("Needed");
    println!("********");

    let strtab = *(process.sections(DT_STRTAB).unwrap().first().unwrap());
    let needed = process.sections(DT_NEEDED).unwrap();
    for s in needed.into_iter() {
        let c_str2 = process.string_at(strtab + (s.as_voidptr() as i64))
            .expect("could not read string");
        println!("Needed: {:?}", c_str2);
    }

    println!("");
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_pid.is_some() {
    }

    let filename = args.flag_command.unwrap();

    // dwarf::elf::load(&filename.clone()).expect("could not read dwarf data");

    match fork().expect("fork failed") {
        ForkResult::Parent { child } => {
            println!("{}", Yellow.paint(format!("Waiting for child with pid {:}.", child)));

            let mut wait_status: c_int = Process::wait();

            println!("{}", Yellow.paint("Child ready."));

            let mut process = Process::new(child);

            let map = Maps::load(child).unwrap();

            let base_addr = InferiorPointer(0x400000);

            if !process.is_elf() {
                println!("{}", Red.paint("Child is not an elf binary."));
                return;
            }

            dump_program_headers(&mut process);

            dump_needed(&mut process);

            dump_symbols(&mut process);

            dump_relocations(&mut process);

            let ehdr: Elf64_Ehdr = process.ehdr().unwrap();

            let symtab = *(process.sections(DT_SYMTAB).unwrap().first().unwrap());
            let strtab = *(process.sections(DT_STRTAB).unwrap().first().unwrap());
            let dt_jmprel = *(process.sections(DT_JMPREL).unwrap().first().unwrap());

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

                let engine = capstone::Capstone::new(capstone::CsArch::ARCH_X86, capstone::CsMode::MODE_64)
                    .expect("could not init engine");

                match engine
                    .disasm(&raw_bytes, p.as_voidptr() as u64, 0) {
                        Some(insns) => {
                            for i in insns.iter() {
                                next+=i.size as u64;

                                let size = i.size as u8;
                                let bytes : &[u8] = unsafe { std::slice::from_raw_parts(i.bytes.as_ptr(), size as usize) };

                                // jmp far
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

                process.single_step().unwrap();

                wait_status = Process::wait();
            }
        }
        ForkResult::Child => {
            ptrace_util::trace_me();

            let c_filename = CString::new(filename).unwrap(); //.to_str().unwrap()).unwrap();


            let args = &[c_filename];

            execve(&args[0], args, &[])
                .ok()
                .expect("Failed execve");
        }
    }
}
