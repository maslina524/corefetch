use alloc::string::String;

use crate::{
    sync::OnceLock,
    os::https::Request,
    format
};

const WTTR_URL: &str = "https://wttr.in/?lang=en&format=%t %E2%80%94 %C (%l)";

static WEATHER: OnceLock<Weather> = OnceLock::new();

#[derive(Debug)]
pub struct Weather {
    pub result: String
}

impl Weather {
    pub fn new() -> Self {
        let response = Request::new(WTTR_URL).unwrap().get();
        let result = if response.is_success() {
            response.as_text().unwrap()
        } else {
            format!("Unknown: code {}", response.code())
        };

        Self { result }
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