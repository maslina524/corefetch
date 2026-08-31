use alloc::{
    string::String,
    borrow::ToOwned,
    vec::Vec
};

use crate::format;

#[derive(Debug)]
pub struct Url {
    pub protocol: String,
    pub subdomains: Vec<String>,
    pub domain: String,
    pub tld: String,
    pub port: Option<u16>,
    pub path: String,
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
        let slash_sep = url.find('/').unwrap_or(url.len());
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

        let mut parts = base.split('.').map(ToOwned::to_owned).collect::<Vec<String>>();
        
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

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(match self.protocol.as_str() {
            "http"  => 80,
            "https" => 443,
            _ => 0
        })
    }
}

impl core::fmt::Display for Url {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // protocol
        let mut ret = format!("{}://", self.protocol);

        // subdomains
        if !self.subdomains.is_empty() {
            ret.push_str(&format!("{}.", self.subdomains.join(".")));
        }

        // domain.tld
        ret.push_str(&format!("{}.{}", self.domain, self.tld));

        // port
        if let Some(port) = self.port {
            ret.push_str(&format!(":{}", port));
        }

        // path
        ret.push_str(&self.path);

        write!(f, "{ret}")
    }
}