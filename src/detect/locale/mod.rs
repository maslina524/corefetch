use alloc::string::String;

use crate::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
    }
}

pub struct LocaleInfo {
    pub locale: String
}