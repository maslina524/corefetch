use alloc::{
    string::String,
    borrow::Cow
};

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
    pub id: String,
    pub id_like: String,
    pub version: String,
    pub version_id: String,
    pub codename: Cow<'static, str>,
    pub variant: Cow<'static, str>,
    pub variant_id: Cow<'static, str>,
    pub nerd: char
}

#[cfg(target_os = "android")]
pub fn get_id() -> String {
    use alloc::borrow::ToOwned;

    "android".to_owned()
}

#[cfg(target_os = "linux")]
pub fn get_id() -> String {
    use alloc::borrow::ToOwned;
    use crate::linux::parser::LinuxInfo;

    let os_release = LinuxInfo::parse_os_release().unwrap();
    os_release.get_default("ID", "Unknown").to_owned()
}

#[cfg(target_os = "windows")]
pub fn get_id() -> String {
    use crate::{
        windows::env,
        format
    };

    let (_, _, build) = env::get_version();
    let version = OsInfo::version(build as i32);
    format!("Windows {version}")
}