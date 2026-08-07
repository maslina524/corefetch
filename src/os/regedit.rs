use core::{
    ffi::c_void,
    ptr
};

use crate::{
    os::windows::{RegCreateKeyExW, HANDLE},
    os::encoding::utf8_to_utf16le,
    os::error
};

#[repr(u32)]
pub enum Hkey {
    ClassesRoot   = 0x8000_0000,
    CurrentUser   = 0x8000_0001,
    LocalMachine  = 0x8000_0002,
    Users         = 0x8000_0003,
    CurrentConfig =	0x8000_0005
}

#[repr(u32)]
pub enum Access {
    Read  = 0x20019,
    Set   = 0x0002,
    Write = 0x20006,
    All   = 0xF003F
}

pub struct Regedit(HANDLE);

impl Regedit {
    pub fn create(root: Hkey, subkey: &str, access: Access) -> error::Result<Self> {
        let mut handle = ptr::null_mut();
        let wide = utf8_to_utf16le(subkey)?;

        // SAFETY: All parameters have been verified
        // against the documentation, safe
        let ret = unsafe {
            RegCreateKeyExW(
                root as u32 as *mut c_void, 
                wide.as_ptr(), 
                0, 
                ptr::null(), 
                0, 
                access as u32, 
                ptr::null(),
                &raw mut handle, 
                ptr::null_mut()
            )
        };

        Ok(Self(handle))
    }
}