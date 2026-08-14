use alloc::{
    string::{String, ToString},
};

use crate::{
    os::error::{self, ErrorCode},
    os::path::Path,
    os::windows::SystemParametersInfoW,
    os::encoding::{utf16le_to_utf8, Utf16Len}
};

const MAX_PATH: usize = 260 + 1; // `+1` for `\0`
const SPI_GETDESKWALLPAPER: u32 = 0x0073;

pub fn full_path() -> error::Result<Path> {
    let mut buf = [0u16; MAX_PATH];

    // SAFETY: Completely safe
    let ret = unsafe {
        SystemParametersInfoW(
            SPI_GETDESKWALLPAPER, 
            MAX_PATH as u32, 
            (&raw mut buf).cast(), 
            0
        )
    };
    if ret == 0 {
        return Err(ErrorCode::last());
    }

    let utf8 = utf16le_to_utf8(&buf, Utf16Len::NullTerminated)?;
    let path = Path::from(utf8);
    Ok(path)
}

pub fn file_name(path: &Path) -> String {
    path
        .parts()
        .last()
        .unwrap_or(&"")
        .to_string()
}