use alloc::{
    collections::BTreeMap,
    string::String,
    borrow::ToOwned
};

use crate::{
    linux::fs,
    linux::path::Path
};

#[repr(transparent)]
pub struct LinuxInfo {
    inner: BTreeMap<String, String>
}

impl LinuxInfo {
    pub fn parse_os_release() -> Result<Self, fs::ReadError> {
        Self::parse_file("/etc/os-release")
    }

    pub fn parse_file(path: impl Into<Path>) -> Result<Self, fs::ReadError> {
        let string = fs::read_to_string(path)?;
        Ok(Self::parse(&string))
    }

    pub fn parse(s: &str) -> Self {
        let mut ret = BTreeMap::new();

        for line in s.lines() {
            let eq_count = line.chars().filter(|&c| c == '=').count();
            if eq_count != 1 {
                continue;
            }

            let eq_index = line.find('=').unwrap();
            let k = line[..eq_index].to_ascii_uppercase();
            let mut v = &line[eq_index + 1..];

            if v.starts_with('"') && v.ends_with('"') {
                v = &v[1..v.len() - 1];
            }

            ret.insert(k, v.to_owned());
        }

        Self { inner: ret }
    }
}