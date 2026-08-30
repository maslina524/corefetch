use core::ffi::{c_void, c_char, c_int};

unsafe extern "C" {
    pub unsafe fn write(fd: c_int, buf: *const c_char, len: usize);
    pub unsafe fn malloc(size: usize) -> *mut c_void;
    pub unsafe fn free(ptr: *mut c_void);
    pub unsafe fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub unsafe fn exit(status: c_int) -> !;
    pub unsafe fn getenv(name: *const c_char) -> *mut c_char;
    pub unsafe fn access(path: *const c_char, mode: c_int) -> c_int;
}