use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    format, 
    warning, 
    windows::env,
    windows::path::Path
};

#[derive(Debug)]
pub struct InitSystemInfo {
    pub exe: Path,
    pub pid: u32,
    pub name: String,
    pub version: (u32, u32, u32, u32)
}

impl InitSystemInfo {
    pub fn new() -> Self {
        let path = Self::path_str();
        let name = Self::name();
        let pid = env::find_pid_by_name(&format!("{name}.exe"));
        let version = match env::get_file_product_version(&path) {
            Ok(v) => v,
            Err(e) => {
                warning!("Failed to get file version: {e} (initsystem)");
                (0, 0, 0, 0)
            }
        };

        Self { 
            exe: Path::from(path), 
            pid, 
            name, 
            version
        }
    }

    pub fn path_str() -> String {
        "C:/Windows/System32/smss.exe".to_owned()
    }

    pub fn name() -> String {
        "smss".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::initsystem::InitSystemInfo;

    #[test]
    fn init_info_test() {
        let info = InitSystemInfo::new();
        println!("{info:#?}");
    }
}