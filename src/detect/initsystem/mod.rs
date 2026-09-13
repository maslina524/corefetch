use alloc::string::String;

use crate::imp::path::Path;

crate::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    } else if #[cfg(target_os = "android")] {
        mod android;
    }
}

pub struct InitSystemInfo {
    pub exe: Path,
    pub pid: u32,
    pub name: String,
    pub version: String
}