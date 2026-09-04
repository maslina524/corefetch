pub mod break_;     // 7)  Break         : Print an empty line
pub mod colors;     // 14) Colors        : Display the terminal's 16-color palette
pub mod commit;     // NF) Commit        : Display last commit
pub mod cpu;        // 15) CPU           : Print CPU name, frequency, etc.
pub mod custom;     // 19) Custom        : Print a custom string, with or without key
pub mod datetime;   // 20) DateTime      : Print the current date and time
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
    boxed::Box,
    string::String,
    collections::BTreeMap
};

use crate::{
    Docs,
    config::ConfigModule,
    json::Value
};

#[derive(Default)]
pub struct FormatValue<'a> {
    pub format: Option<&'a str>,
    pub color: Option<&'a str>
}

pub struct DocsVtable {
    pub format: fn(),
    pub lua: fn()
}

impl DocsVtable {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "break"      => Some(Self { format: Break::print_format,      lua: Break::print_lua      }),
            "colors"     => Some(Self { format: Colors::print_format,     lua: Colors::print_lua     }),
            "commit"     => Some(Self { format: Commit::print_format,     lua: Commit::print_lua     }),
            "cpu"        => Some(Self { format: Cpu::print_format,        lua: Cpu::print_lua        }),
            "custom"     => Some(Self { format: Custom::print_format,     lua: Custom::print_lua     }),
            "datetime"   => Some(Self { format: Datetime::print_format,   lua: Datetime::print_lua   }),
            "gpu"        => Some(Self { format: Gpu::print_format,        lua: Gpu::print_lua        }),
            "initsystem" => Some(Self { format: Initsystem::print_format, lua: Initsystem::print_lua }),
            "kernel"     => Some(Self { format: Kernel::print_format,     lua: Kernel::print_lua     }),
            "locale"     => Some(Self { format: Locale::print_format,     lua: Locale::print_lua     }),
            "memory"     => Some(Self { format: Memory::print_format,     lua: Memory::print_lua     }),
            "os"         => Some(Self { format: Os::print_format,         lua: Os::print_lua         }),
            "processes"  => Some(Self { format: Processes::print_format,  lua:Processes::print_lua   }),
            "separator"  => Some(Self { format: Separator::print_format,  lua: Separator::print_lua  }),
            "title"      => Some(Self { format: Title::print_format,      lua: Title::print_lua      }),
            "uptime"     => Some(Self { format: Uptime::print_format,     lua: Uptime::print_lua     }),
            "version"    => Some(Self { format: Version::print_format,    lua: Version::print_lua    }),
            "wallpaper"  => Some(Self { format: Wallpaper::print_format,  lua: Wallpaper::print_lua  }),
            "weather"    => Some(Self { format: Weather::print_format,    lua: Weather::print_lua    }),
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
    fn format(&self, key: FormatValue, format: FormatValue, map: &BTreeMap<String, Value>) -> alloc::string::String;
}

pub fn from_preset_module(module: &ConfigModule) -> Option<Box<dyn Module>> {
    match module.typ.to_lowercase().as_str() {
        "break"      => Some(Box::new( Break::new()      )),
        "colors"     => Some(Box::new( Colors::new()     )),
        "commit"     => Some(Box::new( Commit::new()     )),
        "cpu"        => Some(Box::new( Cpu::new()        )),
        "custom"     => Some(Box::new( Custom::new()     )),
        "datetime"   => Some(Box::new( Datetime::new()   )),
        "gpu"        => Some(Box::new( Gpu::new()        )),
        "initsystem" => Some(Box::new( Initsystem::new() )),
        "kernel"     => Some(Box::new( Kernel::new()     )),
        "locale"     => Some(Box::new( Locale::new()     )),
        "memory"     => Some(Box::new( Memory::new()     )),
        "os"         => Some(Box::new( Os::new()         )),
        "processes"  => Some(Box::new( Processes::new()  )),
        "separator"  => Some(Box::new( Separator::new()  )),
        "title"      => Some(Box::new( Title::new()      )),
        "uptime"     => Some(Box::new( Uptime::new()     )),
        "version"    => Some(Box::new( Version::new()    )),
        "wallpaper"  => Some(Box::new( Wallpaper::new()  )),
        "weather"    => Some(Box::new( Weather::new()    )),
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
            _map: &alloc::collections::BTreeMap<alloc::string::String, $crate::json::Value>
        ) -> alloc::string::String {
            let title_raw = format.format.unwrap_or(self.title());
            let value_raw = if let Some(code) = title_raw.strip_prefix("lua:") {
                #[allow(unused_mut)]
                let mut vars = alloc::collections::BTreeMap::new();

                $(
                    let key_str = stringify!($field).trim_start_matches("r#");
                    vars.insert(alloc::borrow::ToOwned::to_owned(key_str), $crate::lua::AsLua::as_lua(&self.$field));
                )*

                $crate::lua::LuaLib::get().exec(code, vars)
            } else {
                alloc::borrow::ToOwned::to_owned(title_raw)
            };

            let key_color = key.color.unwrap_or($crate::logo::LogoInfo::get().unwrap().color_keys);
            let key_raw = key.format.unwrap_or(self.key());
            

            let separator = $crate::config::Config::get().get_display_separator();

            let full_string = if key_raw.len() == 0 {
                value_raw
            } else {
                $crate::format!("\x1b[{key_color};1m{key_raw}\x1b[0m{separator}{value_raw}")
            };
            $crate::format_module!(&full_string, self, $($field),*)
        }
    };
}

#[macro_export]
macro_rules! format_module {
    ($format:expr, $obj:ident, $($field:ident),*) => {{
        #[allow(unused_mut)]
        let mut result = alloc::string::ToString::to_string($format);
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut idx = 1;
        
        $(
            let placeholder_underscore = alloc::fmt::format(format_args!("{{{}}}", stringify!($field).trim_start_matches("r#")));
            let placeholder_hyphen = placeholder_underscore.replace('_', "-");
            
            let value = &$crate::format_module!(@to_string &$obj.$field);
            
            result = result.replace(&placeholder_underscore, value);
            result = result.replace(&placeholder_hyphen, value);
            
            let placeholder_idx = alloc::fmt::format(format_args!("{{{}}}", idx));
            result = result.replace(&placeholder_idx, value);
            
            #[allow(unused_assignments)]
            { idx += 1 };
        )*
        
        result
    }};
    
    (@to_string $expr:expr) => {{
        alloc::string::ToString::to_string($expr)
    }};
}