use core::ptr;

use alloc::string::String;

use crate::{
    os::windows::{GetLocaleInfoEx},
    os::error::ErrorCode
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