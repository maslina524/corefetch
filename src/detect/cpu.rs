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
    os::windows::{GetLogicalProcessorInformation, SYSTEM_LOGICAL_PROCESSOR_INFORMATION, GetActiveProcessorCount, GetNumaHighestNodeNumber},
    os::error::{self, ErrorCode},
    sync::OnceLock
};

type LogicalInfo = SYSTEM_LOGICAL_PROCESSOR_INFORMATION;

static VEC_LOGICAL_INFO: OnceLock<Vec<LogicalInfo>> = OnceLock::new();

#[derive(Debug)]
pub struct Cores {
    pub physical: usize,
    pub logical: usize,
    pub online: usize,
}

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
        assert!(err.code() != 122 && err.code() != 0, "`GetLogicalProcessorInformation` (size) failed");

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
        assert!(ErrorCode::last().code() == 0, "`GetLogicalProcessorInformation` (info) failed");

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

pub fn cores_count() -> error::Result<Cores> {
    let mut size = 0;
    
    // SAFETY: Completely safe
    let ret = unsafe {
        GetLogicalProcessorInformation(
            ptr::null_mut(), 
            &raw mut size
        )
    };
    let err = ErrorCode::last();
    if err.code() != 122 && err.code() != 0 {
        return Err(ErrorCode::last());
    }

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
    if ret == 0 {
        return Err(ErrorCode::last());
    }
    // SAFETY: WinAPI modifies data in `Vec<_>`, you must update the len
    unsafe { buf.set_len(buf_size) };

    let mut logical = 0;
    let mut physical = 0;
    // SAFETY: Completely safe
    let online = unsafe {
        GetActiveProcessorCount(0)
    } as usize;
    for info in buf {
        if info.Relationship == 0 {
            physical += 1;
            let mut mask = info.ProcessorMask;
            while mask != 0 {
                mask &= (mask - 1);
                logical += 1;
            }
        }
    }

    Ok(Cores { physical, logical, online })
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
        let cores = cpu::cores_count().unwrap();
        println!("{cores:#?}");
    }

    #[test]
    fn numa_nodes_test() {
        let numa = cpu::numa_nodes_count();
        assert!(numa != 0);
        println!("{numa}");
    }
}