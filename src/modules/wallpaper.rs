use alloc::{
    string::String,
    borrow::ToOwned
};
use doc::Docs;

use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock,
    detect::wallpaper::WallpaperInfo,
    imp::path::Path
};

static WALLPAPER: OnceLock<Wallpaper> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Wallpaper {
    #[doc = "File name"]
    pub file_name: String,
    #[doc = "Full path"]
    pub full_path: String
}

impl Module for Wallpaper {
    #[allow(clippy::redundant_closure)]
    fn new() -> Self {
        let info = WallpaperInfo::new();

        let file_name = info.full_path
            .as_ref()
            .map_or_else(
                || "No graphical env".to_owned(), 
                |p| Path::from(p).pop().unwrap()
            );

        let full_path = info.full_path
            .unwrap_or_else(|| "No graphical env".to_owned());

        Self {
            file_name,
            full_path
        }
    }

    fn get() -> &'static Self {
        WALLPAPER.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Wallpaper"
    }

    fn title(&self) -> &'static str {
        "{file-name}"
    }

    fn string_name(&self) -> &'static str {
        "wallpaper"
    }

    format_for_module!(
        Wallpaper,
        file_name, full_path
    );
}

impl_display_for_module!(Wallpaper);