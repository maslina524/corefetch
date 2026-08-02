use alloc::string::String;

use crate::{
    sync::OnceLock,
    os::https::Request,
    format
};

const WTTR_URL: &str = "https://wttr.in/?lang=en&format=%t %E2%80%94 %C (%l)";

static WEATHER: OnceLock<Weather> = OnceLock::new();

pub fn get() -> &'static Weather {
    WEATHER.get_or_init(|| {
        Weather::new()
    })
}

#[derive(Debug)]
pub struct Weather {
    result: String
}

impl Weather {
    pub fn new() -> Self {
        let response = Request::new(WTTR_URL).unwrap().get();
        let result = if !response.is_success() {
            format!("Unknown: code {}", response.code())
        } else {
            response.as_text().unwrap()
        };

        Self { result }
    }
}

impl core::fmt::Display for Weather {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.result)
    }
} 