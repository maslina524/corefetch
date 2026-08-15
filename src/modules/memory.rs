use alloc::string::String;

use crate::{
    format,
    impl_display_for_module,
    format_for_module,
    detect::memory::MemoryInfo,
    modules::Module, 
    sync::OnceLock
};

static MEMORY: OnceLock<Memory> = OnceLock::new();

#[derive(Debug)]
pub struct Memory {
    used: String,
    total: String,
    percentage: String,
    percentage_bar: String,
}

impl Module for Memory {
    fn new() -> Self {
        let mem = MemoryInfo::new();
        let total = mem.total;
        let used = mem.in_use;
        let percentage = (used.as_bytes() / total.as_bytes() * 100) as u8;

        Self {
            used: format!("{used:.02}"),
            total: format!("{total:.02}"),
            percentage: format!("{percentage}%"),
            percentage_bar: String::new(),
        }
    }

    fn get() -> &'static Self {
        MEMORY.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Memory"
    }

    fn title(&self) -> &'static str {
        "{used} GiB / {total} GiB ({percentage})"
    }

    fn string_name(&self) -> &'static str {
        "cpu"
    }

    format_for_module!(
        Memory,
        used, total, percentage, percentage_bar
    );
}

impl_display_for_module!(Memory);