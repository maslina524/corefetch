use crate::{cfg_if, formats::MemorySize};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[derive(Default)]
pub struct MemoryInfo {
    pub total: MemorySize,
    pub in_use: MemorySize
}