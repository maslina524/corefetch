use alloc::string::String;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;

#[derive(Debug)]
pub struct Url {
    protocol: String,
    subdomains: Vec<String>,
    domain: String,
    tld: String,
    port: Option<u16>,
    path: String,
    // no query
}

impl Url {
    pub fn new(url: impl Into<String>) -> Option<Self> {
        let mut url = url.into();

        // Protocol
        let proto_sep = url.find("://")?;
        let protocol = url[..proto_sep].to_owned();
        url = url[proto_sep + 3..].to_owned();

        // Path
        let slash_sep = url.find("/").unwrap_or(url.len());
        let path = if slash_sep == url.len() {
            String::from("/")
        } else {
            url[slash_sep..].to_owned()
        };
        let mut base = url[..slash_sep].to_owned();

        // Port
        let port = if let Some(colon_sep) = base.rfind(':') {
            let port_string = &base[colon_sep + 1..];
            let port = port_string.parse::<u16>().ok()?;
            base = base[..colon_sep].to_owned();
            Some(port)
        } else {
            None
        };

        let mut parts = base.split(".").map(|s| s.to_owned()).collect::<Vec<String>>();
        
        // Tld
        let tld = parts.pop()?;
        // Domain
        let domain = parts.pop()?;
        // Subdomains
        let subdomains = parts;

        Some(
            Self { protocol, subdomains, domain, tld, port, path }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::os::https::Url;

    extern crate std;

    #[test]
    fn url_parse_test() {
        let string = "http://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let url = Url::new(string);
        std::println!("{url:?}")
    }
}