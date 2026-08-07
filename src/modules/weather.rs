use alloc::string::String;

use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock,
    detect::weather
};



static WEATHER: OnceLock<Weather> = OnceLock::new();

#[derive(Debug)]
pub struct Weather {
    pub result: String
}

impl Module for Weather {
    fn new() -> Self {
        Self {
            result: weather::weather()
        }
    }

    fn get() -> &'static Self {
        WEATHER.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Weather"
    }

    fn title(&self) -> &'static str {
        "{result}"
    }

    fn string_name(&self) -> &'static str {
        "weather"
    }

    format_for_module!(Weather, result);
}

impl_display_for_module!(Weather);