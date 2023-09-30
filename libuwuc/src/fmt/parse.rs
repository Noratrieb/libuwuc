use core::ffi::{c_int, c_long};

use crate::{
    error::{set_errno, EINVAL},
    utils::SharedThinCstr,
};

pub fn parse_long<'a>(
    str: SharedThinCstr<'a>,
    endptr: Option<&mut Option<SharedThinCstr<'a>>>,
    base: c_int,
) -> c_long {
    if base != 10 {
        todo!();
    }
    let mut chars = str.into_iter().enumerate().peekable();

    while chars
        .peek()
        .map(|&(_, c)| super::is_space(c as _))
        .unwrap_or(false)
    {
        chars.next();
    }

    let mut negate = false;
    let mut acc: i64 = 0;

    let write_end = |pos: usize| {
        if let Some(endptr) = endptr {
            *endptr = Some(unsafe { str.add(pos) });
        }
    };

    let mut last_pos;

    let c = chars.peek();
    match c {
        Some((pos, b'+')) => {
            last_pos = *pos;
            chars.next();
        }
        Some((pos, b'-')) => {
            last_pos = *pos;
            chars.next();
            negate = true
        }
        Some((_, c)) if !c.is_ascii_digit() => {
            write_end(0);
            set_errno(EINVAL);
            return 0;
        }
        None => {
            write_end(0);
            set_errno(EINVAL);
            return 0;
        }
        Some((pos, _)) => {
            last_pos = *pos;
        }
    }

    loop {
        match chars.next() {
            Some((pos, c)) if c.is_ascii_digit() => {
                last_pos = pos;
                let n = c - b'0';
                acc *= 10;
                if negate {
                    acc -= n as i64;
                } else {
                    acc += n as i64;
                }
            }
            Some((pos, _)) => {
                write_end(pos);
                break;
            }
            None => {
                write_end(last_pos + 1);
                break;
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use crate::{cstr, utils::SharedThinCstr};

    fn test_strtol(str: SharedThinCstr<'_>, expected: i64, base: i32, parsed_len: usize) {
        let mut end = None;
        let result = super::parse_long(str, Some(&mut end), base);
        assert_eq!(result, expected);
        let end = end.unwrap();
        let read = end.as_raw() as usize - str.as_raw() as usize;
        assert_eq!(read, parsed_len);
    }

    #[test]
    fn l_zero() {
        test_strtol(cstr!("0"), 0, 10, 1);
    }

    #[test]
    fn l_ten() {
        test_strtol(cstr!("10"), 10, 10, 2);
    }

    #[test]
    fn l_eleven() {
        test_strtol(cstr!("11"), 11, 10, 2);
    }

    #[test]
    fn negative_zero() {
        test_strtol(cstr!("-0"), 0, 10, 2);
    }

    #[test]
    fn negative_ten() {
        test_strtol(cstr!("-10"), -10, 10, 3);
    }

    #[test]
    fn negative_eleven() {
        test_strtol(cstr!("-11"), -11, 10, 3);
    }

    #[test]
    fn l_leading_whitespace() {
        test_strtol(cstr!("\t 1"), 1, 10, 3);
    }

    #[test]
    fn l_trailing_garbage_one() {
        test_strtol(cstr!("3uwu"), 3, 10, 1);
    }

    #[test]
    fn l_trailing_garbage_two() {
        test_strtol(cstr!("32uwu"), 32, 10, 2);
    }

    #[test]
    fn l_trailing_garbage_many() {
        test_strtol(cstr!("12345uwu"), 12345, 10, 5);
    }

    #[test]
    fn l_long_max() {
        test_strtol(cstr!("9223372036854775807"), i64::MAX, 10, 19);
    }

    #[test]
    fn l_long_min() {
        test_strtol(cstr!("-9223372036854775808"), i64::MIN, 10, 20);
    }
}
