
use crate::{
    detect::wallpaper::WallpaperInfo, 
    linux::path::Path
};

impl WallpaperInfo {
    pub fn new() -> Self {
        Self {
            full_path: Path::new(),
        }
    }
}