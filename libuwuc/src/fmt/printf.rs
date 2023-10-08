use core::ffi::VaList;

use crate::{io::IoWrite, utils::CStrRef, error::Error};

pub unsafe fn printf_generic(
    mut sink: impl IoWrite,
    format: CStrRef<'_>,
    mut args: VaList<'_, '_>,
) -> Result<(), Error> {
    let mut chars = format.into_iter();

    while let Some(c) = chars.next() {
        if c == b'%' {
            let Some(c) = chars.next() else {
                return Err(Error::INVAL);
            };
            // todo: do this properly
            match c {
                b'c' => {
                    let char = args.arg::<u8>();
                    write!(sink, "{}", char as char)?;
                }
                b'l' => {
                    let Some(c) = chars.next() else {
                        return Err(Error::INVAL);
                    };
                    if c != b'd' {
                        todo!();
                    }
                    let long = args.arg::<i64>();
                    write!(sink, "{}", long)?;
                }
                _ => todo!(),
            };
        } else {
            sink.write_byte(c)?;
        }
    }

    Ok(())
}

#[cfg(test)]
#[allow(improper_ctypes_definitions)]
mod tests {
    extern crate std;
    use std::string::String;
    use std::vec::Vec;

    use crate::utils::{cstr, CStrRef};

    use super::printf_generic;

    unsafe extern "C" fn test_printf(expected: &str, fmt: CStrRef<'_>, mut args: ...) {
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

    #[test]
    #[cfg_attr(miri, ignore = "variadic")]
    fn char() {
        unsafe { test_printf("a", cstr!("%c"), b'a' as u64) }
    }

    #[test]
    #[cfg_attr(miri, ignore = "variadic")]
    fn long() {
        unsafe { test_printf("234", cstr!("%ld"), 234_u64) }
    }
}
