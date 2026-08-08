use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    format,
    os::error::ErrorCode,
    os::windows::{
        DYNAMIC_TIME_ZONE_INFORMATION, GetDynamicTimeZoneInformation, GetLocalTime,
        GetTimeZoneInformation, SYSTEMTIME, TIME_ZONE_INFORMATION
    }
};

#[derive(Debug)]
pub enum AmPm {
    Am, Pm
}

impl core::fmt::Display for AmPm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = match self {
            Self::Am => "am",
            Self::Pm => "pm"
        }.to_owned();

        write!(f, "{string}")
    }
}

#[derive(Debug)]
pub struct Date {
    year: u16,
    month: u16,

    hour: u16,
    minute: u16,
    second: u16,

    week: u16,
    weekday: String,
    weekday_short: String,
    day_in_year: u16,
    day_in_month: u16,
    day_in_week: u16,

    offset_utc: String,
    timezone_name: String,
    am_pm: AmPm
}

impl Date {
    pub fn new() -> Self {
        let mut time = SYSTEMTIME::default();
        // SAFETY: Completely safe
        unsafe { GetLocalTime(&raw mut time) };

        let year = time.wYear;
        let month = time.wMonth;

        let hour = time.wHour;
        let minute = time.wMinute;
        let second = time.wSecond;

        let day_in_year = Self::day_in_year(&time);
        let week = day_in_year / 7 + 1;
        let weekday = Self::week_of_day(time.wDayOfWeek);
        let weekday_short = Self::week_of_day_short(time.wDayOfWeek);

        let day_in_month = time.wDay;
        let day_in_week = time.wDayOfWeek;

        let offset_utc = Self::offset_utc_string();
        let timezone_name = Self::time_zone_string();
        let am_pm = if hour < 12 {
            AmPm::Am
        } else {
            AmPm::Pm
        };

        Self {
            year, month, hour,
            minute, second, week, weekday,
            weekday_short,  day_in_year, day_in_month, day_in_week,
            offset_utc, timezone_name, am_pm
        }
    }
}