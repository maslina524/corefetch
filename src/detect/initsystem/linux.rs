use alloc::borrow::ToOwned;

use crate::{
    warning,
    linux::fs,
    linux::path::Path,
    detect::initsystem::InitSystemInfo
};

impl InitSystemInfo {
    #[todo::todo("Too time-consuming to implement for every distribution and system")]
    pub fn new() -> Self {
        let name = fs::read_to_string("/proc/1/comm")
            .unwrap_or_else(|_| "Unknown".to_owned());

        let exe = fs::read_to_string("/proc/1/cmdline").map_or_else(|_| {
            warning!("Failed to read /proc/1/cmdline");
            Path::new()
        }, |c| fs::read_link(c, 64).map_or(Path::new(), Path::from));

        let version = "0.0.0.0".to_owned();

        Self { 
            exe, 
            pid: 1, 
            name, 
            version
        }
    }
}