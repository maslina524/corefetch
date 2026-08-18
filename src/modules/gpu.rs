use alloc::string::{String, ToString};

use crate::{
    impl_display_for_module,
    format_for_module,
    detect::gpu::GpuInfo,
    modules::Module, 
    sync::OnceLock,
    format
};

static GPU: OnceLock<Gpu> = OnceLock::new();

#[derive(Debug)]
pub struct Gpu {
    vendor: &'static str,
    name: String,
    driver: String,
    temperature: String,
    core_count: usize,
    r#type: &'static str,
    dedicated_total: String,
    dedicated_used: String,
    shared_total: String,
    shared_used: String,
    platform_api: String,
    frequency: String,
    index: u32,
    dedicated_percentage_num: String,
    dedicated_percentage_bar: String,
    shared_percentage_num: String,
    shared_percentage_bar: String,
    core_usage_num: String,
    core_usage_bar: String,
    memory_type: &'static str,
    pcie_max_speed: String,
    pcie_curr_speed: String
}

impl Module for Gpu {
    fn new() -> Self {
        let info = GpuInfo::new();
        Self {
            vendor: info.vendor,
            name: info.name,
            driver: info.driver,
            temperature: format!("{:.2} °C", info.temperature),
            core_count: 0,
            r#type: info.typ,
            dedicated_total: info.memory_total.to_string(),
            dedicated_used: String::new(),
            shared_total: String::new(),
            shared_used: String::new(),
            platform_api: String::new(),
            frequency: String::new(),
            index: info.device_id,
            dedicated_percentage_num: String::new(),
            dedicated_percentage_bar: String::new(),
            shared_percentage_num: String::new(),
            shared_percentage_bar: String::new(),
            core_usage_num: String::new(),
            core_usage_bar: String::new(),
            memory_type: "",
            pcie_max_speed: String::new(),
            pcie_curr_speed: String::new(),
        }
    }

    fn get() -> &'static Self {
        GPU.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "GPU"
    }

    fn title(&self) -> &'static str {
        "{name} @ {frequency} ({dedicated_total}) [{type}]"
    }

    fn string_name(&self) -> &'static str {
        "gpu"
    }

    format_for_module!(
        Gpu,
        vendor, name, driver, temperature, 
        core_count, r#type, dedicated_total, dedicated_used, 
        shared_total, shared_used, platform_api, frequency, 
        index, dedicated_percentage_num, dedicated_percentage_bar, shared_percentage_num, 
        shared_percentage_bar, core_usage_num, core_usage_bar, memory_type, 
        pcie_max_speed, pcie_curr_speed
    );
}

impl_display_for_module!(Gpu);

