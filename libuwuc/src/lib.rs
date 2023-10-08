#![no_std]
#![feature(c_variadic)]
#![feature(thread_local)]
#![warn(unreachable_pub)]
#![warn(rust_2018_idioms)]
#![allow(clippy::missing_safety_doc)]

#[cfg(test)]
extern crate std;

pub mod alloc;
pub mod env;
pub mod error;
pub mod fmt;
pub mod io;
pub mod mem;
pub mod misc;
pub mod start;
pub mod string;
mod stubs;
mod sys;
pub mod utils;

pub mod syscall {
    pub use crate::sys::syscall::*;
}
