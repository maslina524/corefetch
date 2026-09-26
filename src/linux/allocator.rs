use core::{
    alloc::{GlobalAlloc, Layout},
    sync::atomic::{
        AtomicUsize, 
        Ordering::{Release, Relaxed}
    }
};

use crate::imp::libc::{malloc, calloc, free, realloc};

pub struct Allocator;

static ALLOC_COUNTER    : AtomicUsize = AtomicUsize::new(0);
static REALLOC_COUNTER  : AtomicUsize = AtomicUsize::new(0);
static DEALLOC_COUNTER  : AtomicUsize = AtomicUsize::new(0);

static ALLOCATED_TOTAL  : AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED_TOTAL: AtomicUsize = AtomicUsize::new(0);

static CURRENT_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static MAX_IN_RUNTIME   : AtomicUsize = AtomicUsize::new(0);

fn allocated(size: usize) {
    ALLOCATED_TOTAL.fetch_add(size, Relaxed);
    CURRENT_ALLOCATED.fetch_add(size, Relaxed);
    let cur = CURRENT_ALLOCATED.load(Relaxed);
    if cur > MAX_IN_RUNTIME.load(Relaxed) {
        MAX_IN_RUNTIME.store(cur, Release);
    }
}

fn deallocated(size: usize) {
    DEALLOCATED_TOTAL.fetch_add(size, Relaxed);
    CURRENT_ALLOCATED.fetch_sub(size, Relaxed);
}

// SAFETY: All unsafe code has SAFETY comments
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: if the OS fails to allocate memory, it
        // returns NULL which is checked in the block
        let ptr = unsafe {
            malloc(layout.size())
        };
        assert!(!ptr.is_null(), "`malloc` error!");
        ALLOC_COUNTER.fetch_add(1, Relaxed);
        allocated(layout.size());

        ptr.cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: According to the documentation,
        // the pointer passed to `free` may be NULL (check `man 3 free``);
        unsafe { free(ptr.cast()) };
        DEALLOC_COUNTER.fetch_add(1, Relaxed);
        deallocated(layout.size());
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
        ALLOC_COUNTER.fetch_add(1, Relaxed);
        allocated(layout.size());

        ptr.cast::<u8>()
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
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
        REALLOC_COUNTER.fetch_add(1, Relaxed);
        
        let old_size = layout.size();
        if new_size > old_size {
            allocated(new_size - old_size);
        } else {
            allocated(old_size - new_size);
        }
        
        new_ptr.cast::<u8>()
    }
}

pub struct AllocationReport {
    pub alloc: usize,
    pub realloc: usize,
    pub dealloc: usize,
    pub alloc_total: usize,
    pub dealloc_total: usize,
    pub max_in_runtime: usize
}

impl AllocationReport {
    pub fn get() -> Self {
        let alloc = ALLOC_COUNTER.load(Relaxed);
        let realloc = REALLOC_COUNTER.load(Relaxed);
        let dealloc = DEALLOC_COUNTER.load(Relaxed);

        let alloc_total = ALLOCATED_TOTAL.load(Relaxed);
        let dealloc_total = DEALLOCATED_TOTAL.load(Relaxed);

        let max_in_runtime = MAX_IN_RUNTIME.load(Relaxed);

        Self { alloc, realloc, dealloc, alloc_total, dealloc_total, max_in_runtime }
    }
}