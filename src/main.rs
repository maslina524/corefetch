#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::all)]
#![deny(
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unsafe_removed_from_name, 
    clippy::unsafe_derive_deserialize,
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

use core::ffi::c_int;

use crate::{
    logo::LogoInfo,
    modules::{Colors, Locale, Os, Processes, Version, Weather},
    os::allocator::Allocator
};

extern crate alloc;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

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

#[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    println!("{}",    Colors::get());
    println!("{:#?}", Locale::get());
    println!("{:#?}", Os::get());
    println!("{:#?}", Processes::get());
    println!("{:#?}", Version::get());
    println!("{:#?}", Weather::get());

    let lines = LogoInfo::new("Windows 7").get_ansi_lines();
    for line in lines {
        let (string, len) = line;
        print!("{string}");
        print!("{}", " ".repeat(40 - len));
        println!("If `len` is calculated correctly, everything should be in one column");
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