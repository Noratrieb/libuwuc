#![cfg_attr(not(test), no_std)]
#![no_main]

use core::ffi::c_char;

extern crate libuwuc;

#[panic_handler]
#[cfg(not(test))]
fn handler(_arg: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn main(_argc: i32, _argv: *const *const c_char) -> i32 {
    0
}
