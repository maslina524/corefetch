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
    detect::cpu::CpuInfo,
    imp::error::ErrorCode,
    imp::link::{
        GetActiveProcessorCount, GetLogicalProcessorInformation, GetNumaHighestNodeNumber,
        SYSTEM_LOGICAL_PROCESSOR_INFORMATION
    },
    imp::regedit::{Regedit, Access, Hkey},
    todo_or_default,
    format
};

type LogicalInfo = SYSTEM_LOGICAL_PROCESSOR_INFORMATION;

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