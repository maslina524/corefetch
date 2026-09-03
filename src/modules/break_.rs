use alloc::{
    string::String,
    collections::BTreeMap
};

use doc::Docs;

use crate::{
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock,
    json::Value
};

static BREAK: OnceLock<Break> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Break;

impl Module for Break {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        BREAK.get_or_init(|| {
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
        "break"
    }

    fn format(&self, _key: super::FormatValue, _format: super::FormatValue, _map: &BTreeMap<String, Value>) -> String {
        String::from('\n')
    }
}

impl_display_for_module!(Break);