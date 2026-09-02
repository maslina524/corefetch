use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    cfg_if,
    formats::Size, 
    nvidia::NvidiaLib,
    format
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[derive(Debug, Default)]
pub enum GpuType {
    #[default]
    Unknown,
    Discrete,
    BuiltIn
}

impl GpuType {
    pub fn get_by_vendor_and_bus(vendor_id: u32, device_id: u32, pci_address: &str) -> Self {
        match vendor_id {
            0x10DE => Self::Discrete,
            0x8086 => Self::BuiltIn,
            0x1002 => {
                if pci_address.starts_with("0000:00:") {
                    let dev_str = format!("{:04x}", device_id);
                    if dev_str.starts_with("67") || 
                    dev_str.starts_with("68") ||
                    dev_str.starts_with("69") ||
                    dev_str.starts_with("73") ||
                    dev_str.starts_with("74") {
                        return Self::Discrete;
                    }
                    return Self::BuiltIn;
                }
                Self::Discrete
            },
            _ => Self::Unknown
        }
    }

    pub fn get_old(vendor: u32, memory: &Size) -> Self {
        if *memory > Size::Mb(256.0) && [0x10DE, 0x1002, 0x1022].contains(&vendor) {
            Self::Discrete
        } else {
            Self::BuiltIn
        }
    }
}

impl core::fmt::Display for GpuType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unknown  => write!(f, "Unknown"),
            Self::Discrete => write!(f, "Discrete"),
            Self::BuiltIn  => write!(f, "Built-in")
        }
    }
}

#[derive(Default)]
pub struct GpuInfo {
    pub vendor: &'static str,
    pub name: String,
    pub device_id: u32,
    pub driver: String,
    pub temperature: f32,
    pub typ: GpuType,
    pub memory_total: Size,
    pub frequency: f32
}

impl GpuInfo {
    fn name(vendor_id: u32) -> String {
        match vendor_id {
            0x10DE => NvidiaLib::get().device_name(),
            _ => "Unknown".to_owned(),
        }
    }

    fn temperature(vendor_id: u32) -> f32 {
        match vendor_id {
            0x10DE => NvidiaLib::get().gpu_temperature() as f32,
            _ => 0.0,
        }
    }

    fn frequency(vendor_id: u32) -> f32 {
        match vendor_id {
            0x10DE => NvidiaLib::get().get_frequency_ghz() as f32,
            _ => 0.0,
        }
    }

    const fn vendor_name(vendor_id: u32) -> &'static str {
        match vendor_id {
            0x10DE => "NVIDIA",
            0x1002 | 0x1022 => "AMD",
            0x8086 => "Intel",
            0x1414 => "Microsoft (Software/WARP)",
            0x5143 => "Qualcomm",
            0x15ad => "VMware",
            _ => "Unknown",
        }
    }
}