pub mod colors;    // 14) Colors        : Display the terminal's 16-color palette
pub mod locale;    // 37) Locale        : Print system locale name
pub mod os;        // 47) OS            : Print the OS or Linux distribution name and version
pub mod processes; // 53) Processes     : Print number of running processes
pub mod version;   // 68) Version       : Print the Fastfetch version and build information
pub mod weather;   // 71) Weather       : Print weather information

pub use colors::Colors;
pub use locale::Locale;
pub use os::Os;
pub use processes::Processes;
pub use version::Version;
pub use weather::Weather;

#[derive(Default)]
pub struct FormatValue<'a> {
    pub format: Option<&'a str>,
    pub color: Option<&'a str>
}

pub trait Module {
    fn new() -> Self;
    fn get() -> &'static Self;
    fn key() -> &'static str;
    fn title() -> &'static str;
    fn format(&self, key: FormatValue, format: FormatValue) -> alloc::string::String;
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
            let key_format = key.format.unwrap_or($name::key());
            let key_string = $crate::format_module!(key_format, self, $($field),*);

            let value_color = format.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_title);
            let value_format = format.format.unwrap_or($name::title());
            let value_string = $crate::format_module!(value_format, self, $($field),*);

            $crate::format!("\x1b[{key_color}m{key_string}\x1b[0m: \x1b[{value_color}m{value_string}\x1b[0m")
        }
    };
}

#[macro_export]
macro_rules! format_module {
    ($format:expr, $obj:ident, $($field:ident),*) => {{
        let mut result = alloc::string::ToString::to_string($format);
        let mut idx = 1;
        
        $(
            result = result.replace(
                &alloc::fmt::format(format_args!("{{{}}}", stringify!($field))),
                &$crate::format_module!(@to_string &$obj.$field)
            );
            
            result = result.replace(
                &alloc::fmt::format(format_args!("{{{}}}", idx)),
                &$crate::format_module!(@to_string &$obj.$field)
            );
            
            idx += 1;
        )*
        
        result
    }};
    
    (@to_string $expr:expr) => {{
        alloc::string::ToString::to_string($expr)
    }};
}