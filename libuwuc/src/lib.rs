#![no_std]
#![warn(unreachable_pub)]

#![allow(clippy::missing_safety_doc)]

#[cfg(test)]
extern crate std;

pub mod alloc;
pub mod env;
pub mod io;
pub mod mem;
pub mod start;
mod stubs;
mod sys;
pub mod utils;

pub mod syscall {
    pub use crate::sys::syscall::*;
}
