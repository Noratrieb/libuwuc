use core::{
    cell::UnsafeCell,
    ffi::{c_char, CStr},
    fmt::Debug,
    ptr::NonNull,
};

#[repr(transparent)]
#[derive(Default)]
pub(crate) struct SyncUnsafeCell<T>(pub(crate) UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedThinCstr(pub NonNull<c_char>);

impl SharedThinCstr {
    pub unsafe fn from_ptr(ptr: NonNull<c_char>) -> Self {
        Self(ptr)
    }

    pub fn from_array<const N: usize>(arr: &[u8; N]) -> Self {
        assert!(arr[N - 1] == 0);
        unsafe { Self(NonNull::new_unchecked(arr as *const u8 as *mut c_char)) }
    }

    pub fn as_ptr(self) -> NonNull<c_char> {
        self.0
    }

    pub unsafe fn add(self, amount: usize) -> Self {
        Self(NonNull::new_unchecked(self.0.as_ptr().add(amount)))
    }
}

impl IntoIterator for SharedThinCstr {
    type Item = c_char;

    type IntoIter = CStrIter;

    fn into_iter(self) -> Self::IntoIter {
        CStrIter(self)
    }
}

pub struct CStrIter(SharedThinCstr);

impl Iterator for CStrIter {
    type Item = c_char;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let c = self.0 .0.as_ptr().read();
            if c == 0 {
                return None;
            }

            self.0 = self.0.add(1);
            Some(c)
        }
    }
}

unsafe impl Send for SharedThinCstr {}
unsafe impl Sync for SharedThinCstr {}

impl Debug for SharedThinCstr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = <&CStr>::from(*self).to_str();
        match str {
            Ok(str) => f.write_str(str),
            Err(_) => f.write_str("<invalid UTF-8>"),
        }
    }
}

impl From<SharedThinCstr> for &CStr {
    fn from(value: SharedThinCstr) -> Self {
        unsafe { CStr::from_ptr(value.0.as_ptr()) }
    }
}

impl PartialEq for SharedThinCstr {
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(*other)
    }
}

impl Eq for SharedThinCstr {}

#[repr(transparent)]
pub(crate) struct SyncPtr<T>(pub(crate) *mut T);

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}
