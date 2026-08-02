use core::{
    ptr,
    ffi::c_void
};

use alloc::{
    string::String,
    string::ToString,
    vec::Vec,
    borrow::ToOwned,
    ffi::CString
};

use crate::{
    os::error::ErrorCode,
    os::windows::*,
    format,
    L
};

const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: u32        = 0;
const WINHTTP_NO_PROXY_NAME            : *const u16 = ptr::null();
const WINHTTP_NO_PROXY_BYPASS          : *const u16 = ptr::null();
const WINHTTP_OPTION_RECEIVE_TIMEOUT   : u32        = 95;

pub struct Response {
    code: u16,
    content: Vec<u8>
}

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

    pub fn as_cstr(&self) -> CString {
        CString::new(self.to_string()).unwrap()
    }

    pub fn port(&self) -> u16 {
        if let Some(port) = self.port {
            port
        } else {
            match self.protocol.as_str() {
                "http"  => 80,
                "https" => 443,
                _ => 0
            }
        }
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

pub struct Request {
    url: Url,
}

impl Request {
    pub fn new(url: impl Into<String>) -> Option<Self> {
        let url = Url::new(url.into())?;
        Some(
            Self { url }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::os::https::{Request, Url};

    extern crate std;

    #[test]
    fn example_response_test() {
        let url = "http://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let response = Request::new(url).unwrap();
    }

    #[test]
    fn url_parse_test() {
        let string = "http://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        let url = Url::new(string);
        std::println!("{url:?}")
    }
}