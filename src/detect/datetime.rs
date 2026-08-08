use alloc::{
    string::{String, ToString},
    borrow::ToOwned
};

pub enum AmPm {
    Am, Pm
}

impl ToString for AmPm {
    fn to_string(&self) -> String {
        match self {
            Self::Am => "am",
            Self::Pm => "pm"
        }.to_owned()
    }
}

pub struct Date {
    year: u32,
    month: u32,
    day: u32,

    hour: u32,
    minute: u32,
    second: u32,

    week: u32,
    weekday: u32,
    day_in_year: u32,
    day_in_month: u32,
    day_in_week: u32,

    offset_utc: u8,
    timezone_name: String,
    am_pm: AmPm
}