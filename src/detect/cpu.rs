use core::arch::x86_64::__cpuid;

use alloc::{
    string::String,
    vec::Vec,
    vec
};

fn vendor() -> String {
    let ret = __cpuid(0);
    let (eax, ebx, ecx, edx) = (ret.eax, ret.ebx, ret.ecx, ret.edx);

    let vendor = vec![
        (ebx & 0xFF) as u8,
        ((ebx >> 8) & 0xFF) as u8,
        ((ebx >> 16) & 0xFF) as u8,
        ((ebx >> 24) & 0xFF) as u8,
        (edx & 0xFF) as u8,
        ((edx >> 8) & 0xFF) as u8,
        ((edx >> 16) & 0xFF) as u8,
        ((edx >> 24) & 0xFF) as u8,
        (ecx & 0xFF) as u8,
        ((ecx >> 8) & 0xFF) as u8,
        ((ecx >> 16) & 0xFF) as u8,
        ((ecx >> 24) & 0xFF) as u8,
    ];
    
    String::from_utf8(vendor).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::detect::cpu;

    extern crate std;

    #[test]
    fn vendor_test() {
        std::println!("Vendor: {}", cpu::vendor());
    }
}