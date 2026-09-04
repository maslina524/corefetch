use core::{
    ffi::c_int, 
    ptr
};

use alloc::{
    ffi::CString, 
    string::{String, ToString}, 
    vec::Vec,
};

use crate::{
    abort,
    linux::libc::{
        AddrInfo, close, connect, freeaddrinfo, getaddrinfo, recv, send, socket
    }, 
    url::{Response, Url},
    format
};

const BUF_SIZE: usize = 1024;

const AF_UNSPEC: c_int = 0;
const SOCK_STREAM: c_int = 1;
const IPPROTO_TCP: c_int = 6;

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

    pub const fn from_url(url: Url) -> Self {
        Self { url }
    }

    pub fn get(self) -> Response {
        self.send("GET")
    }

    fn send(self, method: &str) -> Response {
        // crate::println!("Url: {:#?}", self.url);
        let full_domain = format!("{}.{}", self.url.domain, self.url.tld);
        let c_hostname = CString::new(full_domain).expect("invalid hostname");
        let c_port = CString::new(self.url.port().to_string()).expect("invalid port");

        let hints = AddrInfo {
            ai_family: AF_UNSPEC,
            ai_socktype: SOCK_STREAM,
            ai_protocol: IPPROTO_TCP,
            ..Default::default()
        };

        let mut result: *mut AddrInfo = ptr::null_mut();
        let ret = getaddrinfo(
            c_hostname.as_ptr(),
            c_port.as_ptr(),
            &raw const hints,
            &raw mut result,
        );
        if ret != 0 {
            abort!("getaddrinfo failed, url: `{}`", self.url);
        }

        let mut sockfd: c_int = -1;
        let mut rp = result;
        while !rp.is_null() {
            // SAFETY: Libc always returns a valid pointer
            let ai = unsafe { &*rp };
            sockfd = socket(ai.ai_family, ai.ai_socktype, ai.ai_protocol);
            if sockfd == -1 {
                rp = ai.ai_next;
                continue;
            }
            if connect(sockfd, ai.ai_addr, ai.ai_addrlen) == 0 {
                break;
            }
            close(sockfd);
            sockfd = -1;
            rp = ai.ai_next;
        }

        freeaddrinfo(result);

        if sockfd == -1 {
            abort!("could not connect to host");
        }

        let mut req = String::new();
        req.push_str(method);
        req.push(' ');
        req.push_str(&self.url.path);
        req.push_str(" HTTP/1.1\r\nHost: ");
        req.push_str(&self.url.domain);
        req.push_str("\r\nConnection: close\r\n\r\n");

        let req_bytes = req.as_bytes();

        let sent = send(sockfd, req_bytes.as_ptr().cast(), req_bytes.len(), 0);
        if sent == -1 {
            close(sockfd);
            abort!("send failed");
        }

        let mut response_data = Vec::new();
        let mut buffer = [0u8; BUF_SIZE];
        loop {
            let n = recv(sockfd, buffer.as_mut_ptr().cast(), buffer.len(), 0);
            if n < 0 {
                close(sockfd);
                abort!("recv failed");
            }
            if n == 0 {
                break;
            }
            response_data.extend_from_slice(&buffer[..n as usize]);
        }

        close(sockfd);

        Response::from_raw(&response_data)
    }
}