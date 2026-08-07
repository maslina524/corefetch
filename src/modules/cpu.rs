use alloc::string::String;

use crate::{
    format,
    impl_display_for_module,
    format_for_module,
    detect::cpu,
    modules::Module, 
    sync::OnceLock
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
        Self {
            name: cpu::name(),
            vendor: cpu::vendor(),
            cores_physical: cpu::physical_cores_count(),
            cores_logical: cpu::logical_cores_count(),
            cores_online: cpu::online_cores_count(),
            freq_base: cpu::base_freq_formatted(),
            freq_max: cpu::max_freq_formatted(),
            temperature: cpu::temperature(),
            core_types: cpu::logical_grouped(),
            packages: cpu::package_count(),
            march: cpu::micro_arch(),
            numa_nodes: cpu::numa_nodes_count(),
            code_name: cpu::code_name(),
            technology: cpu::technology()
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
        "{name} {cores-physical}:{cores-logical}:{cores-online}"
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