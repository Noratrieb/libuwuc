#![feature(panic_info_message)]
#![cfg_attr(not(test), no_std)]
#![no_main]

use core::ffi::c_char;

use libuwuc::{println, utils::SharedThinCstr};

extern crate libuwuc;

#[panic_handler]
#[cfg(not(test))]
fn handler(arg: &core::panic::PanicInfo) -> ! {
    let args = format_args!("<no message>");
    let payload = arg.message().unwrap_or(&args);
    libuwuc::io::println!("panicked: {payload}");
    if let Some(loc) = arg.location() {
        libuwuc::io::println!("  at {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    libuwuc::start::exit(1);
}

#[no_mangle]
extern "C" fn main(_argc: i32, _argv: *const *const c_char) -> i32 {
    println!("Hello, world!");
    let pwd = libuwuc::env::getenv(SharedThinCstr::from_array(b"PWD\0"));
    println!("PWD={pwd:?}");
    0
}

// libcore seems to require this symbol, even though it's unused.
#[no_mangle]
fn rust_eh_personality() {
    unsafe {
        libuwuc::trap!();
    }
}
