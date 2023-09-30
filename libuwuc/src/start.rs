use core::ffi::{c_char, c_int};

use crate::utils::SharedThinCstr;

/// The entrypoint of the program.
/// This is called by a bit of assembly handling architecture-specific _start.
pub(crate) unsafe extern "C" fn start(argc: u64, argv: *const *const c_char, rsp: u64) -> ! {
    let envp = (8 + 8 * argc + rsp + 8) as *mut Option<SharedThinCstr>;

    crate::env::init(envp);

    extern "C" {
        fn main(argc: c_int, argv: *const *const c_char) -> c_int;
    }

    // crate::env::debug_env();

    let result = main(argc as i32, argv);

    exit(result as u64);
}

pub fn exit(code: u64) -> ! {
    unsafe {
        crate::sys::syscall::syscall!(crate::sys::syscall::SYS_EXIT, code);
        crate::sys::helpers::trap!();
        core::hint::unreachable_unchecked()
    }
}

pub fn abort() -> ! {
    // FIXME: we actually need to do signal shenanigans
    unsafe {
        crate::sys::syscall::syscall!(crate::sys::syscall::SYS_EXIT, 1);
        crate::sys::helpers::trap!();
        core::hint::unreachable_unchecked()
    }
}
