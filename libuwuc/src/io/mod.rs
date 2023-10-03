pub mod fd;
pub mod stream;
pub mod traits;

pub use traits::IoWrite;

use core::ffi::c_char;

use crate::{error::Error, sys::syscall};

pub const STDIN: i32 = 0;
pub const STDOUT: i32 = 1;
pub const STDERR: i32 = 2;

pub const EOF: i32 = -1;

#[doc(hidden)]
pub struct Printer(pub i32);

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { write_all(self.0, s.as_bytes()).map_err(|_| core::fmt::Error) }
    }
}

#[macro_export]
macro_rules! println {
    ($($tt:tt)*) => {
        {
            use ::core::fmt::Write;
            ::core::writeln!($crate::io::Printer($crate::io::STDOUT), $($tt)*).unwrap();
        }
    };
}
pub use println;

pub unsafe fn sys_write(fd: i32, buf: &[u8]) -> Result<usize, Error> {
    let result = syscall::syscall!(syscall::SYS_WRITE, fd, buf.as_ptr(), buf.len()) as i64;
    if result < 0 {
        Err(Error(result as _))
    } else {
        Ok(result as _)
    }
}

pub unsafe fn write_all(fd: i32, mut buf: &[u8]) -> Result<(), Error> {
    while !buf.is_empty() {
        let result = sys_write(fd, buf)?;
        buf = &buf[result..];
    }
    Ok(())
}

pub unsafe fn puts(s: *const c_char) -> i32 {
    let len = crate::mem::strlen(s as _);
    let result = write_all(STDOUT, core::slice::from_raw_parts(s as _, len));
    if result.is_err() {
        return EOF;
    }
    let result = write_all(STDOUT, &[b'\n']);
    if result.is_err() {
        return EOF;
    }
    1
}
