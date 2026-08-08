use alloc::string::String;

use crate::{
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static BREAK: OnceLock<Break> = OnceLock::new();

#[derive(Debug)]
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

    fn format(&self, key: super::FormatValue, format: super::FormatValue) -> String {
        String::from('\n')
    }
}

impl_display_for_module!(Break);