use core::{
    ffi::{c_void, CStr},
    ptr,
    mem,
    slice
};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

use crate::{
    sync::OnceLock,
    windows::error::{self, ErrorCode},
    windows::fs::{Access, File},
    windows::path::Path,
    windows::encoding::wide,
    windows::link::{
        CONSOLE_SCREEN_BUFFER_INFO, CloseHandle, CommandLineToArgvW, CreateToolhelp32Snapshot, 
        EnumProcesses, FILETIME, GetCommandLineW, GetConsoleScreenBufferInfo, 
        GetSystemTimeAsFileTime, OSVERSIONINFOW, PROCESSENTRY32, Process32First, 
        Process32Next, RtlGetVersion, VerQueryValueW, GetFileVersionInfoW, GetFileVersionInfoSizeW
    }, 
    windows::regedit::{self, Hkey, RegValue, Regedit}
};

const EPOCH_DIFF           : u64               = 116_444_736_000_000_000;
const INVALID_HANDLE       : *mut c_void       = (-1isize).cast_unsigned() as *mut c_void;

static TERMINAL_HANDLE     : OnceLock<isize>   = OnceLock::new();
static CURRENT_VERSION     : OnceLock<Regedit> = OnceLock::new();

#[repr(C)]
#[derive(Default, Debug)]
#[allow(non_snake_case, reason = "Copied from the Windows docs")]
// https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo
struct VS_FIXEDFILEINFO {
  pub dwSignature: u32,
  pub dwStrucVersion: u32,
  pub dwFileVersionMS: u32,
  pub dwFileVersionLS: u32,
  pub dwProductVersionMS: u32,
  pub dwProductVersionLS: u32,
  pub dwFileFlagsMask: u32,
  pub dwFileFlags: u32,
  pub dwFileOS: u32,
  pub dwFileType: u32,
  pub dwFileSubtype: u32,
  pub dwFileDateMS: u32,
  pub dwFileDateLS: u32
}

pub fn current_version() -> &'static Regedit {
    CURRENT_VERSION.get_or_init(|| {
        Regedit::open(
            Hkey::LocalMachine, 
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion", 
            regedit::Access::Read
        ).unwrap()
    })
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
    let mut pids = [0u32; 1024];
    let mut needed = 0;

    // SAFETY: Completely safe
    let ret = unsafe {
        EnumProcesses(
            pids.as_mut_ptr(), 
            (pids.len() * 4) as u32, 
            &raw mut needed
        )
    };
    if ret == 0 {
        ErrorCode::last().panic();
    }
    needed as usize / 4
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

pub fn timestamp_hours() -> u64 {
    timestamp_secs() / 3600
}

pub fn args() -> Vec<String> {
    // SAFETY: Completely safe
    let ptr = unsafe { GetCommandLineW() };
    let mut argv_count = 0;

    // SAFETY: Just a WinAPI function,
    // I don't know what to write, it's safe
    let argv_ptrs = unsafe {
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

pub fn find_pid_by_name(name: &str) -> u32 {
    let mut pid = 0;
    // SAFETY: Completely safe
    let snapshot = unsafe { CreateToolhelp32Snapshot(2, 0) };
    if snapshot == INVALID_HANDLE { return 0; }

    let mut pe = PROCESSENTRY32 {
        dwSize: size_of::<PROCESSENTRY32>() as u32,
        .. PROCESSENTRY32::default()
    };

    // SAFETY: Completely safe
    let first = unsafe { Process32First(snapshot, &raw mut pe) };
    if first != 0 {
        loop {
            // SAFETY: 
            let proc_name = unsafe { CStr::from_ptr(pe.szExeFile.as_ptr()) };
            if proc_name.to_bytes() == name.as_bytes() {
                pid = pe.th32ProcessID;
                break;
            }
            // SAFETY: Completely safe
            let ret = unsafe { Process32Next(snapshot, &raw mut pe) };
            if ret == 0 {
                break;
            }
        }
    }

    // SAFETY: Completely safe
    unsafe { CloseHandle(snapshot) };
    pid
}

pub fn get_file_product_version(path: impl Into<Path>) -> error::Result<(u32, u32, u32, u32)> {
    let path_str = path.into().into_inner();
    let path_wide = wide(path_str)?;

    // SAFETY: Completely safe
    let buf_size = unsafe { 
        GetFileVersionInfoSizeW(
            path_wide.as_ptr(), 
            ptr::null_mut()
        ) 
    };
    if buf_size == 0 {
        return Err(ErrorCode::last());
    }

    let mut buf = vec![0u8; buf_size as usize];
    // SAFETY: Completely safe
    let ret = unsafe {
        GetFileVersionInfoW(
            path_wide.as_ptr(),
            0,
            buf_size,
            buf.as_mut_ptr().cast(),
        )
    };
    if ret == 0 {
        return Err(ErrorCode::last());
    }

    let mut info_ptr: *mut c_void = ptr::null_mut();
    let mut len = 0;
    let wide_subblock = wide("\\")?;

    // SAFETY: Completely safe
    let ret = unsafe {
        VerQueryValueW(
            buf.as_ptr().cast(),
            wide_subblock.as_ptr(),
            &raw mut info_ptr,
            &raw mut len,
        )
    };
    if ret == 0 {
        return Err(ErrorCode::last());
    }

    // SAFETY: A guaranteed non-null pointer
    let info = unsafe { &*(info_ptr as *const VS_FIXEDFILEINFO) };

    let major = (info.dwProductVersionMS >> 16) & 0xFFFF;
    let minor = info.dwProductVersionMS & 0xFFFF;
    let build = (info.dwProductVersionLS >> 16) & 0xFFFF;
    let rev = info.dwProductVersionLS & 0xFFFF;

    Ok((major, minor, build, rev))
}

#[cfg(test)]
mod tests {
    use crate::windows::env;
  
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
        assert!(args[0].contains("corefetch"));
    }

    #[test]
    fn terminal_size_test() {
        let size = env::terminal_size();
        println!("{size:?}");
    }

    #[test]
    fn find_pid_test() {
        let name = "System";
        let pid = env::find_pid_by_name(name);
        assert_eq!(pid, 4);
    }

    #[test]
    fn file_version_test() {
        let path = "C:/Windows/System32/smss.exe";
        let version = env::get_file_product_version(path);
        println!("{version:#?}");
    }
}