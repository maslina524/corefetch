use core::alloc::{GlobalAlloc, Layout};

use crate::imp::libc::{malloc, calloc, free, realloc};

pub struct Allocator;

// SAFETY: All unsafe code has SAFETY comments
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        unsafe {
            let ptr = malloc(layout.size());
            assert!(!ptr.is_null(), "`malloc` error!");
            ptr.cast::<u8>()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: According to the documentation,
        // the pointer passed to `free` may be NULL (check `man 3 free``);
        unsafe { free(ptr.cast()) };
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The `HeapAlloc` function always
        // receives a valid handle from `GetProcessHeap`;
        // if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        unsafe {
            let ptr = calloc(
                layout.size(), 
                1
            );
            assert!(!ptr.is_null(), "`calloc` error!");
            ptr.cast::<u8>()
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: When passing a null pointer, 
        // the function will behave like `HeapAlloc`; 
        // the returned pointer is checked
        unsafe {
            let new_ptr = realloc(
                ptr.cast(),
                new_size
            );
            assert!(!new_ptr.is_null(), "`realloc` error!");
            new_ptr.cast::<u8>()
        }
    }
}