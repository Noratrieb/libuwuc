use core::{cell::UnsafeCell, fmt::Debug, ptr::addr_of};

use crate::{io::fd::Fd, utils::SyncUnsafeCell};

mod errno;
pub use errno::*;

// Todo: This should be a thread local once we have threads.
static ERRNO: SyncUnsafeCell<i32> = SyncUnsafeCell(UnsafeCell::new(0));

pub fn errno_location() -> *const i32 {
    addr_of!(ERRNO).cast()
}

pub fn errno() -> i32 {
    unsafe { *ERRNO.0.get() }
}

pub fn set_errno(errno: i32) {
    unsafe { ERRNO.0.get().write(errno) }
}

pub fn strerror(errnum: Error) -> Option<&'static str> {
    errnum.simple_str()
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Error(pub i32);

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.simple_str().unwrap_or("<error: invalid error>"))
    }
}

pub trait SyscallResultExt: Sized {
    fn syscall_resultify<N: FromSyscall>(self) -> Result<N, Error>;
}

pub trait FromSyscall {
    fn from_isize(n: isize) -> Self;
}

macro_rules! usize_int_impl {
    ($($ty:ty)*) => {
        $(impl FromSyscall for $ty {
            fn from_isize(n: isize) -> Self {
                n as _
            }
        })*
    };
}

usize_int_impl!(isize usize u64 i64 u32 i32);

impl FromSyscall for Fd {
    fn from_isize(n: isize) -> Self {
        Self(n as _)
    }
}

impl SyscallResultExt for isize {
    fn syscall_resultify<N: FromSyscall>(self) -> Result<N, Error> {
        if (-4096..0).contains(&self) {
            Err(Error(-self as i32))
        } else {
            Ok(N::from_isize(self))
        }
    }
}

pub trait IntoOkOrErrno {
    type Int: ReturnInt;
    fn into_ok_or_errno(self) -> Self::Int;
}

impl<T: ReturnInt> IntoOkOrErrno for Result<T, Error> {
    type Int = T;
    fn into_ok_or_errno(self) -> Self::Int {
        self.unwrap_or_else(|err| {
            set_errno(err.0);
            T::negative_one()
        })
    }
}

pub trait ReturnInt {
    fn negative_one() -> Self;
}

macro_rules! return_int_impl_s {
    ($($ty:ty)*) => {
        $(impl ReturnInt for $ty {
            fn negative_one() -> Self {
                -1
            }
        })*
    };
}

return_int_impl_s!(i8 i16 i32 i64 isize);

impl ReturnInt for Fd {
    fn negative_one() -> Self {
        Self(-1)
    }
}
