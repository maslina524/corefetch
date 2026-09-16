use alloc::{
    string::String,
    collections::BTreeMap,
    vec::Vec
};

use doc::Docs;

use crate::{
    detect::disk::get_disks, 
    format_for_module, 
    formats::Percent, 
    impl_display_for_module, 
    json::Value, 
    modules::Module, 
    sync::OnceLock
};

static DISK_LIST: OnceLock<DiskList> = OnceLock::new();

pub struct DiskList {
    pub list: Vec<Disk>
}

impl Module for DiskList {
    fn new() -> Self {
        Self {
            list: get_disks()
        }
    }

    fn get() -> &'static Self {
        DISK_LIST.get_or_init(|| {
            Self::new()
        })
    }

    fn title(&self) -> &'static str {
        ""
    }

    fn key(&self) -> &'static str {
        "disk"
    }

    fn string_name(&self) -> &'static str {
        "disk"
    }

    fn format(&self, key: super::FormatValue, format: super::FormatValue, map: Option<&BTreeMap<String, Value>>) -> Option<String> {
        let s = self.list.iter()
            .filter_map(|d| d.format(key, format, map))
            .collect::<Vec<_>>()
            .join("\n");

        Some(s)
    }
}

#[derive(Debug, Clone, Default, Docs)]
pub struct Disk {
    pub size_used: String,
    pub size_total: String,
    pub size_percentage: Percent,
    pub files_used: String,
    pub files_total: String,
    pub files_percentage: Percent,
    pub is_external: bool,
    pub is_hidden: bool,
    pub filesystem: String,
    pub name: String,
    pub is_readonly: bool,
    pub create_time: String,
    pub size_percentage_bar: String,
    pub files_percentage_bar: String,
    pub days: u32,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
    pub mountpoint: String,
    pub mount_from: String
}

impl Disk {
    fn title(&self) -> &'static str {
        "{size-used} / {size-total} ({size-percentage}) - {filesystem}"
    }

    fn key(&self) -> &'static str {
        "Disk ({mountpoint})"
    }

    format_for_module!(
        Disk,
        size_used, size_total, size_percentage, files_used, 
        files_total, files_percentage, is_external, is_hidden, 
        filesystem, name, is_readonly, create_time, 
        size_percentage_bar, files_percentage_bar, days, hours, 
        minutes, seconds, milliseconds, mountpoint, 
        mount_from
    );
}

impl_display_for_module!(DiskList);