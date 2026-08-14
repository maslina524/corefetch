use alloc::string::String;

use crate::{
    impl_display_for_module, 
    format_for_module, 
    modules::Module, 
    sync::OnceLock,
    detect::locale
};

static LOCALE: OnceLock<Locale> = OnceLock::new();

#[derive(Debug)]
pub struct Locale {
    pub result: String
}

impl Module for Locale {
    fn new() -> Self {
        Self {
            result: locale::locale()
        }
    }

    fn get() -> &'static Self {
        LOCALE.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Locale"
    }

    fn title(&self) -> &'static str {
        "{result}"
    }

    fn string_name(&self) -> &'static str {
        "locale"
    }

    format_for_module!(Locale, result);
}


impl_display_for_module!(Locale);