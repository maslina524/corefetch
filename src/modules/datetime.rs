use alloc::string::{String, ToString};

use crate::{
    format_for_module,
    impl_display_for_module,
    format,
    modules::Module,
    detect::datetime::DatetimeInfo,
    sync::OnceLock
};

static DATETIME: OnceLock<Datetime> = OnceLock::new();

#[derive(Debug)]
pub struct Datetime {
    pub year: u16,
    pub year_short: u16,
    pub month: u16,
    pub month_pretty: String,
    pub month_name: String,
    pub month_name_short: String,
    pub week: u16,
    pub weekday: String,
    pub weekday_short: String,
    pub day_in_year: u16,
    pub day_in_month: u16,
    pub day_in_week: u16,
    pub hour: u16,
    pub hour_pretty: String,
    pub hour_12: u16,
    pub hour_12_pretty: String,
    pub minute: u16,
    pub minute_pretty: String,
    pub second: u16,
    pub second_pretty: String,
    pub offset_from_utc: String,
    pub timezone_name: String,
    pub day_pretty: String,
    pub am_pm: String
}

impl Module for Datetime {
    fn new() -> Self {
        let time = DatetimeInfo::new();
        Self {
            year: time.year,
            year_short: time.year - (time.year / 100 * 100),
            month: time.month,
            month_pretty: format!("{:02}", time.month),
            month_name: time.month_name,
            month_name_short: time.month_name_short,
            week: time.week,
            weekday: time.weekday,
            weekday_short: time.weekday_short,
            day_in_year: time.day_in_year,
            day_in_month: time.day_in_month,
            day_in_week: time.day_in_week,
            hour: time.hour,
            hour_pretty: format!("{:02}", time.hour),
            hour_12: time.hour % 12,
            hour_12_pretty: format!("{:02}", time.hour % 12),
            minute: time.minute,
            minute_pretty: format!("{:02}", time.minute),
            second: time.second,
            second_pretty: format!("{:02}", time.second),
            offset_from_utc: time.offset_utc,
            timezone_name: time.timezone_name,
            day_pretty: format!("{:02}", time.day_in_month),
            am_pm: time.am_pm.to_string()
        }
    }

    fn get() -> &'static Self {
        DATETIME.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Date & Time"
    }

    fn title(&self) -> &'static str {
        "{year}-{month-pretty}-{day-pretty} {hour-pretty}:{minute-pretty}:{second-pretty}"
    }

    fn string_name(&self) -> &'static str {
        "datetime"
    }

    format_for_module!(
        Processes,
        year, year_short, month, month_pretty,
        month_name, month_name_short, week, weekday,
        weekday_short, day_in_year, day_in_month, day_in_week,
        hour, hour_pretty, hour_12, hour_12_pretty,
        minute, minute_pretty, second, second_pretty,
        offset_from_utc, timezone_name, day_pretty, am_pm
    );
}

impl_display_for_module!(Datetime);
