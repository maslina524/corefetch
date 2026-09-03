use alloc::string::String;
use doc::Docs;

use crate::{
    detect::gpu::{GpuInfo, GpuType}, 
    format, 
    format_for_module, 
    formats::{Percent, MemorySize, Temperature}, 
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static GPU: OnceLock<Gpu> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Gpu {
    #[doc = "Vendor"]
    pub vendor: &'static str,
    #[doc = "Name"]
    pub name: String,
    #[doc = "Driver"]
    pub driver: String,
    #[doc = "Temperature"]
    pub temperature: Temperature,
    #[doc = "Core count"]
    pub core_count: usize,
    #[doc = "Type"]
    pub r#type: GpuType,
    #[doc = "Total dedicated memory"]
    pub dedicated_total: MemorySize,
    #[doc = "Used dedicated memory"]
    pub dedicated_used: MemorySize,
    #[doc = "Total shared memory"]
    pub shared_total: MemorySize,
    #[doc = "Used shared memory"]
    pub shared_used: MemorySize,
    #[doc = "The platform API used when detecting the GPU"]
    pub platform_api: String,
    #[doc = "Current frequency in GHz"]
    pub frequency: String,
    #[doc = "GPU vendor specific index"]
    pub index: u32,
    #[doc = "Dedicated memory usage percentage num"]
    pub dedicated_percentage_num: Percent,
    #[doc = "Dedicated memory usage percentage bar"]
    pub dedicated_percentage_bar: String,
    #[doc = "Shared memory usage percentage num"]
    pub shared_percentage_num: Percent,
    #[doc = "Shared memory usage percentage bar"]
    pub shared_percentage_bar: String,
    #[doc = "Core usage percentage num"]
    pub core_usage_num: String,
    #[doc = "Core usage percentage bar"]
    pub core_usage_bar: String,
    #[doc = "Memory type (Windows only)"]
    pub memory_type: &'static str,
    #[doc = "`PCIe` maximum speed in gen and lanes"]
    pub pcie_max_speed: String,
    #[doc = "`PCIe` current speed in gen and lanes"]
    pub pcie_curr_speed: String
}

impl Module for Gpu {
    fn new() -> Self {
        let info = GpuInfo::new();
        Self {
            vendor: info.vendor,
            name: info.name,
            driver: info.driver,
            temperature: Temperature::Celsius(info.temperature),
            core_count: 0,
            r#type: info.typ,
            dedicated_total: info.memory_total,
            dedicated_used: MemorySize::default(),
            shared_total: MemorySize::default(),
            shared_used: MemorySize::default(),
            platform_api: String::new(),
            frequency: format!("{:.2} GHz", info.frequency),
            index: info.device_id,
            dedicated_percentage_num: Percent::default(),
            dedicated_percentage_bar: String::new(),
            shared_percentage_num: Percent::default(),
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