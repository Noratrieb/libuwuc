use core::ffi::{c_char, c_int};

/// The entrypoint of the program.
/// This is called by a bit of assembly handling architecture-specific _start.
pub(crate) unsafe extern "C" fn start(
    argc: i32,
    argv: *const *const c_char,
    _envp: *const *const c_char,
) -> ! {
    extern "C" {
        fn main(argc: c_int, argv: *const *const c_char) -> c_int;
    }

    let result = main(argc, argv);
   
    exit(result as u64);
}

pub fn exit(code: u64) -> ! {
    unsafe {
        crate::sys::syscall::syscall!(crate::sys::syscall::SYS_EXIT, code);
        core::hint::unreachable_unchecked()
    }
}
