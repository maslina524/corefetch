use core::{
    ffi::c_void,
    mem,
    slice
};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec
};

use crate::{
    os::error::{self, ErrorCode},
    os::fs::{Access, File},
    os::windows::{
        CONSOLE_SCREEN_BUFFER_INFO, CloseHandle, CommandLineToArgvW, CreateToolhelp32Snapshot,
        FILETIME, GetCommandLineW, GetConsoleScreenBufferInfo, GetSystemTimeAsFileTime,
        OSVERSIONINFOW, PROCESSENTRY32, Process32First, Process32Next, RtlGetVersion
    },
    os::regedit::{self, Regedit, RegValue, Hkey},
    sync::OnceLock
};

const TH32CS_SNAPPROCESS   : u32         = 0x0002;
const INVALID_HANDLE       : *mut c_void = (-1isize).cast_unsigned() as *mut c_void;
const TIME_ZONE_ID_INVALID : u32         = u32::MAX;
const TIME_ZONE_ID_DAYLIGHT: u32         = 2;
const EPOCH_DIFF           : u64         = 116_444_736_000_000_000;

static TERMINAL_HANDLE     : OnceLock<isize> = OnceLock::new();

pub struct OsVersion {
    pub sysname: &'static str,
    pub name: &'static str,
    pub version: String,
    pub codename: String,
    pub variant: String
}

pub fn get_version() -> (u32, u32, u32) {
    let mut osvi = OSVERSIONINFOW::default();
    // SAFETY: Completely safe
    unsafe { RtlGetVersion(&raw mut osvi) };

    (
        osvi.dwMajorVersion,
        osvi.dwMinorVersion,
        osvi.dwBuildNumber
    )
}

pub fn os_version() -> OsVersion {
    let sysname = "WIN32_NT";
    let name = "Windows";

    let (_, _, build) = get_version();
    
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
        950 => "4.00",
        1381 => "NT 4.0",
        1998 => "4.10",
        2195 => "NT 5.0",
        3000 => "4.90",
        2600 | 2700 | 2710 => "NT 5.1",
        3790 => "NT 5.2",
        6002 => "NT 6.0",
        7601 => "NT 6.1",
        9200 => "NT 6.2",
        9600 => "NT 6.3",
        10240 => "NT 10.0",
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
        19044 | 22000 => "21H2",
        19045 | 22621 => "22H2",
        22631 => "23H2",
        26100 => "24H2",
        26200 => "25H2",
        28000 => "26H1",
        _ => ""
    }.to_owned();

    let value = Regedit::open(
        Hkey::LocalMachine, 
        "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", 
        regedit::Access::Read
    ).unwrap().read("ProductName").unwrap_or(RegValue::None);

    let mut variant = value
        .as_string()
        .unwrap_or("")
        .to_owned();

    let idx = variant
        .rfind(' ')
        .unwrap_or(0);
    
    variant = variant[idx + 1..].to_owned();

    OsVersion { sysname, name, version, codename, variant }
}

pub fn terminal_handle() -> isize {
    *TERMINAL_HANDLE.get_or_init(|| {
        let file = File::open("CONOUT$", Access::Read).unwrap();
        let handle = file.as_handle();
        assert!(!handle.is_null());
        mem::forget(file);
        handle as isize
    })
}

pub fn terminal_size() -> (usize, usize) {
    let mut buf = CONSOLE_SCREEN_BUFFER_INFO::default();

    // SAFETY: Comletely safe
    let ret = unsafe {
        GetConsoleScreenBufferInfo(
            terminal_handle() as *mut c_void, 
            &raw mut buf
        )
    };
    if ret == 0 {
        ErrorCode::last().panic();
    }

    let w = buf.srWindow.Right - buf.srWindow.Left + 1;
    let h = buf.srWindow.Bottom - buf.srWindow.Top + 1;
    (w as usize, h as usize)
}

pub fn close_terminal_handle() -> error::Result<()> {
    // SAFETY: Completely safe
    let ret = unsafe {
        CloseHandle(terminal_handle() as *mut c_void)
    };
    if ret == 0 {
        return Err(ErrorCode::last());
    }
    Ok(())
}

pub fn processes_count() -> usize {
    // SAFETY: Just a WinAPI function, the return value is checked
    let snapshot = unsafe {
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    };
    if snapshot == INVALID_HANDLE {
        ErrorCode::last().panic();
    }

    let mut pe = PROCESSENTRY32 {
        dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
        ..Default::default()
    };

    let mut count = 0;
    // SAFETY: Completely safe
    let first = unsafe {
        Process32First(snapshot, &raw mut pe)
    };
    
    if first == 1 {
        count += 1;
        loop {
            // SAFETY: Just a WinAPI function, the return value is checked
            let next = unsafe {
                Process32Next(snapshot, &raw mut pe)
            };
            if next == 0 {
                break;
            }
            count += 1;
        }
    }

    // SAFETY: Completely safe
    unsafe { CloseHandle(snapshot); }
    
    count
}

pub fn timestamp_mils() -> u64 {
    let mut info = FILETIME::default();

    // SAFETY: Completely safe
    unsafe {
        GetSystemTimeAsFileTime(&raw mut info);
    };

    let uli = ((info.dwHighDateTime as u64) << 32) | (info.dwLowDateTime as u64);
    (uli - EPOCH_DIFF) / 10_000
}

pub fn timestamp_secs() -> u64 {
    timestamp_mils() / 1_000
}

pub fn timestamp_mins() -> u64 {
    timestamp_secs() / 60
}

pub fn timestamp_hours() -> u64 {
    timestamp_secs() / 3600
}

pub fn args() -> Vec<String> {
    // SAFETY: Completely safe
    let ptr = unsafe { GetCommandLineW() };
    let mut argv_count = 0;

    // SAFETY: Just a WinAPI function,
    // I don't know what to write, it's safe
    let mut argv_ptrs = unsafe {
        CommandLineToArgvW(
            ptr, 
            &raw mut argv_count
        )
    };

    let mut ret = Vec::new();
    for i in 0..argv_count {
        // SAFETY: Moving strictly within the allocated memory by WinAPI
        let start_ptr = unsafe { *argv_ptrs.add(i as usize) };

        let mut len = 0;
        // SAFETY: Moving until the end of the allocated string
        while unsafe { *start_ptr.add(len) } != 0 {
            len += 1;
        }

        // SAFETY: All parameters were correctly calculated earlier.
        let slice = unsafe { slice::from_raw_parts(start_ptr, len) };
        let string = String::from_utf16_lossy(slice);
        ret.push(string);
    }

    ret
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::os::env;
  
    extern crate std;

    #[test]
    fn timestamp_secs_test() {
        for _ in 0..100_000 {
            let timestamp = env::timestamp_secs();
            println!("{timestamp}");
        }
    }

    #[test]
    fn args_test() {
        let args = env::args();
        println!("{args:?}");

        assert!(!args.is_empty());
        assert!(args[0].contains("nofetch"));
    }

    #[test]
    fn terminal_size_test() {
        let size = env::terminal_size();
        println!("{size:?}");
    }
}