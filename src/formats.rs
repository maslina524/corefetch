use alloc::{
    string::String,
    vec::Vec
};

use crate::format;

pub enum FileSize {
    Byte(u16),
    Kb(f32),
    Mb(f32),
    Gb(f32)
}

impl FileSize {
    pub fn from_bytes(bytes: u32) -> Self {
        let mut divisions = 0;
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
}

impl core::fmt::Display for FileSize {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Byte(b) => write!(f, "{b} Bytes"),
            Self::Kb(b)   => write!(f, "{b:.02} Kb"),
            Self::Mb(b)   => write!(f, "{b:.02} Mb"),
            Self::Gb(b)   => write!(f, "{b:.02} Gb"),
        }
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

pub fn unicode_to_hex(s: &str) -> String {
    let mut ret = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        if pos + 6 <= chars.len()
            && chars[pos] == '\\'
            && chars[pos + 1] == 'u'
        {
            let hex: String = chars[pos + 2..pos + 6].iter().collect();
            if let Ok(num) = u8::from_str_radix(&hex, 16) {
                ret.push(num as char);
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
    use crate::formats::{FileSize, unicode_to_hex};

    #[test]
    fn test_conversion() {
        println!("{}", unicode_to_hex(r"\u001b[31m \u001b[32m \u001b[33m \u001b[34m \u001b[0m"));
    }

    #[test]
    fn from_bytes_test() {
        assert_eq!(FileSize::from_bytes(512).to_string(),           "512 Bytes");
        assert_eq!(FileSize::from_bytes(1024).to_string(),          "1.00 Kb");
        assert_eq!(FileSize::from_bytes(1536).to_string(),          "1.50 Kb");
        assert_eq!(FileSize::from_bytes(536_870_912).to_string(),   "512.00 Mb");
        assert_eq!(FileSize::from_bytes(2_147_483_648).to_string(), "2.00 Gb");
    }
}