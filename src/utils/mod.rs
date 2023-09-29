use core::{
    cell::UnsafeCell,
    ffi::{c_char, CStr},
};

#[repr(transparent)]
#[derive(Default)]
pub(crate) struct SyncUnsafeCell<T>(pub(crate) UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

#[repr(transparent)]
pub(crate) struct SharedThinCstr(pub(crate) *const c_char);

unsafe impl Send for SharedThinCstr {}
unsafe impl Sync for SharedThinCstr {}

impl From<SharedThinCstr> for &CStr {
    fn from(value: SharedThinCstr) -> Self {
        unsafe { CStr::from_ptr(value.0) }
    }
}

#[repr(transparent)]
pub(crate) struct SyncPtr<T>(pub(crate) *mut T);

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}
