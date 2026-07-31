use core::ptr;

use alloc::string::String;

use crate::{
    os::windows::GetLocaleInfoEx,
    sync::OnceLock
};

static LOCALE                 : OnceLock<Locale> = OnceLock::new();

const LOCALE_NAME_MAX_LENGTH  : usize            = 85;
const LOCALE_NAME_USER_DEFAULT: *const u16       = ptr::null_mut();
const LOCALE_SNAME            : u32              = 0x0000005c;

pub fn get() -> &'static Locale {
    LOCALE.get_or_init(|| {
        Locale::new()
    })
}

#[derive(Debug)]
pub struct Locale {
    result: String
}

impl Locale {
    pub fn new() -> Self {
        let mut buf = [0u16; LOCALE_NAME_MAX_LENGTH];
        let len = unsafe {
            GetLocaleInfoEx(
                LOCALE_NAME_USER_DEFAULT, 
                LOCALE_SNAME, 
                buf.as_mut_ptr() , 
                LOCALE_NAME_MAX_LENGTH as i32
            )
        };

        let result = String::from_utf16_lossy(&buf[..len as usize]).rsplit("\0").collect();

        Self {
            result
        }
    }
}

impl core::fmt::Display for Locale {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.result)
    }
}