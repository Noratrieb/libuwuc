use core::{ffi::CStr, ptr::NonNull};

use crate::{println, utils::SharedThinCstr};

mod global {
    use core::{cell::UnsafeCell, ptr::NonNull};

    use crate::utils::{SharedThinCstr, SyncUnsafeCell};

    use super::EnvP;

    static ENVP: SyncUnsafeCell<EnvP> = SyncUnsafeCell(UnsafeCell::new(EnvP(None)));

    pub(super) unsafe fn init(envp: *mut Option<SharedThinCstr>) {
        assert!((*ENVP.0.get()).0.is_none());
        *ENVP.0.get() = EnvP(Some(NonNull::new(envp).unwrap()));
    }

    pub(super) fn get() -> EnvP {
        let ptr = unsafe { *ENVP.0.get() };
        assert!(ptr.0.is_some());
        ptr
    }
}

pub(crate) unsafe fn init(envp: *mut Option<SharedThinCstr>) {
    global::init(envp);
}

#[derive(Clone, Copy)]
struct EnvP(Option<NonNull<Option<SharedThinCstr>>>);

unsafe impl Sync for EnvP {}

impl Iterator for EnvP {
    type Item = SharedThinCstr;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let value: Option<SharedThinCstr> = self.0.unwrap().as_ptr().read();

            value.map(|value| {
                self.0 = Some(NonNull::new_unchecked(self.0.unwrap().as_ptr().add(1)));
                value
            })
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

pub fn getenv(name: SharedThinCstr) -> Option<SharedThinCstr> {
    getenv_inner(global::get(), name)
}

fn getenv_inner(mut envp: EnvP, name: SharedThinCstr) -> Option<SharedThinCstr> {
    let mut eq_idx = 0;
    envp.find(|env| {
        println!("trying {env:?}");
        // Find ENV
        // EN=x
        // ENV=x   <- this one
        // ENVNO=x

        let mut name_iter = name.into_iter();
        let mut env_iter = env.into_iter();
        eq_idx = 0;
        loop {
            let name = name_iter.next().map(|c| c as u8);
            let env = env_iter.next().map(|c| c as u8);
            if let (None, Some(b'=')) = (name, env) {
                return true;
            }
            if name.is_none() || env == Some(b'=') {
                return false;
            }
            if name != env {
                return false;
            }
            eq_idx += 1;
        }
    })
    .map(|elem| {
        let value_idx = eq_idx + 1;
        unsafe { elem.add(value_idx) }
    })
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;
    use std::string::ToString;
    use std::{ffi::CString, vec::Vec};

    use crate::utils::SharedThinCstr;

    use super::EnvP;

    fn cstr(s: &str) -> SharedThinCstr {
        assert_eq!(s.as_bytes()[s.len() - 1], 0);
        unsafe { SharedThinCstr::from_ptr(NonNull::new(s.as_ptr() as _).unwrap()) }
    }

    fn with_envp(env: &[&str], f: impl FnOnce(EnvP)) {
        let cstrs = env
            .iter()
            .map(|s| CString::new(s.to_string()).unwrap())
            .collect::<Vec<_>>();

        let mut envs: Vec<Option<SharedThinCstr>> = cstrs
            .iter()
            .map(|cstr| unsafe {
                Some(SharedThinCstr::from_ptr(
                    NonNull::new(cstr.as_ptr() as _).unwrap(),
                ))
            })
            .collect();
        envs.push(None);

        let envp = EnvP(Some(NonNull::new(envs.as_ptr() as _).unwrap()));
        f(envp)
    }

    #[test]
    fn getenv_exact_first() {
        with_envp(&["UWU=a"], |envp| {
            assert_eq!(
                super::getenv_inner(envp, cstr("UWU\0")).unwrap(),
                cstr("a\0")
            );
        })
    }

    #[test]
    fn getenv_previous_mismatches() {
        with_envp(&["UW=a", "UWUU=b", "UWU=c"], |envp| {
            assert_eq!(
                super::getenv_inner(envp, cstr("UWU\0")).unwrap(),
                cstr("c\0")
            );
        })
    }

    #[test]
    fn getenv_name_long() {
        with_envp(&["U=w"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr("LONG_NAME\0")), None);
        })
    }

    #[test]
    fn getenv_same_length() {
        with_envp(&["OWO=a", "UWU=b"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr("UWU\0")), Some(cstr("b\0")));
        })
    }
}
