#![cfg_attr(not(test), no_std)]
#![no_main]

use core::ffi::c_char;

use libuwuc::println;

extern crate libuwuc;

#[panic_handler]
#[cfg(not(test))]
fn handler(_arg: &core::panic::PanicInfo) -> ! {
    libuwuc::io::println!("panic!");
    libuwuc::start::exit(1);
}

#[no_mangle]
extern "C" fn main(_argc: i32, _argv: *const *const c_char) -> i32 {
    println!("Hello, world!");
    0
}
