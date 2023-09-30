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

#[cfg(test)]
mod tests {
    #[test]
    fn memcpy_null() {
        unsafe { super::memcpy(std::ptr::null_mut(), std::ptr::null_mut(), 0) };
    }

    #[test]
    fn memcpy() {
        let src = [1, 2, 3];
        let mut dest = [0; 3];
        unsafe { super::memcpy(dest.as_mut_ptr(), src.as_ptr(), 3) };
        assert_eq!(dest, src);
    }

    #[test]
    fn memset_null() {
        unsafe { super::memset(std::ptr::null_mut(), 0, 0) };
    }

    #[test]
    fn memset() {
        let mut dest = [1; 10];
        unsafe { super::memset(dest.as_mut_ptr(), 0, 9) };
        assert_eq!(&dest[..9], &[0; 9]);
        assert_eq!(dest[9], 1);
    }

    #[test]
    fn memcmp_null() {
        let result = unsafe { super::memcmp(std::ptr::null(), std::ptr::null(), 0) };
        assert_eq!(result, 0);
    }

    #[test]
    fn memcmp_eq_one() {
        let a = [1];
        let b = [1];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 1)};
        assert_eq!(result, 0);
    }

    #[test]
    fn memcmp_eq_two() {
        let a = [1, 2];
        let b = [1, 2];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 2) };
        assert_eq!(result, 0);
    }

    #[test]
    fn memcmp_eq_many() {
        let a = [1, 2, 5, 3, 5, 67, 7];
        let b = [1, 2, 5, 3, 5, 67, 7];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 7) };
        assert_eq!(result, 0);
    }

    #[test]
    fn memcmp_lt_one() {
        let a = [0];
        let b = [1];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 1)};
        assert_eq!(result, -1);
    }

    #[test]
    fn memcmp_lt_two() {
        let a = [1, 1];
        let b = [1, 2];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 2) };
        assert_eq!(result, -1);
    }

    #[test]
    fn memcmp_lt_many() {
        let a = [1, 2, 5, 3, 4, 67, 7];
        let b = [1, 2, 5, 3, 5, 67, 7];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 7) };
        assert_eq!(result, -1);
    }

    #[test]
    fn memcmp_gt_one() {
        let a = [255];
        let b = [1];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 1)};
        assert_eq!(result, 1);
    }

    #[test]
    fn memcmp_gt_two() {
        let a = [1, 4];
        let b = [1, 2];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 2) };
        assert_eq!(result, 1);
    }

    #[test]
    fn memcmp_gt_many() {
        let a = [1, 2, 6, 3, 4, 67, 7];
        let b = [1, 2, 5, 3, 5, 67, 7];
        let result = unsafe { super::memcmp(a.as_ptr(), b.as_ptr(), 7) };
        assert_eq!(result, 1);
    }

    #[test]
    fn strlen_empty() {
        let str = b"\0";
        assert_eq!(unsafe { super::strlen(str.as_ptr()) }, 0);
    }

    #[test]
    fn strlen_one() {
        let str = b"A\0";
        assert_eq!(unsafe { super::strlen(str.as_ptr()) }, 1);
    }

    #[test]
    fn strlen_many() {
        let str = b"meow meow meow meow\0";
        assert_eq!(unsafe { super::strlen(str.as_ptr()) }, 19);
    }
}
