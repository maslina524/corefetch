use alloc::string::{String, ToString};

use crate::{
    format,
    impl_display_for_module,
    format_for_module,
    detect::kernel,
    modules::Module, 
    sync::OnceLock
};

static KERNEL: OnceLock<Kernel> = OnceLock::new();

#[derive(Debug)]
pub struct Kernel {
    pub sysname: &'static str,
    pub release: String,
    pub version: String,
    pub arch: &'static str,
    pub display_version: String, // Exists in fastfetch, but not supported (?)
    pub page_size: String
}

impl Module for Kernel {
    fn new() -> Self {
        let sysname = kernel::sysname();
        let release = kernel::release();
        let page_size = kernel::page_size();
        let display_version = format!("{sysname} {release}");

        Self {
            sysname,
            release,
            version: kernel::version(),
            arch: env!("TARGET_ARCH"),
            display_version,
            page_size: page_size.to_string()
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

