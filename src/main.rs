#![feature(custom_attribute)]
#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

// http://system.joekain.com/2015/07/15/rust-load-and-ptrace.html
// http://eli.thegreenplace.net/2011/01/23/how-debuggers-work-part-1
// https://github.com/hoelzro/rust-execution-tracer/blob/master/posix.rs

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

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};


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


pub struct AddressMap {
      pub lower_addr      : u64,
      pub upper_addr      : u64,
pub path : String,
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

            let file = match File::open(format!("/proc/{:}/maps", child)) {
               Err(why) => panic!("couldn't open proc map: {}", 
                                   why),
               Ok(file) => file,
            };

let mut addressmap = vec![];

            let f = BufReader::new(file);
            for line in f.lines() {
                let l = line.unwrap();
                println!("{:}", l);
                let (lower, upper, lib) = scan_fmt!(l.as_str(), "{x}-{x} {*} {*} {*} {*} {}", String, String, String);
                //println!("file {:?}", lib.unwrap_or(String::from("")));
                //println!("lower: {:?}", lower.unwrap());
                //println!("upper: {:?}", upper.unwrap());

                let lower_addr = u64::from_str_radix(&lower.unwrap().as_str(), 16).map_err(| _ | {
                    ()
                });

                let upper_addr = u64::from_str_radix(&upper.unwrap().as_str(), 16).map_err(| _ | {
                    ()
                });

                println!("lower: {:?}", lower_addr.unwrap());
                println!("upper: {:?}", upper_addr.unwrap());
                addressmap.push(AddressMap{
                    lower_addr: lower_addr.unwrap(),
                    upper_addr: upper_addr.unwrap(),
                    path: lib.unwrap_or(String::from("")),
                });
            }           

            /*
               match file.read_to_string(&mut s) {
               Err(why) => panic!("couldn't read proc map: {}", 
               why),
               Ok(_) => print!("contains:\n{}", s),
               }
               */

            // now we should parse the proc map, find address ranges and link them to the file
            let mut next = 0;

            while Process::WIFSTOPPED(wait_status) {
                let p = process.rip().ok().expect("");

                if next != p.as_voidptr() as u64 {
                    println!("");
                }

                let curr_addr_map : Option<&AddressMap> = {
                    for m in addressmap.iter() {
                        if m.lower_addr < (p.as_voidptr() as u64) && (p.as_voidptr() as u64) < m.upper_addr {
                            println!("{} {:2x}", m.path, p);
                        }
                    };
                    None
                };

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
                match engine
                    .disasm(&raw_bytes, p.as_voidptr() as u64, 0) {
                        Some(insns) => {
                            for i in insns.iter() {
                                next+=i.size as u64;

                                let size = i.size as u8;
                                let bytes : &[u8] = unsafe { std::slice::from_raw_parts(i.bytes.as_ptr(), size as usize) };

                                if raw_bytes[0x0] == 0x0f && raw_bytes[0x1] == 0x05 {
                                    let rax = ptrace_util::get_rax(child).as_voidptr() as u16;

                                    let text = match sysmap.get(&rax) {
                                        Some(v) => v,
                                        None => "unknown",
                                    };

                                    if rax == 2 {
                                        let rdi = ptrace_util::get_rdi(child);
                                        let rsi = ptrace_util::get_rsi(child);
                                        let rdx = ptrace_util::get_rdx(child);

                                        println!("RDI: {:2x} RSI: {:2x} RDX: {:2x}", rdi, rsi, rdx);

                                        let mut raw_filename : [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
                                        process.peek_text_bytes(rdi, &mut raw_filename[0..8]);

                                        let s = match str::from_utf8(&raw_filename) {
                                            Ok(v) => v,
                                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                        };

                                        println!("{:} {:2x} RDI: {:2x} RSI: {:2x} RDX: {:2x}", s, ByteBuf(&raw_filename), rdi, rsi, rdx);
                                    }

                                    println!("{:08x} {:<20} {:} {:}{:}({:2x})", p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), Yellow.paint(i.mnemonic().unwrap()), i.op_str().unwrap(), text, rax);
                                } else {
                                    if args.flag_assembly {

                                        println!("{:08x} {:<20} {:} {:}", p.as_voidptr() as u64, format!("{:2x}", ByteBuf(bytes)), i.mnemonic().unwrap(), i.op_str().unwrap());
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

