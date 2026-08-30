use core::ffi::{c_void, c_char, c_int};

pub type FILE = *mut c_void;
#[allow(non_camel_case_types, reason = "C type")]
pub type c_size = usize;

unsafe extern "C" {
    pub safe fn write(fd: c_int, buf: *const c_char, len: c_size);
    pub safe fn malloc(size: c_size) -> *mut c_void;
    pub safe fn free(ptr: *mut c_void);
    pub safe fn calloc(nmemb: c_size, size: c_size) -> *mut c_void;
    pub safe fn realloc(ptr: *mut c_void, size: c_size) -> *mut c_void;
    pub safe fn exit(status: c_int) -> !;
    pub safe fn getenv(name: *const c_char) -> *mut c_char;
    pub safe fn access(path: *const c_char, mode: c_int) -> c_int;
    pub safe fn fopen(pathname: *const c_char, mode: *const c_char) -> FILE;
    pub safe fn fclose(stream: FILE) -> c_int;
    pub safe fn errno_location() -> *mut c_int;
    pub safe fn strerror(errnum: c_int) -> *mut c_char;
}

pub fn errno() -> i32 {
    (unsafe { *errno_location() }) as i32
}