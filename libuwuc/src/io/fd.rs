use crate::{
    error::{Error, SyscallResultExt},
    utils::SharedThinCstr,
};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Fd(pub i32);

pub fn open(arg: SharedThinCstr<'_>, flags: i32) -> Result<Fd, Error> {
    sys_open(arg, flags)
}

pub fn sys_open(arg: SharedThinCstr<'_>, flags: i32) -> Result<Fd, Error> {
    unsafe {
        crate::syscall::syscall!(crate::syscall::SYS_OPEN, arg.as_raw(), flags).syscall_resultify()
    }
}

pub const O_ACCMODE: i32 = 0o0003;
pub const O_RDONLY: i32 = 0o00;
pub const O_WRONLY: i32 = 0o01;
pub const O_RDWR: i32 = 0o02;
pub const O_CREAT: i32 = 0o0100; /* Not fcntl.  */
pub const O_EXCL: i32 = 0o0200; /* Not fcntl.  */
pub const O_NOCTTY: i32 = 0o0400; /* Not fcntl.  */
pub const O_TRUNC: i32 = 0o01000; /* Not fcntl.  */
pub const O_APPEND: i32 = 0o02000;
pub const O_NONBLOCK: i32 = 0o04000;
pub const O_NDELAY: i32 = O_NONBLOCK;
pub const O_SYNC: i32 = 0o04010000;
pub const O_FSYNC: i32 = O_SYNC;
pub const O_ASYNC: i32 = 0o020000;

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4;
