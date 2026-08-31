use core::ptr;

use alloc::string::String;

crate::cfg_if! {
    if #[cfg(target_os = "windows")] {
        use crate::{
            windows::link::{GetLocaleInfoEx},
            windows::error::ErrorCode
        };

        const LOCALE_NAME_MAX_LENGTH  : usize            = 85;
        const LOCALE_NAME_USER_DEFAULT: *const u16       = ptr::null_mut();
        const LOCALE_SNAME            : u32              = 0x005c;

        pub fn locale() -> String {
            let mut buf = [0u16; LOCALE_NAME_MAX_LENGTH];
            // SAFETY: Just a WinAPI function, the return value is checked
            let len = unsafe {
                GetLocaleInfoEx(
                    LOCALE_NAME_USER_DEFAULT, 
                    LOCALE_SNAME, 
                    buf.as_mut_ptr() , 
                    LOCALE_NAME_MAX_LENGTH as i32
                )
            };
            if len == 0 { ErrorCode::last().panic(); }

            let len_usize = len as usize;
            String::from_utf16_lossy(&buf[..len_usize]).rsplit('\0').collect()
        }
    } else if #[cfg(target_os = "linux")] {
        use core::ffi::CStr;

        use crate::linux::libc::setlocale;
        
        const LC_ALL: i32 = 0;

        pub fn locale() -> String {
            setlocale(LC_ALL, c"".as_ptr());
            let ptr = setlocale(LC_ALL, ptr::null());
            let c_str = unsafe { CStr::from_ptr(ptr) };
            c_str.to_string_lossy().into_owned()
        }
    }
}
