use alloc::{
    string::String,
    vec::Vec,
    borrow::ToOwned
};

use crate::{
    os::encoding::utf8_to_utf16le,
    os::error
};

#[repr(transparent)]
pub struct Path {
    inner: String
}

impl Path {
    pub const fn new() -> Self {
        Self { inner: String::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: String::with_capacity(cap) }
    }

    fn clear(self) -> Self {
        let inner = self.inner
            .replace('\\', "/")
            .split('/')
            .filter(|x| !x.is_empty())
            .collect();

        Self { inner }
    }

    pub const fn as_str(&self) -> &String {
        &self.inner
    }

    pub fn as_utf16le_str(&self) -> error::Result<Vec<u16>> {
        utf8_to_utf16le(&self.inner)
    }

    pub fn parts(&self) -> Vec<&str> {
        self.inner.split('/').collect()
    }

    pub fn pop(&mut self) -> Option<String> {
        let mut parts = self.parts();
        if parts.len() <= 1 {
            return None;
        }
        let popped = parts.pop().map(String::from);
        self.inner = parts.join("/");
        popped
    }

    pub fn join(&mut self, path: impl Into<Self>) {
        let mut parts = self.parts();
        let path = path.into();
        let path_parts = path.parts();
        parts.extend(path_parts);
        self.inner = parts.join("/");
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self { inner: value }.clear()
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self { inner: value.to_owned() }.clear()
    }
}

impl From<Vec<String>> for Path {
    fn from(value: Vec<String>) -> Self {
        Self { inner: value.join("/") }.clear()
    }
}

impl From<Vec<&str>> for Path {
    fn from(value: Vec<&str>) -> Self {
        Self { inner: value.join("/") }.clear()
    }
}