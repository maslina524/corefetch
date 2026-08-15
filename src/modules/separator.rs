use alloc::{
    string::String,
    collections::BTreeMap
};

use crate::{
    format,
    impl_display_for_module, 
    modules::{Module, Title, FormatValue},
    sync::OnceLock,
    formats,
    json::Value
};

static SEPARATOR: OnceLock<Separator> = OnceLock::new();

#[derive(Debug)]
pub struct Separator;

impl Module for Separator {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        SEPARATOR.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn string_name(&self) -> &'static str {
        "separator"
    }

    fn format(&self, _key: super::FormatValue, _format: super::FormatValue, _map: &BTreeMap<String, Value>) -> String {
        let title = Title::get().format(FormatValue::default(), FormatValue::default(), &BTreeMap::new());
        "-".repeat(formats::visible_len(&title))
    }
}

impl_display_for_module!(Separator);