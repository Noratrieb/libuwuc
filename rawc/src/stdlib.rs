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
