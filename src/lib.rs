#![no_std]
#![warn(unreachable_pub)]

mod basic_mem;
pub mod io;
pub mod start;
mod stubs;
mod sys;

pub mod syscall {
    pub use crate::sys::syscall::*;
}
