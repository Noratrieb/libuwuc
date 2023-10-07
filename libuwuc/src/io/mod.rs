pub mod fd;
pub mod stream;
pub mod traits;

pub use traits::IoWrite;

use core::ffi::c_char;

use crate::{
    error::{Error, SyscallResultExt},
    sys::syscall,
};

pub const STDIN: Fd = Fd(0);
pub const STDOUT: Fd = Fd(1);
pub const STDERR: Fd = Fd(2);

pub const EOF: i32 = -1;

#[doc(hidden)]
pub struct Printer(pub Fd);

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

use self::fd::Fd;

pub unsafe fn sys_read(fd: Fd, buf: &mut [u8]) -> Result<usize, Error> {
    syscall::syscall!(syscall::SYS_READ, fd.0, buf.as_ptr(), buf.len()).syscall_resultify()
}

pub unsafe fn sys_write(fd: Fd, buf: &[u8]) -> Result<usize, Error> {
    syscall::syscall!(syscall::SYS_WRITE, fd.0, buf.as_ptr(), buf.len()).syscall_resultify()
}

pub unsafe fn write_all(fd: Fd, mut buf: &[u8]) -> Result<(), Error> {
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
