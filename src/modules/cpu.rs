use alloc::string::String;

use crate::{
    impl_display_for_module,
    format_for_module,
    detect::cpu::CpuInfo,
    modules::Module, 
    sync::OnceLock,
    format
};

static CPU: OnceLock<Cpu> = OnceLock::new();

#[derive(Debug)]
pub struct Cpu {
    name: String,
    vendor: String,
    cores_physical: usize,
    cores_logical: usize,
    cores_online: usize,
    freq_base: String,
    freq_max: String,
    temperature: String,
    core_types: String,
    packages: usize,
    march: String,
    numa_nodes: usize,
    code_name: String,
    technology: String
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
            temperature: format!("{:.2} °C", info.temperature),
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