use alloc::{
    borrow::ToOwned,
    string::String,
    vec::Vec,
    vec
};

use crate::{
    color,
    format,
    sync::OnceLock
};

mod w;

const UNKNOWN: &str = include_str!("_/unknown.txt");
static UNKNOWN_PTR: &LogoInfo = &LogoInfo {
    names: vec![], 
    lines: UNKNOWN, 
    colors: vec![], 
    color_keys: color::FG_DEFAULT, 
    color_title: color::FG_DEFAULT
};

static LOGO_INFO: OnceLock<&'static LogoInfo> = OnceLock::new();

pub struct LogoInfo {
    names: Vec<&'static str>,
    lines: &'static str,
    colors: Vec<&'static str>,
    color_keys: &'static str,
    color_title: &'static str
}

impl LogoInfo {
    pub fn new(name: &str) -> &Self {
        LOGO_INFO.get_or_init(|| {
            let first_char = name.chars().next().unwrap();

            let stack = match first_char {
                'w' | 'W' => w::get(),
                _ => return UNKNOWN_PTR
            };

            for logo in stack {
                if logo.names.contains(&name)  {
                    return logo;
                }
            }

            UNKNOWN_PTR
        })
    }

    pub fn get() -> Option<&'static Self> {
        LOGO_INFO.get().map(|v| &**v)
    }

    pub fn get_ansi_lines(&self) -> Vec<(String, usize)> {
        let lines: Vec<String> = self.lines.lines().map(ToOwned::to_owned).collect();
        let mut ret = Vec::new();

        for line in lines {
            let mut ret_len = 0;
            let mut ret_line = String::new();
            let mut in_percent = false;
            for ch in line.chars() {
                if ch == '$' {
                    in_percent = true;
                    continue;
                }
                if in_percent {
                    if let Some(i) = ch.to_digit(10) && i > 0 {
                        let code = self.colors[i as usize - 1];
                        ret_line.push_str(&format!("\x1b[{code}m"));
                    } else {
                        ret_line.push('$');
                        ret_line.push(ch);
                        ret_len += 2;
                    }
                    in_percent = false;
                    continue;
                }
                ret_line.push(ch);
                ret_len += 1;
            }
            ret_line.push_str("\x1b[0m");

            ret.push((ret_line, ret_len));
        }

        ret
    }
}