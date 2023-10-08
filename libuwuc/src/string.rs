use crate::utils::CStrRef;

pub unsafe fn strtok(str: *mut u8, delim: CStrRef<'_>, saveptr: *mut *mut u8) -> *const u8 {
    if !str.is_null() {
        saveptr.write(str);
    }
    let start = saveptr.read();

    let mut end = saveptr.read();
    while end.read() != 0 || !delim.into_iter().any(|c| c == end.read()) {
        end = end.add(1);
    }

    end.write(0);

    if start == end {
        core::ptr::null()
    } else {
        start
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::borrow::ToOwned;
    use std::vec::Vec;

    use crate::{cstr, utils::CStrRef};

    fn test_strtok(string: &str, delim: CStrRef<'_>, expected: &[CStrRef<'_>]) {
        unsafe {
            let mut str = string.to_owned().into_bytes();
            str.push(0);

            let mut saveptr = core::ptr::null_mut();

            let mut out = Vec::new();
            loop {
                let s = super::strtok(str.as_mut_ptr(), delim, &mut saveptr);
                if s.is_null() {
                    break;
                }
                out.push(CStrRef::from_raw(s));
            }

            assert_eq!(out, expected);
        }
    }

    #[test]
    fn strtok_manpage_example() {
        test_strtok(
            "a/bbb///cc;xxxx:yyy:",
            cstr!(":;"),
            &[cstr!("a/bbbb///cc"), cstr!("xxx"), cstr!("yyy")],
        );
    }
}
