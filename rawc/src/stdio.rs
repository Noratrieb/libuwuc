use core::ffi::{c_char, c_int};

use libuwuc::{
    error::IntoOkOrErrno,
    io::{stream::FileStream, traits::WriteCounter, STDERR, STDIN, STDOUT},
    utils::SharedThinCstr,
};

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> i32 {
    libuwuc::io::puts(s)
}

#[no_mangle]
pub unsafe extern "C" fn putchar(char: i32) -> i32 {
    libuwuc::io::stream::fputc(char, stdout)
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

    result.map(|()| sink.1 as i32).into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn printf(format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(stdout, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    result.map(|()| sink.1 as i32).into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn __fprintf_chk(
    _flag: c_int,
    file: &FileStream,
    format: *const u8,
    mut args: ...
) -> c_int {
    let mut sink = WriteCounter(file, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    result.map(|()| sink.1 as i32).into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn fprintf(file: &FileStream, format: *const u8, mut args: ...) -> c_int {
    let mut sink = WriteCounter(file, 0);

    let result = libuwuc::fmt::printf::printf_generic(
        &mut sink,
        SharedThinCstr::from_raw(format),
        args.as_va_list(),
    );

    result.map(|()| sink.1 as i32).into_ok_or_errno()
}

// STREAMS:

#[no_mangle]
pub static stdin: &FileStream = &FileStream::from_raw_fd(STDIN);
#[no_mangle]
pub static stdout: &FileStream = &FileStream::from_raw_fd(STDOUT);
#[no_mangle]
pub static stderr: &FileStream = &FileStream::from_raw_fd(STDERR);

#[no_mangle]
pub unsafe extern "C" fn fopen<'a>(
    pathname: SharedThinCstr<'_>,
    mode: SharedThinCstr<'_>,
) -> Option<&'a FileStream> {
    libuwuc::io::stream::fopen(pathname, mode)
        .map_err(|err| libuwuc::error::set_errno(err.0))
        .ok()
}

#[no_mangle]
pub unsafe extern "C" fn fgetc(_stream: *mut FileStream) -> c_int {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn ungetc(_c: c_int, _stream: *mut FileStream) -> c_int {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut FileStream) -> c_int {
    libuwuc::io::stream::fputc(c, &*stream)
}

#[no_mangle]
pub unsafe extern "C" fn fread(
    _ptr: *const u8,
    _size: usize,
    _nmemb: usize,
    _stream: *mut FileStream,
) -> usize {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn fwrite(
    ptr: *const u8,
    size: usize,
    nmemb: usize,
    stream: &FileStream,
) -> usize {
    libuwuc::io::stream::fwrite(ptr, size, nmemb, stream)
}
