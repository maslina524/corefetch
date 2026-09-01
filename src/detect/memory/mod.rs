use crate::{cfg_if, formats::Size};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[derive(Default)]
pub struct MemoryInfo {
    pub total: Size,
    pub in_use: Size
}