pub mod file;

use crate::{
    alloc,
    error::Error,
    io::{fd, stream::file::OpenMode},
    utils::SharedThinCstr,
};

use super::{fd::Fd, IoWrite, EOF};

/// A `FILE`.
#[repr(C)]
pub struct FileStream {
    fd: Fd,
}

impl FileStream {
    pub const fn from_raw_fd(fd: Fd) -> Self {
        Self { fd }
    }

    fn write_byte(&self, c: u8) -> Result<(), Error> {
        unsafe { super::write_all(self.fd, &[c]) }
    }
}

impl IoWrite for &FileStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        unsafe { super::sys_write(self.fd.0, buf) }
    }
}

pub unsafe fn fopen<'a>(
    pathname: SharedThinCstr<'_>,
    mode: SharedThinCstr<'_>,
) -> Result<&'a FileStream, Error> {
    let Ok(mode) = OpenMode::parse(mode) else {
        return Err(Error::INVAL);
    };

    let fd = fd::open(pathname, mode.flags())?;
    unsafe { Ok(&*alloc::boxed(FileStream { fd })) }
}

pub fn fputc(c: u8, stream: &FileStream) -> i32 {
    match stream.write_byte(c) {
        Ok(_) => c as _,
        Err(_) => EOF,
    }
}

pub unsafe fn fwrite(ptr: *const u8, size: usize, nitems: usize, stream: &FileStream) -> usize {
    if nitems == 0 {
        return 0;
    }

    for i in 0..nitems {
        unsafe {
            let ptr = ptr.add(i * size);
            for j in 0..size {
                let ptr = ptr.add(j);
                let result = stream.write_byte(ptr.read());
                match result {
                    Ok(()) => {}
                    Err(_) => {
                        return i;
                    }
                }
            }
        }
    }

    nitems
}
