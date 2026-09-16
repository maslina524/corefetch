use alloc::{
    vec::Vec
};

use crate::{
    cfg_if, 
    modules::Disk
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod linux;
    }
}

pub fn get_disks() -> Vec<Disk> {
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            Vec::new()
        } else if #[cfg(any(target_os = "linux", target_os = "android"))] {
            linux::get_disks_linux()
        }
    }
}

