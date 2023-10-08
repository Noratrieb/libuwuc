use libuwuc::{error::IntoOkOrErrno, utils::SharedThinCstr, io::fd::Fd};

#[no_mangle]
pub unsafe extern "C" fn open(path: SharedThinCstr<'_>, flags: i32) -> Fd {
    libuwuc::io::fd::open(path, flags).into_ok_or_errno()
}
