use core::{
    arch::x86_64::__cpuid,
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
    os::error::{self, ErrorCode},
    os::windows::{
        GetActiveProcessorCount, GetLogicalProcessorInformation, GetNumaHighestNodeNumber,
        SYSTEM_LOGICAL_PROCESSOR_INFORMATION
    },
    os::regedit::{Regedit, RegValue, Access, Hkey},
    sync::OnceLock,
    todo_or,
    format
};

type LogicalInfo = SYSTEM_LOGICAL_PROCESSOR_INFORMATION;

static VEC_LOGICAL_INFO: OnceLock<Vec<LogicalInfo>> = OnceLock::new();

fn logical_info() -> &'static Vec<LogicalInfo> {
    VEC_LOGICAL_INFO.get_or_init(|| {
        let mut size = 0;

        // SAFETY: Completely safe
        let ret = unsafe {
            GetLogicalProcessorInformation(
                ptr::null_mut(), 
                &raw mut size
            )
        };
        let err = ErrorCode::last();
        assert!(err.code() == 122 || err.code() == 0, "`GetLogicalProcessorInformation` (size) failed");

        let struct_size = mem::size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>();
        let buf_size = size as usize / struct_size;
        let mut buf = Vec::with_capacity(buf_size);

        // SAFETY: Completely safe
        let ret = unsafe {
            GetLogicalProcessorInformation(
                buf.as_mut_ptr(), 
                &raw mut size
            )
        };
        assert!(ErrorCode::last().code() != 0, "`GetLogicalProcessorInformation` (info) failed");

        // SAFETY: WinAPI modifies data in `Vec<_>`, you must update the len
        unsafe { buf.set_len(buf_size) };
        buf
    })
}

pub fn vendor() -> String {
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

pub fn numa_nodes_count() -> usize {
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

pub fn physical_cores_count() -> usize {
    let buf = logical_info();

    let mut physical = 0;
    for info in buf {
        if info.Relationship == 0 {
            physical += 1;
        }
    }

    physical
}

pub fn logical_cores_count() -> usize {
    let buf = logical_info();

    let mut logical = 0;
    for info in buf {
        if info.Relationship == 0 {
            let mut mask = info.ProcessorMask;
            while mask != 0 {
                mask &= (mask - 1);
                logical += 1;
            }
        }
    }

    logical
}

pub fn online_cores_count() -> usize {
    // SAFETY: Completely safe
    (unsafe { GetActiveProcessorCount(0) }) as usize
}

pub const fn code_name() -> String {
    if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
        todo_or!("fn code_name is not implemented", String::new())
    } else {
        String::new()
    }
}

pub fn technology() -> String {
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

pub fn base_freq_formatted() -> String {
    let handle = Regedit::open(
        Hkey::LocalMachine, 
        "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", 
        Access::Read
    ).unwrap();
    handle.read("~MHz").map_or_else(|_| String::new(), |key| {
        let mhz = key.as_u32().unwrap_or(0);
        let ghz = mhz as f64 / 1000.0;
        format!("{ghz:.2} GHz")
    })
}

#[cfg(test)]
mod tests {
    use crate::detect::cpu;

    extern crate std;

    #[test]
    fn vendor_test() {
        std::println!("Vendor: {}", cpu::vendor());
    }

    #[test]
    fn cores_test() {
        let physical = cpu::physical_cores_count();
        assert!(physical != 0);

        let logical = cpu::logical_cores_count();
        assert!(logical != 0);

        let online = cpu::online_cores_count();
        
        assert!(online != 0);

        println!("Physical: {physical}, Logical: {logical}, Online: {online}");
    }

    #[test]
    fn numa_nodes_test() {
        let numa = cpu::numa_nodes_count();
        assert!(numa != 0);
        println!("{numa}");
    }

    #[test]
    fn technology_test() {
        let tech = cpu::technology();
        assert!(!tech.is_empty());
        println!("{tech}");
    }

    #[test]
    fn base_freq_test() {
        let bf = cpu::base_freq_formatted();
        assert!(bf.contains("GHz"));
        println!("{bf}");
    }
}