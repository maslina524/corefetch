use core::{
    ffi::c_void,
    mem
};

use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::os::{
    error::{self, ErrorCode},
    windows::*
};

const TH32CS_SNAPPROCESS: u32 = 0x00000002;
const INVALID_HANDLE: *mut c_void = -1isize as usize as *mut c_void;

pub struct OsVersion {
    pub sysname: String,
    pub name: String,
    pub version: String,
    pub codename: String,
    pub variant: String
}

pub fn os_version() -> error::Result<OsVersion> {
    let sysname = "WIN32_NT".to_owned();
    let name = "Windows".to_owned();

    let mut osvi = OSVERSIONINFOW::default();
    unsafe { RtlGetVersion(&mut osvi) };

    let build = osvi.dwBuildNumber;
    
    let version = match build {
        6002 => "Vista",
        7601 => "7",
        9200 => "8",
        9600 => "8.1",
        _ if (10240..22000).contains(&build) => "10",
        _ if (22000..=28000).contains(&build) => "11",
        _ => "Unknown"
    }.to_owned();

    let codename = match build {
        10240 => "1507",
        10586 => "1511",
        14393 => "1607",
        15063 => "1703",
        16299 => "1709",
        17134 => "1803",
        17763 => "1809",
        18362 => "1903",
        18363 => "1909",
        19041 => "2004",
        19042 => "20H2",
        19043 => "21H1",
        19044 => "21H2",
        19045 => "22H2",
        22000 => "21H2",
        22621 => "22H2",
        22631 => "23H2",
        26100 => "24H2",
        26200 => "25H2",
        28000 => "26H1",
        _ => ""
    }.to_owned();

    let variant = "Home".to_owned();

    Ok(OsVersion { sysname, name, version, codename, variant })
}

pub fn processes_count() -> usize {
    let snapshot = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    };
    if snapshot == INVALID_HANDLE { ErrorCode::last().panic(); }

    let mut pe = PROCESSENTRY32 {
        dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
        ..Default::default()
    };

    let mut count = 0;
    let first = unsafe {
        Process32First(snapshot, &mut pe)
    };
    if first == 1 {
        count += 1;
        loop {
            let next = unsafe { Process32Next(snapshot, &mut pe) };
            if next == 0 { break; }
            count += 1;
        }
    }

    count
}