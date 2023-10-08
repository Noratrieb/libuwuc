use core::{cell::UnsafeCell, ffi::CStr, fmt::Debug, marker::PhantomData, ptr::NonNull};

#[repr(transparent)]
#[derive(Default)]
pub struct SyncUnsafeCell<T>(pub UnsafeCell<T>);

unsafe impl<T: Sync> Sync for SyncUnsafeCell<T> {}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CStrRef<'a>(NonNull<u8>, PhantomData<&'a CStr>);

#[macro_export]
macro_rules! cstr {
    ($value:literal) => {{
        let s = concat!($value, "\0");
        #[allow(unused_unsafe)]
        unsafe {
            $crate::utils::CStrRef::from_raw(s.as_ptr().cast())
        }
    }};
}

pub use cstr;

impl<'a> CStrRef<'a> {
    pub unsafe fn from_raw(ptr: *const u8) -> Self {
        Self(NonNull::new_unchecked(ptr as *mut u8), PhantomData)
    }

    pub unsafe fn from_nullable(ptr: *const u8) -> Option<Self> {
        NonNull::new(ptr as *mut u8).map(|ptr| Self(ptr, PhantomData))
    }

    pub fn as_ptr(self) -> NonNull<u8> {
        self.0
    }

    pub fn as_raw(self) -> *const u8 {
        self.0.as_ptr()
    }

    pub unsafe fn add(self, amount: usize) -> Self {
        Self::from_raw(self.0.as_ptr().add(amount))
    }

    pub fn first(self) -> Option<u8> {
        let c = unsafe { self.0.as_ptr().read() };
        (c != 0).then_some(c as _)
    }
}

impl<'a> IntoIterator for CStrRef<'a> {
    type Item = u8;

    type IntoIter = CStrIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CStrIter(self)
    }
}

pub struct CStrIter<'a>(CStrRef<'a>);

impl<'a> Iterator for CStrIter<'a> {
    type Item = u8;

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

unsafe impl<'a> Send for CStrRef<'a> {}
unsafe impl<'a> Sync for CStrRef<'a> {}

impl<'a> Debug for CStrRef<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = <&CStr>::from(*self).to_str();
        match str {
            Ok(str) => f.write_str(str),
            Err(_) => f.write_str("<invalid UTF-8>"),
        }
    }
}

impl<'a> From<CStrRef<'a>> for &'a CStr {
    fn from(value: CStrRef<'a>) -> Self {
        unsafe { CStr::from_ptr(value.0.as_ptr().cast()) }
    }
}

impl<'a> PartialEq for CStrRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.into_iter().eq(*other)
    }
}

impl<'a> Eq for CStrRef<'a> {}

#[repr(transparent)]
pub struct SyncPtr<T>(pub *mut T);

unsafe impl<T> Send for SyncPtr<T> {}
unsafe impl<T> Sync for SyncPtr<T> {}
