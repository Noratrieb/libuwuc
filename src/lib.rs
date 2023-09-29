#![no_std]
#![warn(unreachable_pub)]

#[cfg(test)]
extern crate std;

mod basic_mem;
pub mod env;
pub mod io;
pub mod start;
mod stubs;
mod sys;
pub mod utils;

pub mod syscall {
    pub use crate::sys::syscall::*;
}
