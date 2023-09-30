use core::ffi::VaList;

use crate::{io::IoWrite, utils::SharedThinCstr};

pub unsafe fn printf_generic(
    mut sink: impl IoWrite,
    format: SharedThinCstr<'_>,
    _args: VaList<'_, '_>,
) -> Result<(), i32> {
    let mut chars = format.into_iter();

    while let Some(c) = chars.next() {
        if c == b'%' {
            todo!();
        }

        sink.write_byte(c)?;
    }

    Ok(())
}

#[cfg(test)]
#[allow(improper_ctypes_definitions)]
mod tests {
    extern crate std;
    use std::string::String;
    use std::vec::Vec;

    use crate::utils::{cstr, SharedThinCstr};

    use super::printf_generic;

    unsafe extern "C" fn test_printf(expected: &str, fmt: SharedThinCstr<'_>, mut args: ...) {
        let mut sink = Vec::new();

        printf_generic(&mut sink, fmt, args.as_va_list()).unwrap();

        let result = String::from_utf8(sink).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    #[cfg_attr(miri, ignore = "variadic")]
    fn empty_format() {
        unsafe { test_printf("", cstr!("")) }
    }

    #[test]
    #[cfg_attr(miri, ignore = "variadic")]
    fn constant_string() {
        unsafe { test_printf("hello, world", cstr!("hello, world")) }
    }
}
