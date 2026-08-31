use alloc::string::String;

crate::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
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