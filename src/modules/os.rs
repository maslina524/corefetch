use alloc::string::String;

use crate::{
    os::env,
    sync::OnceLock,
    format
};

static OS: OnceLock<Os> = OnceLock::new();

#[derive(Debug)]
pub struct Os<'a> {
    sysname: String,
    name: String,
    pretty_name: String,
    id: String,
    id_like: String,
    variant: String,
    variant_id: String,
    version: String,
    version_id: String,
    codename: String,
    build_id: String,
    arch: &'a str,
}

impl<'a> Os<'a> {
    pub fn new() -> Self {
        let ver = env::os_version().expect("Failed to get os version");
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

    pub fn get() -> &'static Os<'a> {
        OS.get_or_init(|| {
            Os::new()
        })
    }
}

impl<'a> core::fmt::Display for Os<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.pretty_name, self.arch)
    }
}