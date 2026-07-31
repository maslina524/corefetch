use core::ptr;

use crate::os::windows::WideCharToMultiByte;

const CP_UTF8: u32 = 65001;
const DEFAULT: u8  = b'?';

pub unsafe fn utf16le_to_utf8(src: &[u16], src_len: isize, dst: &mut [u8], dst_size: usize) -> usize {
    // SAFETY: Passing invalid UTF-16-LE bytes will panic;
    // you should only use this function when you are 
    // absolutely sure the bytes are completely valid,
    // or for converting strings created by the WinAPI.

    let ret = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src as *const _ as *const u16, 
            src_len as i32, 
            dst.as_mut_ptr(), 
            dst_size as i32, 
            &DEFAULT, 
            ptr::null_mut()
        ) 
    };
    if ret == 0 { panic!("WideCharToMultiByte error!"); }
    ret as usize
}