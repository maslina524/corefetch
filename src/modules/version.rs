use crate::sync::OnceLock;

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

impl Version<'_> {
    pub const fn new() -> Self {
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

    pub fn get() -> &'static Self {
        VERSION.get_or_init(|| {
            Self::new()
        })
    }
}

impl core::fmt::Display for Version<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} ({})", self.project_name, self.version, self.arch)
    }
}