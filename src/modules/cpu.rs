use alloc::string::String;
use doc::Docs;

use crate::{
    Docs,
    impl_display_for_module,
    format_for_module,
    detect::cpu::CpuInfo,
    modules::Module, 
    sync::OnceLock,
    formats::Temperature,
    format
};

static CPU: OnceLock<Cpu> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Cpu {
    #[doc = "Name"]
    pub name: String,
    #[doc = "Vendor"]
    pub vendor: String,
    #[doc = "Physical core count"]
    pub cores_physical: usize,
    #[doc = "Logical core count"]
    pub cores_logical: usize,
    #[doc = "Online core count"]
    pub cores_online: usize,
    #[doc = "Base frequency (formatted)"]
    pub freq_base: String,
    #[doc = "Max frequency (formatted)"]
    pub freq_max: String,
    #[doc = "Temperature (not available in windows)"]
    pub temperature: Temperature,
    #[doc = "Logical core count grouped by frequency (not available)"]
    pub core_types: String,
    #[doc = "Package count"]
    pub packages: usize,
    #[doc = "Microarchitecture"]
    pub march: String,
    #[doc = "NUMA node count"]
    pub numa_nodes: usize,
    #[doc = "Code name, like \"Raptor Lake\""]
    pub code_name: String,
    #[doc = "Technology"]
    pub technology: String
}

impl Module for Cpu {
    fn new() -> Self {
        let info = CpuInfo::new();
        Self {
            name: info.name,
            vendor: info.vendor,
            cores_physical: info.physical_cores,
            cores_logical: info.logical_cores,
            cores_online: info.online_cores,
            freq_base: format!("{:.2} GHz", info.base_freq),
            freq_max: format!("{:.2} GHz", info.max_freq),
            temperature: Temperature::Celsius(info.temperature),
            core_types: info.logical_grouped,
            packages: info.packages,
            march: info.micro_arch,
            numa_nodes: info.numa_nodes,
            code_name: info.code_name,
            technology: info.technology
        }
    }

    fn get() -> &'static Self {
        CPU.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "CPU"
    }

    fn title(&self) -> &'static str {
        "{name} ({core-types}) @ {freq-base}"
    }

    fn string_name(&self) -> &'static str {
        "cpu"
    }

    format_for_module!(
        Cpu,
        name, vendor, cores_physical, cores_logical,
        cores_online, freq_base, freq_max, temperature,
        core_types, packages, march, numa_nodes,
        code_name, technology
    );
}

impl_display_for_module!(Cpu);