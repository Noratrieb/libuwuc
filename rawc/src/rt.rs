use core::ffi::c_uint;

use libuwuc::utils::SharedThinCstr;

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

#[no_mangle]
pub unsafe extern "C" fn __assert_fail(
    assertion: *const u8,
    file: *const u8,
    line: c_uint,
    function: *const u8,
) -> ! {
    libuwuc::misc::assert_failed(
        SharedThinCstr::from_raw(assertion),
        SharedThinCstr::from_raw(file),
        line,
        SharedThinCstr::from_nullable(function),
    )
}
