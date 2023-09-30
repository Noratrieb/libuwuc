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
