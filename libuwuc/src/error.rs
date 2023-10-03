use core::{cell::UnsafeCell, ptr::addr_of};

use crate::utils::SyncUnsafeCell;

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

pub const EPERM: i32 = 1; /* Operation not permitted */
pub const ENOENT: i32 = 2; /* No such file or directory */
pub const ESRCH: i32 = 3; /* No such process */
pub const EINTR: i32 = 4; /* Interrupted system call */
pub const EIO: i32 = 5; /* I/O error */
pub const ENXIO: i32 = 6; /* No such device or address */
pub const E2BIG: i32 = 7; /* Argument list too long */
pub const ENOEXEC: i32 = 8; /* Exec format error */
pub const EBADF: i32 = 9; /* Bad file number */
pub const ECHILD: i32 = 10; /* No child processes */
pub const EAGAIN: i32 = 11; /* Try again */
pub const ENOMEM: i32 = 12; /* Out of memory */
pub const EACCES: i32 = 13; /* Permission denied */
pub const EFAULT: i32 = 14; /* Bad address */
pub const ENOTBLK: i32 = 15; /* Block device required */
pub const EBUSY: i32 = 16; /* Device or resource busy */
pub const EEXIST: i32 = 17; /* File exists */
pub const EXDEV: i32 = 18; /* Cross-device link */
pub const ENODEV: i32 = 19; /* No such device */
pub const ENOTDIR: i32 = 20; /* Not a directory */
pub const EISDIR: i32 = 21; /* Is a directory */
pub const EINVAL: i32 = 22; /* Invalid argument */
pub const ENFILE: i32 = 23; /* File table overflow */
pub const EMFILE: i32 = 24; /* Too many open files */
pub const ENOTTY: i32 = 25; /* Not a typewriter */
pub const ETXTBSY: i32 = 26; /* Text file busy */
pub const EFBIG: i32 = 27; /* File too large */
pub const ENOSPC: i32 = 28; /* No space left on device */
pub const ESPIPE: i32 = 29; /* Illegal seek */
pub const EROFS: i32 = 30; /* Read-only file system */
pub const EMLINK: i32 = 31; /* Too many links */
pub const EPIPE: i32 = 32; /* Broken pipe */
pub const EDOM: i32 = 33; /* Math argument out of domain of func */
pub const ERANGE: i32 = 34; /* Math result not representable */
