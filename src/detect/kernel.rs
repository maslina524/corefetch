use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    windows::env,
    windows::regedit::RegValue,
    windows::link::{SYSTEM_INFO, GetSystemInfo},
    formats::Size,
    format
};

pub const fn sysname() -> &'static str {
    "WIN32_NT"
}

fn ubr() -> u32 {
    let value = env::current_version().read("UBR").unwrap_or(RegValue::None);
    value.as_u32().unwrap_or(0)
}

pub fn release() -> String {
    let (major, minor, build) = env::get_version();
    let ubr = ubr();
    format!("{major}.{minor}.{build}.{ubr}")
}

pub fn version() -> String {
    let value = env::current_version().read("BuildLabEx").unwrap_or(RegValue::None);
    value.as_string().map(ToOwned::to_owned).unwrap_or_default()
}

pub fn page_size() -> Size {
    let mut si = SYSTEM_INFO::default();
    // SAFETY: Completely safe
    unsafe {
        GetSystemInfo(&raw mut si);
    }

    Size::from_bytes(si.dwPageSize as u64)
}

#[cfg(test)]
mod tests {
    use crate::detect::kernel;


    #[test]
    fn release_test() {
        let release = kernel::release();
        println!("{release}");
    }

    #[test]
    fn version_test() {
        let version = kernel::version();
        println!("{version}");
    }

    #[test]
    fn page_size_test() {
        let size = kernel::page_size();
        println!("{size:.02} KiB");
    }
}