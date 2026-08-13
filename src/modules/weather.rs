use alloc::string::String;

use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock,
    detect::weather::WeatherInfo
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

#[derive(Debug)]
pub struct Weather {
    pub result: String,
    pub condition_emoji: String,
    pub condition: String,
    pub condition_symbol: char,
    pub humidity: String,
    pub temperature_actual: String,
    pub temperature_feels: String,
    pub wind: String,
    pub location: String,
    pub moon_emoji: String,
    pub moon_day: u8,
    pub precipitation: String,
    pub pressure: String,
    pub uv_index: u8,
    pub dawn: String,
    pub sunrise: String,
    pub zenith: String,
    pub sunset: String,
    pub dusk: String,
    pub time: String,
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

    format_for_module!(Weather, result);
}

impl_display_for_module!(Weather);