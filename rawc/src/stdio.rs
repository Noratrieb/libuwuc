use core::ffi::{c_char, c_int};

use libuwuc::{
    io::{stream::FileStream, traits::WriteCounter, STDERR, STDIN, STDOUT},
    utils::SharedThinCstr,
};

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> i32 {
    libuwuc::io::puts(s)
}

// PRINTF:

#[no_mangle]
pub unsafe extern "C" fn __printf_chk(_flag: c_int, format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(stdout, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    match result {
        Ok(()) => sink.1 as _,
        Err(err) => err,
    }
}

#[no_mangle]
pub unsafe extern "C" fn printf(format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(stdout, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    match result {
        Ok(()) => sink.1 as _,
        Err(err) => err,
    }
}


#[no_mangle]
pub unsafe extern "C" fn __fprintf_chk(file: &FileStream, _flag: c_int, format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(file, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    match result {
        Ok(()) => sink.1 as _,
        Err(err) => err,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fprintf(file: &FileStream, format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(file, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    match result {
        Ok(()) => sink.1 as _,
        Err(err) => err,
    }
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
