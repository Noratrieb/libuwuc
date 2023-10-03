use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_int,
    sync::atomic::{AtomicUsize, Ordering},
};

use linked_list_allocator::LockedHeap;

static ALLOCATOR: LockedHeap = LockedHeap::empty();

const UNINIT: usize = 0;
const INITIALIZING: usize = 1;
const INIT: usize = 2;

static INIT_STATE: AtomicUsize = AtomicUsize::new(0);

fn init() {
    let state =
        INIT_STATE.compare_exchange(UNINIT, INITIALIZING, Ordering::Relaxed, Ordering::Acquire);

    match state {
        Ok(_) => unsafe {
            const HEAP_SIZE: usize = 0x100000;
            let map_private = 0x0002;
            let map_anon = 0x20;
            let prot_read = 1;
            let prod_write = 2;

            let start = sys_mmap(
                core::ptr::null(),
                HEAP_SIZE,
                prot_read | prod_write,
                map_anon | map_private,
                0,
                0,
            );
            if (start as isize) < 0 {
                todo!("mmap failed");
            }
            ALLOCATOR.lock().init(start, HEAP_SIZE);

            INIT_STATE.store(INIT, Ordering::Release);
        },
        Err(INITIALIZING) => {
            while INIT_STATE.load(Ordering::Acquire) != INIT {
                core::hint::spin_loop();
            }
        }
        Err(INIT) => {}
        Err(_) => unreachable!(),
    }
}

pub unsafe fn malloc_zeroed(size: usize, align: usize) -> *mut u8 {
    /*
    |start  |align  |offset |RETURN VALUE
    */

    init();
    let (layout, offset) = Layout::array::<usize>(3)
        .unwrap_unchecked()
        .extend(Layout::from_size_align_unchecked(size, align))
        .unwrap_unchecked();

    let ptr = ALLOCATOR.alloc_zeroed(layout);
    if ptr.is_null() {
        return ptr;
    }
    ptr.cast::<[usize; 2]>()
        .write([layout.size(), layout.align()]);

    let ret_ptr = ptr.add(offset);
    ret_ptr.cast::<usize>().sub(1).write(offset);

    ret_ptr
}

pub unsafe fn free(ptr: *mut u8) {
    init();
    let offset = ptr.cast::<usize>().sub(1).read();
    let start = ptr.sub(offset);
    let [size, align] = start.cast::<[usize; 2]>().read();
    let layout = Layout::from_size_align_unchecked(size, align);
    ALLOCATOR.dealloc(start, layout);
}

#[cfg_attr(miri, allow(unused_variables, unreachable_code))]
pub unsafe fn sys_mmap(
    addr: *const u8,
    size: usize,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: u64,
) -> *mut u8 {
    #[cfg(miri)]
    {
        extern crate std;
        return std::alloc::System.alloc_zeroed(Layout::from_size_align_unchecked(size, 4096));
    }
    crate::sys::syscall::syscall!(
        crate::sys::syscall::SYS_MMAP,
        addr,
        size,
        prot,
        flags,
        fd,
        offset
    ) as *mut u8
}

#[cfg(test)]
mod tests {
    #[test]
    fn init() {
        super::init();
    }

    #[test]
    fn malloc_free() {
        unsafe {
            let x = super::malloc_zeroed(10, 8);
            x.cast::<usize>().write(10);
            super::free(x);
        }
    }
}
