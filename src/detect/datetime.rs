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
    pub year: u16,
    pub month: u16,
    pub month_name: String,

    pub hour: u16,
    pub minute: u16,
    pub second: u16,

    pub week: u16,
    pub weekday: String,
    pub weekday_short: String,
    pub day_in_year: u16,
    pub day_in_month: u16,
    pub day_in_week: u16,

    pub offset_utc: String,
    pub timezone_name: String, 
    pub am_pm: AmPm
}

impl Date {
    pub fn new() -> Self {
        let mut time = SYSTEMTIME::default();
        // SAFETY: Completely safe
        unsafe { GetLocalTime(&raw mut time) };

        let year = time.wYear;
        let month = time.wMonth;
        let month_name = Self::month_name(time.wMonth);

        let hour = time.wHour;
        let minute = time.wMinute;
        let second = time.wSecond;

        let day_in_year = Self::day_in_year(&time);
        let week = day_in_year / 7 + 1;
        let weekday = Self::day_of_week(time.wDayOfWeek);
        let weekday_short = Self::day_of_week_short(time.wDayOfWeek);

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
            year, month, month_name,
            hour, minute, second, week,
            weekday, weekday_short, day_in_year, day_in_month,
            day_in_week, offset_utc, timezone_name, am_pm
        }
    }

    fn month_name(num: u16) -> String {
        match num {
            1  => "January",
            2  => "February",
            3  => "March",
            4  => "April",
            5  => "May",
            6  => "June",
            7  => "July",
            8  => "August",
            9  => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!()
        }.to_owned()
    }

    fn day_of_week(num: u16) -> String {
        match num {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => unreachable!()
        }.to_owned()
    }

    fn day_of_week_short(num: u16) -> String {
        match num {
            0 => "Sun",
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            _ => unreachable!()
        }.to_owned()
    }
    
    fn day_in_year(time: &SYSTEMTIME) -> u16 {
        let mut days_in_month: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap = time.wYear.is_multiple_of(400) || 
                    (time.wYear.is_multiple_of(4) && time.wYear.is_multiple_of(100));
        
        if is_leap {
            days_in_month[1] = 29;
        }
        
        let mut  day_os_year = 0u16;
        for i in days_in_month.iter().take(time.wMonth as usize - 1) {
            day_os_year += *i as u16;
        }
        day_os_year += time.wDay;
        
        day_os_year
    }

    fn time_zone() -> i16 {
        let mut tzi = TIME_ZONE_INFORMATION::default();

        // SAFETY: Completely safe
        let ret = unsafe {
            GetTimeZoneInformation(&raw mut tzi)
        };
        if ret == 0xFFFF_FFFF {
            ErrorCode::last().panic()
        }

        let mut bias_mins = tzi.Bias;

        if ret == 2 {
            bias_mins += tzi.DaylightBias;
        }

        (-bias_mins) as i16
    }

    fn offset_utc_string() -> String {
        let mut tz = Self::time_zone();
        let symb = if tz < 0 { '-' } else { '+' };
        tz = tz.abs();
        let hours = tz / 60;
        let mins = tz % 60;

        format!("{symb}{hours:02}{mins:02}")
    }

    fn is_daylight() -> bool {
        let mut tzi = TIME_ZONE_INFORMATION::default();

        // SAFETY: Completely safe
        let ret = unsafe {
            GetTimeZoneInformation(&raw mut tzi)
        };
        if ret == 0xFFFF_FFFF {
            ErrorCode::last().panic()
        }

        let current_bias = tzi.Bias + if tzi.DaylightBias != 0 {
            tzi.DaylightBias
        } else {
            tzi.StandardBias
        };
        
        tzi.DaylightBias != 0 &&current_bias != (tzi.Bias + tzi.StandardBias)
    }

    fn time_zone_string() -> String {
        let mut tzi = DYNAMIC_TIME_ZONE_INFORMATION::default();
        // SAFETY: Completely safe
        unsafe {
            GetDynamicTimeZoneInformation(&raw mut tzi)
        };

        let is_daylight = Self::is_daylight();
        let buf = if is_daylight {
            tzi.DaylightName
        } else {
            tzi.StandardName
        };

        String::from_utf16_lossy(&buf).rsplit('\0').collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::datetime::Date;

    #[test]
    fn date_test() {
        let date = Date::new();
        println!("{date:#?}");
    }
}