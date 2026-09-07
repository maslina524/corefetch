use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    cfg_if,
    formats::MemorySize, 
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

#[derive(Default)]
pub enum GpuType {
    #[default]
    Unknown,
    Discrete,
    BuiltIn
}

impl core::fmt::Debug for GpuType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unknown  => write!(f, "\"Unknown\""),
            Self::Discrete => write!(f, "\"Discrete\""),
            Self::BuiltIn  => write!(f, "\"Built-in\""),
        }
    }
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

    pub fn get_old(vendor: u32, memory: MemorySize) -> Self {
        if memory > MemorySize::Mb(256.0) && [0x10DE, 0x1002, 0x1022].contains(&vendor) {
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
    pub memory_total: MemorySize,
    pub frequency: f32
}

impl GpuInfo {
    fn name(vendor_id: u32, device_id: u32) -> String {
        match vendor_id {
            0x10DE => NvidiaLib::get().device_name(),
            0x15ad => format!("VMware {}", Self::vmware_name(device_id)),
            0x1002 => format!("AMD {}", Self::amd_name(device_id)),
            0x8086 => format!("Intel {}", Self::intel_name(device_id)),
            _ => "Unknown".to_owned(),
        }
    }

    const fn amd_name(device_id: u32) -> &'static str {
        match device_id {
            0x1114 => "Krackan",
            0x130F => "Kaveri",
            0x13C0 => "Granite Ridge",
            0x1435 => "Sephiroth",
            0x150E => "Strix",
            0x15BF => "Phoenix",
            0x15D8 => "Picasso",
            0x15DD => "Raven Ridge",
            0x15E7 => "Barcelo",
            0x1636 => "Renoir",
            0x1638 => "Cezanne",
            0x163F => "VanGogh",
            0x164C => "Lucienne",
            0x164E => "Raphael",
            0x1681 => "Rembrandt",
            0x1900 | 0x1901 => "HawkPoint",
            _ => "Unknown"
        }
    }

    const fn intel_name(device_id: u32) -> &'static str {
        match device_id {
            0x0166 => "HD Graphics 4000",
            0x0412 => "HD Graphics 4600",
            0x0A16 => "HD Graphics 4400",
            0x5916 => "UHD Graphics 620",
            0x5912 | 0x3E9B | 0x3E92 => "UHD Graphics 630",
            0x8A52 => "Iris Plus Graphics",
            0x9BC4 | 0x9A49 | 0x46A6 => "Iris Xe Graphics",
            0x56A0 | 0x56A1 => "Arc A380",
            0x56B0 => "Arc A580",
            0x56C0 => "Arc A750",
            0x56D0 => "Arc A770",
            _ => "Unknown",
        }
    }

    const fn vmware_name(device_id: u32) -> &'static str {
        match device_id {
            0x0405 => "SVGA II",
            0x0710 => "SVGA",
            _ => "Unknown"
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