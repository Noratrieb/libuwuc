use core::ffi::c_int;

pub mod parse;
pub mod printf;

pub fn is_space(c: c_int) -> bool {
    // todo: is this correct?
    char::from_u32(c as _)
        .map(char::is_whitespace)
        .unwrap_or_default()
}
