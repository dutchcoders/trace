// For Keystone Engine. AUTO-GENERATED FILE, DO NOT EDIT [keystone_const.rs]
//
extern crate std;
extern crate libc;

use libc::pid_t;
use nix::sys::ptrace::*;
use nix::sys::ptrace::ptrace::*;
use std::ptr;
use libc::c_void;
use std::str;

use std::slice;

use std::mem;

use libc::wait;
use libc::c_int;
use libc::WIFSTOPPED;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::ffi::CString;

pub mod elftools_const;
