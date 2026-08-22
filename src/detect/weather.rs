use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned
};

use crate::{
    format,
    windows::env,
    windows::fs::{self, Access, File},
    windows::https::Request,
    windows::path::Path,
};

const WTTR_URL: &str = "https://wttr.in/?lang=en&format=%c;%C;%x;%h;%t;%f;%w;%l;%m;%M;%p;%P;%u;%D;%S;%z;%s;%d;%T;%Z";

pub struct WeatherInfo {
    pub result: String,
    pub condition_emoji: String,
    pub condition: String,
    pub condition_symbol: String,
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

impl WeatherInfo {
    pub fn new() -> Self {
        let cur_hour = env::timestamp_hours();
        let raw = if let Some((hours, data)) = Self::read_cache() {
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

        let parts: Vec<&str> = raw.split(';').collect();
        let condition_emoji = parts[0].trim();
        let condition = parts[1].trim();
        let condition_symbol = parts[2].trim();
        let humidity = parts[3].trim();
        let temperature_actual = parts[4].trim();
        let temperature_feels = parts[5].trim();
        let wind = parts[6].trim();
        let location = parts[7].trim();
        let moon_emoji = parts[8].trim();
        let moon_day = parts[9].trim();
        let precipitation = parts[10].trim();
        let pressure = parts[11].trim();
        let uv_index = parts[12].trim();
        let dawn = parts[13].trim();
        let sunrise = parts[14].trim();
        let zenith = parts[15].trim();
        let sunset = parts[16];
        let dusk = parts[17].trim();
        let time = parts[18].trim();
        let timezone = parts[19].trim();

        Self {
            result: format!("{temperature_actual} — {condition} ({location})"),
            condition_emoji: condition_emoji.to_owned(),
            condition: condition.to_owned(), 
            condition_symbol: condition_symbol.to_owned(), 
            humidity: humidity.to_owned(), 
            temperature_actual: temperature_actual.to_owned(), 
            temperature_feels: temperature_feels.to_owned(), 
            wind: wind.to_owned(), 
            location: location.to_owned(), 
            moon_emoji: moon_emoji.to_owned(), 
            moon_day: moon_day
                .parse::<u8>()
                .expect("Strange response from wttr.is"), 
            precipitation: precipitation.to_owned(), 
            pressure: pressure.to_owned(), 
            uv_index: uv_index
                .parse::<u8>()
                .expect("Strange response from wttr.is"), 
            dawn: dawn.to_owned(), 
            sunrise: sunrise.to_owned(), 
            zenith: zenith.to_owned(), 
            sunset: sunset.to_owned(), 
            dusk: dusk.to_owned(), 
            time: time.to_owned(), 
            timezone: timezone.to_owned() 
        }
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
}