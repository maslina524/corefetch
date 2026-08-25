use core::ptr;

use alloc::{
    string::String,
    vec::Vec,
    vec
};

use crate::windows::{
    error::{self, ErrorCode},
    link::{WideCharToMultiByte, MultiByteToWideChar}
};

const CP_UTF8: u32 = 65001;

#[derive(Clone, Copy)]
pub enum Utf16Len {
    NullTerminated,
    Len(usize)
}

pub fn utf16le_to_utf8(
    src: &[u16], 
    len: Utf16Len,
) -> error::Result<String> {
    let len_isize = match len {
        Utf16Len::NullTerminated => -1isize,
        Utf16Len::Len(l) => l as isize
    };
    if len_isize == 0 {
        return Ok(String::new());
    }

    // SAFETY: Just a WinAPI function, the return value is checked
    let size = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            len_isize as i32, 
            ptr::null_mut(), 
            0, 
            ptr::null(), 
            ptr::null_mut()
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let size_usize = size as usize;
    let mut buf = vec![0u8; size_usize];

    // SAFETY: Just a WinAPI function, the return value is checked
    let ret = unsafe {
        WideCharToMultiByte(
            CP_UTF8, 
            0, 
            src.as_ptr(), 
            len_isize as i32, 
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
            src.len() as i32, 
            ptr::null_mut(), 
            0,
        ) 
    };
    if size == 0 { 
        return Err(ErrorCode::last()) 
    }

    let mut buf = vec![0u16; size as usize];

    // SAFETY: Just a WinAPI function, the return value is checked
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

pub fn wide(src: impl Into<String>) -> error::Result<Vec<u16>> {
    let mut vec = utf8_to_utf16le(src)?;
    vec.push(0);
    Ok(vec)
}