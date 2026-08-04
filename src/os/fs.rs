use core::{
    ffi::c_void,
    ptr
};

use alloc::{
    string::String,
    vec::Vec
};

use crate::{
    os::error::{self, ErrorCode},
    os::path::Path,
    os::windows::{CreateFileW, HANDLE, WriteFile}
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

    pub fn write(&self, buf: impl Into<Vec<u8>>) -> error::Result<()> {
        let mut buf = buf.into();
        let len = u32::try_from(buf.len()).expect("UNREACHABLE");
        let mut written = 0;

        // SAFETY: The handle is always correct, 
        // errors are checked, the function is safe
        let ret = unsafe {
            WriteFile(
                self.0, 
                buf.as_mut_ptr(), 
                len, 
                &raw mut written, 
                ptr::null_mut()
            )
        };
        if ret == 0 || written != len {
            return Err(ErrorCode::last());
        }
        
        let written_usize = usize::try_from(written).expect("UNREACHABLE");
        // SAFETY: WinAPI modifies data in `Vec<_>`, you must update the len
        unsafe { buf.set_len(written_usize) };
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::os::fs::{File, Access};
  
    extern crate std;

    #[test]
    fn write_file_test() {
        let file = File::open("test.txt", Access::Write).unwrap();
        let _ = file.write("Hello World!");
    }
}