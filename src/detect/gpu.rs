use core::ptr;

use alloc::string::String;

use crate::{
    os::windows::{
        CreateDXGIFactory, IID_IDXGIFactory, IDXGIFactory_Vtbl, DXGI_ADAPTER_DESC, IDXGIAdapter_Vtbl, DXGI_ERROR_NOT_FOUND
    },
    os::encoding::{utf16le_to_utf8, Utf16Len},
    abort
};

pub struct GpuInfo {
    pub vendor: &'static str,
    pub name: String,
    pub device_id: u32
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

        Self {
            vendor: Self::vendor_name(desc.VendorId),
            name,
            device_id: desc.DeviceId
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
}