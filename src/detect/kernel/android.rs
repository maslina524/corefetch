use core::ffi::CStr;

use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    abort, detect::kernel::KernelInfo, formats::MemorySize, linux::{
        fs, libc::{Utsname, sysconf, uname}
    }
};

const SYSNAME: &str = "Linux";
const SC_PAGESIZE: i32 = 30;

impl KernelInfo {
    pub fn new() -> Self {
        let mut info = Utsname::default();
        let (release, version) = if uname(&raw mut info) == 0 {
            let release = if info.release.is_null() {
                "Unknown".to_owned()
            } else {
                unsafe { CStr::from_ptr(info.release) }
                    .to_string_lossy()
                    .into_owned()
            };

            let version = if info.version.is_null() {
                "Unknown".to_owned()
            } else {
                unsafe { CStr::from_ptr(info.version) }
                    .to_string_lossy()
                    .into_owned()
            };

            (release, version)
        } else {
            ("Unknown".to_owned(), "Unknown".to_owned())
        };

        Self { 
            sysname: SYSNAME, 
            release, 
            version, 
            display_version: String::new(), 
            page_size: Self::page_size()
        }
    }

    fn page_size() -> MemorySize {
        let bytes = sysconf(SC_PAGESIZE);
        MemorySize::from_bytes(bytes as u64)
    }
}