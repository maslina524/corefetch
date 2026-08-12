pub mod breakk;    // 7)  Break         : Print an empty line
pub mod colors;    // 14) Colors        : Display the terminal's 16-color palette
pub mod cpu;       // 15) CPU           : Print CPU name, frequency, etc.
pub mod custom;    // 19) Custom        : Print a custom string, with or without key
pub mod datetime;  // 20) DateTime      : Print the current date and time
pub mod kernel;    // 33) Kernel        : Print system kernel version
pub mod locale;    // 37) Locale        : Print system locale name
pub mod memory;    // 41) Memory        : Print system memory usage information
pub mod os;        // 47) OS            : Print the OS or Linux distribution name and version
pub mod processes; // 53) Processes     : Print number of running processes
pub mod separator; // 55) Separator     : Print a separator line
pub mod title;     // 63) Title         : Print the title, including your username and hostname
pub mod version;   // 68) Version       : Print the Fastfetch version and build information
pub mod weather;   // 71) Weather       : Print weather information

pub use breakk::Break;
pub use colors::Colors;
pub use cpu::Cpu;
pub use custom::Custom;
pub use datetime::Datetime;
pub use kernel::Kernel;
pub use locale::Locale;
pub use memory::Memory;
pub use os::Os;
pub use processes::Processes;
pub use separator::Separator;
pub use title::Title;
pub use version::Version;
pub use weather::Weather;

use alloc::{
    boxed::Box,
    string::String,
    collections::BTreeMap
};

use crate::{
    preset::PresetModule,
    json::Value
};

#[derive(Default)]
pub struct FormatValue<'a> {
    pub format: Option<&'a str>,
    pub color: Option<&'a str>
}

pub trait Module {
    fn new() -> Self where Self: Sized;
    fn get() -> &'static Self where Self: Sized;
    fn key(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn string_name(&self) -> &'static str;
    fn format(&self, key: FormatValue, format: FormatValue, map: &BTreeMap<String, Value>) -> alloc::string::String;
}

pub fn from_preset_module(module: &PresetModule) -> Option<Box<dyn Module>> {
    match module.typ.to_lowercase().as_str() {
        "break"     => Some(Box::new( Break::new()     )),
        "colors"    => Some(Box::new( Colors::new()    )),
        "cpu"       => Some(Box::new( Cpu::new()       )),
        "custom"    => Some(Box::new( Custom::new()    )),
        "datetime"  => Some(Box::new( Datetime::new()  )),
        "kernel"    => Some(Box::new( Kernel::new()    )),
        "locale"    => Some(Box::new( Locale::new()    )),
        "memory"    => Some(Box::new( Memory::new()    )),
        "os"        => Some(Box::new( Os::new()        )),
        "processes" => Some(Box::new( Processes::new() )),
        "separator" => Some(Box::new( Separator::new() )),
        "title"     => Some(Box::new( Title::new()     )),
        "version"   => Some(Box::new( Version::new()   )),
        "weather"   => Some(Box::new( Weather::new()   )),
        _ => None,
    }
}

#[macro_export]
macro_rules! impl_display_for_module {
    ($name:ident) => {
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f, "{}", self.format(
                        $crate::modules::FormatValue::default(), 
                        $crate::modules::FormatValue::default(), 
                        &alloc::collections::BTreeMap::new()
                    )
                )
            }
        }
    };
    ($name:ident, $lt:lifetime) => {
        impl core::fmt::Display for $name<$lt> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(
                    f, "{}", self.format(
                        $crate::modules::FormatValue::default(), 
                        $crate::modules::FormatValue::default(), 
                        &alloc::collections::BTreeMap::new()
                    )
                )
            }
        }
    };
}

#[macro_export]
macro_rules! format_for_module {
    ($name:ident, $($field:ident),*) => {
        fn format(
            &self,
            key: super::FormatValue, 
            format: super::FormatValue, 
            map: &alloc::collections::BTreeMap<alloc::string::String, $crate::json::Value>
        ) -> alloc::string::String {
            let key_color = key.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_keys);
            let key_raw = key.format.unwrap_or(self.key());
            let value_raw = format.format.unwrap_or(self.title());

            let separator = $crate::preset::Preset::get().get_display_separator();

            let full_string = if key_raw.len() == 0 {
                alloc::borrow::ToOwned::to_owned(value_raw)
            } else {
                $crate::format!("\x1b[1;{key_color}m{key_raw}\x1b[0m{separator}{value_raw}")
            };
            $crate::format_module!(&full_string, self, $($field),*)
        }
    };
}

#[macro_export]
macro_rules! format_for_module_wo_key {
    ($name:ident, $($field:ident),*) => {
        fn format(
            &self, 
            _key: super::FormatValue, 
            format: super::FormatValue,
            map: &alloc::collections::BTreeMap<alloc::string::String, $crate::json::Value>
        ) -> alloc::string::String {
            let value_color = format.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_title);
            let value_format = format.format.unwrap_or(self.title());
            $crate::format_module!(value_format, self, $($field),*)
        }
    };
}

#[macro_export]
macro_rules! format_module {
    ($format:expr, $obj:ident, $($field:ident),*) => {{
        let mut result = alloc::string::ToString::to_string($format);
        let mut idx = 1;
        
        $(
            let placeholder_underscore = alloc::fmt::format(format_args!("{{{}}}", stringify!($field)));
            let placeholder_hyphen = placeholder_underscore.replace('_', "-");
            
            let value = &$crate::format_module!(@to_string &$obj.$field);
            
            result = result.replace(&placeholder_underscore, value);
            result = result.replace(&placeholder_hyphen, value);
            
            let placeholder_idx = alloc::fmt::format(format_args!("{{{}}}", idx));
            result = result.replace(&placeholder_idx, value);
            
            idx += 1;
        )*
        
        result
    }};
    
    (@to_string $expr:expr) => {{
        alloc::string::ToString::to_string($expr)
    }};
}