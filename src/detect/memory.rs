use core::mem;

use crate::{
    os::windows::{GlobalMemoryStatusEx, MEMORYSTATUSEX},
    formats::FileSize
};

#[derive(Default)]
pub struct MemoryInfo {
    pub total: FileSize,
    pub in_use: FileSize
}

impl MemoryInfo {
    pub fn new() -> Self {
        let mut mem_status = MEMORYSTATUSEX {
            dwLength: mem::size_of::<MEMORYSTATUSEX>() as u32,
            .. MEMORYSTATUSEX::default()
        };

        // SAFETY: Completely safe
        let ret = unsafe {
            GlobalMemoryStatusEx(&raw mut mem_status)
        };
        if ret == 0 {
            return Self::default();
        }

        let total_raw = mem_status.ullTotalPhys;
        let in_use_raw = mem_status.ullTotalPhys - mem_status.ullAvailPhys;

        let total = FileSize::from_bytes(total_raw);
        let in_use = FileSize::from_bytes(in_use_raw);

        Self { total, in_use }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        detect::memory::MemoryInfo,
        formats::FileSize
    };

    #[test]
    fn ram_test() {
        let mem = MemoryInfo::new();
        let total = mem.total;
        let in_use = mem.in_use;

        println!("Total: {total}, In use: {in_use}");

        assert!(total != FileSize::default());
        assert!(in_use != FileSize::default());
    }
}