use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
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
        Self::parse_file("/etc/os-release", '=')
    }

    pub fn parse_cpu_info() -> Result<Self, fs::ReadError> {
        Self::parse_file("/proc/cpuinfo", ':')
    }

    pub fn parse_file(path: impl Into<Path>, split: char) -> Result<Self, fs::ReadError> {
        let string = fs::read_to_string(path)?;
        Ok(Self::parse(&string, split))
    }

    pub fn parse(s: &str, split: char) -> Self {
        let mut ret = BTreeMap::new();

        for line in s.lines() {
            let eq_count = line.chars().filter(|&c| c == split).count();
            if eq_count != 1 {
                continue;
            }

            let eq_index = line.find(split).unwrap();
            let k = line[..eq_index].trim().to_owned();
            let mut v = line[eq_index + 1..].trim();

            if v.starts_with('"') && v.ends_with('"') {
                v = &v[1..v.len() - 1];
            }

            ret.insert(k, v.to_owned());
        }

        Self { inner: ret }
    }

    pub fn get_default(&self, key: &str, default: &impl ToString) -> String {
        self
            .inner
            .get(key)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self
            .inner
            .get(key)
            .cloned()
    }
}

pub fn parse_range_notation(s: &str, capacity: Option<usize>) -> Vec<usize> {
    let mut ret = capacity.map_or_else(Vec::new, Vec::with_capacity);

    for part in s.split(',').map(str::trim) {
        if let Some(pos) = part.find('-') {
            if let (Ok(start), Ok(end)) = (
                part[..pos].parse::<usize>(),
                part[pos + 1..].parse::<usize>(),
            ) && start <= end {
                ret.extend(start..=end);
            }
        } else if let Ok(n) = part.parse::<usize>() {
            ret.push(n);
        }
    }

    ret.sort_unstable();
    ret.dedup();
    ret
}