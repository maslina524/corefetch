use alloc::{
    string::String,
    collections::BTreeMap,
    boxed::Box
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

    fn format(&self, _key: super::FormatValue, _format: super::FormatValue, _map: Option<&BTreeMap<String, Value>>) -> Option<String> {
        Some(String::from('\n'))
    }

    fn field_registry(&self) -> Box<[(&'static str, &dyn core::fmt::Display)]> {
        unreachable!()
    }
}

impl_display_for_module!(Break);