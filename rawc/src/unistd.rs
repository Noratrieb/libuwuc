use libuwuc::error::IntoOkOrErrno;
use libuwuc::io::fd::Fd;

#[no_mangle]
pub unsafe extern "C" fn read(fd: Fd, buf: *mut u8, count: usize) -> isize {
    libuwuc::io::sys_read(fd, core::slice::from_raw_parts_mut(buf, count))
        .map(|n| n as isize)
        .into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn write(fd: Fd, buf: *const u8, count: usize) -> isize {
    libuwuc::io::sys_write(fd, core::slice::from_raw_parts(buf, count))
        .map(|n| n as isize)
        .into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn lseek(fd: Fd, offset: i64, whence: i32) -> i64 {
    libuwuc::io::sys_lseek(fd, offset, whence).into_ok_or_errno()
}
