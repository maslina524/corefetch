use alloc::string::String;

use crate::{
    format,
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static COLORS: OnceLock<Colors> = OnceLock::new();

#[derive(Debug)]
pub struct Colors;

impl Module for Colors {
    fn new() -> Self {
        Self {}
    }

    fn get() -> &'static Self {
        COLORS.get_or_init(|| {
            Self::new()
        })
    }

    fn key() -> &'static str {
        ""
    }

    fn title() -> &'static str {
        ""
    }

    fn format(&self, key: super::FormatValue, format: super::FormatValue) -> String {
        let mut ret = String::with_capacity(8 * 8 * 2);
        for i in 40..=47 {
            ret.push_str(&format!("\x1b[{i}m   "));
        }
        ret.push_str("\x1b[0m\n");
        for i in 100..=107 {
            ret.push_str(&format!("\x1b[{i}m   "));
        }
        ret.push_str("\x1b[0m");
        ret
    }
}

impl_display_for_module!(Colors);