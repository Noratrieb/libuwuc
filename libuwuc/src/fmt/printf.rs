use core::ffi::VaList;

use crate::{io::IoWrite, utils::SharedThinCstr};

pub unsafe fn printf_generic(
    mut sink: impl IoWrite,
    format: SharedThinCstr,
    args: VaList<'_, '_>,
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

    unsafe extern "C" fn test_printf(expected: &str, fmt: SharedThinCstr, mut args: ...) {
        let mut sink = Vec::new();

        printf_generic(&mut sink, fmt, args.as_va_list()).unwrap();

        let result = String::from_utf8(sink).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn empty_format() {
        unsafe { test_printf("\0", cstr("\0")) }
    }

    #[test]
    fn constant_string() {
        unsafe { test_printf("hello, world\0", cstr("hello, world\0")) }
    }
}
