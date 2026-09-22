use alloc::string::String;

use crate::{
    format, 
    imp::http::Request, 
    json::Json, 
    modules::publicip::PublicIP, 
    warning
};

const PUBLICIP_URL: &str = "http://ip-api.com/json/";

pub fn get() -> PublicIP {
    let response = match Request::new(PUBLICIP_URL).unwrap().get() {
        Ok(r) => r,
        Err(e) => {
            warning!("Failed to connect to server (publicip): {}", e.code());
            return PublicIP::default();
        }
    };
    let text = if response.is_success() {
        response.as_text().unwrap()
    } else {
        warning!("Failed to get response from `{PUBLICIP_URL}`: {}", response.code());
        return PublicIP::default();
    };

    let json = match Json::from_str(&text) {
        Ok(c) => c,
        Err(e) => {
            warning!("Failed to parse response (publicip): {e}");
            return PublicIP::default();
        }
    };

    let ip = json.get_string("query").cloned().unwrap_or_else(|| {
        warning!("Failed to get query (ip) from response");
        String::new()
    });
    let city = json.get_string("city").cloned().unwrap_or_else(|| {
        warning!("Failed to get city from response");
        String::new()
    });
    let country_code = json.get_string("countryCode").cloned().unwrap_or_else(|| {
        warning!("Failed to get countryCode from response");
        String::new()
    });

    PublicIP { ip, location: format!("{city}, {country_code}") }
}