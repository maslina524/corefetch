use core::arch::x86_64::{__cpuid, __cpuid_count, _xgetbv};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec
};

use crate::{
    cfg_if,
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
    pub temperature: f32,
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

    fn get_family_and_model() -> (u32, u32) {
        let info = __cpuid(1);
        let eax = info.eax;

        let base_family = (eax >> 8) & 0xF;
        let extended_family = (eax >> 20) & 0xFF;
        let base_model  = (eax >> 4) & 0xF;
        let extended_model = (eax >> 16) & 0xF;

        let family = if base_family == 0xF {
            base_family + extended_family
        } else {
            base_family
        };

        let model = if base_family == 0x6 || base_family == 0xF {
            (extended_model << 4) | base_model
        } else {
            base_model
        };

        (family, model)
    }

    fn code_name(vendor: &str, family: u32, model: u32) -> String {
        if cfg!(not(any(target_arch = "x86_64", target_arch = "x86"))) {
            return String::new();
        }
        
        match vendor {
            "GenuineIntel" => Self::code_name_intel(family, model),
            "AuthenticAMD" => Self::code_name_amd(family, model),
            _ => String::new()
        }
    }

    fn code_name_intel(family: u32, model: u32) -> String {
        if family != 0x6 {
            return String::new();
        }
        match model {
            0x3A => "Ivy Bridge",
            0x3C | 0x45 | 0x46 => "Haswell",
            0x4E | 0x5E => "Skylake",
            0x8E | 0x9E => "Kaby Lake",
            0xA5 | 0xA7 => "Comet Lake",
            0x7D | 0x7E => "Ice Lake",
            0x8C | 0x8D => "Tiger Lake",
            0x9A => "Alder Lake",
            0xB7 | 0xBA | 0xBF => "Raptor Lake",
            0xCF | 0xAA => "Meteor Lake",
            0xBD | 0xAF => "Arrow Lake",
            _ => return String::new()
        }.to_owned()
    }

    fn code_name_amd(family: u32, model: u32) -> String {
        match (family, model) {
            (0x15, 0x00) => "K8",
            (0x16, 0x00) => "Jaguar",
            (0x17, 0x01) => "Zen",
            (0x17, 0x08) => "Zen+",
            (0x17, 0x11 | 0x31 | 0x60 | 0x90) => "Zen 2",
            (0x17, 0x98 | 0xA0) | (0x19, 0x01 | 0x21) => "Zen 3",
            (0x19, 0x40 | 0x60 | 0x70) => "Zen 4",
            (0x19, 0x90) => "Zen 5",
            _ => return String::new(),
        }.to_owned()
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
    #[todo::todo("Will be implemented in the future")]
    const fn logical_grouped() -> String {
        String::new()
    }

    const fn max_freq_formatted() -> f64 {
        0.0
    }

    const fn temperature() -> f32 {
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