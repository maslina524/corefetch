use core::ptr;

use alloc::string::String;

use crate::{
    format, 
    format_module, 
    impl_display_for_module, 
    format_for_module, 
    logo::LogoInfo, 
    modules::Module, 
    os::error::ErrorCode,
    os::windows::GetLocaleInfoEx, 
    sync::OnceLock
};

const LOCALE_NAME_MAX_LENGTH  : usize            = 85;
const LOCALE_NAME_USER_DEFAULT: *const u16       = ptr::null_mut();
const LOCALE_SNAME            : u32              = 0x005c;

static LOCALE                 : OnceLock<Locale> = OnceLock::new();

#[derive(Debug)]
pub struct Locale {
    pub result: String
}

impl Module for Locale {
    fn new() -> Self {
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
        let result = String::from_utf16_lossy(&buf[..len_usize]).rsplit('\0').collect();

        Self { result }
    }

    fn get() -> &'static Self {
        LOCALE.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Locale"
    }

    fn title(&self) -> &'static str {
        "{result}"
    }

    fn string_name(&self) -> &'static str {
        "locale"
    }

    format_for_module!(Locale, result);
}


impl_display_for_module!(Locale);