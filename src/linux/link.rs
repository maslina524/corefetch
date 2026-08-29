pub mod libc {
    use core::ffi::{c_void, c_char};

    unsafe extern "C" {
        pub unsafe fn printf(format: *const c_char, ...);
        pub unsafe fn malloc(size: isize) -> *mut c_void;
        pub unsafe fn free(ptr: *mut c_void);
        pub unsafe fn calloc(nmemb: isize, size: isize) -> *mut c_void;
        pub unsafe fn realloc(ptr: *mut c_void, size: isize) -> *mut c_void;
    }
}