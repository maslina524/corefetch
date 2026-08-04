use alloc::string::String;

use crate::{
    os::env,
    sync::OnceLock,
    format
};

static OS: OnceLock<Os> = OnceLock::new();

#[derive(Debug)]
pub struct Os<'a> {
    pub sysname: String,
    pub name: String,
    pub pretty_name: String,
    pub id: String,
    pub id_like: String,
    pub variant: String,
    pub variant_id: String,
    pub version: String,
    pub version_id: String,
    pub codename: String,
    pub build_id: String,
    pub arch: &'a str,
}

impl Os<'_> {
    pub fn new() -> Self {
        let ver = env::os_version();
        let pretty_name = format!("{} {} ({})", ver.name, ver.version, ver.codename);
        let id = format!("{} {}", ver.name, ver.version);
        Self { 
            sysname: ver.sysname.clone(), 
            name: ver.name, 
            pretty_name, 
            id, 
            id_like: ver.sysname, 
            variant: ver.variant, 
            variant_id: String::new(), 
            version: ver.version, 
            version_id: String::new(), 
            codename: ver.codename, 
            build_id: String::new(), 
            arch: env!("TARGET_ARCH")
        }
    }

    pub fn get() -> &'static Self {
        OS.get_or_init(|| {
            Self::new()
        })
    }
}

impl core::fmt::Display for Os<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.pretty_name, self.arch)
    }
}