use alloc::string::String;
use doc::Docs;

use crate::{
    format,
    impl_display_for_module,
    format_for_module,
    detect::kernel::KernelInfo,
    modules::Module, 
    sync::OnceLock
};

static KERNEL: OnceLock<Kernel> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Kernel {
    #[doc = "Sysname"]
    pub sysname: &'static str,
    #[doc = "Release"]
    pub release: String,
    #[doc = "Version"]
    pub version: String,
    #[doc = "Architecture"]
    pub arch: &'static str,
    #[doc = "Display version"]
    pub display_version: String, // Exists in fastfetch, but not supported (?)
    #[doc = "Page size"]
    pub page_size: String
}

impl Module for Kernel {
    fn new() -> Self {
        let info = KernelInfo::new();

        Self {
            sysname: info.sysname,
            release: info.release,
            version: info.version,
            arch: env!("TARGET_ARCH"),
            display_version: info.display_version,
            page_size: format!("{}", info.page_size)
        }
    }

    fn get() -> &'static Self {
        KERNEL.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Kernel"
    }

    fn title(&self) -> &'static str {
        "{sysname} {release}"
    }

    fn string_name(&self) -> &'static str {
        "kernel"
    }

    format_for_module!(
        Kernel,
        sysname, release, version, arch,
        display_version, page_size
    );
}

impl_display_for_module!(Kernel);