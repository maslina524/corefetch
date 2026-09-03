use alloc::string::String;
use doc::Docs;

use crate::{
    detect::weather::WeatherInfo, 
    format_for_module, 
    formats::{Percent, Temperature}, 
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static WEATHER: OnceLock<Weather> = OnceLock::new();

// condition_emoji    : [0]  : c : 🌤️
// condition          : [1]  : C : Partly Cloudy
// condition_symbol   : [2]  : x : m
// humidity           : [3]  : h : 44%
// temperature_actual : [4]  : t : +17°C
// temperature_feels  : [5]  : f : +11°C
// wind               : [6]  : w : ↘22km/h
// location           : [7]  : l : Tver, Tver Oblast, RU
// moon_emoji         : [8]  : m : 🌑
// moon_day           : [9]  : M : 1
// precipitation      : [10] : p : 0.0mm
// pressure           : [11] : P : 1017hPa
// uv_index           : [12] : u : 3
// dawn               : [13] : D : 04:11:18
// sunrise            : [14] : S : 04:59:30
// zenith             : [15] : z : 12:41:21
// sunset             : [16] : s : 20:23:09
// dusk               : [17] : d : 21:11:16
// time               : [18] : T : 16:45:01+0300
// timezone           : [19] : Z : Europe/Moscow

// Format string:
// %c;%C;%x;%h;%t;%f;%w;%l;%m;%M;%p;%P;%u;%D;%S;%z;%s;%d;%T;%Z

#[derive(Debug, Docs)]
pub struct Weather {
    #[doc = "Pretty string, like `+14°C — Overcast (Tver, Tver Oblast, RU)`"]
    pub result: String,
    #[doc = "Condition emoji, like `🌤️`"]
    pub condition_emoji: String,
    #[doc = "Condition, like `Partly Cloudy`"]
    pub condition: String,
    #[doc = "Condition symbol, like `Partly m`"]
    pub condition_symbol: String,
    #[doc = "Humidity in percent"]
    pub humidity: Percent,
    #[doc = "Actual temperature"]
    pub temperature_actual: Temperature,
    #[doc = "Felt temperature."]
    pub temperature_feels: Temperature,
    #[doc = "Wind, like `↘22km/h`"]
    pub wind: String,
    #[doc = "Location, like `Tver, Tver Oblast, RU`"]
    pub location: String,
    #[doc = "Moon emoji"]
    pub moon_emoji: String,
    #[doc = "Moon day"]
    pub moon_day: u8,
    #[doc = "Precipitation"]
    pub precipitation: String,
    #[doc = "Pressure, like `1017hPa`"]
    pub pressure: String,
    #[doc = "UV index (0 - 11+)"]
    pub uv_index: u8,
    #[doc = "Dawn time"]
    pub dawn: String,
    #[doc = "Sunrise time"]
    pub sunrise: String,
    #[doc = "Zenith time"]
    pub zenith: String,
    #[doc = "Sunset time"]
    pub sunset: String,
    #[doc = "Dusk time"]
    pub dusk: String,
    #[doc = "Time, like `16:45:01+0300`"]
    pub time: String,
    #[doc = "Timezone, like `Europe/Moscow`"]
    pub timezone: String
}

impl Module for Weather {
    fn new() -> Self {
        let info = WeatherInfo::new();

        Self {
            result: info.result,
            condition_emoji: info.condition_emoji,
            condition: info.condition,
            condition_symbol: info.condition_symbol,
            humidity: info.humidity,
            temperature_actual: info.temperature_actual,
            temperature_feels: info.temperature_feels,
            wind: info.wind,
            location: info.location,
            moon_emoji: info.moon_emoji,
            moon_day: info.moon_day,
            precipitation: info.precipitation,
            pressure: info.pressure,
            uv_index: info.uv_index,
            dawn: info.dawn,
            sunrise: info.sunrise,
            zenith: info.zenith,
            sunset: info.sunset,
            dusk: info.dusk,
            time: info.time,
            timezone: info.timezone
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

    format_for_module!(
        Weather,
        result, condition_emoji, condition, condition_symbol, 
        humidity, temperature_actual, temperature_feels, wind, 
        location, moon_emoji, moon_day, precipitation, 
        pressure, uv_index, dawn, sunrise, 
        zenith, sunset, dusk, time, 
        timezone
    );
}

impl_display_for_module!(Weather);