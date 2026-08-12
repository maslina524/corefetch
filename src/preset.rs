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
    // Why does clippy think this variant is better than `colors::*`?
    color::{
        MODE_RESET, MODE_DIM, MODE_ITALIC, MODE_UNDERLINE, MODE_BLINK, MODE_INVERSE,
        MODE_HIDDEN, MODE_STRIKETHROUGH, FG_BLACK, FG_LIGHT_BLACK, BG_BLACK, BG_LIGHT_BLACK,
        FG_RED, FG_LIGHT_RED, BG_RED, BG_LIGHT_RED, FG_GREEN, FG_LIGHT_GREEN, BG_GREEN,
        BG_LIGHT_GREEN, FG_YELLOW, FG_LIGHT_YELLOW, BG_YELLOW, BG_LIGHT_YELLOW, FG_BLUE,
        FG_LIGHT_BLUE, BG_BLUE, BG_LIGHT_BLUE, FG_MAGENTA, FG_LIGHT_MAGENTA, BG_MAGENTA,
        BG_LIGHT_MAGENTA, FG_CYAN, FG_LIGHT_CYAN, BG_CYAN, BG_LIGHT_CYAN, FG_WHITE,
        FG_LIGHT_WHITE, BG_WHITE, BG_LIGHT_WHITE, BG_DEFAULT, FG_DEFAULT
    }
};

static PRESET: OnceLock<Preset> = OnceLock::new();

macro_rules! add_prefix {
    ($prefixes:expr, $ret:expr, $lit:literal, $constant:expr) => {{
        if $prefixes.contains(&$lit) {
            $ret.push($constant);
        }
    }};
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Plan {
    FG, BG
}

pub struct Preset {
    modules: Vec<PresetModule>,
    display: PresetDisplay,
    logo: PresetLogo,
}

impl Preset {
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

                let preset_mod = PresetModule::new(typ, format, key, key_color, map);
                ret_modules.push(preset_mod);
            } else if let Some(typ) = m.as_string() {
                let preset_mod = PresetModule::from_str(typ);
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
            .map_or_else(PresetLogo::default, |padding| PresetLogo {
                padding: PresetPadding {
                    top: padding.get_number("top").unwrap_or(0.0) as usize,
                    bottom: padding.get_number("bottom").unwrap_or(2.0) as usize,
                    right: padding.get_number("right").unwrap_or(3.0) as usize,
                    left: padding.get_number("left").unwrap_or(0.0) as usize,
                }
            }
        );

        Self {
            modules: ret_modules,
            display: PresetDisplay { separator },
            logo
        }
    }

    pub fn get() -> &'static Self {
        PRESET
            .get()
            .expect("Preset was not initialized at the start of the program")
    }

    pub fn module_by_typ(&self, string: &str) -> Option<&PresetModule> {
        for m in &self.modules {
            if m.typ == string {
                return Some(m)
            }
        }
        None
    }

    pub const fn modules(&self) -> &Vec<PresetModule> {
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

    pub const fn get_logo_padding(&self) -> &PresetPadding {
        &self.logo.padding
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            modules: vec![
                PresetModule::from_str("title"),
                PresetModule::from_str("separator"),
                PresetModule::from_str("os"),
                PresetModule::from_str("kernel"),
                PresetModule::from_str("datetime"),
                PresetModule::from_str("precesses"),
                PresetModule::from_str("cpu"),
                PresetModule::from_str("memory"),
                PresetModule::from_str("weather"),
                PresetModule::from_str("locale"),
                PresetModule::from_str("break"),
                PresetModule::from_str("colors")
            ],
            display: PresetDisplay {
                separator: ": ".to_owned()
            },
            logo: PresetLogo::default()
        }
    }
}

pub struct PresetModule {
    pub typ: String,
    pub format: Option<String>,
    pub key: Option<String>,
    pub key_color: Option<String>,
    pub map: BTreeMap<String, Value>
}

impl PresetModule {
    pub fn from_str(typ: &str) -> Self {
        Self { typ: typ.to_owned(), format: None, key: None, key_color: None, map: BTreeMap::new() }
    }

    pub fn new(typ: &str, format: Option<String>, key: Option<String>, key_color: Option<String>, map: BTreeMap<String, Value>) -> Self {
        let key_color = key_color.map(|s| Self::format_color(&s, Plan::FG));
        Self { typ: typ.to_owned(), format, key, key_color, map }
    }

    fn format_color(s: &str, plan: Plan) -> String {
        let count = s.matches('_').count();
        let mut ret = Vec::with_capacity(count + 1);

        let (color, prefixes) = if count == 0 {
            (s, Vec::new())
        } else {
            let mut parts: Vec<&str> = s.split('_').collect();
            let color = parts.pop().unwrap();
            (color, parts)
        };

        // Supported named prefixes:
        // reset_, bright_, dim_, italic_, underline_,
        // blink_, inverse_, hidden_, strike_, light_
        add_prefix!(prefixes, ret, "reset",     MODE_RESET);
        add_prefix!(prefixes, ret, "dim",       MODE_DIM);
        add_prefix!(prefixes, ret, "italic",    MODE_ITALIC);
        add_prefix!(prefixes, ret, "underline", MODE_UNDERLINE);
        add_prefix!(prefixes, ret, "blink",     MODE_BLINK);
        add_prefix!(prefixes, ret, "inverse",   MODE_INVERSE);
        add_prefix!(prefixes, ret, "hidden",    MODE_HIDDEN);
        add_prefix!(prefixes, ret, "strike",    MODE_STRIKETHROUGH);

        let is_light = prefixes.contains(&"light");
        let color_str = match (color, is_light, plan) {
            // Black
            ("black", false, Plan::FG) => FG_BLACK,
            ("black", true,  Plan::FG) => FG_LIGHT_BLACK,
            ("black", false, Plan::BG) => BG_BLACK,
            ("black", true,  Plan::BG) => BG_LIGHT_BLACK,
            // Red
            ("red", false, Plan::FG) => FG_RED,
            ("red", true,  Plan::FG) => FG_LIGHT_RED,
            ("red", false, Plan::BG) => BG_RED,
            ("red", true,  Plan::BG) => BG_LIGHT_RED,
            // Green
            ("green", false, Plan::FG) => FG_GREEN,
            ("green", true,  Plan::FG) => FG_LIGHT_GREEN,
            ("green", false, Plan::BG) => BG_GREEN,
            ("green", true,  Plan::BG) => BG_LIGHT_GREEN,
            // Yellow
            ("yellow", false, Plan::FG) => FG_YELLOW,
            ("yellow", true,  Plan::FG) => FG_LIGHT_YELLOW,
            ("yellow", false, Plan::BG) => BG_YELLOW,
            ("yellow", true,  Plan::BG) => BG_LIGHT_YELLOW,
            // Blue
            ("blue", false, Plan::FG) => FG_BLUE,
            ("blue", true,  Plan::FG) => FG_LIGHT_BLUE,
            ("blue", false, Plan::BG) => BG_BLUE,
            ("blue", true,  Plan::BG) => BG_LIGHT_BLUE,
            // Magenta
            ("magenta", false, Plan::FG) => FG_MAGENTA,
            ("magenta", true,  Plan::FG) => FG_LIGHT_MAGENTA,
            ("magenta", false, Plan::BG) => BG_MAGENTA,
            ("magenta", true,  Plan::BG) => BG_LIGHT_MAGENTA,
            // Cyan
            ("cyan", false, Plan::FG) => FG_CYAN,
            ("cyan", true,  Plan::FG) => FG_LIGHT_CYAN,
            ("cyan", false, Plan::BG) => BG_CYAN,
            ("cyan", true,  Plan::BG) => BG_LIGHT_CYAN,
            // White
            ("white", false, Plan::FG) => FG_WHITE,
            ("white", true,  Plan::FG) => FG_LIGHT_WHITE,
            ("white", false, Plan::BG) => BG_WHITE,
            ("white", true,  Plan::BG) => BG_LIGHT_WHITE,
            // Unknown color → default
            _ => if plan == Plan::BG { BG_DEFAULT } else { FG_DEFAULT },
        };
        ret.push(color_str);

        ret.join(";")
    }
}

pub struct PresetDisplay {
    pub separator: String
}

impl Default for PresetDisplay {
    fn default() -> Self {
        Self { separator: String::from(": ") }
    }
}

pub struct PresetPadding {
    pub top: usize,
    pub bottom: usize,
    pub right: usize,
    pub left: usize,
}

impl Default for PresetPadding {
    fn default() -> Self {
        Self { top: 0, bottom: 2, right: 3, left: 0 }
    }
}

#[derive(Default)]
pub struct PresetLogo {
    pub padding: PresetPadding
}