#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::all)]

mod sync;
mod os;
mod modules;

use core::ffi::c_int;

use crate::{
    modules::*,
    os::allocator::Allocator
};

extern crate alloc;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

// #[cfg(not(test))]
#[unsafe(no_mangle)]
extern "C" fn main() -> c_int {
    println!("{}",    Colors::get());
    println!("{:#?}", Locale::get());
    println!("{:#?}", Processes::get());
    println!("{:#?}", Version::get());
    println!("{:#?}", Weather::get());
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

