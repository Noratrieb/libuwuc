use libuwuc::{error::IntoOkOrErrno, utils::CStrRef, io::fd::Fd};

#[no_mangle]
pub unsafe extern "C" fn open(path: CStrRef<'_>, flags: i32) -> Fd {
    libuwuc::io::fd::open(path, flags).into_ok_or_errno()
}
