use crate::{io::fd, utils::SharedThinCstr};

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
    pub fn flags(self) -> i32 {
        match self {
            OpenMode::R => fd::O_RDONLY,
            OpenMode::RP => fd::O_RDWR,
            OpenMode::W => fd::O_WRONLY | fd::O_CREAT | fd::O_TRUNC,
            OpenMode::WP => fd::O_RDWR | fd::O_CREAT | fd::O_TRUNC,
            OpenMode::A => fd::O_WRONLY | fd::O_CREAT | fd::O_APPEND,
            OpenMode::AP => fd::O_RDWR | fd::O_CREAT | fd::O_APPEND,
        }
    }

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
