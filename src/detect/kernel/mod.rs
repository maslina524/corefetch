use alloc::string::String;

use crate::{
    formats::Size,
    cfg_if
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

pub struct KernelInfo {
    pub sysname: &'static str,
    pub release: String,
    pub version: String,
    pub display_version: String,
    pub page_size: Size
}