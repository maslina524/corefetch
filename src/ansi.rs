use alloc::{
    string::String,
    vec::Vec
};

use crate::format;

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
    use crate::ansi::unicode_to_hex;

    #[test]
    fn test_conversion() {
        println!("{}", unicode_to_hex(r"\u001b[31m \u001b[32m \u001b[33m \u001b[34m \u001b[0m"));
    }
}