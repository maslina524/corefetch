use core::{
    ffi::c_void,
    mem
};

use crate::os::{
    error::ErrorCode, windows::{CreateToolhelp32Snapshot, PROCESSENTRY32, Process32First, Process32Next}
};

const TH32CS_SNAPPROCESS: u32 = 0x00000002;
const INVALID_HANDLE: *mut c_void = -1isize as usize as *mut c_void;

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