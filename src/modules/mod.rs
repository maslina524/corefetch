pub mod break_;     // 7)  Break         : Print an empty line
pub mod colors;     // 14) Colors        : Display the terminal's 16-color palette
pub mod commit;     // NF) Commit        : Display last commit
pub mod cpu;        // 15) CPU           : Print CPU name, frequency, etc.
pub mod custom;     // 19) Custom        : Print a custom string, with or without key
pub mod datetime;   // 20) DateTime      : Print the current date and time
pub mod disk;       // 22) Disk          : Print partitions, space usage, file system, etc
pub mod gpu;        // 29) GPU           : Print GPU names, memory sizes, types, etc
pub mod initsystem; // 32) InitSystem    : Print init system (pid 1) name and version
pub mod kernel;     // 33) Kernel        : Print system kernel version
pub mod locale;     // 37) Locale        : Print system locale name
pub mod memory;     // 41) Memory        : Print system memory usage information
pub mod os;         // 47) OS            : Print the OS or Linux distribution name and version
pub mod processes;  // 53) Processes     : Print number of running processes
pub mod separator;  // 55) Separator     : Print a separator line
// pub mod swap;    // 58) Swap          : Print swap (paging file) space usage
pub mod title;      // 63) Title         : Print the title, including your username and hostname
pub mod uptime;     // 66) Uptime        : Print how long the system has been running
pub mod version;    // 68) Version       : Print the Fastfetch version and build information
pub mod wallpaper;  // 70) Wallpaper     : Print the file path of the current wallpaper
pub mod weather;    // 71) Weather       : Print weather information

pub use break_::Break;
pub use colors::Colors;
pub use commit::Commit;
pub use cpu::Cpu;
pub use custom::Custom;
pub use datetime::Datetime;
pub use disk::{DiskList, Disk};
pub use gpu::Gpu;
pub use initsystem::Initsystem;
pub use kernel::Kernel;
pub use locale::Locale;
pub use memory::Memory;
pub use os::Os;
pub use processes::Processes;
pub use separator::Separator;
pub use title::Title;
pub use uptime::Uptime;
pub use version::Version;
pub use wallpaper::Wallpaper;
pub use weather::Weather;

use alloc::{
    string::{String, ToString},
    collections::BTreeMap
};

use crate::{
    json::Value,
    formats
};

type ModulePtr = &'static dyn Module;
type Registy   = (&'static str, fn() -> ModulePtr);
type Example   = (&'static str, String);

static UNSUPPORTED_FIELDS: [&str; 1] = ["{cmake-built-type}"];

static REGISTRY: &[Registy] = &[
    ("break",      || Break::get()),
    ("colors",     || Colors::get()),
    ("commit",     || Commit::get()),
    ("cpu",        || Cpu::get()),
    ("custom",     || Custom::get()),
    ("datetime",   || Datetime::get()),
    ("disk",       || DiskList::get()),
    ("gpu",        || Gpu::get()),
    ("initsystem", || Initsystem::get()),
    ("kernel",     || Kernel::get()),
    ("locale",     || Locale::get()),
    ("memory",     || Memory::get()),
    ("os",         || Os::get()),
    ("processes",  || Processes::get()),
    ("separator",  || Separator::get()),
    ("title",      || Title::get()),
    ("uptime",     || Uptime::get()),
    ("version",    || Version::get()),
    ("wallpaper",  || Wallpaper::get()),
    ("weather",    || Weather::get()),
];

#[derive(Default, Clone, Copy)]
pub struct FormatValue<'a> {
    pub format: Option<&'a str>,
    pub color: Option<&'a str>
}

#[derive(Debug)]
pub struct DocString {
    pub name: &'static str,
    pub second: &'static str,
    pub desc: Option<&'static str>
}

// 473kb -> 428kb
pub trait Docs {
    fn strings_format() -> Option<&'static [DocString]>;
    fn strings_lua() -> Option<&'static [DocString]>;
    fn strings_example(self) -> Option<alloc::vec::Vec<(&'static str, String)>>;
}

pub struct DocsVtable {
    pub format: fn() -> Option<&'static [DocString]>,
    pub lua: fn() -> Option<&'static [DocString]>,
    pub example: fn() -> Option<alloc::vec::Vec<Example>>
}

impl DocsVtable {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "break"      => Some(Self { format: Break::strings_format,      lua: Break::strings_lua,      example: || Break::strings_example(Break::new())           }),
            "colors"     => Some(Self { format: Colors::strings_format,     lua: Colors::strings_lua,     example: || Colors::strings_example(Colors::new())         }),
            "commit"     => Some(Self { format: Commit::strings_format,     lua: Commit::strings_lua,     example: || Commit::strings_example(Commit::new())         }),
            "cpu"        => Some(Self { format: Cpu::strings_format,        lua: Cpu::strings_lua,        example: || Cpu::strings_example(Cpu::new())               }),
            "custom"     => Some(Self { format: Custom::strings_format,     lua: Custom::strings_lua,     example: || Custom::strings_example(Custom::new())         }),
            "datetime"   => Some(Self { format: Datetime::strings_format,   lua: Datetime::strings_lua,   example: || Datetime::strings_example(Datetime::new())     }),
            "disk"       => Some(Self { format: Disk::strings_format,       lua: Disk::strings_lua,       example: || Disk::strings_example(DiskList::new().list[0].clone()) }),
            "gpu"        => Some(Self { format: Gpu::strings_format,        lua: Gpu::strings_lua,        example: || Gpu::strings_example(Gpu::new())               }),
            "initsystem" => Some(Self { format: Initsystem::strings_format, lua: Initsystem::strings_lua, example: || Initsystem::strings_example(Initsystem::new()) }),
            "kernel"     => Some(Self { format: Kernel::strings_format,     lua: Kernel::strings_lua,     example: || Kernel::strings_example(Kernel::new())         }),
            "locale"     => Some(Self { format: Locale::strings_format,     lua: Locale::strings_lua,     example: || Locale::strings_example(Locale::new())         }),
            "memory"     => Some(Self { format: Memory::strings_format,     lua: Memory::strings_lua,     example: || Memory::strings_example(Memory::new())         }),
            "os"         => Some(Self { format: Os::strings_format,         lua: Os::strings_lua,         example: || Os::strings_example(Os::new())                 }),
            "processes"  => Some(Self { format: Processes::strings_format,  lua: Processes::strings_lua,  example: || Processes::strings_example(Processes::new())   }),
            "separator"  => Some(Self { format: Separator::strings_format,  lua: Separator::strings_lua,  example: || Separator::strings_example(Separator::new())   }),
            "title"      => Some(Self { format: Title::strings_format,      lua: Title::strings_lua,      example: || Title::strings_example(Title::new())           }),
            "uptime"     => Some(Self { format: Uptime::strings_format,     lua: Uptime::strings_lua,     example: || Uptime::strings_example(Uptime::new())         }),
            "version"    => Some(Self { format: Version::strings_format,    lua: Version::strings_lua,    example: || Version::strings_example(Version::new())       }),
            "wallpaper"  => Some(Self { format: Wallpaper::strings_format,  lua: Wallpaper::strings_lua,  example: || Wallpaper::strings_example(Wallpaper::new())   }),
            "weather"    => Some(Self { format: Weather::strings_format,    lua: Weather::strings_lua,    example: || Weather::strings_example(Weather::new())       }),
            _ => None,
        }
    }
}

pub trait Module {
    fn new() -> Self where Self: Sized;
    fn get() -> &'static Self where Self: Sized;
    fn key(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn string_name(&self) -> &'static str;
    fn format(&self, key: FormatValue, format: FormatValue, map: Option<&BTreeMap<String, Value>>) -> Option<String>;
}

pub fn from_preset_module(s: &str) -> Option<&'static dyn Module> {
    REGISTRY
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(s))
        .map(|(_, f)| f())
}

pub fn replace_fields<T>(mut s: String, fields: &[(&str, T)]) -> String
where 
    T: ToString
{
    for (idx, (k, v)) in (1..).zip(fields.iter()) {
        let placeholder_underscore = alloc::fmt::format(format_args!("{{{}}}", k.trim_start_matches("r#")));
        let placeholder_hyphen = placeholder_underscore.replace('_', "-");
        let value = || { v.to_string() };

        s = formats::lazy_replace(&s, placeholder_hyphen.as_str(), value).into_owned();
        s = formats::lazy_replace(&s, &crate::format!("{{{idx}}}"), value).into_owned();
    }
    s
}

#[macro_export]
macro_rules! impl_display_for_module {
    ($name:ident) => {
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self.format(
                    $crate::modules::FormatValue::default(),
                    $crate::modules::FormatValue::default(),
                    None,
                ) {
                    Some(s) => f.write_str(&s),
                    None => Ok(()),
                }
            }
        }
    };
    ($name:ident < $lt:lifetime >) => {
        impl core::fmt::Display for $name<$lt> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self.format(
                    $crate::modules::FormatValue::default(),
                    $crate::modules::FormatValue::default(),
                    None,
                ) {
                    Some(s) => f.write_str(&s),
                    None => Ok(()),
                }
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
            _map: Option<&alloc::collections::BTreeMap<alloc::string::String, $crate::json::Value>>,
        ) -> Option<alloc::string::String> {
            let title_raw = format.format.unwrap_or(self.title());

            let value_raw: alloc::string::String = if let Some(code) = title_raw.strip_prefix("lua:") {
                #[allow(unused_mut)]
                let mut vars = alloc::collections::BTreeMap::new();

                $(
                    let key_str = stringify!($field).trim_start_matches("r#");
                    vars.insert(
                        alloc::borrow::ToOwned::to_owned(key_str),
                        $crate::lua::AsLua::as_lua(&self.$field),
                    );
                )*

                $crate::lua::LuaLib::get().exec(code, vars)
            } else {
                alloc::borrow::ToOwned::to_owned(title_raw)
            };

            let fields: &[(&str, alloc::string::String)] = &[$(
                (
                    stringify!($field),
                    $crate::format_module!(@to_string &self.$field),
                ),
            )*];

            let value_substituted = $crate::modules::replace_fields(value_raw, fields);

            if value_substituted.is_empty() {
                return None;
            }

            let key_color = key.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_keys);
            let key_raw   = key.format.unwrap_or(self.key());

            let key_substituted = $crate::modules::replace_fields(
                alloc::borrow::ToOwned::to_owned(key_raw),
                fields,
            );

            if key_substituted.is_empty() {
                return Some(value_substituted);
            }

            let separator = $crate::config::Config::get().get_display_separator();

            let mut full_string = alloc::string::String::with_capacity(
                key_color.len() + key_substituted.len() + separator.len()
                    + value_substituted.len() + 8,
            );
            {
                use alloc::fmt::Write as _;
                let _ = write!(
                    full_string,
                    "\x1b[{key_color};1m{key_substituted}\x1b[0m{separator}{value_substituted}"
                );
            }

            Some(full_string)
        }
    };
}

#[macro_export]
macro_rules! format_module {
    ($format:expr, $obj:ident $(,)?) => {{
        $crate::format_module!(@to_string $format)
    }};

    ($format:expr, $obj:ident, $($field:ident),*) => {{
        let result = $crate::format_module!(@to_string $format);        
        $crate::modules::replace_fields(
            result, 
            &[$((
                stringify!($field), 
                &$crate::format_module!(@to_string &$obj.$field)
            )),*]
        )
    }};
    
    (@to_string $expr:expr) => {{
        alloc::string::ToString::to_string($expr)
    }};
}