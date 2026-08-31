use alloc::string::String;

use crate::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

pub struct LocaleInfo {
    pub locale: String
}