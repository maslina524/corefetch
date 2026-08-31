use alloc::string::String;

use crate::{
    format,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    detect::os::OsInfo, 
    sync::OnceLock
};

static OS: OnceLock<Os> = OnceLock::new();

#[derive(Debug)]
pub struct Os {
    pub sysname: &'static str,
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
    pub arch: &'static str,
    pub nerd_emoji: char
}

impl Module for Os {
    fn new() -> Self {
        let info = OsInfo::new();
        let pretty_name = format!("{} {} ({})", info.id, info.variant, info.codename);

        Self { 
            sysname: info.sysname, 
            name: info.name, 
            pretty_name, 
            id: info.id, 
            id_like: info.id_like, 
            variant: info.variant, 
            variant_id: info.variant_id, 
            version: info.version, 
            version_id: info.version_id, 
            codename: info.codename, 
            build_id: String::new(), 
            arch: env!("TARGET_ARCH"),
            nerd_emoji: info.nerd
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
        "{pretty-name} {arch}"
    }

    fn string_name(&self) -> &'static str {
        "os"
    }

    format_for_module!(
        Os,
        sysname, name, pretty_name, id,
        id_like, variant, variant_id, version,
        version_id, codename, build_id, arch,
        nerd_emoji
    );
}

impl_display_for_module!(Os);