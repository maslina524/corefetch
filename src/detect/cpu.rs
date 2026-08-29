use core::{
    arch::x86_64::{__cpuid, __cpuid_count, _xgetbv},
    mem,
    ptr
};

use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec,
    vec
};

use crate::{
    windows::error::ErrorCode,
    windows::link::{
        GetActiveProcessorCount, GetLogicalProcessorInformation, GetNumaHighestNodeNumber,
        SYSTEM_LOGICAL_PROCESSOR_INFORMATION
    },
    windows::regedit::{Regedit, Access, Hkey},
    todo_or_default,
    format
};

type LogicalInfo = SYSTEM_LOGICAL_PROCESSOR_INFORMATION;

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
    pub fn new() -> Self {
        let cpu_regedit_handle = Regedit::open(
            Hkey::LocalMachine, 
            "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", 
            Access::Read
        ).unwrap();

        let logical_info = Self::logical_info();

        Self {
            name: Self::name(&cpu_regedit_handle),
            vendor: Self::vendor(),
            numa_nodes: Self::numa_nodes_count(),
            physical_cores: Self::physical_cores_count(&logical_info),
            logical_cores: Self::logical_cores_count(&logical_info),
            online_cores: Self::online_cores_count(),
            packages: Self::package_count(&logical_info),
            code_name: Self::code_name(),
            technology: Self::technology(),
            base_freq: Self::base_freq_formatted(&cpu_regedit_handle),
            temperature: Self::temperature(),
            max_freq: Self::max_freq_formatted(),
            logical_grouped: Self::logical_grouped(),
            micro_arch: Self::micro_arch()
        }
    }

    fn logical_info() -> Vec<LogicalInfo> {
        let mut size = 0;

        // SAFETY: Completely safe
        unsafe {
            GetLogicalProcessorInformation(
                ptr::null_mut(), 
                &raw mut size
            )
        };
        let err = ErrorCode::last();
        assert!(err.code() == 122 || err.code() == 0, "`GetLogicalProcessorInformation` (size) failed");

        let struct_size = mem::size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>();
        let buf_size = size as usize / struct_size;
        let mut buf = vec![SYSTEM_LOGICAL_PROCESSOR_INFORMATION::default(); buf_size];

        // SAFETY: Completely safe
        unsafe {
            GetLogicalProcessorInformation(
                buf.as_mut_ptr(), 
                &raw mut size
            )
        };
        assert!(ErrorCode::last().code() != 0, "`GetLogicalProcessorInformation` (info) failed");

        buf
    }

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

    fn numa_nodes_count() -> usize {
        let mut highest = 0;
        // SAFETY: Completely safe
        let ret = unsafe {
            GetNumaHighestNodeNumber(&raw mut highest)
        };
        if ret == 0 {
            return 0
        }
        highest as usize + 1
    }

    fn physical_cores_count(buf: &Vec<LogicalInfo>) -> usize {
        let mut physical = 0;
        for info in buf {
            if info.Relationship == 0 {
                physical += 1;
            }
        }

        physical
    }

    fn logical_cores_count(buf: &Vec<LogicalInfo>) -> usize {
        let mut logical = 0;
        for info in buf {
            if info.Relationship == 0 {
                let mut mask = info.ProcessorMask;
                while mask != 0 {
                    mask &= mask - 1;
                    logical += 1;
                }
            }
        }

        logical
    }

    const fn package_count(buf: &[LogicalInfo]) -> usize {
        let mut package = 0;
        let mut idx = 0;
        while idx < buf.len() {
            let info = &buf[idx];
            if info.Relationship == 3 {
                package += 1;
            }
            idx += 1;
        }

        package
    }

    fn online_cores_count() -> usize {
        // SAFETY: Completely safe
        (unsafe { GetActiveProcessorCount(0) }) as usize
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

    fn base_freq_formatted(handle: &Regedit) -> f64 {
        handle.read("~MHz").map_or_else(|_| 0.0, |key| {
            let mhz = key.as_u32().unwrap_or(0);
            mhz as f64 / 1000.0
        })
    }

    fn name(handle: &Regedit) -> String {
        let reg = handle.read("ProcessorNameString").unwrap();
        let string = reg.as_string().unwrap();
        string.to_owned()
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

#[cfg(test)]
mod tests {
    use crate::detect::cpu::CpuInfo;

    extern crate std;

    #[test]
    fn vendor_test() {
        let info = CpuInfo::new();
        std::println!("Vendor: {}", info.vendor);
    }

    #[test]
    fn cores_test() {
        let info = CpuInfo::new();

        assert!(info.physical_cores != 0);
        assert!(info.logical_cores != 0);
        assert!(info.online_cores != 0);
    }

    #[test]
    fn numa_nodes_test() {
        let info = CpuInfo::new();
        assert!(info.numa_nodes != 0);
    }

    #[test]
    fn technology_test() {
        let info = CpuInfo::new();
        assert!(!info.technology.is_empty());
    }

    #[test]
    fn base_freq_test() {
        let info = CpuInfo::new();
        assert!(info.base_freq != 0.0);
    }

    #[test]
    fn micro_arch_test() {
        let info = CpuInfo::new();
        println!("{}", info.micro_arch);
    }

    #[test]
    fn package_test() {
        let info = CpuInfo::new();
        assert!(info.packages != 0);
    }
}