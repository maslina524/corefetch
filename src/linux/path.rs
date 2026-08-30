use core::{
    ffi::{CStr, c_char}, slice, str::FromStr
};

use alloc::{
    borrow::ToOwned, 
    ffi::CString, 
    string::{String, ToString}, 
    vec::Vec
};

use crate::{
    abort, 
    linux::libc::{getenv, access}
};

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Path {
    inner: String
}

impl Path {
    pub const fn new() -> Self {
        Self { inner: String::new() }
    }

    // ONLY FOR LINUX
    fn home() -> Self {
        let raw_name = c"HOME";
        let ptr = unsafe { getenv(raw_name.as_ptr()) };
        if ptr.is_null() {
            abort!("Failed to get HOME");
        }
        let c_str = unsafe { CStr::from_ptr(ptr) };
        let string = c_str.to_string_lossy().into_owned();
        Self::from(string)
    }

    pub fn local() -> Self {
        Self::home().join(".local/share")
    }

    pub fn corefetch() -> Self {
        Self::local().join("corefetch")
    }

    pub fn cache() -> Self {
        Self::home().join("cache/corefetch")
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: String::with_capacity(cap) }
    }

    fn clear(self) -> Self {
        let inner = self.inner
            .replace('\\', "/")
            .split('/')
            .filter(|x| !x.is_empty())
            .fold(String::new(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('/');
                }
                acc.push_str(s);
                acc
            });

        Self { inner }
    }

    pub const fn as_str(&self) -> &String {
        &self.inner
    }

    // ONLY FOR LINUX
    pub fn as_c_str(&self) -> CString {
        CString::new(self.inner.clone()).unwrap()
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

    pub fn join(&self, path: impl Into<Self>) -> Self {
        let mut parts = self.parts();
        let path = path.into();
        let path_parts = path.parts();
        parts.extend(path_parts);
        Self::from(parts)
    }
    
    pub fn exists(&self) -> bool {
        let c_str = self.as_c_str();
        // SAFETY: Completely safe
        (unsafe { access(c_str.as_ptr(), 0) }) == 1
    }

    pub fn parent(&self) -> Option<Self> {
        let mut clone = self.clone();
        clone.pop()?;
        Some(clone)
    }

    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self { inner: value }.clear()
    }
}

impl From<&String> for Path {
    fn from(value: &String) -> Self {
        Self { inner: value.to_owned() }.clear()
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

impl From<&Self> for Path {
    fn from(value: &Self) -> Self {
        Self { inner: value.as_str().to_owned() }.clear()
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::linux::path::Path;
  
    extern crate std;

    #[test]
    fn get_local_test() {
        let local = Path::local();
        let string = local.as_str();
        println!("{string}");
        assert!(string.ends_with(".local/share"));
    }
}