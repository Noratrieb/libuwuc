#[inline]
pub unsafe fn memset(ptr: *mut u8, constant: u8, len: usize) {
    for i in 0..len {
        unsafe {
            *ptr.add(i) = constant;
        }
    }
}

#[inline]
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    for i in 0..size {
        dest.add(i).write(src.add(i).read());
    }

    dest
}

#[inline]
pub unsafe fn memcmp(s1: *const u8, s2: *const u8, size: usize) -> i32 {
    for i in 0..size {
        let a = s1.add(i).read();
        let b = s2.add(i).read();

        let cmp = a.cmp(&b) as i8;
        if cmp != 0 {
            return cmp as i32;
        }
    }

    0
}

#[inline]
pub unsafe fn strlen(mut s: *const u8) -> usize {
    let mut len = 0;
    while s.read() != 0 {
        len += 1;
        s = s.add(1);
    }
    len
}
