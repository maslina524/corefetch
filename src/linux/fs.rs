use core::ffi::{CStr, c_void, c_int};

use alloc::{
    string::{FromUtf8Error, String}, 
    vec::Vec,
    vec
};

use crate::{
    linux::libc::{
        FILE, fopen, fread, fclose, fwrite, fseek, ftell, rewind, mkdir, DIR, opendir, readdir
    },
    linux::error::{self, ErrorCode},
    linux::path::Path
};

const SEEK_END  : c_int = 2;
const DT_REG    : u8    = 8;
const DT_DIR    : u8    = 4;
const DT_LNK    : u8    = 10;

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

#[derive(Clone)]
pub enum ItemType {
    File, Dir, Link, Unknown
}

impl From<u8> for ItemType {
    fn from(value: u8) -> Self {
        match value {
            DT_REG => ItemType::File,
            DT_DIR => ItemType::Dir,
            DT_LNK => ItemType::Link,
            _ => ItemType::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct Item {
    typ: ItemType,
    name: String
}

impl Item {
    pub fn typ(&self) -> ItemType {
        self.typ.clone()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

pub struct ReadDirIter {
    dir: DIR
}

impl Iterator for ReadDirIter {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let raw_item = readdir(&raw mut self.dir);
            if raw_item.is_null() {
                return None
            }

            // SAFETY: We check if the pointer is null, safe
            let item = unsafe { raw_item.read() };

            // SAFETY: Libc is guaranteed to return a valid string
            let c_str_name = unsafe { CStr::from_ptr(item.d_name.as_ptr()) };
            if c_str_name == c"." || c_str_name == c".." {
                continue;
            }

            let name = c_str_name.to_string_lossy().into_owned();
            let typ = ItemType::from(item.d_type);

            return Some(Item { name, typ });
        }
    }
}

pub fn read_dir(path: impl Into<Path>) -> error::Result<ReadDirIter> {
    let path = path.into();
    let c_path = path.as_c_str();

    let dir = opendir(c_path.as_ptr());
    if dir.is_null() {
        return Err(ErrorCode::last());
    }

    Ok(ReadDirIter { dir })
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