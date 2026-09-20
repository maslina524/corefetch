use alloc::string::String;

use crate::cfg_if;

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    } else if #[cfg(target_os = "android")] {
        mod android;
    }
}

pub struct OsInfo {
    pub sysname: &'static str,
    pub name: &'static str,
    pub id: &'static str,
    pub id_like: &'static str,
    pub version: String,
    pub version_id: String,
    pub codename: &'static str,
    pub variant: &'static str,
    pub variant_id: &'static str,
    pub nerd: char
}

#[cfg(target_os = "android")]
pub fn get_id() -> &'static str {
    "android"
}

#[cfg(target_os = "linux")]
pub fn get_id() -> &'static str {
    use crate::linux::parser::LinuxInfo;

    let os_release = LinuxInfo::parse_os_release().unwrap();
    os_release.get_default("ID", "Unknown")
}

#[cfg(target_os = "windows")]
pub fn get_id() -> &'static str {
    use crate::windows::env;
    use alloc::borrow::ToOwned;

    let (_, _, build) = env::get_version();
    let version = OsInfo::version(build as i32).to_owned();
    crate::format!("Windows {version}").leak()
}
