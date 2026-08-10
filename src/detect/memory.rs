use core::mem;

use crate::os::windows::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[derive(Default)]
pub struct MemoryInfo {
    pub total_gb: f64,
    pub in_use_gb: f64
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

        #[allow(clippy::cast_precision_loss)]
        let total_gb = total_raw as f64 / (1024.0 * 1024.0 * 1024.0);
        #[allow(clippy::cast_precision_loss)]
        let in_use_gb = in_use_raw as f64 / (1024.0 * 1024.0 * 1024.0);

        Self { total_gb, in_use_gb }
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::memory::MemoryInfo;

    #[test]
    fn ram_test() {
        let mem = MemoryInfo::new();
        let total = mem.total_gb;
        let in_use = mem.in_use_gb;

        println!("Total: {total:.02}, In use: {in_use:.02}");

        assert!(total != 0.0);
        assert!(in_use != 0.0);
    }
}