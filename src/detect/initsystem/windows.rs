use alloc::borrow::ToOwned;

use crate::{
    format, 
    warning,
    windows::env,
    windows::path::Path,
    detect::initsystem::InitSystemInfo
};

impl InitSystemInfo {
    pub fn new() -> Self {
        let path = "C:/Windows/System32/smss.exe".to_owned();
        let name = "smss".to_owned();
        let pid = env::find_pid_by_name(&format!("{name}.exe"));
        let version = match env::get_file_product_version(&path) {
            Ok(v) => v,
            Err(e) => {
                warning!("Failed to get file version: {e} (initsystem)");
                "0.0.0.0".to_owned()
            }
        };

        Self { 
            exe: Path::from(path), 
            pid, 
            name, 
            version
        }
    }
}