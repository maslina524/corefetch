use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

use crate::{
    modules::Module, 
    sync::OnceLock
};

static PRESET: OnceLock<Preset> = OnceLock::new();

pub struct Preset<'a> {
    modules: Vec<PresetModule<'a>>
}

impl Preset<'_> {
    pub fn get_or_init(config: Self) -> &'static Self {
        PRESET.get_or_init(|| {
            config
        })
    }

    pub fn get() -> &'static Self {
        PRESET
            .get()
            .expect("Preset was not initialized at the start of the program")
    }

    pub fn module_by_typ(&self, string: &str) -> Option<&PresetModule<'_>> {
        for m in &self.modules {
            if m.typ == string {
                return Some(m)
            }
        }
        None
    }

    pub const fn modules(&self) -> &Vec<PresetModule<'_>> {
        &self.modules
    }

    pub fn get_module_format(&self, module: &dyn Module) -> &str {
        let preset_module = self.module_by_typ(module.string_name()).unwrap();
        preset_module
        .format
        .unwrap_or_else(|| module.title())
    }
}

impl Default for Preset<'_> {
    fn default() -> Self {
        Self {
            modules: vec![
                PresetModule::from_str("title"),
                PresetModule::from_str("separator"),
                PresetModule::from_str("os"),
                PresetModule::from_str("kernel"),
                PresetModule::from_str("cpu"),
                PresetModule::from_str("memory"),
                PresetModule::from_str("locale"),
                PresetModule::from_str("break"),
                PresetModule::from_str("colors")
            ]
        }
    }
}

pub struct PresetModule<'a> {
    pub typ: &'a str,
    pub format: Option<&'a str>
}

impl<'a> PresetModule<'a> {
    pub const fn from_str(typ: &'a str) -> Self {
        Self { typ, format: None }
    }

    pub const fn new(typ: &'a str, format: Option<&'a str>) -> Self {
        Self { typ, format }
    }
}