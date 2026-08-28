use alloc::string::String;

use crate::{
    impl_display_for_module,
    format_for_module,
    detect::initsystem::InitSystemInfo,
    modules::Module, 
    sync::OnceLock,
    format
};

static INITSYSTEM: OnceLock<InitSystem> = OnceLock::new();

#[derive(Debug)]
pub struct InitSystem {
    pub name: String,
    pub exe: String,
    pub version: String,
    pub pid: u32
}

impl Module for InitSystem {
    fn new() -> Self {
        let info = InitSystemInfo::new();
        let (v0, v1, v2, v3) = info.version;
        Self {
            name: info.name,
            exe: info.exe.into_inner(),
            version: format!("{v0}.{v1}.{v2}.{v3}"),
            pid: info.pid
        }
    }

    fn get() -> &'static Self {
        INITSYSTEM.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "InitSystem"
    }

    fn title(&self) -> &'static str {
        "{name} {version}"
    }

    fn string_name(&self) -> &'static str {
        "initsystem"
    }

    format_for_module!(
        InitSystem,
        name, exe, version, pid
    );
}

impl_display_for_module!(InitSystem);

