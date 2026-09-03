use alloc::string::String;

use crate::{
    cfg_if,
    format
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[derive(Debug)]
pub struct UptimeInfo {
    pub years: u16,
    pub days: u32,
    pub hours: u8,
    pub mins: u8,
    pub secs: u8,
    pub ms: u16,
    pub boot_time: String,
    pub days_of_year: u32,
    pub years_fraction: f32,
    pub formatted: String
}

impl UptimeInfo {
    fn formatted(days: u32, hours: u8, mins: u8, secs: u8) -> String {
        if days > 0 {
            format!("{days} days, {hours} hours, {mins} mins")
        } else if hours > 0 {
            format!("{hours} hours, {mins} mins")
        } else if mins > 0 {
            format!("{mins} mins")
        } else {
            format!("{secs} seconds")
        }
    }
}