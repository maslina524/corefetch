#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(
    clippy::all,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unsafe_removed_from_name, 
    clippy::unsafe_derive_deserialize,
    clippy::missing_safety_doc,
    clippy::as_ptr_cast_mut,
    clippy::mut_from_ref,
    clippy::fn_to_numeric_cast_any,
)]
#![allow(unused)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

mod sync;
mod os;
mod macros;
mod modules;
mod color;
mod logo;
mod detect;

extern crate alloc;

use core::ffi::c_int;

use alloc::{
    string::{String, ToString},
    borrow::ToOwned,
    vec::Vec
};

use crate::{
    logo::LogoInfo, modules::{Colors, FormatValue, Locale, Module, Os, Processes, Version, Weather}, os::{allocator::Allocator, env}
};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

// --- THESE CONSTANTS ARE CONFIG PLACEHOLDERS ---
pub mod padding {
    pub const TOP   : usize = 1;
    pub const BOTTOM: usize = 2;
    pub const RIGHT : usize = 3;
    pub const LEFT  : usize = 4;
}
pub const MODULES: [&str; 7] = [
    "locale",
    "cpu",
    "os",
    "processes",
    "version",
    "weather",
    "color"
];
// -----------------------------------------------

const MIN_OFFSET: usize = 24;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    use crate::{
        os::windows::ExitProcess,
        eprintln
    };

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        if let Some(loc) = info.location() {
            eprintln!("{loc}");
        }
        eprintln!("{}", info.message());
        // SAFETY: The function is used in the binary, everything is safe
        unsafe { ExitProcess(101) }
    }
}

fn max_line_len(lines: &Vec<(String, usize)>) -> usize {
    let mut ret = 0;
    for (_, len) in lines {
        if *len > ret {
            ret = *len;
        }
    }
    ret
}

fn build_logo_buf(lines: &Vec<(String, usize)>, max_len: usize) -> Vec<String> {
    let max_len_padding = max_len + padding::LEFT + padding::RIGHT;
    let mut ret = Vec::new();

    let (w, h) = env::terminal_size();
    if w < max_len_padding {
        return Vec::new();
    }
    
    for _ in 0..padding::TOP {
        ret.push(" ".repeat(max_len_padding));
    }
    for (line, len) in lines {
        let string = format!(
            "{}{}{}",
            " ".repeat(padding::LEFT),
            line,
            " ".repeat(padding::RIGHT + max_len - len)
        );
        ret.push(string);
    }
    for _ in 0..padding::BOTTOM {
        ret.push(" ".repeat(max_len_padding));
    }

    ret
}

fn split_by_len(string: &str, len: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut chars = string.chars().peekable();
    let mut start = 0;
    let mut byte_pos = 0;

    while chars.peek().is_some() {
        let mut count = 0;
        let mut end_byte = byte_pos;
        
        while count < len && chars.peek().is_some() {
            let ch = chars.next().unwrap();
            count += 1;
            end_byte += ch.len_utf8();
        }
        
        let slice = &string[byte_pos..end_byte];
        if let Some(newline_pos) = slice.rfind('\n') {
            let split_at = byte_pos + newline_pos + 1;
            result.push(&string[byte_pos..split_at - 1]);
            byte_pos = split_at;
            chars = string[byte_pos..].chars().peekable();
        } else {
            result.push(slice);
            byte_pos = end_byte;
        }
    }

    result
}

fn get_module_lines(name: &str, max_len_line: usize) -> Option<Vec<String>> {
    modules::from_str(name).map(|module| {
        let string = module.format(FormatValue::default(), FormatValue::default());
        let splitted = split_by_len(&string, max_len_line);
        splitted.iter().map(ToString::to_string).collect()
    })
}

fn build_info_buf(max_len: usize) -> Vec<String> {
    let (w, _) = env::terminal_size();
    let max_len_line = w - max_len - padding::LEFT - padding::RIGHT; 
    let mut ret = Vec::new();

    for name in MODULES {
        if let Some(m) = get_module_lines(name, max_len_line) {
            ret.extend(m);
        }
    }

    ret
}

// #[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    // println!("{}",    Colors::get());
    // println!("{:#?}", Locale::get());
    // println!("{:#?}", Os::get());
    // println!("{:#?}", Processes::get());
    // println!("{:#?}", Version::get());
    // println!("{:#?}", Weather::get());
    let (w, _) = env::terminal_size();

    let logo_lines = LogoInfo::new(&Os::get().id).get_ansi_lines();
    let max_logo_len = max_line_len(&logo_lines);
    let max_logo_len_padding = max_logo_len + padding::LEFT + padding::RIGHT;

    let split_len = if max_logo_len_padding + MIN_OFFSET < w {
        max_logo_len
    } else {
        w
    };

    let logo_buf = build_logo_buf(&logo_lines, max_logo_len);
    let info_buf = build_info_buf(split_len);
    let max_lines = logo_buf.len().max(info_buf.len());

    if max_logo_len_padding + MIN_OFFSET < w {
        let empty_logo_line = " ".repeat(max_logo_len_padding);
        for i in 0..max_lines {
            let logo_line = logo_buf.get(i).map_or(empty_logo_line.as_str(), String::as_str);
            let info_line = info_buf.get(i).map_or("", String::as_str);
            println!("{logo_line}{info_line}\x1b[0m");
        }
    } else {
        for line in logo_buf {
            println!("{line}\x1b[0m");
        }
        for line in info_buf {
            println!("{line}\x1b[0m");
        }
    }
    

    0
}