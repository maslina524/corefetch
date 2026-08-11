use alloc::string::String;

use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock,
    detect::weather
};

static WEATHER: OnceLock<Weather> = OnceLock::new();

// Weather condition              : [0]  : %c
// Weather condition textual name : [1]  : %C
// Weather condition, symbol      : [2]  : %x
// Humidity                       : [3]  : %h
// Temperature (Actual)           : [4]  : %t
// Temperature (Feels Like)       : [5]  : %f
// Wind                           : [6]  : %w
// Location                       : [7]  : %l
// Moon phase                     : [8]  : %m
// Moon day                       : [9]  : %M
// Precipitation (mm/3 hours)     : [10] : %p
// Pressure (hPa)                 : [11] : %P
// Dew point                      : [12] : %e
// UV index (1-12)                : [13] : %u
// Dawn*                          : [14] : %D
// Sunrise*                       : [15] : %S
// Zenith*                        : [16] : %z
// Sunset*                        : [17] : %s
// Dusk*                          : [18] : %d
// Current time*                  : [19] : %T
// Local timezone                 : [20] : %Z

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