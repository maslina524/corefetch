use alloc::string::String;

use crate::{
    windows::encoding::{Utf16Len, utf16le_to_utf8},
    windows::link::SystemParametersInfoW,
    imp::error::{self, ErrorCode},
    detect::wallpaper::WallpaperInfo,
    warning
};

const MAX_PATH: usize = 260 + 1; // `+1` for `\0`
const SPI_GETDESKWALLPAPER: u32 = 0x0073;

impl WallpaperInfo {
    pub fn new() -> Self {
        let full_path = match Self::full_path() {
            Ok(p) => p,
            Err(e) => {
                warning!("Failed to get full wallpaper path: {e}");
                String::new()
            }
        };

        Self {
            full_path: Some(full_path) 
        }
    }
    
    fn full_path() -> error::Result<String> {
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
        Ok(utf8)
    }
}