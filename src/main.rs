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
mod modules;
mod color;
mod logo;

extern crate alloc;

use core::ffi::c_int;

use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec
};

use crate::{
    logo::LogoInfo,
    modules::{Colors, Locale, Os, Processes, Version, Weather},
    os::allocator::Allocator
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
// -----------------------------------------------

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

fn build_logo_buf() -> Vec<String> {
    let lines = LogoInfo::new(&Os::get().id).get_ansi_lines();
    let max_len = max_line_len(&lines);
    let mut ret = Vec::new();

    for _ in 0..padding::TOP {
        ret.push(String::new());
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
        ret.push(String::new());
    }

    ret
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    println!("{}",    Colors::get());
    println!("{:#?}", Locale::get());
    println!("{:#?}", Os::get());
    println!("{:#?}", Processes::get());
    println!("{:#?}", Version::get());
    println!("{:#?}", Weather::get());

    let lines = build_logo_buf();
    for line in lines {
        println!("{line}");
    }

    0
}

#[cfg(test)]
mod tests {  
    extern crate std;

    #[test]
    fn test_test() {
        std::println!("Hello World!");
    }
}