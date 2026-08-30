use core::ffi::{CStr, c_void};

use alloc::vec::Vec;

use crate::{
    linux::libc::{FILE, fopen, fclose, fwrite},
    linux::error::{self, ErrorCode},
    linux::path::Path
};

#[repr(u32)]
pub enum Access {
    Read,
    Write,
    Append,
    ReadWrite,
    ReadWriteNewFile,
    ReadAppendNewFile
}

impl Access {
    fn as_cstr(&self) -> &'static CStr {
        match self {
            Self::Read => c"r",
            Self::Write => c"w",
            Self::Append => c"a",
            Self::ReadWrite => c"r+",
            Self::ReadWriteNewFile => c"w+",
            Self::ReadAppendNewFile => c"a+",
        }
    }
}

pub struct File(FILE);

impl File {
    pub fn new(path: impl Into<Path>, access: Access) -> error::Result<Self> {
        let path = path.into();
        let c_str = path.as_c_str();
        let file = fopen(c_str.as_ptr(), access.as_cstr().as_ptr());

        if file.is_null() {
            Err(ErrorCode::last())
        } else {
            Ok(Self(file))
        }
    }

    pub fn create_new(path: impl Into<Path>, access: Access) -> error::Result<Self> {
        let new_access = match access {
            Access::Read | Access::Write | Access::ReadWrite => Access::ReadWriteNewFile,
            Access::Append => Access::ReadAppendNewFile,
            _ => access
        };
        Self::new(path, new_access)
    }

    pub fn create_always(path: impl Into<Path>, access: Access) -> error::Result<Self> {
        Self::create_new(path, access)
    }

    pub const fn as_handle(&self) -> FILE {
        self.0
    }

    pub fn write(&self, buf: &[u8]) -> error::Result<()> {
        let written = fwrite(buf.as_ptr() as *const c_void, 1, buf.len(), self.0);
        if written == buf.len() {
            Ok(())
        } else {
            Err(ErrorCode::last())
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        fclose(self.0);
    }
}