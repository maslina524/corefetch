use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ushort, c_void};

pub type FILE = *mut c_void;
#[allow(non_camel_case_types, reason = "C type")]
pub type c_size = usize;
#[allow(non_camel_case_types, reason = "C type")]
pub type c_mode = c_uint;
#[allow(non_camel_case_types, reason = "C type")]
pub type c_time = c_long; // i64
#[allow(non_camel_case_types, reason = "C type")]
pub type c_clockid = c_int; // WORK ONLY IN LINUX

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
    pub safe fn fwrite(ptr: *const c_void, size: c_size, nmemb: c_size, stream: FILE) -> c_size;
    pub safe fn fread(ptr: *mut c_void, size: c_size, nmemb: c_size, stream: FILE) -> c_size;
    pub safe fn fseek(stream: FILE, offset: c_long, whence: c_int) -> c_int;
    pub safe fn ftell(stream: FILE) -> c_long;
    pub safe fn rewind(stream: FILE);
    pub safe fn mkdir(pathname: *const c_char, mode: c_mode) -> c_int;
    pub safe fn clock_gettime(clockid: c_clockid, tp: *mut Timespec) -> c_int;
    pub safe fn sysinfo(info: *mut Sysinfo) -> c_int;
}

#[repr(C)]
#[derive(Default)]
pub struct Timespec {
    pub tv_sec: c_time,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Default)]
pub struct Sysinfo {
    pub uptime: c_long,
    pub loads: [c_ulong; 3],
    pub totalram: c_ulong,
    pub freeram: c_ulong,
    pub sharedram: c_ulong,
    pub bufferram: c_ulong,
    pub totalswap: c_ulong,
    pub freeswap: c_ulong,
    pub procs: c_ushort,
    pub totalhigh: c_ulong,
    pub freehigh: c_ulong,
    pub mem_unit: c_uint,
    _f: [c_char; 20 - 2 * size_of::<c_long>() - size_of::<c_int>()],
}

pub fn errno() -> i32 {
    (unsafe { *errno_location() }) as i32
}