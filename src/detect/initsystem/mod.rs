use alloc::string::String;

use crate::{
    imp::path::Path,
    cfg_if
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
    }
}

pub struct InitSystemInfo {
    pub exe: Path,
    pub pid: u32,
    pub name: String,
    pub version: String
}