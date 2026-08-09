use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

use crate::sync::OnceLock;

static CONFIG: OnceLock<Preset> = OnceLock::new();

pub struct Preset {
    modules: Vec<Module>
}

impl Preset {
    pub fn get_or_init(config: Self) -> &'static Self {
        CONFIG.get_or_init(|| {
            config
        })
    }

    pub fn get() -> Option<&'static Self> {
        CONFIG.get()
    }

    pub fn module_by_typ(&self, string: &str) -> Option<&Module> {
        for m in &self.modules {
            if m.typ == string {
                return Some(m)
            }
        }
        None
    }

    pub fn modules(&self) -> &Vec<Module> {
        &self.modules
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            modules: vec![
                Module::from_str("title"),
                Module::from_str("separator"),
                Module::from_str("os"),
                Module::from_str("cpu"),
                Module::from_str("weather"),
                Module::from_str("locale"),
                Module::from_str("break"),
                Module::from_str("colors")
            ]
        }
    }
}

pub struct Module {
    pub typ: String,
    pub format: Option<String>
}

impl Module {
    pub fn from_str(string: &str) -> Self {
        Self { typ: string.to_owned(), format: None }
    }
}