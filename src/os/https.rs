use core::{
    ptr,
    ffi::c_void
};

use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned
};

use crate::{
    os::error::ErrorCode,
    os::windows::{
        WinHttpOpen, WinHttpConnect, WinHttpCloseHandle, WinHttpOpenRequest,
        WinHttpSendRequest, WinHttpReceiveResponse, WinHttpQueryHeaders, WinHttpReadData
    },
    os::encoding::wide,
    format,
};

const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: u32               = 0;
const WINHTTP_NO_PROXY_NAME            : *const u16        = ptr::null();
const WINHTTP_NO_PROXY_BYPASS          : *const u16        = ptr::null();
const WINHTTP_NO_REFERER               : *const u16        = ptr::null();
const WINHTTP_DEFAULT_ACCEPT_TYPES     : *const *const u16 = ptr::null();
const WINHTTP_FLAG_SECURE              : u32               = 0x0080_0000;
const WINHTTP_NO_ADDITIONAL_HEADERS    : *const u16        = ptr::null();
const WINHTTP_NO_REQUEST_DATA          : *mut c_void       = ptr::null_mut();
const WINHTTP_QUERY_STATUS_CODE        : u32               = 19;
const WINHTTP_QUERY_FLAG_NUMBER        : u32               = 0x2000_0000;

#[derive(Debug)]
pub struct Response {
    code: u16,
    content: Vec<u8>
}

impl Response {
    pub const fn code(&self) -> u16 {
        self.code
    }

    pub const fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }

    pub const fn content(&self) -> &Vec<u8> {
        &self.content
    }

    pub fn as_text(&self) -> Result<String, alloc::string::FromUtf8Error> {
        String::from_utf8(self.content.clone())
    }
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

    pub fn get(self) -> Response {
        self.send("GET")
    }

    #[cfg(target_os = "windows")]
    fn send(self, method: &str) -> Response {
        // SAFETY: Parameters are fully correct, return value is checked
        let session = unsafe {
            let header = wide("UserAgent/1.0").unwrap();
            WinHttpOpen(
                header.as_ptr(),
                WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
                WINHTTP_NO_PROXY_NAME,
                WINHTTP_NO_PROXY_BYPASS,
                0
            )
        };
        if session.is_null() { ErrorCode::last().panic(); }

        let mut hostname = String::new();
        if !self.url.subdomains.is_empty() {
            hostname.push_str(&self.url.subdomains.join("."));
            hostname.push('.');
        }
        hostname.push_str(&self.url.domain);
        hostname.push('.');
        hostname.push_str(&self.url.tld);

        let server = wide(hostname).unwrap();
        // SAFETY: Just a WinAPI function, the return value is checked
        let conn = unsafe {
            WinHttpConnect(session, server.as_ptr(), self.url.port(), 0)
        };
        if conn.is_null() {
            let err = ErrorCode::last();
            // SAFETY: Completely safe
            unsafe { WinHttpCloseHandle(session) };
            err.panic();
        }

        let method = wide(method).unwrap();
        let path = wide(self.url.path).unwrap();
        let flags = if self.url.protocol == "https" {
            WINHTTP_FLAG_SECURE
        } else {
            0
        };

        // SAFETY: Parameters are fully correct, return value is checked
        let req = unsafe {
            WinHttpOpenRequest(
                conn,
                method.as_ptr(),
                path.as_ptr(),
                ptr::null(),
                WINHTTP_NO_REFERER,
                WINHTTP_DEFAULT_ACCEPT_TYPES,
                flags
            )
        };
        if req.is_null() {
            let err = ErrorCode::last();
            // SAFETY: Completely safe
            unsafe { WinHttpCloseHandle(conn) };
            // SAFETY: Completely safe
            unsafe { WinHttpCloseHandle(session) };
            err.panic();
        }
        
        // SAFETY: Just a WinAPI function, the return value is checked
        let send = unsafe {
            WinHttpSendRequest(
                req, 
                WINHTTP_NO_ADDITIONAL_HEADERS, 
                0, 
                WINHTTP_NO_REQUEST_DATA, 
                0, 
                0, 
                0
            )
        };
        if send == 0 {
            let err = ErrorCode::last();
            // SAFETY: Completely safe
            unsafe { 
                WinHttpCloseHandle( req);
                WinHttpCloseHandle(conn);
                WinHttpCloseHandle(session);
            };
            err.panic_code();
        }
        
        // SAFETY: Just a WinAPI function, the return value is checked
        let ret = unsafe {
            WinHttpReceiveResponse(
                req, 
                ptr::null_mut()
            )
        };
        if ret == 0 {
            let err = ErrorCode::last();
            // SAFETY: Completely safe
            unsafe {
                WinHttpCloseHandle(req);
                WinHttpCloseHandle(conn);
                WinHttpCloseHandle(session);
            };
            err.panic();
        }

        let mut status_code: u32 = 0;
        let mut size = size_of::<u32>() as u32;
        // SAFETY: Parameters are fully correct, return value is checked
        let query = unsafe {
            WinHttpQueryHeaders(
                req,
                WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                ptr::null(),
                (&raw mut status_code).cast(),
                &raw mut size,
                ptr::null_mut(),
            )
        };
        if query == 0 {  ErrorCode::last().panic(); }

        let mut buf = Vec::new();
        let mut read = 0;
        loop {
            let mut chunk = [0u8; 4096];
            // SAFETY: Parameters are fully correct, return value is checked
            let ret = unsafe {
                WinHttpReadData(
                    req,
                    chunk.as_mut_ptr().cast(),
                    4096,
                    &raw mut read,
                )
            };
            if ret == 0 || read == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..read as usize]);
        }

        // SAFETY: Completely safe
        unsafe {
            WinHttpCloseHandle(req);
            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }

        let status_u16 = status_code as u16;
        Response { code: status_u16, content: buf }
    }
}

#[cfg(test)]
mod tests {
    use crate::os::https::{Request, Url};

    extern crate std;

    #[test]
    fn example_response_test() {
        let url = "https://wttr.in/?format=%c;%C;%x;%h;%t;%f;%H;%L;%w;%l;%m;%M;%p;%P;%e;%u;%D;%S;%z;%s;%d;%T;%Z";
        let response = Request::new(url).unwrap().get();
        let string = String::from_utf8(response.content).unwrap();
        println!("Response code: {}", response.code);
        assert!(string.split(';').next().is_some());
    }

    #[test]
    fn url_parse_test() {
        let string = "https://wttr.in/";
        let url = Url::new(string);
        std::println!("{url:?}");
    }
}