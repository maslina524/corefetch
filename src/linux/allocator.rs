use core::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{AtomicUsize, Ordering}
};

use crate::imp::libc::{malloc, calloc, free, realloc};

pub struct Allocator;

static ALLOC_COUNTER  : AtomicUsize = AtomicUsize::new(0);
static REALLOC_COUNTER: AtomicUsize = AtomicUsize::new(0);
static DEALLOC_COUNTER: AtomicUsize = AtomicUsize::new(0);

// SAFETY: All unsafe code has SAFETY comments
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        let ptr = unsafe {
            malloc(layout.size())
        };
        assert!(!ptr.is_null(), "`malloc` error!");
        ALLOC_COUNTER.fetch_add(1, Ordering::Relaxed);
        ptr.cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: According to the documentation,
        // the pointer passed to `free` may be NULL (check `man 3 free``);
        unsafe { free(ptr.cast()) };
        DEALLOC_COUNTER.fetch_add(1, Ordering::Relaxed);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The `HeapAlloc` function always
        // receives a valid handle from `GetProcessHeap`;
        // if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        let ptr = unsafe {
            calloc(
                layout.size(), 
                1
            )
        };
        assert!(!ptr.is_null(), "`calloc` error!");
        ALLOC_COUNTER.fetch_add(1, Ordering::Relaxed);
        ptr.cast::<u8>()
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: When passing a null pointer, 
        // the function will behave like `HeapAlloc`; 
        // the returned pointer is checked
        let new_ptr = unsafe {
            realloc(
                ptr.cast(),
                new_size
            )
        };
        assert!(!new_ptr.is_null(), "`realloc` error!");
        REALLOC_COUNTER.fetch_add(1, Ordering::Relaxed);
        new_ptr.cast::<u8>()
    }
}

pub struct AllocationReport {
    pub alloc: usize,
    pub realloc: usize,
    pub dealloc: usize
}

impl AllocationReport {
    pub fn get() -> Self {
        let alloc = ALLOC_COUNTER.load(Ordering::Relaxed);
        let realloc = REALLOC_COUNTER.load(Ordering::Relaxed);
        let dealloc = DEALLOC_COUNTER.load(Ordering::Relaxed);

        Self { alloc, realloc, dealloc }
    }
}