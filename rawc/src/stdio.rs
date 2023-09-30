use core::ffi::{c_char, c_int};

use libuwuc::io::{stream::FileStream, STDERR, STDIN, STDOUT};

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> i32 {
    libuwuc::io::puts(s)
}

// STREAMS:

#[no_mangle]
pub static stdin: &FileStream = &FileStream::from_raw_fd(STDIN);
#[no_mangle]
pub static stdout: &FileStream = &FileStream::from_raw_fd(STDOUT);
#[no_mangle]
pub static stderr: &FileStream = &FileStream::from_raw_fd(STDERR);

#[no_mangle]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut FileStream) -> c_int {
    libuwuc::io::stream::fputc(c as u8, &*stream)
}

#[no_mangle]
pub unsafe extern "C" fn fwrite(
    ptr: *const u8,
    size: usize,
    nitems: usize,
    stream: &FileStream,
) -> usize {
    libuwuc::io::stream::fwrite(ptr, size, nitems, stream)
}
