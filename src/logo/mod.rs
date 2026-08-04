use alloc::{
    vec::Vec,
    vec
};

use crate::color;

mod w;

pub const UNKNOWN: &str = include_str!("_/unknown.txt");
pub const UNKNOWN_LOGO: LogoInfo = LogoInfo {
    names: vec![], 
    lines: UNKNOWN, 
    colors: vec![], 
    color_keys: color::FG_DEFAULT, 
    color_title: color::FG_DEFAULT
};
pub static UNKNOWN_PTR: &LogoInfo = &UNKNOWN_LOGO;
            
pub struct LogoInfo {
    names: Vec<&'static str>,
    lines: &'static str,
    colors: Vec<&'static str>,
    color_keys: &'static str,
    color_title: &'static str
}

impl LogoInfo {
    pub fn new(name: &str) -> &Self {
        let first_char = name.chars().next().unwrap();

        let stack = match first_char {
            'w' => w::get(),
            _ => return UNKNOWN_PTR
        };

        for logo in stack {
            if logo.names.contains(&name)  {
                return logo;
            }
        }

        UNKNOWN_PTR
    }
}