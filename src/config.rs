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
    formats::{ColorPlan, format_color},
    format,
    warning
};

static PRESET: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Default)]
pub struct Config {
    modules: ConfigModuleArray,
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
        let modules = json.get_array("modules").map_or_else(ConfigModuleArray::default, |map_modules| {
            let mut inner = Vec::with_capacity(map_modules.len());

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
                    inner.push(preset_mod);
                } else if let Some(typ) = m.as_string() {
                    let preset_mod = ConfigModule::from_str(typ);
                    inner.push(preset_mod);
                }
            }
            ConfigModuleArray { inner }
        });

        // Display
        let display = json.get_object("display").map_or_else(ConfigDisplay::default, |dspl_obj| {
            let separator = dspl_obj
                .get_string("separator").map_or_else(|| ": ".to_owned(), ToOwned::to_owned);

            let percent = dspl_obj
                .get_object("percent")
                .and_then(|o| o.get_object("color"))
                .map_or_else(ConfigPercent::default, |color_obj|
            {
                let red = color_obj
                    .get_string("red").map_or_else(|| "red".to_owned(), ToOwned::to_owned);

                let yellow = color_obj
                    .get_string("yellow").map_or_else(|| "yellow".to_owned(), ToOwned::to_owned);

                let green = color_obj
                    .get_string("green").map_or_else(|| "green".to_owned(), ToOwned::to_owned);

                ConfigPercent { 
                    green: format_color(&green, ColorPlan::FG), 
                    yellow: format_color(&yellow, ColorPlan::FG), 
                    red: format_color(&red, ColorPlan::FG)
                }
            });

            let temperature = dspl_obj.get_object("temperature").map_or_else(ConfigTemperature::default, |o| {
                // Color
                let (red, yellow, green) = o.get_object("color").map_or(("red", "yellow", "green"), |color_obj| {
                    let red = color_obj
                        .get_string("red").map_or_else(|| "red", String::as_str);

                    let yellow = color_obj
                        .get_string("yellow").map_or_else(|| "yellow", String::as_str);

                    let green = color_obj
                        .get_string("green").map_or_else(|| "green", String::as_str);

                    (red, yellow, green)
                });

                let typ = o.get_string("type").map_or_else(|| "celsius", String::as_str);

                ConfigTemperature { 
                    green: format_color(green, ColorPlan::FG), 
                    yellow: format_color(yellow, ColorPlan::FG), 
                    red: format_color(red, ColorPlan::FG),
                    typ: TemperatureType::from_str(typ)
                }
            });

            ConfigDisplay { separator, percent, temperature }

        });

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
            modules,
            display,
            logo
        }
    }

    pub fn get() -> &'static Self {
        PRESET
            .get()
            .expect("Preset was not initialized at the start of the program")
    }

    pub fn module_by_typ(&self, string: &str) -> Option<&ConfigModule> {
        for m in self.modules.as_inner() {
            if m.typ == string {
                return Some(m)
            }
        }
        None
    }

    pub const fn modules(&self) -> &Vec<ConfigModule> {
        self.modules.as_inner()
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

    /// val -> 0..=100
    pub fn format_percent(&self, val: u8) -> String {
        let color = if val <= 50 {
            self.display.percent.green.as_str()
        } else if val <= 75 {
            self.display.percent.yellow.as_str()
        } else {
            self.display.percent.red.as_str()
        };

        format!("\x1b[{color}m{val}%\x1b[0m")
    }

    pub fn format_celsius(&self, mut val: f32) -> String {
        let typ = &self.display.temperature.typ;
        val = typ.convert_to_celsius(val);

        let color = if val <= 50.0 {
            self.display.temperature.green.as_str()
        } else if val <= 75.0 {
            self.display.temperature.yellow.as_str()
        } else {
            self.display.temperature.red.as_str()
        };

        format!("\x1b[{color}m{val:.01} °{}\x1b[0m", typ.symb())
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct ConfigModuleArray {
    pub inner: Vec<ConfigModule>
}

impl ConfigModuleArray {
    pub fn into_inner(self) -> Vec<ConfigModule> {
        self.inner
    }

    pub const fn as_inner(&self) -> &Vec<ConfigModule> {
        &self.inner
    }
}

impl Default for ConfigModuleArray {
    fn default() -> Self {
        let inner = vec![
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
        ];
        Self { inner }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ConfigDisplay {
    pub separator: String,
    pub percent: ConfigPercent,
    pub temperature: ConfigTemperature
}

impl Default for ConfigDisplay {
    fn default() -> Self {
        Self { 
            separator: String::from(": "),
            percent: ConfigPercent::default(),
            temperature: ConfigTemperature::default()
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ConfigPercent {
    pub green: String,
    pub yellow: String,
    pub red: String,
}

impl Default for ConfigPercent {
    fn default() -> Self {
        Self { green: "32".to_owned(), yellow: "33".to_owned(), red: "31".to_owned() }
    }
}

#[derive(Debug, Default)]
pub enum TemperatureType { 
    #[default]
    Celsius,
    Fahrenheit,
    Kelvin
}

impl TemperatureType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "celsius" => Self::Celsius,
            "fahrenheit" => Self::Fahrenheit,
            "kelvin" => Self::Kelvin,
            _ => {
                warning!("Unknown temperature type ({s})");
                Self::Celsius
            }
        }
    }
    pub const fn symb(&self) -> char {
        match self {
            Self::Celsius    => 'C',
            Self::Fahrenheit => 'F',
            Self::Kelvin     => 'K'
        }
    }

    pub const fn convert_to_celsius(&self, val: f32) -> f32 {
        match self {
            Self::Celsius    => val,
            Self::Fahrenheit => (val * 9.0 / 5.0) + 32.0,
            Self::Kelvin     => val + 273.15
        }
    }
}

#[derive(Debug)]
pub struct ConfigTemperature {
    pub green: String,
    pub yellow: String,
    pub red: String,
    pub typ: TemperatureType,
}

impl Default for ConfigTemperature {
    fn default() -> Self {
        Self {
            green: "32".to_owned(),
            yellow: "33".to_owned(),
            red: "31".to_owned(),
            typ: TemperatureType::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct ConfigLogo {
    pub padding: ConfigPadding
}