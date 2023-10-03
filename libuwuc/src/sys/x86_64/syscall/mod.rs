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
            // rcx and r11 are clobbered https://github.com/torvalds/linux/blob/3b517966c5616ac011081153482a5ba0e91b17ff/tools/include/nolibc/arch-x86_64.h#L19
            out("rcx") _,
            out("r11") _,
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
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
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
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
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
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
        );
        out
    }};
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            lateout("rax") out,
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
        );
        out
    }};
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            in("r8") $arg5,
            lateout("rax") out,
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
        );
        out
    }};
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {{
        let out: u64;
        ::core::arch::asm!(
            "syscall",
            in("rax") $number,
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            in("r8") $arg5,
            in("r9") $arg6,
            lateout("rax") out,
            // rcx and r11 are clobbered
            out("rcx") _,
            out("r11") _,
        );
        out
    }};
}

pub use syscall;
