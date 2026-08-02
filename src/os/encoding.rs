use core::ptr;

use alloc::{
    string::String,
    vec::Vec
};

use crate::os::{
    error::{self, ErrorCode},
    windows::{WideCharToMultiByte, MultiByteToWideChar}
};

const CP_UTF8: u32 = 65001;

pub fn utf16le_to_utf8(
    src: &[u16], 
    len: isize,
) -> error::Result<String> {
    // SAFETY: Passing invalid UTF-16-LE bytes will panic;
    // you should only use this function when you are 
    // absolutely sure the bytes are completely valid,
    // or for converting strings created by the WinAPI.
    
    let size = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            len as i32, 
            ptr::null_mut(), 
            0, 
            ptr::null(), 
            ptr::null_mut()
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let mut buf = Vec::with_capacity(size as usize);
    let ret = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            len as i32, 
            buf.as_mut_ptr(), 
            size, 
            ptr::null(), 
            ptr::null_mut()
        ) 
    };
    if ret == 0 { 
        return Err(ErrorCode::last()) 
    }
    
    let string = String::from_utf8(buf).unwrap();
    Ok(string)
}

pub fn utf8_to_utf16le(
    src: impl Into<String>
) -> error::Result<Vec<u16>> {
    // SAFETY: Passing invalid UTF-16-LE bytes will panic;
    // you should only use this function when you are 
    // absolutely sure the bytes are completely valid,
    // or for converting strings created by the WinAPI.
    
    let src = src.into();
    let size = unsafe {
        MultiByteToWideChar(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            src.len() as i32, 
            ptr::null_mut(), 
            0,
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let mut buf = Vec::with_capacity(size as usize);
    let ret = unsafe {
        MultiByteToWideChar(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            src.len() as i32, 
            buf.as_mut_ptr(), 
            size
        ) 
    };
    if ret == 0 { 
        return Err(ErrorCode::last()) 
    }
    
    Ok(buf)
}

#[macro_export]
macro_rules! L {
    ($v:expr) => {{
        $crate::os::encoding::utf8_to_utf16le($v).unwrap().as_ptr() as *const u16
    }};
}