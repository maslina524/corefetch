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
#![allow(
    dead_code,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

mod sync;
mod macros;
mod formats;
mod xxhash64;
mod preset;
mod color;

mod os;
mod modules;
mod logo;
mod detect;
mod json;
mod zlib;

extern crate alloc;

use core::{
    ffi::c_int,
    slice::Iter
};

use alloc::{
    string::{String, ToString},
    vec::Vec
};

use crate::{
    json::Json,
    logo::LogoInfo, 
    modules::{FormatValue, Module, Os}, 
    os::allocator::Allocator,
    os::env,
    os::fs::{self, ReadError},
    os::https::{Request, Url},
    os::windows::ExitProcess, 
    preset::{Preset, PresetModule}
};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

const MIN_OFFSET: usize = 24;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    use crate::{
        exit,
        eprintln
    };

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        if let Some(loc) = info.location() {
            eprintln!("{loc}");
        }
        eprintln!("{}", info.message());
        exit(101)
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
    let padding = Preset::get().get_logo_padding();
    let max_len_padding = max_len + padding.left + padding.right;
    let mut ret = Vec::new();

    let (w, _) = env::terminal_size();
    if w < max_len_padding {
        return Vec::new();
    }
    
    for _ in 0..padding.top {
        ret.push(" ".repeat(max_len_padding));
    }
    for (line, len) in lines {
        let string = format!(
            "{}{}{}",
            " ".repeat(padding.left),
            line,
            " ".repeat(padding.right + max_len - len)
        );
        ret.push(string);
    }
    for _ in 0..padding.bottom {
        ret.push(" ".repeat(max_len_padding));
    }

    ret
}

fn split_by_len(string: &str, len: usize) -> Vec<&str> {
    let mut result = Vec::new();
    let mut chars = string.chars().peekable();
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

fn get_module_lines(preset_module: &PresetModule, max_len_line: usize) -> Option<Vec<String>> {
    modules::from_preset_module(preset_module).map(|module| {
        let string = module.format(
            FormatValue {
                format: preset_module.key.as_deref(), 
                color: preset_module.key_color.as_deref()
            }, 
            FormatValue {
                format: preset_module.format.as_deref(), 
                color: None
            },
            &preset_module.map
        );
        let splitted = if preset_module.typ == "colors" {
            string.split('\n').collect()
        } else {
            split_by_len(&string, max_len_line)
        };

        splitted.iter().map(ToString::to_string).collect()
    })
}

fn build_info_buf(max_len: usize) -> Vec<String> {
    let padding = Preset::get().get_logo_padding();
    let (w, _) = env::terminal_size();
    let max_len_line = w - max_len - padding.left - padding.right; 
    let mut ret = Vec::new();
    let preset = Preset::get();

    for module in preset.modules() {
        if let Some(m) = get_module_lines(module, max_len_line) {
            ret.extend(m);
        }
    }

    ret
}

pub fn exit(code: u32) -> ! {
    // SAFETY: The function is used in the binary, everything is safe
    unsafe { ExitProcess(code) }
}

fn get_config(args: &mut Iter<'_, String>) -> Preset {
    args.position(|a| a == "--config" || a == "-c").map_or_else(Preset::default, |_| args.next().map_or_else(|| {
            help(None);
        }, |path| Url::new(path).map_or_else(|| { // FS Path
                match Json::from_file(path) {
                    Ok(c) => Preset::from_json(&c),
                    Err(e) => {
                        warning!("Failed to parse the json config: {e}");
                        Preset::default()
                    }
                }
            }, |url| { // Http Url
                let response = Request::from_url(url).get();
                if response.is_success() {
                    match response.as_text() {
                        Ok(t) => match Json::from_str(&t) {
                            Ok(c) => Preset::from_json(&c),
                            Err(e) => {
                                warning!("Failed to parse the json config: {e}");
                                Preset::default()
                            }
                        },
                        Err(e) => {
                            warning!("Failed to parse the response: {e}");
                            Preset::default()
                        }
                    }
                } else {
                    warning!("Failed to get preset from URL, Code: {}", response.code());
                    Preset::default()
                }
            })))
}

fn get_logo_name_and_custom(val: &str) -> (String, Option<String>) {
    let ready_val = val.to_lowercase().replace('_', " ");
    match fs::read_to_string(val) {
        Ok(s) => (Os::get().id.to_lowercase(), Some(s)),
        Err(e) => {
            match e {
                ReadError::Utf8(u) => {
                    warning!("Failed to use logo from fs: {u}");
                    (Os::get().id.to_lowercase(), None)
                }
                ReadError::Code(c) => {
                    if c.is_file_not_found() {
                        (ready_val, None)
                    } else {
                        warning!("Failed to use logo from fs: {c}");
                        (Os::get().id.to_lowercase(), None)
                    }
                }
            }
        }
    }
}

fn help(_theme: Option<&str>) -> ! {
    let multi = multi_string!(
        "Nofetch is a neofetch-like tool for beautiful system information display with flexible output customization",
        "",
        "<underline><bold>Usage:<reset><bold> nofetch<reset><italic> <?options><reset>",
        "",
        "<underline><bold>Commands:<reset>",
        "  -h, --help <?options> \tPrint this message",
        // "  -v, --version         \tPrint nofetch version"
    );
    println!("{}", colored!(multi));
    exit(0)
}

// #[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    let args = env::args();

    // Commands
    if let Some(pos) = args.iter().position(|a| a == "--help" || a == "-h") {
        help(args.get(pos + 1).map(String::as_str))
    }

    // Config init
    let config = get_config(&mut args.iter());
    Preset::get_or_init(config);

    // Logo init
    #[allow(clippy::option_if_let_else)]
    // ^^ Clippy suggests a variant that would require an extra heap allocation
    let (logo_name, custom) = if let Some(pos) = args.iter().position(|a| a == "--logo" || a == "-l") {
        args.get(pos + 1).map_or_else(|| help(None), |val| {
            get_logo_name_and_custom(val)
        })
    } else {
        (Os::get().id.to_lowercase(), None)
    };

    // Build buffers
    let (w, _) = env::terminal_size();
    let logo_lines = LogoInfo::new(&logo_name).get_ready_logo_lines(custom);
    let max_logo_len = max_line_len(&logo_lines);
    let padding = Preset::get().get_logo_padding();
    let max_logo_len_padding = max_logo_len + padding.left + padding.right;

    let split_len = if max_logo_len_padding + MIN_OFFSET < w {
        max_logo_len
    } else {
        w
    };

    let logo_buf = build_logo_buf(&logo_lines, max_logo_len);
    let info_buf = build_info_buf(split_len);
    let max_lines = logo_buf.len().max(info_buf.len());

    // Print buffers
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

    if args.iter().any(|a| a == "--wait" || a == "-w")  {
        loop {
            // SAFETY: Just a nop
            unsafe { core::arch::asm!("nop") };
        }
    }

    // The handle is created not with GetStdHandle,
    // but with `CreateFile`, which requires manual freeing
    let _ = env::close_terminal_handle();

    0
}