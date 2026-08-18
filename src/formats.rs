use core::cmp::Ordering;

use alloc::{
    string::String,
    vec::Vec
};

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