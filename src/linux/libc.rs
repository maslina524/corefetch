#![allow(
    non_camel_case_types, 
    reason = "C type"
)]

use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

use crate::sync::OnceLock;

pub type FILE = *mut c_void;
pub type DIR = *mut c_void;

pub type c_size = usize;
pub type c_ssize = isize;
pub type c_mode = c_uint;
pub type c_time = c_long; // i64
pub type c_clockid = c_int; // WORK ONLY IN LINUX
pub type c_ino = usize;
pub type c_off = isize;
pub type c_uid = u32;
pub type c_gid = u32;
pub type c_pid = i32;

unsafe extern "C" {
    pub safe fn write(fd: c_int, buf: *const c_char, len: c_size);
    pub unsafe fn malloc(size: c_size) -> *mut c_void;
    pub unsafe fn free(ptr: *mut c_void);
    pub unsafe fn calloc(nmemb: c_size, size: c_size) -> *mut c_void;
    pub unsafe fn realloc(ptr: *mut c_void, size: c_size) -> *mut c_void;
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
    pub safe fn opendir(path: *const c_char) -> DIR;
    pub safe fn readdir(dirp: *mut DIR) -> *mut Dirent;
    pub safe fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    pub safe fn getuid() -> c_uid;
    pub safe fn getpwuid(uid: c_uid) -> *const Passwd;
    pub safe fn gethostname(name: *mut c_char, len: c_size) -> c_int;
    pub safe fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: c_size) -> c_ssize;
    pub safe fn getpid() -> c_pid;
    pub safe fn getcwd(buf: *mut c_char, size: c_size) -> *mut c_char;
    pub safe fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    pub safe fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub safe fn dlclose(handle: *mut c_void) -> c_int;
    pub safe fn dlerror() -> *mut c_char;
    pub safe fn sysconf(name: c_int) -> c_long;
    pub safe fn localtime(timep: *const c_time) -> *mut Tm;
    pub safe fn time(tloc: *mut c_time) -> c_time;
}

static SYSINFO: OnceLock<Sysinfo> = OnceLock::new();

pub fn get_sysinfo() -> &'static Sysinfo {
    SYSINFO.get_or_init(|| {
        let mut info = Sysinfo::default();
        sysinfo(&raw mut info);
        info
    })
}

pub fn errno() -> i32 {
    (unsafe { *errno_location() }) as i32
}

#[repr(C)]
#[derive(Default)]
pub struct InAddr {
    pub s_addr: c_uint,
}

#[repr(C)]
#[derive(Default)]
pub struct SockaddrIn {
    pub sin_family: c_ushort,
    pub sin_port: c_ushort,
    pub sin_addr: InAddr,
    pub sin_zero: [c_uchar; 8],
}

#[repr(C)]
#[derive(Default)]
pub struct Hostent {
    pub h_name: *mut c_char,
    pub h_aliases: *mut *mut c_char,
    pub h_addrtype: c_int,
    pub h_length: c_int,
    pub h_addr_list: *mut *mut c_char,
}

#[repr(C)]
#[derive(Default)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

#[repr(C)]
#[derive(Default)]
pub struct Passwd {
    pub pw_name: *mut c_char,
    pub pw_passwd: *mut c_char,
    pub pw_uid: c_uid,
    pub pw_gid: c_gid,
    pub pw_gecos: *mut c_char,
    pub pw_dir: *mut c_char,
    pub pw_shell: *mut c_char,
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

#[repr(C)]
pub struct Dirent {
    pub d_ino: c_ino,
    pub d_off: c_off,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
}

impl Default for Dirent {
    fn default() -> Self {
        Self { 
            d_ino: Default::default(), 
            d_off: Default::default(), 
            d_reclen: Default::default(), 
            d_type: Default::default(), 
            d_name: [0; 256]
        }
    }
}