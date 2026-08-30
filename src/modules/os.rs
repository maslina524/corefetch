use alloc::string::String;

use crate::{
    format,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    windows::env, 
    sync::OnceLock
};

static OS: OnceLock<Os> = OnceLock::new();

#[derive(Debug)]
pub struct Os<'a> {
    pub sysname: &'static str,
    pub name: &'static str,
    pub pretty_name: String,
    pub id: String,
    pub id_like: &'static str,
    pub variant: String,
    pub variant_id: String,
    pub version: &'static str,
    pub version_id: String,
    pub codename: &'static str,
    pub build_id: String,
    pub arch: &'a str,
    pub nerd_emoji: char
}

impl Module for Os<'_> {
    fn new() -> Self {
        let ver = env::os_version();
        let id = format!("{} {}", ver.name, ver.version);
        let pretty_name = format!("{id} {} ({})", ver.variant, ver.codename);
        let nerd_emoji = match (ver.name, ver.version) {
            ("Windows", "11") => '\u{e62a}',
            ("Windows", _)    => '\u{e70f}',
            _ => ' '
        };

        Self { 
            sysname: ver.sysname, 
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
            arch: env!("TARGET_ARCH"),
            nerd_emoji
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

impl_display_for_module!(Os, '_);