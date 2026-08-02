use core::ptr;

use alloc::{
    string::String,
    vec::Vec
};

use crate::os::{
    error::{self, ErrorCode},
    windows::WideCharToMultiByte
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

