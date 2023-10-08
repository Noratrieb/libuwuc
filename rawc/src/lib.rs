#![no_std]
#![feature(c_variadic)]
#![feature(panic_info_message)]
#![deny(clippy::no_mangle_with_rust_abi)]
#![warn(rust_2018_idioms)]

mod fcntl;
mod rt;
mod stdio;
mod stdlib;
mod string;
mod unistd;

// libcore seems to require this symbol, even though it's unused.
#[no_mangle]
#[allow(clippy::no_mangle_with_rust_abi)]
fn rust_eh_personality() {
    unsafe {
        libuwuc::trap!();
    }
}

#[panic_handler]
#[cfg(not(test))]
fn handler(arg: &core::panic::PanicInfo<'_>) -> ! {
    let args = format_args!("<no message>");
    let payload = arg.message().unwrap_or(&args);
    libuwuc::io::println!("panicked: {payload}");
    if let Some(loc) = arg.location() {
        libuwuc::io::println!("  at {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    libuwuc::start::sys_exit(1);
}
