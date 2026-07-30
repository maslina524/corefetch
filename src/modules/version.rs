/// EXAMPLE:
/// 
/// project-name: fastfetch
/// version: 2.66.0
/// version-tweak:  
/// build-type: release 
/// sysname: Windows 
/// arch: x86_64 
/// cmake-built-type: RelWithDebInfo 
/// compile-time: Jul 10 2026, 07:10:53 
/// compiler: clang 22.1.8 
/// libc: ucrt 14.0
#[derive(Debug)]
pub struct Version<'a> {
    project_name: &'a str,
    version: &'a str,
    version_tweak: &'a str,
    build_type: &'a str,
    sysname: &'a str,
    arch: &'a str,
    cmake_built_type: &'a str,
    compile_time: &'a str,
    compiler: &'a str,
    libc: &'a str
}

impl<'a> Version<'a> {
    pub fn new() -> Self {
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
}

impl<'a> core::fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} ({})", self.project_name, self.version, self.arch)
    }
}