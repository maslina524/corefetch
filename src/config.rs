use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec
};

pub struct Config {
    modules: Vec<Module>
}

pub struct Module {
    typ: String,
    format: Option<String>
}

impl Module {
    pub fn from_str(string: &str) -> Self {
        Self { typ: string.to_owned(), format: None }
    }
}