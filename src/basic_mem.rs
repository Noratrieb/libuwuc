#[no_mangle]
pub(crate) unsafe extern "C" fn memset(ptr: *mut u8, constant: u8, len: usize) {
    for i in 0..len {
        unsafe {
            *ptr.add(i) = constant;
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn strlen(mut s: *const u8) -> usize {
    let mut len = 0;
    while s.read() != 0 {
        len += 1;
        s = s.add(1);
    }
    len
}
