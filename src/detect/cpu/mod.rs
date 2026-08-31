use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec
};

use crate::{
    cfg_if,
    todo_or_default,
    format
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[derive(Default)]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub numa_nodes: usize,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub online_cores: usize,
    pub packages: usize,
    pub code_name: String,
    pub technology: String,
    pub base_freq: f64,
    pub max_freq: f64,
    pub temperature: f64,
    pub logical_grouped: String,
    pub micro_arch: String
}

impl CpuInfo {
    fn vendor() -> String {
        let ret = __cpuid(0);
        let (_, ebx, ecx, edx) = (ret.eax, ret.ebx, ret.ecx, ret.edx);

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

    const fn code_name() -> String {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            todo_or_default!("fn code_name is not implemented", String::new())
        } else {
            String::new()
        }
    }

    fn technology() -> String {
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            let eax = __cpuid(1).eax;
            let mut model = (eax >> 4) & 0x0F;
            let mut family = (eax >> 8) & 0xF;

            if family == 0xF {
                family += (eax >> 20) & 0xFF;
            }

            if family == 0x6 {
                model += ((eax >> 16) & 0xF) << 4;
            }

            if family != 0x6 {
                return String::new();
            }

            match model {
                0x97 | 0x9A | 0xB7 | 0xBA | 0xBE => "Intel 7",
                0xAA => "Intel 4",
                0xC7 | 0xD7 => "Intel 20A / 18A",
                0x7D | 0x7E => "Intel 10nm (Ice Lake)",
                0x8C | 0x8D => "Intel 10nm SuperFin (Tiger Lake)",
                0x8E | 0x9E | 0xA7 => "Intel 14nm (Rocket Lake / Coffee Lake)",
                0x4E | 0x5E => "Intel 14nm (Skylake)",
                0x3C | 0x3D => "Intel 22nm (Haswell)",
                _ => ""
            }.to_owned()
        } else {
            String::new()
        }
    }

    fn level_x86_64() -> u8 {
        if !cpuid_has_feature(1, 0, 2, 0) { return 1 }
        if !cpuid_has_feature(1, 0, 2, 19) { return 1 }
        if !cpuid_has_feature(1, 0, 2, 20) { return 1 }
        if !cpuid_has_feature(1, 0, 2, 23) { return 1 }

        if !cpuid_has_feature(0x8000_0001, 0, 2, 0) { return 1 }
        if !cpuid_has_feature(1, 0, 2, 13) { return 1 }

        if !os_supports_ymm() { return 2 }

        if !cpuid_has_feature(1, 0, 2, 28) { return 2 }
        if !cpuid_has_feature(1, 0, 2, 27) { return 2 }
        
        let ret = __cpuid(7);
        let ebx = ret.ebx;
        if ebx & (1 << 5) == 0 { return 2 }
        if ebx & (1 << 3) == 0 { return 2 }
        if ebx & (1 << 8) == 0 { return 2 }
        if !cpuid_has_feature(1, 0, 2, 29) { return 2 }
        if !cpuid_has_feature(1, 0, 2, 12) { return 2 }
        
        if !cpuid_has_feature(1, 0, 2, 22) { return 2 }
        if !cpuid_has_feature(0x8000_0001, 0, 2, 5) { return 2 }

        if !os_supports_zmm() { return 3 }

        if ebx & (1 << 16) == 0 { return 3 }
        if ebx & (1 << 30) == 0 { return 3 }
        if ebx & (1 << 28) == 0 { return 3 }
        if ebx & (1 << 17) == 0 { return 3 }
        if ebx & (1 << 31) == 0 { return 3 }

        4
    }

    fn micro_arch() -> String {
        let arch = env!("TARGET_ARCH");
        if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
            let level = Self::level_x86_64();
            format!("{arch}-v{level}")
        } else {
            arch.to_owned()
        }
    }

    // NOT IMPLEMENTED
    const fn logical_grouped() -> String {
        todo_or_default!("Will be implemented in the future", String::new())
    }

    const fn max_freq_formatted() -> f64 {
        0.0
    }

    const fn temperature() -> f64 {
        28.0
    }
}

fn cpuid_has_feature(leaf: u32, subleaf: u32, reg: usize, bit: usize) -> bool {
    let ret = __cpuid_count(leaf, subleaf);
    let regs = [ret.eax, ret.ebx, ret.ecx, ret.edx];
    ((regs[reg] >> bit) & 1) == 1
}

fn xgetbv_test(bit: u8) -> bool {
    // SAFETY: The index is always zero, safe
    let xcr0 = unsafe { _xgetbv(0) };
    (xcr0 >> bit as u64) == 1
}

fn os_supports_ymm() -> bool {
    xgetbv_test(2)
}

fn os_supports_zmm() -> bool {
    xgetbv_test(7)
}