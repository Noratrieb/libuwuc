#![feature(panic_info_message)]
#![cfg_attr(not(test), no_std)]
#![no_main]

use core::ffi::c_char;

use libuwuc::println;

extern crate rawc;

#[no_mangle]
extern "C" fn main(_argc: i32, _argv: *const *const c_char) -> i32 {
    println!("Hello, world!");
    let pwd = libuwuc::env::getenv(libuwuc::cstr!("PWD"));
    println!("PWD={pwd:?}");
    0
}
