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
    pub name: String,
    pub id: String,
    pub id_like: String,
    pub version: String,
    pub version_id: String,
    pub codename: String,
    pub variant: String,
    pub variant_id: String,
    pub nerd: char
}

#[cfg(target_os = "android")]
pub fn get_id() -> String {
    alloc::borrow::ToOwned::to_owned("android")
}

#[cfg(target_os = "linux")]
pub fn get_id() -> String {
    use crate::linux::parser::LinuxInfo;

    let os_release = LinuxInfo::parse_os_release().unwrap();
    os_release.get_default("ID", &"Unknown")
}

#[cfg(target_os = "windows")]
pub fn get_id() -> String {
    use crate::windows::env;

    let (_, _, build) = env::get_version();
    let version = OsInfo::version(build as i32).to_owned();
    format!("Windows {version}")
}
