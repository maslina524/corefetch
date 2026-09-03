use alloc::string::String;
use doc::Docs;

use crate::{
    impl_display_for_module, 
    format_for_module, 
    modules::Module, 
    sync::OnceLock,
    detect::locale::LocaleInfo
};

static LOCALE: OnceLock<Locale> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Locale {
    #[doc = "Locale code"]
    pub result: String
}

impl Module for Locale {
    fn new() -> Self {
        Self {
            result: LocaleInfo::new().locale
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