use core::ffi::{c_int, c_long};

use crate::utils::SharedThinCstr;

pub unsafe fn parse_long<'a>(
    str: SharedThinCstr<'a>,
    endptr: *mut SharedThinCstr<'a>,
    base: c_int,
) -> c_long {
    let mut cur = str;
    while cur
        .first()
        .map(|c| super::is_space(c as _) == 1)
        .unwrap_or(false)
    {}

    0
}
