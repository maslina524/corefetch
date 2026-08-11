use alloc::string::String;

use crate::{
    format,
    impl_display_for_module,
    format_for_module,
    detect::kernel,
    modules::Module, 
    preset::PresetModule,
    sync::OnceLock
};

static CUSTOM: OnceLock<Custom> = OnceLock::new();

#[derive(Debug)]
pub struct Custom;

impl Module for Custom {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        CUSTOM.get_or_init(|| {
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
        "custom"
    }

    format_for_module!(
        Custom,
    );
}

impl_display_for_module!(Custom);