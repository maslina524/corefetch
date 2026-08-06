use alloc::string::String;

use crate::{
    format,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    os::env, 
    sync::OnceLock
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

impl Module for Os<'_> {
    fn new() -> Self {
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

    fn get() -> &'static Self {
        OS.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "OS"
    }

    fn title(&self) -> &'static str {
        "{pretty_name}"
    }

    fn string_name(&self) -> &'static str {
        "os"
    }

    format_for_module!(
        Os,
        sysname, name, pretty_name, id,
        id_like, variant, variant_id, version,
        version_id, codename, build_id, arch
    );
}

impl_display_for_module!(Os, '_);