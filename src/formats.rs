use core::{
    cmp::Ordering,
    fmt::Write
};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

// Why does clippy think this variant is better than `colors::*`?
use crate::{
    color::{
        MODE_RESET, MODE_BOLD, MODE_DIM, MODE_ITALIC, MODE_UNDERLINE, MODE_BLINK, MODE_INVERSE,
        MODE_HIDDEN, MODE_STRIKETHROUGH, FG_BLACK, FG_LIGHT_BLACK, BG_BLACK, BG_LIGHT_BLACK,
        FG_RED, FG_LIGHT_RED, BG_RED, BG_LIGHT_RED, FG_GREEN, FG_LIGHT_GREEN, BG_GREEN,
        BG_LIGHT_GREEN, FG_YELLOW, FG_LIGHT_YELLOW, BG_YELLOW, BG_LIGHT_YELLOW, FG_BLUE,
        FG_LIGHT_BLUE, BG_BLUE, BG_LIGHT_BLUE, FG_MAGENTA, FG_LIGHT_MAGENTA, BG_MAGENTA,
        BG_LIGHT_MAGENTA, FG_CYAN, FG_LIGHT_CYAN, BG_CYAN, BG_LIGHT_CYAN, FG_WHITE,
        FG_LIGHT_WHITE, BG_WHITE, BG_LIGHT_WHITE, BG_DEFAULT, FG_DEFAULT
    },
    config::Config
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ColorPlan { FG, BG }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemorySize {
    Byte(u16),
    Kb(f32),
    Mb(f32),
    Gb(f32)
}

impl MemorySize {
    pub fn from_bytes(bytes: u64) -> Self {
        let mut divisions = 0;
        #[allow(clippy::cast_precision_loss)]
        // ^^ Mantissa is 52 bits, 2^52 = 4 petabytes
        let mut f_bytes = bytes as f64;

        while f_bytes >= 1024.0 && divisions < 3 {
            f_bytes /= 1024.0;
            divisions += 1;
        }

        match divisions {
            0 => Self::Byte(bytes as u16),
            1 => Self::Kb(f_bytes as f32),
            2 => Self::Mb(f_bytes as f32),
            3 => Self::Gb(f_bytes as f32),
            _ => unreachable!()
        }
    }

    pub fn as_bytes(self) -> u64 {
        match self {
            Self::Byte(b) => b as u64,
            Self::Kb(b) => (b * 1024.0) as u64,
            Self::Mb(b) => (b * 1024.0 * 1024.0) as u64,
            Self::Gb(b) => (b * 1024.0 * 1024.0 * 1024.0) as u64
        }
    }

    pub fn as_kilobytes(self) -> f64 {
        match self {
            Self::Byte(b) => b as f64 / 1024.0,
            Self::Kb(b) => b as f64,
            Self::Mb(b) => (b * 1024.0) as f64,
            Self::Gb(b) => (b * 1024.0 * 1024.0) as f64
        }
    }
}

impl From<MemorySize> for f64 {
    #[allow(clippy::cast_precision_loss)]
    fn from(val: MemorySize) -> Self {
        val.as_bytes() as Self
    }
}

impl Default for MemorySize {
    fn default() -> Self {
        Self::Byte(0)
    }
}

impl core::fmt::Display for MemorySize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Byte(b) => write!(f, "{b} Bytes"),
            Self::Kb(b)   => write!(f, "{b:.02} Kb"),
            Self::Mb(b)   => write!(f, "{b:.02} Mb"),
            Self::Gb(b)   => write!(f, "{b:.02} Gb"),
        }
    }
}

impl PartialOrd for MemorySize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_bytes().partial_cmp(&other.as_bytes())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Percent(u8);

impl Percent {
    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn new(mut percent: u8) -> Self {
        if percent > 100 {
            percent = 100;
        }
        Self(percent)
    }

    pub const fn new_check(percent: u8) -> Option<Self> {
        if percent > 100 {
            None
        } else {
            Some(Self(percent))
        }
    }
}

impl From<Percent> for f64 {
    #[allow(clippy::cast_precision_loss)]
    fn from(val: Percent) -> Self {
        val.get() as Self
    }
}

impl core::fmt::Display for Percent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = Config::get().format_percent(*self);
        write!(f, "{val}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
    Kelvin(f32),
}

impl Temperature {
    pub const fn get(self) -> f32 {
        match self {
            Self::Celsius(t) | Self::Fahrenheit(t) | Self::Kelvin(t) => t
        }
    }

    pub const fn symbol(self) -> char {
        match self {
            Self::Celsius(_)    => 'C',
            Self::Fahrenheit(_) => 'F',
            Self::Kelvin(_)     => 'K'
        }
    }

    pub const fn as_celsius(self) -> Self {
        let temp = match self {
            Self::Celsius(t)    => t,
            Self::Fahrenheit(t) => (t - 32.0) * 5.0 / 9.0,
            Self::Kelvin(t)     => t - 273.15
        };
        Self::Celsius(temp)
    }

    pub const fn as_fahrenheit(self) -> Self {
        let temp = match self {
            Self::Celsius(t)    => (t * 9.0 / 5.0) + 32.0,
            Self::Fahrenheit(t) => t,
            Self::Kelvin(t)     => (t - 273.15) * 9.0 / 5.0 + 32.0
        };
        Self::Fahrenheit(temp)
    }

    pub const fn as_kelvin(self) -> Self {
        let temp = match self {
            Self::Celsius(t)    => t + 273.15,
            Self::Fahrenheit(t) => (t - 32.0) * 5.0 / 9.0 + 273.15,
            Self::Kelvin(t)     => t
        };
        Self::Kelvin(temp)
    }
}

impl From<Temperature> for f64 {
    #[allow(clippy::cast_precision_loss)]
    fn from(val: Temperature) -> Self {
        val.get() as Self
    }
}

impl core::fmt::Display for Temperature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let val = Config::get().format_temperature(*self);
        write!(f, "{val}")
    }
}

pub struct StringFormatter<'a>(&'a mut String);

impl Write for StringFormatter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}

impl<'a> StringFormatter<'a> {
    pub const fn new(ptr: &'a mut String) -> Self {
        Self(ptr)
    }
    
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        self.0.write_str("\n")
    }
}

pub fn visible_len(s: &str) -> usize {
    let mut count = 0;
    let mut skip = false;
    for ch in s.chars() {
        if skip {
            if ch == 'm' {
                skip = false;
            }
            continue;
        }
        if ch == '\x1b' {
            skip = true;
            continue;
        }
        count += 1;
    }
    count
}

pub fn expand_unicode(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        if pos + 6 <= chars.len()
            && chars[pos] == '\\'
            && chars[pos + 1] == 'u'
        {
            let hex: String = chars[pos + 2..pos + 6].iter().collect();
            if let Ok(num) = u32::from_str_radix(&hex, 16) {
                if let Some(ch) = char::from_u32(num) {
                    ret.push(ch);
                }
                pos += 6;
                continue;
            }
        }
        ret.push(chars[pos]);
        pos += 1;
    }
    ret
}

pub fn expand_rust_unicode(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        if pos + 3 <= chars.len()
            && chars[pos] == '\\'
            && chars[pos + 1] == 'u'
            && chars[pos + 2] == '{'
        {   
            pos += 3;
            let mut hex = String::new();
            while pos < chars.len() && chars[pos] != '}' {
                hex.push(chars[pos]);
                pos += 1;
            }
            pos += 1;

            if let Ok(num) = u32::from_str_radix(&hex, 16)
                && let Some(ch) = char::from_u32(num)
            {   
                ret.push(ch);
            }
            continue;
        }
        ret.push(chars[pos]);
        pos += 1;
    }
    ret
}

macro_rules! add_prefix {
    ($prefixes:expr, $ret:expr, $lit:literal, $constant:expr) => {{
        if $prefixes.contains(&$lit) {
            $ret.push($constant);
        }
    }};
}

fn is_ansi_color(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit() || c == ';')
}

pub fn format_color(s: &str, plan: ColorPlan) -> String {
    if is_ansi_color(s) {
        return s.to_owned();
    }

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
    add_prefix!(prefixes, ret, "bold",      MODE_BOLD);
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
        ("black", false, ColorPlan::FG) => FG_BLACK,
        ("black", true,  ColorPlan::FG) => FG_LIGHT_BLACK,
        ("black", false, ColorPlan::BG) => BG_BLACK,
        ("black", true,  ColorPlan::BG) => BG_LIGHT_BLACK,
        // Red
        ("red", false, ColorPlan::FG) => FG_RED,
        ("red", true,  ColorPlan::FG) => FG_LIGHT_RED,
        ("red", false, ColorPlan::BG) => BG_RED,
        ("red", true,  ColorPlan::BG) => BG_LIGHT_RED,
        // Green
        ("green", false, ColorPlan::FG) => FG_GREEN,
        ("green", true,  ColorPlan::FG) => FG_LIGHT_GREEN,
        ("green", false, ColorPlan::BG) => BG_GREEN,
        ("green", true,  ColorPlan::BG) => BG_LIGHT_GREEN,
        // Yellow
        ("yellow", false, ColorPlan::FG) => FG_YELLOW,
        ("yellow", true,  ColorPlan::FG) => FG_LIGHT_YELLOW,
        ("yellow", false, ColorPlan::BG) => BG_YELLOW,
        ("yellow", true,  ColorPlan::BG) => BG_LIGHT_YELLOW,
        // Blue
        ("blue", false, ColorPlan::FG) => FG_BLUE,
        ("blue", true,  ColorPlan::FG) => FG_LIGHT_BLUE,
        ("blue", false, ColorPlan::BG) => BG_BLUE,
        ("blue", true,  ColorPlan::BG) => BG_LIGHT_BLUE,
        // Magenta
        ("magenta", false, ColorPlan::FG) => FG_MAGENTA,
        ("magenta", true,  ColorPlan::FG) => FG_LIGHT_MAGENTA,
        ("magenta", false, ColorPlan::BG) => BG_MAGENTA,
        ("magenta", true,  ColorPlan::BG) => BG_LIGHT_MAGENTA,
        // Cyan
        ("cyan", false, ColorPlan::FG) => FG_CYAN,
        ("cyan", true,  ColorPlan::FG) => FG_LIGHT_CYAN,
        ("cyan", false, ColorPlan::BG) => BG_CYAN,
        ("cyan", true,  ColorPlan::BG) => BG_LIGHT_CYAN,
        // White
        ("white", false, ColorPlan::FG) => FG_WHITE,
        ("white", true,  ColorPlan::FG) => FG_LIGHT_WHITE,
        ("white", false, ColorPlan::BG) => BG_WHITE,
        ("white", true,  ColorPlan::BG) => BG_LIGHT_WHITE,
        // Unknown color -> default
        _ => if plan == ColorPlan::BG { BG_DEFAULT } else { FG_DEFAULT },
    };
    ret.push(color_str);

    ret.join(";")
}

pub fn split_by_len_ansi(s: &str, len: usize) -> Vec<String> {
    if s.trim().is_empty() {
        return vec![String::new()];
    }
    let chars: Vec<char> = s.chars().collect();
    let mut ret = Vec::new();
    let mut build_string_len = 0;
    let mut build_string = String::new();
    let mut currect_ansi = String::from("\x1b[0m");
    let mut pos = 0;

    while pos < chars.len() {
        if chars[pos] == '\x1b' {
            currect_ansi = String::new();
            while pos < chars.len() && chars[pos] != 'm' {
                currect_ansi.push(chars[pos]);
                pos += 1;
            }
            currect_ansi.push('m');
            pos += 1;

            build_string.push_str(&currect_ansi);
            continue;
        }

        if chars[pos] == '\n' {
            if currect_ansi != "\x1b[0m" {
                build_string.push_str("\x1b[0m");
            }

            if visible_len(&build_string) > 0 {
                ret.push(build_string);
            }

            build_string_len = 0;
            build_string = if currect_ansi == "\x1b[0m" {
                String::new()
            } else {
                currect_ansi.clone()
            };
            pos += 1;
            continue;
        }

        build_string.push(chars[pos]);
        pos += 1;
        build_string_len += 1;

        if build_string_len >= len {
            if currect_ansi != "\x1b[0m" {
                build_string.push_str("\x1b[0m");
            }
            ret.push(build_string);
            build_string_len = 0;
            build_string = if currect_ansi == "\x1b[0m" {
                String::new()
            } else {
                currect_ansi.clone()
            };
        }
    }

    if visible_len(&build_string) > 0 {
        ret.push(build_string);
    }

    ret
}

pub fn snake_to_camel_ascii(s: &str) -> String {
    let mut ret = String::with_capacity(s.len());
    let chars = s.chars();
    let mut transition = false;

    for ch in chars {
        if ch == '_' || ch == '-' {
            transition = true;
            continue;
        }
        if transition {
            if ch.is_ascii_lowercase() {
                let idx = ch as u32 - 32;
                ret.push(char::from_u32(idx).unwrap());
            } else {
                ret.push(ch);
            }

            transition = false;
            continue;
        }
        
        ret.push(ch);
    }

    ret
}

#[macro_export]
macro_rules! format {
    ($($tt:tt)*) => {{
        let mut string = alloc::string::String::new();
        let mut formatter = $crate::formats::StringFormatter::new(&mut string);
        let _ = formatter.write_fmt(format_args!($($tt)*));
        string
    }};
}

#[macro_export]
macro_rules! formatln {
    ($($tt:tt)*) => {{
        let mut string = alloc::string::String::new();
        let mut formatter = $crate::formats::StringFormatter::new(&mut string);
        let _ = formatter.write_fmt(format_args!($($tt)*));
        let _ = formatter.write_nl();
        string
    }};
}

#[cfg(test)]
mod tests {
    use crate::formats::{MemorySize, expand_rust_unicode, expand_unicode, split_by_len_ansi};

    #[test]
    fn test_conversion() {
        println!("{}", expand_unicode(r"\u001b[31m \u001b[32m \u001b[33m \u001b[34m \u001b[0m"));
    }

    #[test]
    fn expand_rust_test() {
        println!("{}", expand_rust_unicode(r"\u{1b}[33mHello\u{1b}[0m"));
    }

    #[test]
    fn from_bytes_test() {
        assert_eq!(MemorySize::from_bytes(512).to_string(),           "512 Bytes");
        assert_eq!(MemorySize::from_bytes(1024).to_string(),          "1.00 Kb");
        assert_eq!(MemorySize::from_bytes(1536).to_string(),          "1.50 Kb");
        assert_eq!(MemorySize::from_bytes(536_870_912).to_string(),   "512.00 Mb");
        assert_eq!(MemorySize::from_bytes(2_147_483_648).to_string(), "2.00 Gb");
    }

    #[test]
    fn split_by_len_test() {
        let s = "HelloHelloHello";
        let lines = split_by_len_ansi(s, 5);
        assert_eq!(lines, vec![
            "Hello",
            "Hello",
            "Hello"
        ]);
    }

    #[test]
    fn split_by_len_ansi_test() {
        let s = "\x1b[31mHelloHelloHello\x1b[0m";
        let lines = split_by_len_ansi(s, 5);
        assert_eq!(lines, vec![
            "\x1b[31mHello\x1b[0m",
            "\x1b[31mHello\x1b[0m",
            "\x1b[31mHello\x1b[0m"
        ]);
    }
}