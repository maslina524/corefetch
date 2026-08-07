use core::{
    arch::x86_64::__cpuid,
    mem,
    ptr
};

use alloc::{
    string::String,
    vec::Vec,
    vec
};

use crate::{
    os::{error::{self, ErrorCode}, windows::{GetActiveProcessorCount, GetLogicalProcessorInformation, GetNumaHighestNodeNumber, SYSTEM_LOGICAL_PROCESSOR_INFORMATION}}, sync::OnceLock, todo_or
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

pub const fn technology() -> String {
    if cfg!(any(target_arch = "x86_64", target_arch = "x86")) {
        todo_or!("fn technology is not implemented", String::new())
    } else {
        String::new()
    }
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
}