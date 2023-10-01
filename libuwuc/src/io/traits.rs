use core::fmt;

pub trait IoWrite {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i32>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), i32> {
        while !buf.is_empty() {
            let n = self.write(buf)?;
            buf = &buf[n..];
        }
        Ok(())
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), i32> {
        self.write_all(&[byte])
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> Result<(), i32> {
        // Create a shim which translates a Write to a fmt::Write and saves
        // off I/O errors. instead of discarding them
        struct Adapter<'a, T: ?Sized> {
            inner: &'a mut T,
            error: Result<(), i32>,
        }

        impl<T: IoWrite + ?Sized> fmt::Write for Adapter<'_, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                match self.inner.write_all(s.as_bytes()) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.error = Err(e);
                        Err(fmt::Error)
                    }
                }
            }
        }

        let mut output = Adapter {
            inner: self,
            error: Ok(()),
        };
        match fmt::write(&mut output, fmt) {
            Ok(()) => Ok(()),
            Err(..) => {
                // check if the error came from the underlying `Write` or not
                if output.error.is_err() {
                    output.error
                } else {
                    Err(-1)
                }
            }
        }
    }
}

impl<W: IoWrite> IoWrite for &mut W {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        W::write(self, buf)
    }
}

pub struct WriteCounter<W>(pub W, pub usize);

impl<W: IoWrite> IoWrite for WriteCounter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        let n = self.0.write(buf)?;
        self.1 += n;
        Ok(n)
    }
}

#[cfg(test)]
mod test_impls {
    extern crate std;

    use super::IoWrite;
    use std::vec::Vec;

    impl IoWrite for Vec<u8> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
            self.extend(buf);
            Ok(buf.len())
        }
    }
}
