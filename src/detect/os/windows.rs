use alloc::{
    borrow::ToOwned,
    string::String
};

use crate::{
    detect::os::OsInfo,
    imp::env,
    imp::regedit::RegValue,
    format
};

impl OsInfo {
    pub fn new() -> Self {
        let sysname = "WIN32_NT";
        let name = "Windows".to_owned();

        let (_, _, build) = env::get_version();
        let version = Self::version(build).to_owned();
        let codename = Self::codename(build).to_owned();

        let value = env::current_version().read("ProductName").unwrap_or(RegValue::None);

        let mut variant = value
            .as_string()
            .unwrap_or("")
            .to_owned();

        let idx = variant
            .rfind(' ')
            .unwrap_or(0);
        
        variant = variant[idx + 1..].to_owned();
        let id = format!("{name} {version}");
        let nerd = Self::nerd(&version);

        Self { 
            sysname,
            name,
            id: id.clone(),
            id_like: id,
            version: version.clone(),
            version_id: version,
            codename,
            variant,
            variant_id: String::new(),
            nerd
        }
    }

    fn nerd() -> char {
        match (version) {
            "11" => '\u{e62a}',
            _ => '\u{e70f}',
        }
    }

    fn codename(build: i32) -> &'static str {
        match build {
            950 => "4.00",
            1381 => "NT 4.0",
            1998 => "4.10",
            2195 => "NT 5.0",
            3000 => "4.90",
            2600 | 2700 | 2710 => "NT 5.1",
            3790 => "NT 5.2",
            6002 => "NT 6.0",
            7601 => "NT 6.1",
            9200 => "NT 6.2",
            9600 => "NT 6.3",
            10240 => "NT 10.0",
            10586 => "1511",
            14393 => "1607",
            15063 => "1703",
            16299 => "1709",
            17134 => "1803",
            17763 => "1809",
            18362 => "1903",
            18363 => "1909",
            19041 => "2004",
            19042 => "20H2",
            19043 => "21H1",
            19044 | 22000 => "21H2",
            19045 | 22621 => "22H2",
            22631 => "23H2",
            26100 => "24H2",
            26200 => "25H2",
            28000 => "26H1",
            _ => ""
        }
    }

    fn version(build: i32) -> &'static str {
        match build {
            6002 => "Vista",
            7601 => "7",
            9200 => "8",
            9600 => "8.1",
            _ if (10240..22000).contains(&build) => "10",
            _ if (22000..=28000).contains(&build) => "11",
            _ => "Unknown"
        }
    }
}