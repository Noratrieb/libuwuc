use core::ffi::CStr;

use crate::{println, utils::SharedThinCstr};

mod global {
    use core::cell::UnsafeCell;

    use crate::utils::{SharedThinCstr, SyncUnsafeCell};

    use super::EnvP;

    static ENVP: SyncUnsafeCell<EnvP> =
        SyncUnsafeCell(UnsafeCell::new(EnvP(core::ptr::null_mut())));

    pub(super) unsafe fn init(envp: *mut SharedThinCstr) {
        assert!((*ENVP.0.get()).0.is_null());
        *ENVP.0.get() = EnvP(envp);
    }

    pub(super) fn get() -> EnvP {
        let ptr = unsafe { *ENVP.0.get() };
        assert!(!ptr.0.is_null());
        ptr
    }
}

pub(crate) unsafe fn init(envp: *mut SharedThinCstr) {
    global::init(envp);
}

#[derive(Clone, Copy)]
struct EnvP(*mut SharedThinCstr);

unsafe impl Sync for EnvP {}

impl Iterator for EnvP {
    type Item = SharedThinCstr;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let value: SharedThinCstr = self.0.read();
            if value.0.is_null() {
                None
            } else {
                self.0 = self.0.add(1);
                Some(value)
            }
        }
    }
}

pub(crate) fn debug_env() {
    println!("start vars");
    global::get().for_each(|s| {
        let str = <&CStr>::from(s).to_str().unwrap();
        println!("{str}");
    });
    println!("end vars");
}
