use alloc::string::String;
use doc::Docs;

use crate::{
    detect::datetime::{AmPm, DatetimeInfo}, 
    format, 
    format_for_module, 
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static DATETIME: OnceLock<Datetime> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Datetime {
    #[doc = "Year"]
    pub year: u16,
    #[doc = "Last two digits of year"]
    pub year_short: u16,
    #[doc = "Month"]
    pub month: u16,
    #[doc = "Month with leading zero"]
    pub month_pretty: String,
    #[doc = "Month name"]
    pub month_name: String,
    #[doc = "Month name short"]
    pub month_name_short: String,
    #[doc = "Week number on year"]
    pub week: u16,
    #[doc = "Weekday"]
    pub weekday: String,
    #[doc = "Weekday short"]
    pub weekday_short: String,
    #[doc = "Day in year"]
    pub day_in_year: u16,
    #[doc = "Day in month"]
    pub day_in_month: u16,
    #[doc = "Day in week"]
    pub day_in_week: u16,
    #[doc = "Hour"]
    pub hour: u16,
    #[doc = "Hour with leading zero"]
    pub hour_pretty: String,
    #[doc = "Hour 12h format"]
    pub hour_12: u16,
    #[doc = "Hour 12h format with leading zero"]
    pub hour_12_pretty: String,
    #[doc = "Minute"]
    pub minute: u16,
    #[doc = "Minute with leading zero"]
    pub minute_pretty: String,
    #[doc = "Second"]
    pub second: u16,
    #[doc = "Second with leading zero"]
    pub second_pretty: String,
    #[doc = "Offset from UTC in the ISO 8601 format"]
    pub offset_from_utc: String,
    #[doc = "Locale-dependent timezone name or abbreviation"]
    pub timezone_name: String,
    #[doc = "Day in month with leading zero"]
    pub day_pretty: String,
    #[doc = "AM or PM"]
    pub am_pm: AmPm
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
            am_pm: time.am_pm
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

