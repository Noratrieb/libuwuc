use core::ffi::c_int;

use super::EOF;

/// A `FILE`.
#[repr(C)]
pub struct FileStream {
    fd: c_int,
}

impl FileStream {
    pub const fn from_raw_fd(fd: c_int) -> Self {
        Self { fd }
    }

    fn write_byte(&self, c: u8) -> Result<(), i32> {
        unsafe { super::write_all(self.fd, &[c]).map_err(|e| e as _) }
    }
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
