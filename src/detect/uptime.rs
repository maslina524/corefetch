use alloc::string::String;

use crate::{
    windows::link::{
        GetTickCount64, FILETIME, SYSTEMTIME, FileTimeToSystemTime,
        GetSystemTimeAsFileTime, FileTimeToLocalFileTime
    },
    format
};

const DAY_MS: u64 = 1000 * 60 * 60 * 24;
const HOUR_MS: u64 = 1000 * 60 * 60;
const MIN_MS: u64 = 1000 * 60;
const SEC_MS: u64 = 1000;

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
    pub fn new() -> Self {
        // SAFETY: Completely safe
        let ms = unsafe { GetTickCount64() };


        let days = ms / DAY_MS;
        let rem = ms % DAY_MS;
        let hours = (rem / HOUR_MS) as u8;
        let rem = rem % HOUR_MS;
        let mins = (rem / MIN_MS) as u8;
        let rem = rem % MIN_MS;
        let secs = (rem / SEC_MS) as u8;
        let ms_remainder = (rem % SEC_MS) as u16;

        let years = (days / 365) as u16;

        #[allow(clippy::cast_precision_loss)]
        // ^^ 16_777_216 years is more than enough
        let years_fraction = (days as f32) / 365.0;

        let boot_time = Self::boot_time(ms);
        let formatted = Self::formatted(days as u32, hours, mins, secs);

        Self {
            years,
            days: days as u32,
            hours,
            mins,
            secs,
            ms: ms_remainder,
            boot_time,
            days_of_year: days as u32,
            years_fraction,
            formatted,
        }
    }

    fn boot_time(ms: u64) -> String {
        let mut now_ft = FILETIME::default();
        // SAFETY: Completely safe
        unsafe {
            GetSystemTimeAsFileTime(&raw mut now_ft);
        }
        let now_ns = ((now_ft.dwHighDateTime as u64) << 32) | (now_ft.dwLowDateTime as u64);
        let boot_ns = now_ns - ms * 10_000;

        let boot_ft_utc = FILETIME {
            dwLowDateTime: boot_ns as u32,
            dwHighDateTime: (boot_ns >> 32) as u32,
        };
        let mut boot_ft = FILETIME::default();
        unsafe {
            // SAFETY: Completely safe
            FileTimeToLocalFileTime(&raw const boot_ft_utc, &raw mut boot_ft);
        }

        let mut st = SYSTEMTIME::default();
        // SAFETY: Completely safe
        unsafe {
            FileTimeToSystemTime(&raw const boot_ft, &raw mut st);
        }

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            st.wYear, st.wMonth, st.wDay,
            st.wHour, st.wMinute, st.wSecond
        )
    }

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

#[cfg(test)]
mod tests {
    use crate::detect::uptime::UptimeInfo;

    #[test]
    fn uptime_test() {
        let uptime = UptimeInfo::new();
        println!("{uptime:#?}");
    }
}