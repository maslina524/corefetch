use alloc::{
    string::String,
    vec::Vec,
};

use crate::url::{Url, Response};

const BUF_SIZE: usize = 1024;

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

    #[todo::todo("Http is not implemented")]
    fn send(self, _method: &str) -> Response {
        let content = Vec::from(b"Http is not implemented");
        Response::new(400, content)
    }
}

#[cfg(test)]
mod tests {

}