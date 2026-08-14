use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    format,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    detect::wallpaper,
    os::env, 
    sync::OnceLock
};

static WALLPAPER: OnceLock<Wallpaper> = OnceLock::new();

#[derive(Debug)]
pub struct Wallpaper {
    pub file_name: String,
    pub full_path: String
}

impl Module for Wallpaper {
    fn new() -> Self {
        let (file_name, full_path) = wallpaper::full_path().map_or_else(|_| (String::new(), String::new()), |p| {
            let file = wallpaper::file_name(&p);
            let string = p.into_inner();
            (file, string)
        });
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