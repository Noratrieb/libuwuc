use crate::utils::SharedThinCstr;

#[derive(Debug, PartialEq)]
pub enum OpenMode {
    R,
    RP,
    W,
    WP,
    A,
    AP,
}

impl OpenMode {
    pub fn parse(str: SharedThinCstr<'_>) -> Result<OpenMode, &'static str> {
        let mut buf = [0; 2];
        let mut i = 0;

        for c in str.into_iter().filter(|&c| c != b'c') {
            if i > 1 {
                return Err("too many characters for mode");
            }
            buf[i] = c;
            i += 1;
        }

        Ok(match &buf {
            b"r\0" => OpenMode::R,
            b"r+" => OpenMode::RP,
            b"w\0" => OpenMode::W,
            b"w+" => OpenMode::WP,
            b"a\0" => OpenMode::A,
            b"a+" => OpenMode::AP,
            _ => return Err("invalid mode"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::cstr;

    use super::OpenMode;

    #[test]
    fn parse_modes() {
        use OpenMode::*;
        let modes = [
            (cstr!("r"), R),
            (cstr!("r+"), RP),
            (cstr!("w"), W),
            (cstr!("w+"), WP),
            (cstr!("a"), A),
            (cstr!("a+"), AP),
        ];

        for (str, expected) in modes {
            let mode = OpenMode::parse(str).unwrap();
            assert_eq!(mode, expected);
        }
    }

    #[test]
    fn invalid_modes() {
        let modes = [cstr!("meow"), cstr!(""), cstr!("r-"), cstr!("r++")];

        for str in modes {
            OpenMode::parse(str).expect_err("expected failing parse");
        }
    }
}
