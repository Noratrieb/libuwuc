use libuwuc::utils::SharedThinCstr;

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    libuwuc::alloc::malloc_zeroed(size, 16)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    libuwuc::alloc::free(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn exit(code: i32) -> ! {
    libuwuc::start::exit(code as i64 as _)
}

#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const u8) -> *const u8 {
    libuwuc::env::getenv(SharedThinCstr::from_raw(name))
        .map(SharedThinCstr::as_raw)
        .unwrap_or(core::ptr::null())
}
