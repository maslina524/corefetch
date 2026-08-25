use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    collections::BTreeMap,
    vec
};

use crate::{
    modules::Module, 
    sync::OnceLock,
    json::{Map, Value},
    formats::{ColorPlan, format_color}
};

static PRESET: OnceLock<Config> = OnceLock::new();

pub struct Config {
    modules: Vec<ConfigModule>,
    display: ConfigDisplay,
    logo: ConfigLogo,
}

impl Config {
    pub fn get_or_init(config: Self) -> &'static Self {
        PRESET.get_or_init(|| {
            config
        })
    }

    pub fn from_json(json: &Map) -> Self {
        // Build modules
        let map_modules = json.get_array("modules").expect("msg");
        let mut ret_modules = Vec::with_capacity(map_modules.len());

        for m in map_modules {
            if let Some(obj) = m.as_object() {
                let Some(typ) = obj.get_string("type") else {
                    continue;
                };
                let format = obj.get_string("format").map(String::to_owned);
                let key = obj.get_string("key").map(String::to_owned);
                let key_color = obj.get_string("keyColor").map(String::to_owned);

                let mut map = BTreeMap::new();
                if typ.as_str() == "colors" {
                    let symbol = obj.get("symbol").unwrap_or(&Value::Null).clone();
                    map.insert("symbol".to_owned(), symbol);

                    let padding_left = obj.get("paddingLeft").unwrap_or(&Value::Null).clone();
                    map.insert("paddingLeft".to_owned(), padding_left);
                }

                let preset_mod = ConfigModule::new(typ, format, key, key_color, map);
                ret_modules.push(preset_mod);
            } else if let Some(typ) = m.as_string() {
                let preset_mod = ConfigModule::from_str(typ);
                ret_modules.push(preset_mod);
            }
        }

        // Display
        let separator = json
            .get_object("display")
            .and_then(|m| m.get_string("separator").map(ToOwned::to_owned))
            .unwrap_or_else(|| ": ".to_owned());

        // Logo
        let logo = json
            .get_object("logo")
            .and_then(|logo_obj| logo_obj.get_object("padding"))
            .map_or_else(ConfigLogo::default, |padding| ConfigLogo {
                padding: ConfigPadding {
                    top: padding.get_number("top").unwrap_or(0.0) as usize,
                    bottom: padding.get_number("bottom").unwrap_or(2.0) as usize,
                    right: padding.get_number("right").unwrap_or(3.0) as usize,
                    left: padding.get_number("left").unwrap_or(0.0) as usize,
                }
            }
        );

        Self {
            modules: ret_modules,
            display: ConfigDisplay { separator },
            logo
        }
    }

    pub fn get() -> &'static Self {
        PRESET
            .get()
            .expect("Preset was not initialized at the start of the program")
    }

    pub fn module_by_typ(&self, string: &str) -> Option<&ConfigModule> {
        for m in &self.modules {
            if m.typ == string {
                return Some(m)
            }
        }
        None
    }

    pub const fn modules(&self) -> &Vec<ConfigModule> {
        &self.modules
    }

    pub fn get_module_format(&self, module: &dyn Module) -> &str {
        let preset_module = self.module_by_typ(module.string_name()).unwrap();
        preset_module
            .format
            .as_deref()
            .unwrap_or_else(|| module.title())
    }

    pub fn get_module_key(&self, module: &dyn Module) -> &str {
        let preset_module = self.module_by_typ(module.string_name()).unwrap();
        preset_module
            .key
            .as_deref()
            .unwrap_or_else(|| module.title())
    }

    pub fn get_display_separator(&self) -> &str {
        &self.display.separator
    }

    pub const fn get_logo_padding(&self) -> &ConfigPadding {
        &self.logo.padding
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![
                ConfigModule::from_str("title"),
                ConfigModule::from_str("separator"),
                ConfigModule::from_str("os"),
                ConfigModule::from_str("initsystem"),
                ConfigModule::from_str("kernel"),
                ConfigModule::from_str("uptime"),
                ConfigModule::from_str("datetime"),
                ConfigModule::from_str("processes"),
                ConfigModule::from_str("cpu"),
                ConfigModule::from_str("gpu"),
                ConfigModule::from_str("memory"),
                ConfigModule::from_str("weather"),
                ConfigModule::from_str("locale"),
                ConfigModule::from_str("wallpaper"),
                ConfigModule::from_str("break"),
                ConfigModule::from_str("colors")
            ],
            display: ConfigDisplay {
                separator: ": ".to_owned()
            },
            logo: ConfigLogo::default()
        }
    }
}

pub struct ConfigModule {
    pub typ: String,
    pub format: Option<String>,
    pub key: Option<String>,
    pub key_color: Option<String>,
    pub map: BTreeMap<String, Value>
}

impl ConfigModule {
    pub fn from_str(typ: &str) -> Self {
        Self { typ: typ.to_owned(), format: None, key: None, key_color: None, map: BTreeMap::new() }
    }

    pub fn new(typ: &str, format: Option<String>, key: Option<String>, key_color: Option<String>, map: BTreeMap<String, Value>) -> Self {
        let key_color = key_color.map(|s| format_color(&s, ColorPlan::FG));
        Self { typ: typ.to_owned(), format, key, key_color, map }
    }
}

pub struct ConfigDisplay {
    pub separator: String
}

impl Default for ConfigDisplay {
    fn default() -> Self {
        Self { separator: String::from(": ") }
    }
}

pub struct ConfigPadding {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

impl Default for ConfigPadding {
    fn default() -> Self {
        Self { top: 0, bottom: 2, right: 3, left: 0 }
    }
}

#[derive(Default)]
pub struct ConfigLogo {
    pub padding: ConfigPadding
}