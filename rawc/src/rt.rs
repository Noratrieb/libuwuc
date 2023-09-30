#[no_mangle]
pub extern "C" fn __stack_chk_fail() -> ! {
    unsafe {
        let _ = libuwuc::io::write_all(libuwuc::io::STDERR, b"error: stack overflow");
        libuwuc::start::abort();
    }
}

#[no_mangle]
pub extern "C" fn __errno_location() -> *const i32 {
    libuwuc::error::errno_location()
}