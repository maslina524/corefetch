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
    if len == 0 {
        return Ok(String::new());
    }

    // SAFETY: Just a WinAPI function, the return value is checked
    let size = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            i32::try_from(len).expect("UNREACHABLE"), 
            ptr::null_mut(), 
            0, 
            ptr::null(), 
            ptr::null_mut()
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let size_usize = usize::try_from(size).expect("UNREACHABLE");
    let mut buf = Vec::with_capacity(size_usize);

    // SAFETY: Just a WinAPI function, the return value is checked
    let ret = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            i32::try_from(len).expect("UNREACHABLE"), 
            buf.as_mut_ptr(), 
            size, 
            ptr::null(), 
            ptr::null_mut()
        ) 
    };
    if ret == 0 { 
        return Err(ErrorCode::last()) 
    }
    
    let ret_usize = usize::try_from(ret).expect("UNREACHABLE");

    // SAFETY: WinAPI modifies data in `Vec<_>`, you must update the len
    unsafe { buf.set_len(ret_usize) };
    let string = String::from_utf8(buf).unwrap();
    Ok(string)
}

pub fn utf8_to_utf16le(src: impl Into<String>) -> error::Result<Vec<u16>> {
    let src = src.into();
    if src.is_empty() {
        return Ok(Vec::new());
    }

    // SAFETY: Just a WinAPI function, the return value is checked
    let size = unsafe {
        MultiByteToWideChar(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            i32::try_from(src.len()).expect("UNREACHABLE"), 
            ptr::null_mut(), 
            0,
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let size_usize = usize::try_from(size).expect("UNREACHABLE");
    let mut buf = Vec::with_capacity(size_usize);

    // SAFETY: Just a WinAPI function, the return value is checked
    let ret = unsafe {
        MultiByteToWideChar(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            i32::try_from(src.len()).expect("UNREACHABLE"), 
            buf.as_mut_ptr(), 
            size
        ) 
    };
    if ret == 0 { 
        return Err(ErrorCode::last()) 
    }
    
    let ret_usize = usize::try_from(ret).expect("UNREACHABLE");
    // SAFETY: WinAPI modifies data in `Vec<_>`, you must update the len

    unsafe { buf.set_len(ret_usize) };
    Ok(buf)
}

pub fn wide(src: impl Into<String>) -> error::Result<Vec<u16>> {
    let mut vec = utf8_to_utf16le(src)?;
    vec.push(0);
    Ok(vec)
}