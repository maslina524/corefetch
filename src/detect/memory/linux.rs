use crate::{
    detect::memory::MemoryInfo, 
    formats::Size, 
    linux::libc::get_sysinfo
};

impl MemoryInfo {
    pub fn new() -> Self {
        let info = get_sysinfo();

        let total_raw = info.totalram;
        let in_use_raw = info.totalram - info.freeram;

        let total = Size::from_bytes(total_raw);
        let in_use = Size::from_bytes(in_use_raw);

        Self { total, in_use }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        detect::memory::MemoryInfo,
        formats::Size
    };

    #[test]
    fn ram_test() {
        let mem = MemoryInfo::new();
        let total = mem.total;
        let in_use = mem.in_use;

        println!("Total: {total}, In use: {in_use}");

        assert!(total != Size::default());
        assert!(in_use != Size::default());
    }
}