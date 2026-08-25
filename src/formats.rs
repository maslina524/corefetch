use core::{
    cmp::Ordering,
    fmt::Write
};

use alloc::{
    string::String,
    vec::Vec
};

// Why does clippy think this variant is better than `colors::*`?
use crate::color::{
    MODE_RESET, MODE_DIM, MODE_ITALIC, MODE_UNDERLINE, MODE_BLINK, MODE_INVERSE,
    MODE_HIDDEN, MODE_STRIKETHROUGH, FG_BLACK, FG_LIGHT_BLACK, BG_BLACK, BG_LIGHT_BLACK,
    FG_RED, FG_LIGHT_RED, BG_RED, BG_LIGHT_RED, FG_GREEN, FG_LIGHT_GREEN, BG_GREEN,
    BG_LIGHT_GREEN, FG_YELLOW, FG_LIGHT_YELLOW, BG_YELLOW, BG_LIGHT_YELLOW, FG_BLUE,
    FG_LIGHT_BLUE, BG_BLUE, BG_LIGHT_BLUE, FG_MAGENTA, FG_LIGHT_MAGENTA, BG_MAGENTA,
    BG_LIGHT_MAGENTA, FG_CYAN, FG_LIGHT_CYAN, BG_CYAN, BG_LIGHT_CYAN, FG_WHITE,
    FG_LIGHT_WHITE, BG_WHITE, BG_LIGHT_WHITE, BG_DEFAULT, FG_DEFAULT
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ColorPlan { FG, BG }

#[derive(Clone, PartialEq)]
pub enum Size {
    Byte(u16),
    Kb(f32),
    Mb(f32),
    Gb(f32)
}

impl Size {
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

    pub fn as_bytes(&self) -> u64 {
        match self {
            Self::Byte(b) => *b as u64,
            Self::Kb(b) => (b * 1024.0) as u64,
            Self::Mb(b) => (b * 1024.0 * 1024.0) as u64,
            Self::Gb(b) => (b * 1024.0 * 1024.0 * 1024.0) as u64
        }
    }

    pub fn as_kilobytes(&self) -> f64 {
        match self {
            Self::Byte(b) => *b as f64 / 1024.0,
            Self::Kb(b) => *b as f64,
            Self::Mb(b) => (b * 1024.0) as f64,
            Self::Gb(b) => (b * 1024.0 * 1024.0) as f64
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Byte(0)
    }
}

impl core::fmt::Display for Size {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Byte(b) => write!(f, "{b} Bytes"),
            Self::Kb(b)   => write!(f, "{b:.02} Kb"),
            Self::Mb(b)   => write!(f, "{b:.02} Mb"),
            Self::Gb(b)   => write!(f, "{b:.02} Gb"),
        }
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_bytes().partial_cmp(&other.as_bytes())
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
    let mut ret = String::new();
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

macro_rules! add_prefix {
    ($prefixes:expr, $ret:expr, $lit:literal, $constant:expr) => {{
        if $prefixes.contains(&$lit) {
            $ret.push($constant);
        }
    }};
}

pub fn format_color(s: &str, plan: ColorPlan) -> String {
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
        // Unknown color → default
        _ => if plan == ColorPlan::BG { BG_DEFAULT } else { FG_DEFAULT },
    };
    ret.push(color_str);

    ret.join(";")
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
    use crate::formats::{Size, expand_unicode};

    #[test]
    fn test_conversion() {
        println!("{}", expand_unicode(r"\u001b[31m \u001b[32m \u001b[33m \u001b[34m \u001b[0m"));
    }

    #[test]
    fn from_bytes_test() {
        assert_eq!(Size::from_bytes(512).to_string(),           "512 Bytes");
        assert_eq!(Size::from_bytes(1024).to_string(),          "1.00 Kb");
        assert_eq!(Size::from_bytes(1536).to_string(),          "1.50 Kb");
        assert_eq!(Size::from_bytes(536_870_912).to_string(),   "512.00 Mb");
        assert_eq!(Size::from_bytes(2_147_483_648).to_string(), "2.00 Gb");
    }
}