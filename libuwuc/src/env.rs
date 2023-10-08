use core::{ffi::CStr, ptr::NonNull};

use crate::{println, utils::CStrRef};

mod global {
    use core::{cell::UnsafeCell, ptr::NonNull};

    use crate::utils::{CStrRef, SyncUnsafeCell};

    use super::EnvP;

    static ENVP: SyncUnsafeCell<EnvP> = SyncUnsafeCell(UnsafeCell::new(EnvP(None)));

    pub(super) unsafe fn init(envp: *mut Option<CStrRef<'static>>) {
        assert!((*ENVP.0.get()).0.is_none());
        *ENVP.0.get() = EnvP(Some(NonNull::new(envp).unwrap()));
    }

    pub(super) fn get() -> EnvP {
        let ptr = unsafe { *ENVP.0.get() };
        assert!(ptr.0.is_some());
        ptr
    }
}

pub(crate) unsafe fn init(envp: *mut Option<CStrRef<'static>>) {
    global::init(envp);
}

#[derive(Clone, Copy)]
struct EnvP(Option<NonNull<Option<CStrRef<'static>>>>);

unsafe impl Sync for EnvP {}

impl Iterator for EnvP {
    type Item = CStrRef<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let value: Option<CStrRef<'static>> = self.0.unwrap().as_ptr().read();

            value.map(|value| {
                self.0 = Some(NonNull::new_unchecked(self.0.unwrap().as_ptr().add(1)));
                value
            })
        }
    }
}

#[allow(unused)]
pub(crate) fn debug_env() {
    println!("start vars");
    global::get().for_each(|s| {
        let str = <&CStr>::from(s).to_str().unwrap();
        println!("{str}");
    });
    println!("end vars");
}

pub fn getenv(name: CStrRef<'_>) -> Option<CStrRef<'static>> {
    getenv_inner(global::get(), name)
}

fn getenv_inner(mut envp: EnvP, name: CStrRef<'_>) -> Option<CStrRef<'static>> {
    let mut eq_idx = 0;
    envp.find(|env| {
        // Find ENV
        // EN=x
        // ENV=x   <- this one
        // ENVNO=x

        let mut name_iter = name.into_iter();
        let mut env_iter = env.into_iter();
        eq_idx = 0;
        loop {
            let name = name_iter.next().map(|c| c);
            let env = env_iter.next().map(|c| c);
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

    use crate::utils::{cstr, CStrRef};

    use super::EnvP;

    fn with_envp(env: &[&str], f: impl FnOnce(EnvP)) {
        let cstrs = env
            .iter()
            .map(|s| CString::new(s.to_string()).unwrap())
            .collect::<Vec<_>>();

        let mut envs: Vec<Option<CStrRef<'static>>> = cstrs
            .iter()
            .map(|cstr| unsafe { Some(CStrRef::from_raw(cstr.as_ptr() as _)) })
            .collect();
        envs.push(None);

        let envp = EnvP(Some(NonNull::new(envs.as_ptr() as _).unwrap()));
        f(envp)
    }

    #[test]
    fn getenv_exact_first() {
        with_envp(&["UWU=a"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr!("UWU")).unwrap(), cstr!("a"));
        })
    }

    #[test]
    fn getenv_previous_mismatches() {
        with_envp(&["UW=a", "UWUU=b", "UWU=c"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr!("UWU")).unwrap(), cstr!("c"));
        })
    }

    #[test]
    fn getenv_name_long() {
        with_envp(&["U=w"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr!("LONG_NAME")), None);
        })
    }

    #[test]
    fn getenv_same_length() {
        with_envp(&["OWO=a", "UWU=b"], |envp| {
            assert_eq!(super::getenv_inner(envp, cstr!("UWU")), Some(cstr!("b")));
        })
    }
}
