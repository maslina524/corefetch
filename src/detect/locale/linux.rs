use core::{ffi::CStr, ptr};

use crate::{detect::locale::LocaleInfo, linux::libc::setlocale};

const LC_ALL: i32 = 0;

impl LocaleInfo {
    pub fn new() -> Self {
        setlocale(LC_ALL, c"".as_ptr());
        let ptr = setlocale(LC_ALL, ptr::null());
        // SAFETY: libs are guaranteed to store a valid cstr
        let c_str = unsafe { CStr::from_ptr(ptr) };
        let locale = c_str.to_string_lossy().into_owned();

        Self { locale }
    }
}