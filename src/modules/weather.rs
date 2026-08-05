use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned
};

use crate::{
    sync::OnceLock,
    os::https::Request,
    os::path::Path,
    os::fs::{self, File, Access},
    os::env,
    format
};

const WTTR_URL: &str = "https://wttr.in/?lang=en&format=%t+%E2%80%94+%C+(%l)";

static WEATHER: OnceLock<Weather> = OnceLock::new();

#[derive(Debug)]
pub struct Weather {
    pub result: String
}

impl Weather {
    pub fn new() -> Self {
        let cur_hour = env::timestamp_hours();
        let result = if let Some((hours, data)) = Self::read_cache() {
            if cur_hour == hours {
                data
            } else {
                let data = Self::request();
                Self::set_cache(cur_hour, &data);
                data
            }
        } else {
            let data = Self::request();
            Self::set_cache(cur_hour, &data);
            data
        };

        Self { result }
    }

    fn request() -> String {
        let response = Request::new(WTTR_URL).unwrap().get();
        if response.is_success() {
            response.as_text().unwrap()
        } else {
            format!("Response code: {}", response.code())
        }
    }

    fn read_cache() -> Option<(u64, String)> {
        let path = Path::cache().join("weather");
        let string = fs::read_to_string(path).ok()?;

        let parts: Vec<&str> = string.splitn(2, '\n').collect();
        if parts.len() != 2 {
            return None
        }

        let hours = parts[0].parse::<u64>().ok()?;
        let data = parts[1].to_owned();
        Some((hours, data))
    }

    fn set_cache(hour: u64, data: &str) -> Option<()> {
        let path_dir = Path::cache();
        fs::create_dirs(&path_dir).ok()?;

        let path = path_dir.join("weather");
        let file = File::create_always(path, Access::Write).ok()?;

        let formatted = format!("{hour}\n{data}");
        file.write(formatted).ok()?;

        Some(())
    }

    pub fn get() -> &'static Self {
        WEATHER.get_or_init(|| {
            Self::new()
        })
    }
}

impl core::fmt::Display for Weather {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.result)
    }
} 