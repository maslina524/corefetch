use alloc::vec::Vec;

use crate::{
    modules::Disk,
    windows::link::GetLogicalDrives
};

pub fn get_disks_windows() -> Vec<Disk> {
    // SAFETY: Completely safe
    let ret = unsafe {
        GetLogicalDrives()
    };
    Vec::new()
}