use core::{
    ffi::c_void,
    ptr
};

use alloc::{
    string::String,
    vec::Vec
};

use crate::{
    os::windows::{RegCreateKeyExW, RegOpenKeyExW, HANDLE},
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

#[derive(Debug, Clone)]
pub enum RegValue {
    Binary(Vec<u8>),
    Dword(u32),
    DwordLE(u32),
    DwordBE(u32),
    ExpandSz(String),
    Link(String),
    MultiSz(Vec<String>),
    None,
    Qword(u64),
    QwordLE(u64),
    Sz(String),
}

impl RegValue {
    pub const fn as_u32(&self) -> Option<u32> {
        match self {
            Self::Dword(v) | Self::DwordLE(v) => Some(*v),
            Self::DwordBE(v) => Some(v.to_be()),
            _ => None,
        }
    }

    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Qword(v) | Self::QwordLE(v) => Some(*v),
            _ => None,
        }
    }

    pub const fn as_string(&self) -> Option<&str> {
        match self {
            Self::Sz(s) | Self::ExpandSz(s) | Self::Link(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub const fn as_multi_string(&self) -> Option<&[String]> {
        match self {
            Self::MultiSz(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub const fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::Binary(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
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

    pub fn open(root: Hkey, subkey: &str, access: Access) -> error::Result<Self> {
        let mut handle = ptr::null_mut();
        let wide = utf8_to_utf16le(subkey)?;

        // SAFETY: All parameters have been verified
        // against the documentation, safe
        let ret = unsafe {
            RegOpenKeyExW(
                root as u32 as *mut c_void, 
                wide.as_ptr(), 
                0, 
                access as u32, 
                &raw mut handle,
            )
        };

        Ok(Self(handle))
    }
}