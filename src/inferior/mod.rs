// (partly) based on http://system.joekain.com/2015/09/07/refactoring-ptrace-code.html

use libc::pid_t;
use libc::c_void;

use std::fmt;
use std::ops::{Add, Sub};

pub type TrapInferior = pid_t;

#[derive(Copy, Clone)]
pub enum InferiorState {
    Running,
    Stopped,
    SingleStepping,
}

#[derive(Copy, Clone)]
pub struct Inferior {
    pub pid: pid_t,
    pub state: InferiorState,
}

#[derive(Copy, Clone)]
pub struct InferiorPointer(pub u64);

impl InferiorPointer {
    pub fn as_voidptr(&self) -> *mut c_void {
        let &InferiorPointer(u) = self;
        u as *mut c_void
    }
}

impl fmt::LowerHex for InferiorPointer {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        try!(fmtr.write_fmt(format_args!("{:02x}", self.as_voidptr() as u64)));

        Ok(())
    }
}

impl Add<i64> for InferiorPointer {
    type Output = InferiorPointer;
    fn add(self, rhs: i64) -> InferiorPointer {
        let InferiorPointer(u) = self;
        if rhs >= 0 {
            InferiorPointer(u + rhs as u64)
        } else {
            InferiorPointer(u - rhs as u64)
        }
    }
}

impl Sub<i64> for InferiorPointer {
    type Output = InferiorPointer;
    fn sub(self, rhs: i64) -> InferiorPointer {
        let InferiorPointer(u) = self;
        if rhs >= 0 {
            InferiorPointer(u - rhs as u64)
        } else {
            InferiorPointer(u + rhs as u64)
        }
    }
}
