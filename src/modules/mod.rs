pub mod colors;    // 14) Colors        : Display the terminal's 16-color palette
pub mod cpu;       // 15) CPU           : Print CPU name, frequency, etc.
pub mod locale;    // 37) Locale        : Print system locale name
pub mod os;        // 47) OS            : Print the OS or Linux distribution name and version
pub mod processes; // 53) Processes     : Print number of running processes
pub mod version;   // 68) Version       : Print the Fastfetch version and build information
pub mod weather;   // 71) Weather       : Print weather information

pub use colors::Colors;
pub use cpu::Cpu;
pub use locale::Locale;
pub use os::Os;
pub use processes::Processes;
pub use version::Version;
pub use weather::Weather;

use alloc::boxed::Box;

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
    fn format(&self, key: FormatValue, format: FormatValue) -> alloc::string::String;
}

pub fn from_str(string: &str) -> Option<Box<dyn Module>> {
    match string.to_lowercase().as_str() {
        "color"     => Some(Box::new(Colors::new())),
        "cpu"       => Some(Box::new(Cpu::new())),
        "locale"    => Some(Box::new(Locale::new())),
        "os"        => Some(Box::new(Os::new())),
        "processes" => Some(Box::new(Processes::new())),
        "version"   => Some(Box::new(Version::new())),
        "weather"   => Some(Box::new(Weather::new())),
        _ => None,
    }
}

#[macro_export]
macro_rules! impl_display_for_module {
    ($name:ident) => {
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.format($crate::modules::FormatValue::default(), $crate::modules::FormatValue::default()))
            }
        }
    };
    ($name:ident, $lt:lifetime) => {
        impl core::fmt::Display for $name<$lt> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.format($crate::modules::FormatValue::default(), $crate::modules::FormatValue::default()))
            }
        }
    };
}

#[macro_export]
macro_rules! format_for_module {
    ($name:ident, $($field:ident),*) => {
        fn format(&self, key: super::FormatValue, format: super::FormatValue) -> alloc::string::String {
            let key_color = key.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_keys);
            let key_format = key.format.unwrap_or(self.key());
            let key_string = $crate::format_module!(key_format, self, $($field),*);

            let value_color = format.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_title);
            let value_format = format.format.unwrap_or(self.title());
            let value_string = $crate::format_module!(value_format, self, $($field),*);

            $crate::format!("\x1b[{key_color}m{key_string}\x1b[0m: {value_string}")
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