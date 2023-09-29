// Useful reference: https://filippo.io/linux-syscall-table/

#[allow(non_upper_case_globals)]
mod values;
pub use values::*;

#[macro_export]
macro_rules! syscall {
    ($number:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rdi") $number,
            lateout("rax") out,
        );
        out
    }};
    ($number:expr, $arg1:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            lateout("rax") out,
        );
        out
    }};
    ($number:expr, $arg1:expr, $arg2:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            in("rsi") $arg2,
            lateout("rax") out,
        );
        out
    }};
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            lateout("rax") out,
        );
        out
    }};
}

pub use syscall;