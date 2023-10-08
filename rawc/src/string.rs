use libuwuc::{error::Error, utils::SharedThinCstr};

#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, constant: u8, len: usize) {
    libuwuc::mem::memset(ptr, constant, len)
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    libuwuc::mem::memcpy(dest, src, size)
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    libuwuc::mem::memmove(dest, src, size)
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
    libuwuc::mem::memcmp(s1, s2, size)
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
    libuwuc::mem::memcmp(s1, s2, size)
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: SharedThinCstr<'_>, s2: SharedThinCstr<'_>) -> i32 {
    libuwuc::mem::strcmp(s1, s2)
}

// This technically violates the safety precondition of SharedThinCstr but that's fine, we're careful.
#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: SharedThinCstr<'_>, s2: SharedThinCstr<'_>, n: usize) -> i32 {
    libuwuc::mem::strncmp(s1, s2, n)
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    libuwuc::mem::strlen(s)
}

#[no_mangle]
pub unsafe extern "C" fn strerror(errnum: Error) -> *const u8 {
    libuwuc::error::strerror(errnum)
        .map(str::as_ptr)
        .unwrap_or(core::ptr::null())
}
