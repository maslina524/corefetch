use alloc::string::String;

use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock,
    detect::uptime::UptimeInfo
};

static UPTIME: OnceLock<Uptime> = OnceLock::new();

#[derive(Debug)]
pub struct Uptime {
    pub days: u32,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
    pub boot_time: String,
    pub years: u16,
    pub days_of_year: u32,
    pub years_fraction: f32,
    pub formatted: String,
}

impl Module for Uptime {
    fn new() -> Self {
        let info = UptimeInfo::new();
        Self {
            days: info.days,
            hours: info.hours,
            minutes: info.mins,
            seconds: info.secs,
            milliseconds: info.ms,
            boot_time: info.boot_time,
            years: info.years,
            days_of_year: info.days_of_year,
            years_fraction: info.years_fraction,
            formatted: info.formatted,
        }
    }

    fn get() -> &'static Self {
        UPTIME.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Uptime"
    }

    fn title(&self) -> &'static str {
        "{formatted}"
    }

    fn string_name(&self) -> &'static str {
        "uptime"
    }

    format_for_module!(
        Uptime,
        days, hours, minutes, seconds, 
        milliseconds, boot_time, years, days_of_year, 
        years_fraction, formatted
    );
}

impl_display_for_module!(Uptime);