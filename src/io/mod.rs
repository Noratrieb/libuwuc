use crate::sys::syscall;

pub const STDIN: i32 = 0;
pub const STDOUT: i32 = 1;
pub const STDERR: i32 = 2;

#[doc(hidden)]
pub struct Printer;

impl core::fmt::Write for Printer {
    fn write_str(&mut self, mut s: &str) -> core::fmt::Result {
        unsafe {
            while s.len() > 0 {
                let result =
                    syscall::syscall!(syscall::SYS_WRITE, STDIN, s.as_ptr(), s.len()) as i64;
                if result < 0 {
                    return Err(core::fmt::Error);
                }
                s = &s[(result as usize)..];
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($tt:tt)*) => {
        {
            use ::core::fmt::Write;
            ::core::writeln!($crate::io::Printer, $($tt)*).unwrap();
        }
    };
}
pub use println;
