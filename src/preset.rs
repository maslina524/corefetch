use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

use crate::{
    modules::Module, 
    sync::OnceLock,
    json::Map
};

static PRESET: OnceLock<Preset> = OnceLock::new();

pub struct Preset {
    modules: Vec<PresetModule>
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

                let preset_mod = PresetModule::new(typ, format, key);
                ret_modules.push(preset_mod);
            } else if let Some(typ) = m.as_string() {
                let preset_mod = PresetModule::from_str(typ);
                ret_modules.push(preset_mod);
            }
        }

        Self {
            modules: ret_modules
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
            ]
        }
    }
}

pub struct PresetModule {
    pub typ: String,
    pub format: Option<String>,
    pub key: Option<String>
}

impl PresetModule {
    pub fn from_str(typ: &str) -> Self {
        Self { typ: typ.to_owned(), format: None, key: None }
    }

    pub fn new(typ: &str, format: Option<String>, key: Option<String>) -> Self {
        Self { typ: typ.to_owned(), format, key }
    }
}