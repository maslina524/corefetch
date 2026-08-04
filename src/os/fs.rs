use core::{
    ffi::c_void,
    ptr
};

use crate::os::{
    error::{self, ErrorCode},
    path::Path,
    windows::{CreateFileW, HANDLE}
};

const FILE_SHARE_READ      : u32         = 0x0001;
const OPEN_EXISTING        : u32         = 0x0003;
const FILE_ATTRIBUTE_NORMAL: u32         = 0x0080;
const INVALID_HANDLE       : *mut c_void = (-1isize).cast_unsigned() as *mut c_void;

pub type FileHandle = HANDLE;

#[repr(u32)]
pub enum Access {
    Read      = 0x8000_0000,
    Write     = 0x4000_0000,
    ReadWrite = 0x8000_0000 | 0x4000_0000,
    All       = 0x1000_0000
}

pub struct File(FileHandle);

impl File {
    pub fn open(path: impl Into<Path>, access: Access) -> error::Result<Self> {
        let path = path.into();
        let path_wide = path.as_utf16le_str()?;

        // SAFETY: Parameters are fully correct, return value is checked
        let handle = unsafe {
            CreateFileW(
                path_wide.as_ptr(), 
                access as u32, 
                FILE_SHARE_READ, 
                ptr::null(), 
                OPEN_EXISTING, 
                FILE_ATTRIBUTE_NORMAL, 
                ptr::null_mut()
            )
        };
        if handle == INVALID_HANDLE {
            return Err(ErrorCode::last());
        }
        
        Ok(Self(handle))
    }
}