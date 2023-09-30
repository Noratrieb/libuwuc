#[no_mangle]
pub unsafe extern "C" fn memset(ptr: *mut u8, constant: u8, len: usize) {
    libuwuc::mem::memset(ptr, constant, len)
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    libuwuc::mem::memcpy(dest, src, size)
}

#[no_mangle]
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
    libuwuc::mem::memcmp(s1, s2, size)
}


#[no_mangle]
pub unsafe fn bcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
    libuwuc::mem::memcmp(s1, s2, size)
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const u8) -> usize {
    libuwuc::mem::strlen(s)
}
