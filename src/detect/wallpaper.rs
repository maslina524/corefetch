use alloc::{
    string::{String, ToString},
};

use crate::{
    // windows::encoding::{Utf16Len, utf16le_to_utf8}, 
    imp::error::{self, ErrorCode}, 
    // link::SystemParametersInfoW, 
    imp::path::Path 
};

const MAX_PATH: usize = 260 + 1; // `+1` for `\0`
const SPI_GETDESKWALLPAPER: u32 = 0x0073;

pub fn full_path() -> error::Result<Path> {
    // let mut buf = [0u16; MAX_PATH];

    // // SAFETY: Completely safe
    // let ret = unsafe {
    //     SystemParametersInfoW(
    //         SPI_GETDESKWALLPAPER, 
    //         MAX_PATH as u32, 
    //         (&raw mut buf).cast(), 
    //         0
    //     )
    // };
    // if ret == 0 {
    //     return Err(ErrorCode::last());
    // }

    // let utf8 = utf16le_to_utf8(&buf, Utf16Len::NullTerminated)?;
    // let path = Path::from(utf8);
    // Ok(path)
    Ok(Path::new())
}

pub fn file_name(_path: &Path) -> String {
    // path
    //     .parts()
    //     .last()
    //     .unwrap_or(&"")
    //     .to_string()
    String::new()
}