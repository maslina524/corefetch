use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};

use crate::{
    os::windows::{GetProcessHeap, HeapAlloc, HeapFree},
    sync::OnceLock
};

static HEAP_HANDLE: OnceLock<usize> = OnceLock::new();

fn get_heap_handle() -> *mut c_void {
    // SAFETY: The `GetProcessHeap` function takes no arguments and
    // is guaranteed to return a valid handle
    let ptr = HEAP_HANDLE.get_or_init(|| unsafe { GetProcessHeap() as usize });
    *ptr as *mut c_void
}

pub struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let handle = get_heap_handle();

        // SAFETY: The `HeapAlloc` function always
        // receives a valid handle from `GetProcessHeap`;
        // if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        unsafe {
            let ptr = HeapAlloc(
                handle, 
                0, 
                layout.size()
            );
            if ptr.is_null() { panic!("HeapAlloc error!") }
            ptr as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let handle = get_heap_handle();

        // SAFETY: According to the documentation,
        // the pointer passed to `HeapFree` may be NULL;
        // if the OS fails to free the memory, it
        // returns FALSE which is checked in the same block
        unsafe {
            let ret = HeapFree(
                handle, 
                0, 
                ptr as *mut c_void
            );
            if ret == 0 { panic!("HeapFree error!") }
        }
    }
}