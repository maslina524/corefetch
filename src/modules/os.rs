use alloc::string::String;
use doc::Docs;

use crate::{
    format,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    detect::os::OsInfo, 
    sync::OnceLock
};

static OS: OnceLock<Os> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Os {
    #[doc = "Name of the kernel"]
    pub sysname: &'static str,
    #[doc = "Name"]
    pub name: String,
    #[doc = "Pretty name, if available"]
    pub pretty_name: String,
    #[doc = "ID"]
    pub id: String,
    #[doc = "ID like"]
    pub id_like: String,
    #[doc = "Variant"]
    pub variant: String,
    #[doc = "Variant ID"]
    pub variant_id: String,
    #[doc = "Version"]
    pub version: String,
    #[doc = "Version ID"]
    pub version_id: String,
    #[doc = "Version codename"]
    pub codename: String,
    #[doc = "Build ID"]
    pub build_id: String,
    #[doc = "Architecture"]
    pub arch: &'static str,
    #[doc = "Logo as a nerd emoji"]
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