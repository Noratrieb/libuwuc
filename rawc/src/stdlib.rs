use core::ffi::{c_int, c_long};

use libuwuc::{error::IntoOkOrErrno, utils::SharedThinCstr};

// Allocation functions

const MAX_ALIGN: usize = 16;

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    libuwuc::alloc::malloc_zeroed(size, MAX_ALIGN)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    libuwuc::alloc::free(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    libuwuc::alloc::malloc_zeroed_array(nmemb, size, MAX_ALIGN)
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    libuwuc::alloc::realloc(ptr, size, MAX_ALIGN)
}

#[no_mangle]
pub unsafe extern "C" fn reallocarray(ptr: *mut u8, nmemb: usize, size: usize) -> *mut u8 {
    libuwuc::alloc::reallocarray(ptr, nmemb, size, MAX_ALIGN)
}

// Integer parsing functions

#[no_mangle]
pub unsafe extern "C" fn strtol(nptr: *const u8, endptr: *mut *const u8, base: c_int) -> c_long {
    let str = SharedThinCstr::from_raw(nptr);
    libuwuc::fmt::parse::parse_long(
        str,
        core::mem::transmute::<*mut *const u8, Option<&mut Option<SharedThinCstr<'_>>>>(endptr),
        base,
    )
    .into_ok_or_errno()
}

#[no_mangle]
pub unsafe extern "C" fn strtoll(nptr: *const u8, endptr: *mut *const u8, base: c_int) -> c_long {
    strtol(nptr, endptr, base)
}

// Other functions

#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const u8) -> *const u8 {
    libuwuc::env::getenv(SharedThinCstr::from_raw(name))
        .map(SharedThinCstr::as_raw)
        .unwrap_or(core::ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn exit(code: i32) -> ! {
    libuwuc::start::sys_exit(code as i64 as _)
}
