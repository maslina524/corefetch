use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    sync::OnceLock
};

static VERSION: OnceLock<Version> = OnceLock::new();

#[derive(Debug)]
pub struct Version<'a> {
    pub project_name: &'a str,
    pub version: &'a str,
    pub version_tweak: &'a str,
    pub build_type: &'a str,
    pub sysname: &'a str,
    pub arch: &'a str,
    pub cmake_built_type: &'a str,
    pub compile_time: &'a str,
    pub compiler: &'a str,
    pub libc: &'a str
}

impl Module for Version<'_> {
    fn new() -> Self {
        let build_type = if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        };
        Self {
            project_name: env!("CARGO_BIN_NAME"), 
            version: env!("CARGO_PKG_VERSION"), 
            version_tweak: "", 
            build_type, 
            sysname: env!("TARGET_OS"),
            arch: env!("TARGET_ARCH"), 
            cmake_built_type: "", 
            compile_time: env!("COMPILE_TIME"), 
            compiler: env!("RUSTC_VERSION"), 
            libc: ""
        }
    }

    fn get() -> &'static Self {
        VERSION.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Version"
    }

    fn title(&self) -> &'static str {
        "{project-name} {version} ({arch})"
    }

    fn string_name(&self) -> &'static str {
        "version"
    }

    format_for_module!(
        Version,
        project_name, version, version_tweak, build_type,
        sysname, arch, cmake_built_type, compile_time,
        compiler, libc
    );
}

impl_display_for_module!(Version, '_);