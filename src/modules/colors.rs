use alloc::{
    string::String,
    collections::BTreeMap,
    vec
};
use doc::Docs;

use crate::{
    format,
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock,
    json::Value
};

static COLORS: OnceLock<Colors> = OnceLock::new();

#[derive(Debug, Docs)]
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

    fn key(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn string_name(&self) -> &'static str {
        "colors"
    }

    fn format(&self, _key: super::FormatValue, _format: super::FormatValue, map: &BTreeMap<String, Value>) -> String {
        let padding_left_num = map
            .get("paddingLeft")
            .unwrap_or(&Value::Null)
            .as_number()
            .unwrap_or(0.0) as usize;
        let padding_left = " ".repeat(padding_left_num);

        let symbol_map = map
            .get("symbol")
            .unwrap_or(&Value::Null)
            .as_string()
            .map_or_else(|| "block", String::as_str);

        let symbol = match symbol_map {
            "block" => "███",
            "circle" => "● ",
            _ => symbol_map
        };

        let ranges = if symbol_map == "block" {
            vec![30, 90]
        } else {
            vec![30]
        };

        let mut ret = String::with_capacity(8 * (symbol.len() + 5) * ranges.len());
        
        for r in ranges {
            ret.push_str(&padding_left);
            for i in r..=r + 7 {
                ret.push_str(&format!("\x1b[{i}m{symbol}"));
            }
            ret.push_str("\x1b[0m\n");
        }
        ret
    }
}

impl_display_for_module!(Colors);