pub mod libc {
    use core::ffi::{c_void, c_char};

    unsafe extern "C" {
        unsafe fn printf(format: *const c_char, ...);
        unsafe fn malloc(size: isize) -> *mut c_void;
        unsafe fn free(ptr: *mut c_void);
        unsafe fn calloc(nmemb: isize, size: isize) -> *mut c_void;
        unsafe fn realloc(ptr: *mut c_void, size: isize) -> *mut c_void;
    }
}