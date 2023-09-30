use core::ffi::c_char;

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> i32 {
    libuwuc::io::puts(s)
}
