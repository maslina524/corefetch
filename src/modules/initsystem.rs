use alloc::string::String;

use crate::{
    impl_display_for_module,
    format_for_module,
    detect::initsystem::InitSystemInfo,
    modules::Module, 
    sync::OnceLock,
    imp::path::Path
};

static INITSYSTEM: OnceLock<InitSystem> = OnceLock::new();

#[derive(Debug)]
pub struct InitSystem {
    pub name: String,
    pub exe: Path,
    pub version: String,
    pub pid: u32
}

impl Module for InitSystem {
    fn new() -> Self {
        let info = InitSystemInfo::new();
        Self {
            name: info.name,
            exe: info.exe,
            version: info.version,
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

