use alloc::borrow::ToOwned;

use crate::{
    warning,
    todo_or_default,
    linux::fs,
    linux::path::Path,
    detect::initsystem::InitSystemInfo
};

impl InitSystemInfo {
    pub fn new() -> Self {
        let name = fs::read_to_string("/proc/1/comm")
            .unwrap_or("Unknown".to_owned());

        let exe = if let Ok(c) = fs::read_to_string("/proc/1/cmdline") {
            fs::read_link(c, 64)
                .and_then(|s| Some(Path::from(s)))
                .unwrap_or(Path::new())
        } else {
            warning!("Failed to read /proc/1/cmdline");
            Path::new()
        };

        let version = todo_or_default!(
            "Too time-consuming to implement for every distribution and system",
            "0.0.0.0".to_owned()
        );

        Self { 
            exe, 
            pid: 1, 
            name, 
            version
        }
    }
}