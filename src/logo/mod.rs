use alloc::{
    borrow::ToOwned,
    string::String,
    vec::Vec,
    vec
};

use crate::{
    color,
    format,
    sync::OnceLock,
    zlib::decompress
};

mod a;
mod b;
mod c;
mod d;
mod e;
mod f;
mod g;
mod h;
mod i;
mod j;
mod k;
mod l;
mod m;
mod n;
mod o;
mod p;
mod q;
mod r;
mod s;
mod t;
mod u;
mod v;
mod w;
mod x;
mod y;
mod z;

const UNKNOWN: &[u8] = include_bytes!("../../temp/_/unknown.txt");
static UNKNOWN_PTR: &LogoInfo = &LogoInfo {
    names: &[], 
    lines: UNKNOWN, 
    colors: &[], 
    color_keys: color::FG_DEFAULT, 
    color_title: color::FG_DEFAULT
};

static LOGO_INFO: OnceLock<&'static LogoInfo> = OnceLock::new();

pub struct LogoInfo {
    pub names: &'static [&'static str],
    pub lines: &'static [u8],
    pub colors: &'static [&'static str],
    pub color_keys: &'static str,
    pub color_title: &'static str
}

impl LogoInfo {
    pub fn new(name: &str) -> &Self {
        LOGO_INFO.get_or_init(|| {
            let first_char = name.chars().next().unwrap();

            let stack = match first_char {
                'a' | 'A' => a::get(),
                'b' | 'B' => b::get(),
                'c' | 'C' => c::get(),
                'd' | 'D' => d::get(),
                'e' | 'E' => e::get(),
                'f' | 'F' => f::get(),
                'g' | 'G' => g::get(),
                'h' | 'H' => h::get(),
                'i' | 'I' => i::get(),
                'j' | 'J' => j::get(),
                'k' | 'K' => k::get(),
                'l' | 'L' => l::get(),
                'm' | 'M' => m::get(),
                'n' | 'N' => n::get(),
                'o' | 'O' => o::get(),
                'p' | 'P' => p::get(),
                'q' | 'Q' => q::get(),
                'r' | 'R' => r::get(),
                's' | 'S' => s::get(),
                't' | 'T' => t::get(),
                'u' | 'U' => u::get(),
                'v' | 'V' => v::get(),
                'w' | 'W' => w::get(),
                'x' | 'X' => x::get(),
                'y' | 'Y' => y::get(),
                'z' | 'Z' => z::get(),
                _ => return UNKNOWN_PTR
            };

            for logo in stack {
                if logo.names.contains(&name.to_lowercase().as_str())  {
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
        let mut decompressed = Vec::new();
        decompress(self.lines.to_vec(), &mut decompressed);
        let lines_string = String::from_utf8(decompressed).expect("Non Utf8 in logo");

        let lines: Vec<String> = lines_string.lines().map(ToOwned::to_owned).collect();
        let mut ret = Vec::new();
        let mut cur_code = "";
        for line in lines {
            let mut ret_len = 0;
            let mut ret_line = format!("\x1b[1;{cur_code}m");
            let mut in_percent = false;
            
            for ch in line.chars() {
                if ch == '$' {
                    if in_percent {
                        ret_line.push('$');
                        ret_len += 1;
                        in_percent = false;
                        continue;
                    }
                    in_percent = true;
                    continue;
                }
                if in_percent {
                    if let Some(i) = ch.to_digit(10) && i > 0 {
                        let code = self.colors.get(i as usize - 1).unwrap_or(&"0");
                        cur_code = code;
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