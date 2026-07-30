use crate::os::windows::WideCharToMultiByte;

const CP_UTF8: u32 = 65001;
const DEFAULT: u8  = b'?';

pub unsafe fn utf16le_to_utf8(src: *const u16, len: isize, dst: &mut [u8], size: usize) -> usize {
    // SAFETY: Passing invalid UTF-16-LE bytes will panic;
    // you should only use this function when you are 
    // absolutely sure the bytes are completely valid,
    // or for converting strings created by the WinAPI.

    let mut default_chs = 0;
    let ret = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src, 
            len as i32, 
            dst.as_mut_ptr(), 
            size as i32, 
            &DEFAULT, 
            &mut default_chs
        ) 
    };
    if ret == 0 || default_chs == 0 { panic!("WideCharToMultiByte error!"); }
    ret as usize
}