use core::ffi::{CStr, c_void, c_int};

use alloc::{
    string::{FromUtf8Error, String}, 
    vec::Vec,
    vec
};

use crate::{
    linux::libc::{FILE, fopen, fread, fclose, fwrite, fseek, ftell, rewind, mkdir},
    linux::error::{self, ErrorCode},
    linux::path::Path
};

const SEEK_END: c_int = 2;

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

#[derive(Debug)]
pub enum ReadError {
    Utf8(FromUtf8Error),
    Code(ErrorCode),
}

impl From<FromUtf8Error> for ReadError {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<ErrorCode> for ReadError {
    fn from(err: ErrorCode) -> Self {
        Self::Code(err)
    }
}

impl core::fmt::Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Utf8(s) => write!(f, "Utf8Error: {s}"),
            Self::Code(s) => write!(f, "ErrorCode: {s}"),
        }
    }
}

impl core::error::Error for ReadError {}

pub struct File(FILE);

impl File {
    pub fn open(path: impl Into<Path>, access: Access) -> error::Result<Self> {
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
        Self::open(path, new_access)
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

    pub fn read(&self) -> error::Result<Vec<u8>> {
        if fseek(self.0, 0, SEEK_END) != 0 {
            return Err(ErrorCode::last());
        }
        let size = ftell(self.0);
        if size < 0 {
            return Err(ErrorCode::last());
        }
        let size = size as usize;
        rewind(self.0);

        let mut buf = vec![0u8; size];
        let readed = fread(buf.as_mut_ptr().cast(), 1, size, self.0);
        if readed != size {
            return Err(ErrorCode::last());
        }

        Ok(buf)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        fclose(self.0);
    }
}

pub fn read(path: impl Into<Path>) -> error::Result<Vec<u8>> {
    let handler = File::open(path, Access::Read)?;
    let bytes = handler.read()?;
    Ok(bytes)
}

pub fn read_to_string(path: impl Into<Path>) -> Result<String, ReadError> {
    let handler = File::open(path, Access::Read)?;
    let bytes = handler.read()?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
}

pub fn create_dir(path: impl Into<Path>) -> error::Result<()> {
    let path = path.into();
    let c_path = path.as_c_str();

    let ret = mkdir(c_path.as_ptr(), 0o755);
    if ret == -1 {
        Err(ErrorCode::last())
    } else {
        Ok(())
    }
}

pub fn create_dirs(path: impl Into<Path>) -> error::Result<()> {
    let path = path.into();
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_dirs(parent)?;
        }
        create_dir(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::string::String;

    use crate::linux::fs::{self, Access, File};

    #[test]
    fn write_file_test() {
        let file = File::open("test.txt", Access::Write).unwrap();
        let _ = file.write(b"Hello World!");
    }

    #[test]
    fn read_file_test() {
        let file = File::open("README.md", Access::Read).unwrap();
        let buf = file.read().unwrap();
        let string = String::from_utf8(buf).unwrap();

        assert!(string.starts_with("# Corefetch"));
    }

    #[test]
    fn read_to_string_test() {
        let string = fs::read_to_string("README.md").unwrap();
        assert!(string.starts_with("# Corefetch"));
    }

    #[test]
    fn create_dirs_test() {
        fs::create_dirs("a/b/c/d").expect("ErrorCode");
    }
}