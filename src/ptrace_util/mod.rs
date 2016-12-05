// (partly) based on http://system.joekain.com/2015/09/07/refactoring-ptrace-code.html
    
extern crate std;

use libc::pid_t;
use nix::sys::ptrace::*;
use nix::sys::ptrace::ptrace::*;
use std::ptr;
use libc::c_void;
use std::str;

use inferior::InferiorPointer;
use std::slice;

use std::mem;

use libc::wait;
use libc::c_int;
use libc::WIFSTOPPED;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::ffi::{CString, CStr};
use std::ffi::IntoStringError;

pub mod user {
    pub mod regs {
        pub const R15: i64 = 0 * 8;
        pub const R14: i64 = 1 * 8;
        pub const R13: i64 = 2 * 8;
        pub const R12: i64 = 3 * 8;
        pub const RBP: i64 = 4 * 8;
        pub const RBX: i64 = 5 * 8;
        pub const R11: i64 = 6 * 8;
        pub const R10: i64 = 7 * 8;
        pub const R9: i64 = 8 * 8;
        pub const R8: i64 = 9 * 8;
        pub const RAX: i64 = 10 * 8;
        pub const RCX: i64 = 11 * 8;
        pub const RDX: i64 = 12 * 8;
        pub const RSI: i64 = 13 * 8;
        pub const RDI: i64 = 14 * 8;
        pub const ORIG_RAX: i64 = 15 * 8;
        pub const RIP: u64 = 16 * 8;
        pub const CS: i64 = 17 * 8;
        pub const EFLAGS: i64 = 18 * 8;
        pub const RSP: i64 = 19 * 8;
        pub const SS: i64 = 20 * 8;
        pub const FS_BASE: i64 = 21 * 8;
        pub const GS_BASE: i64 = 22 * 8;
        pub const DS: i64 = 23 * 8;
        pub const ES: i64 = 24 * 8;
        pub const FS: i64 = 25 * 8;
        pub const GS: i64 = 26 * 8;
    }
}

// get_regs

pub struct Process {
    pub pid: pid_t,
    pub memfile: BufReader<fs::File>,
}

impl Process {
    pub fn new(pid: pid_t) -> Self {
        let mut memfile = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(format!("/proc/{:}/mem", pid))
            .unwrap();

        let mut reader = BufReader::new(memfile);

        Process {
            pid: pid,
            memfile: reader,
        }
    }

    pub fn rax(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RAX as *mut c_void)
    }

    pub fn rbx(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RBX as *mut c_void)
    }

    pub fn rdx(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RDX as *mut c_void)
    }

    pub fn rdi(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RDI as *mut c_void)
    }

    pub fn rsi(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RSI as *mut c_void)
    }

    pub fn rip(&self) -> Result<InferiorPointer, String> {
        self.peek_user(user::regs::RIP as *mut c_void)
    }

    pub fn peek_user(&self, addr: *mut c_void) -> Result<InferiorPointer, String> {
        match ptrace(PTRACE_PEEKUSER, self.pid, addr, ptr::null_mut()) {
            Ok(raw) => Result::Ok(InferiorPointer(raw as u64)),
            Err(e) => Result::Err(String::from("")),
        }
    }

    pub fn peek_text(&self, address: InferiorPointer) -> i64 {
        ptrace(PTRACE_PEEKTEXT,
               self.pid,
               address.as_voidptr(),
               ptr::null_mut())
            .ok()
            .expect("Failed PTRACE_PEEKTEXT")
    }

    pub fn read_struct<T>(&mut self, address: InferiorPointer) -> io::Result<T> {
        self.memfile.seek(SeekFrom::Start(address.as_voidptr() as u64)).expect("seek");

        let num_bytes = ::std::mem::size_of::<T>();
        unsafe {
            let mut s = ::std::mem::uninitialized();
            let mut buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
            match self.memfile.read_exact(buffer) {
                Ok(()) => Ok(s),
                Err(e) => {
                    ::std::mem::forget(s);
                    Err(e)
                }
            }
        }
    }

    // pub fn struct_at<A, B>(&mut self, address: InferiorPointer)  -> Result<String, ()> {
    // let mut str2 : A = unsafe {std::mem::uninitialized()};
    // unsafe {
    // let mut buf = Vec::with_capacity(std::mem::size_of::<A>());
    // self.memfile.read(&buf );
    //
    // memfile.seek(p.as_voidptr() as usize, io::SeekFrom::Start);
    //
    // let x2 : A = mem::transmute(&mut str2);
    // self.memfile.seek(SeekFrom::Start(address.as_voidptr() as u64)).expect("seek");
    // process.peek_text_bytes(strtab, std::mem::transmute::<&mut u32, &mut [u8; 4]>(&mut str2));
    // let mut buf = std::mem::transmute::<&A, *mut u8>(&mut str2);
    // self.memfile.read(buf as &mut[u8]);
    // }
    // Ok(String::from("test"))
    // }
    //

    pub fn string_at(&mut self, address: InferiorPointer) -> Result<String, IntoStringError> {
        self.memfile.seek(SeekFrom::Start(address.as_voidptr() as u64)).expect("seek");

        let mut buf = Vec::<u8>::new();
        self.memfile.read_until(b'\0', &mut buf).expect("read_until failed");

        // let c_str: &CStr = unsafe { CStr::from_ptr(buf.as_ref()) };
        // let buf: &[u8] = c_str.to_bytes();
        // let str_slice: &str = str::from_utf8(buf).unwrap();
        //


        let s = CStr::from_bytes_with_nul(buf.as_ref()).unwrap();
        Ok(s.to_str().unwrap().to_string())

        // let str_slice: &str = str::from_utf8(buf.as_ref()).unwrap();
        // Ok(str_slice.to_string())
        // String::from_str(str_slice)
        // (unsafe { CString::from_vec_unchecked(buf).into_string() })
    }

    pub fn peek_text_bytes(&mut self, address: InferiorPointer, bytes: &mut [u8]) {
        self.memfile.seek(SeekFrom::Start(address.as_voidptr() as u64)).expect("seek");
        // memfile.seek(p.as_voidptr() as usize, io::SeekFrom::Start);
        self.memfile.read(&mut bytes[0..]);

        // let filename = ptrace(PTRACE_PEEKTEXT, self.pid, address.as_voidptr(), ptr::null_mut())
        // .ok()
        // .expect("Failed PTRACE_PEEKTEXT");
        // let mut raw_filename : Vec<u8> = Vec::new();
        // let mut raw_filename : [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        //
        // let mut i = 0;
        // while i < 8 { // mem::size_of::<libc::c_void>() {
        // raw_filename.push(((filename >> (i*8)) & 0xFF) as u8);
        // raw_filename[i] = (((filename >> (i*8)) & 0xFF) as u8);
        // i+=1;
        // }
        //
        // bytes[0..8].clone_from_slice(&raw_filename);
        // raw_filename
        //
    }


    pub fn single_step(&self) -> Result<(), ()> {
        match ptrace(PTRACE_SINGLESTEP,
                     self.pid,
                     ptr::null_mut(),
                     ptr::null_mut()) {
            Ok(_) => Ok(()),
            Err(e) => Err(()),
        }
    }

    // TODO: don't think this should be in process. It is more of parent.
    pub fn wait() -> c_int {
        let mut wait_status: c_int = 0;
        unsafe {
            wait(&mut wait_status);
        }
        wait_status
    }

    pub fn WIFSTOPPED(wait_status: c_int) -> bool {
        unsafe { WIFSTOPPED(wait_status) }
    }
}

pub fn trace_me() -> () {
    ptrace(PTRACE_TRACEME, 0, ptr::null_mut(), ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_TRACEME");
}

pub fn syscall(pid: pid_t) -> () {
    let raw = ptrace(PTRACE_SYSCALL, pid, ptr::null_mut(), ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_SYSCALL");
}


pub fn get_rax(pid: pid_t) -> InferiorPointer {
    let raw = ptrace(PTRACE_PEEKUSER,
                     pid,
                     user::regs::RAX as *mut c_void,
                     ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKUSER");
    InferiorPointer(raw as u64)
}

pub fn get_rdx(pid: pid_t) -> InferiorPointer {
    let raw = ptrace(PTRACE_PEEKUSER,
                     pid,
                     user::regs::RDX as *mut c_void,
                     ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKUSER");
    InferiorPointer(raw as u64)
}


pub fn get_rsi(pid: pid_t) -> InferiorPointer {
    let raw = ptrace(PTRACE_PEEKUSER,
                     pid,
                     user::regs::RSI as *mut c_void,
                     ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKUSER");
    InferiorPointer(raw as u64)
}


pub fn get_rdi(pid: pid_t) -> InferiorPointer {
    let raw = ptrace(PTRACE_PEEKUSER,
                     pid,
                     user::regs::RDI as *mut c_void,
                     ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKUSER");
    InferiorPointer(raw as u64)
}

pub fn get_instruction_pointer(pid: pid_t) -> InferiorPointer {
    let raw = ptrace(PTRACE_PEEKUSER,
                     pid,
                     user::regs::RIP as *mut c_void,
                     ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKUSER");
    InferiorPointer(raw as u64)
}

// pub fn set_instruction_pointer(pid: pid_t, ip: InferiorPointer) -> () {
// ptrace(PTRACE_POKEUSER, pid, user::regs::RIP as * mut c_void, ip.as_voidptr())
// .ok()
// .expect("Failed PTRACE_POKEUSER");
// }
//
// pub fn cont(pid: pid_t) -> () {
// ptrace(PTRACE_CONT, pid, ptr::null_mut(), ptr::null_mut())
// .ok()
// .expect("Failed PTRACE_CONTINUE");
// }
//

pub fn peek_text(pid: pid_t, address: InferiorPointer) -> i64 {
    ptrace(PTRACE_PEEKTEXT, pid, address.as_voidptr(), ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKTEXT")
}

pub fn peek_text_bytes(pid: pid_t, address: InferiorPointer) -> [u8; 8] {
    let filename = ptrace(PTRACE_PEEKTEXT, pid, address.as_voidptr(), ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_PEEKTEXT");
    // let mut raw_filename : Vec<u8> = Vec::new();
    let mut raw_filename: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    let mut i = 0;
    while i < 8 {
        // mem::size_of::<libc::c_void>() {
        // raw_filename.push(((filename >> (i*8)) & 0xFF) as u8);
        raw_filename[i] = (((filename >> (i * 8)) & 0xFF) as u8);
        i += 1;
    }

    raw_filename
}

// pub fn peek_text_str(pid: pid_t, address: InferiorPointer) -> &str {
// let filename = ptrace(PTRACE_PEEKTEXT, pid, address.as_voidptr(), ptr::null_mut())
// .ok()
// .expect("Failed PTRACE_PEEKTEXT");
// let mut raw_filename : Vec<u8> = Vec::new();
// let mut raw_filename : [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
//
// let mut i = 0;
// while i < 8 { // mem::size_of::<libc::c_void>() {
// raw_filename.push(((filename >> (i*8)) & 0xFF) as u8);
// raw_filename[i] = (((filename >> (i*8)) & 0xFF) as u8);
// i+=1;
// }
//
// let s = match str::from_utf8(&raw_filename) {
// Ok(v) => v,
// Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
// };
//
// s.clone()
// }
//

pub fn poke_text(pid: pid_t, address: InferiorPointer, value: i64) -> () {
    ptrace(PTRACE_POKETEXT,
           pid,
           address.as_voidptr(),
           value as *mut c_void)
        .ok()
        .expect("Failed PTRACE_POKETEXT");
}

pub fn single_step(pid: pid_t) -> () {
    ptrace(PTRACE_SINGLESTEP, pid, ptr::null_mut(), ptr::null_mut())
        .ok()
        .expect("Failed PTRACE_SINGLESTEP");
}
