use core::{
    ptr,
    mem,
    ffi::c_void,
    sync::atomic::AtomicPtr
};

use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    windows::link::{
        CreateDXGIFactory, IID_IDXGIFactory, IDXGIFactory_Vtbl, DXGI_ADAPTER_DESC,
        IDXGIAdapter_Vtbl, DXGI_ERROR_NOT_FOUND, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
        DIGCF_PRESENT, GUID_DEVCLASS_DISPLAY, SP_DEVINFO_DATA, SetupDiOpenDevRegKey,
    },
    windows::regedit::Regedit,
    windows::encoding::{utf16le_to_utf8, Utf16Len},
    formats::Size,
    nvidia::NvidiaLib,
    abort,
    warning
};

const INVALID_HANDLE: *mut c_void = (-1isize).cast_unsigned() as *mut c_void;

pub struct GpuInfo {
    pub vendor: &'static str,
    pub name: String,
    pub device_id: u32,
    pub driver: String,
    pub temperature: f32,
    pub typ: &'static str,
    pub memory_total: Size
}

impl GpuInfo {
    pub fn new() -> Self {
        let desc = Self::dxgi_adapter_desc().unwrap_or_else(
            |e| abort!("CreateDXGIFactory error: {e}")
        );

        let name = utf16le_to_utf8(
            &desc.Description, 
            Utf16Len::NullTerminated
        ).expect("WinAPI passed an invalid UTF-16LE string");

        let driver = Self::driver_version().unwrap_or_else(
            || { warning!("Failed to get driver version"); String::from("Unknown") }
        );

        let memory_total = Size::from_bytes(desc.DedicatedVideoMemory as u64);

        Self {
            vendor: Self::vendor_name(desc.VendorId),
            name,
            device_id: desc.DeviceId,
            driver,
            temperature: Self::temperature(desc.VendorId),
            typ: Self::typ(desc.VendorId, &memory_total),
            memory_total
        }
    }

    pub fn typ(vendor: u32, memory: &Size) -> &'static str {
        if *memory > Size::Mb(256.0) && [0x10DE, 0x1002, 0x1022].contains(&vendor) {
            "Discrete"
        } else {
            "Built-in"
        }
    }

    pub fn temperature(vendor_id: u32) -> f32 {
        match vendor_id {
            0x10DE => NvidiaLib::get().gpu_temperature() as f32,
            _ => 0.0,
        }
    }

    pub const fn vendor_name(vendor_id: u32) -> &'static str {
        match vendor_id {
            0x10DE => "NVIDIA",
            0x1002 | 0x1022 => "AMD",
            0x8086 => "Intel",
            0x1414 => "Microsoft (Software/WARP)",
            0x5143 => "Qualcomm",
            _ => "Unknown",
        }
    }

    pub fn dxgi_adapter_desc() -> Result<DXGI_ADAPTER_DESC, i32> {
        let mut factory_void = ptr::null_mut();
        
        // SAFETY: Completely safe
        let hr = unsafe {
            CreateDXGIFactory(&IID_IDXGIFactory, &raw mut factory_void)
        };
        
        if hr < 0 || factory_void.is_null() {
            return Err(hr);
        }

        // SAFETY: We check that the raw pointer is not null
        let factory_vtbl = unsafe { *factory_void.cast::<*mut IDXGIFactory_Vtbl>() };

        let mut i = 0;
        loop {
            let mut adapter_void = ptr::null_mut();
            
            // SAFETY: A virtual table is guaranteed to be located at the raw pointer
            let hr = unsafe { ((*factory_vtbl).EnumAdapters)(factory_void, i, &raw mut adapter_void) };
            if hr == DXGI_ERROR_NOT_FOUND {
                return Err(hr);
            }

            if !adapter_void.is_null() {
                let mut desc = DXGI_ADAPTER_DESC::default();
                
                // SAFETY: We check that the raw pointer is not null
                let adapter_vtbl = unsafe { *adapter_void.cast::<*mut IDXGIAdapter_Vtbl>() };
                // SAFETY: A virtual table is guaranteed to be located at the raw pointer
                let hr = unsafe { ((*adapter_vtbl).GetDesc)(adapter_void, &raw mut desc) };
                
                if hr >= 0 {
                    return Ok(desc);
                }
            }

            i += 1;
        }
    }

    pub fn driver_version() -> Option<String> {
        // SAFETY: Completely safe
        let handle = unsafe {
            SetupDiGetClassDevsW(
                &GUID_DEVCLASS_DISPLAY, 
                ptr::null(), 
                ptr::null_mut(), 
                DIGCF_PRESENT
            )
        };
        if handle == -1 {
            return None
        }

        let mut info = SP_DEVINFO_DATA {
            cbSize: mem::size_of::<SP_DEVINFO_DATA>() as u32,
            .. SP_DEVINFO_DATA::default()
        };

        // SAFETY: Completely safe
        let ret = unsafe {
            SetupDiEnumDeviceInfo(
                handle, 
                0, 
                &raw mut info
            )
        };
        if ret == 0 {
            return None
        }

        // SAFETY: Completely safe
        let hkey = unsafe {
            SetupDiOpenDevRegKey(
                handle, 
                &raw mut info, 
                1, 
                0,
                2, 
                0x20019
            )
        };
        if hkey == INVALID_HANDLE {
            return None;
        }

        let reg = Regedit::from_handle(AtomicPtr::new(hkey));
        let value = reg.read("DriverVersion").ok()?;
        let string = value.as_string()?.to_owned();

        Some(string)
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::gpu::GpuInfo;

    #[test]
    fn vendor_test() {
        let info = GpuInfo::new();
        let name = info.vendor;
        assert!(!name.is_empty());
        println!("Vendor: {name}");
    }

    #[test]
    fn name_test() {
        let info = GpuInfo::new();
        let name = info.name;
        assert!(!name.is_empty());
        println!("Name: {name}");
    }

    #[test]
    fn device_id_test() {
        let info = GpuInfo::new();
        let id = info.device_id;
        assert!(id != 0);
        println!("Id: {id}");
    }

    #[test]
    fn driver_test() {
        let info = GpuInfo::new();
        let driver = info.driver;
        assert!(driver != "Unknown");
        println!("Driver: {driver}");
    }

    #[test]
    fn temperature_test() {
        let info = GpuInfo::new();
        if info.vendor == "NVIDIA" {
            assert!(info.temperature != 0.0);
            println!("Temperature: {}", info.temperature);
        }
    }

    #[test]
    fn type_test() {
        let info = GpuInfo::new();
        println!("Type: {}", info.typ);
    }
}