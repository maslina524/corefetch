use alloc::{
    borrow::ToOwned,
    string::String
};

use crate::{
    detect::gpu::{GpuInfo, GpuType}, 
    linux::fs::{self, ItemType},
    linux::parser::LinuxInfo,
    formats::Size,
    warning,
    format
};

const GPU_CLASSES: [&str; 6] = ["0x0300", "0x0301", "0x0302", "0x0380", "0x0381", "0x0382"];

impl GpuInfo {
    pub fn new() -> Self {
        let pci_address = if let Some(id) = Self::pci_address() {
            id
        } else {
            return GpuInfo::default()
        };

        let vendor_id = Self::vendor_id(&pci_address);
        let device_id = Self::device_id(&pci_address);
        let driver = Self::driver(&pci_address).unwrap_or(String::new());
        let memory_total = Self::memory_total();

        Self { 
            vendor: Self::vendor_name(vendor_id),
            name: Self::name(vendor_id),
            device_id,
            driver,
            temperature: Self::temperature(vendor_id),
            typ: GpuType::get_by_vendor_and_bus(vendor_id, device_id, &pci_address),
            memory_total,
            frequency: Self::frequency(vendor_id)
        }
    }

    #[todo::todo("Hard to implement on a VM, will be implemented later")]    
    fn memory_total() -> Size {
        Size::default()
    }

    fn device_id(pci_address: &str) -> u32 {
        let path = format!("/sys/bus/pci/devices/{pci_address}/device");
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                warning!("Failed to read {path}: {e}");
                return 0;
            }
        };

        u32::from_str_radix(&content, 16)
            .unwrap_or(0)
    }

    fn driver(pci_address: &str) -> Option<String> {
        let path = format!("/sys/bus/pci/devices/{pci_address}/uevent");
        let parsed = match LinuxInfo::parse_file(&path, '=') {
            Ok(c) => c,
            Err(e) => {
                warning!("Failed to parse {path}: {e}");
                return None;
            }
        };

        parsed.get("DRIVER")
    }

    fn pci_address() -> Option<String> {
        let iter = match fs::read_dir("/sys/bus/pci/devices") {
            Ok(i) => i,
            Err(e) => {
                warning!("Failed to get devices: {e}");
                return None;
            }
        };

        for entry in iter {
            if entry.typ() != ItemType::Dir {
                continue;
            }

            let name = entry.name();
            let content_path = format!("/sys/bus/pci/devices/{name}/class");
            let content = if let Ok(c) = fs::read_to_string(content_path) {
                c
            } else {
                continue;
            };

            if GPU_CLASSES.contains(&content.trim()) {
                return Some(content);
            }
        }

        warning!("Failed to find graphics card in devices");
        None
    }

    fn vendor_id(pci_address: &str) -> u32 {
        let content = fs::read_to_string(format!("/sys/bus/pci/devices/{pci_address}/vendor"))
            .unwrap_or_else(|e| { 
                warning!("Failed to read /sys/class/drm/card0/device/vendor: {e}");
                "0x0".to_owned()
            });

        u32::from_str_radix(&content, 16)
            .unwrap_or(0)
    }
}