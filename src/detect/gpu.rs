use core::ptr;

use crate::{
    os::windows::{
        CreateDXGIFactory, IID_IDXGIFactory, IDXGIFactory_Vtbl, DXGI_ADAPTER_DESC, IDXGIAdapter_Vtbl, DXGI_ERROR_NOT_FOUND
    },
    warning
};

pub struct GpuInfo {
    pub vendor: &'static str
}

impl GpuInfo {
    pub fn new() -> Self {
        let vendor_id = Self::vendor_id();
        Self {
            vendor: Self::vendor_name(vendor_id)
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

    pub fn vendor_id() -> u32 {
        let mut factory_void = ptr::null_mut();
        
        // SAFETY: Completely safe
        let hr = unsafe {
            CreateDXGIFactory(&IID_IDXGIFactory, &raw mut factory_void)
        };
        
        if hr < 0 || factory_void.is_null() {
            warning!("CreateDXGIFactory error: {hr}");
            return 0;
        }

        // SAFETY: We check that the raw pointer is not null
        let factory_vtbl = unsafe { *factory_void.cast::<*mut IDXGIFactory_Vtbl>() };

        let mut i = 0;
        loop {
            let mut adapter_void = ptr::null_mut();
            
            // SAFETY: A virtual table is guaranteed to be located at the raw pointer
            let hr = unsafe { ((*factory_vtbl).EnumAdapters)(factory_void, i, &raw mut adapter_void) };
            if hr == DXGI_ERROR_NOT_FOUND {
                return 0;
            }

            if !adapter_void.is_null() {
                let mut desc = DXGI_ADAPTER_DESC::default();
                
                // SAFETY: We check that the raw pointer is not null
                let adapter_vtbl = unsafe { *adapter_void.cast::<*mut IDXGIAdapter_Vtbl>() };
                // SAFETY: A virtual table is guaranteed to be located at the raw pointer
                let hr = unsafe { ((*adapter_vtbl).GetDesc)(adapter_void, &raw mut desc) };
                
                if hr >= 0 {
                    return desc.VendorId;
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
        let id = GpuInfo::vendor_id();
        assert!(id != 0);
        println!("Vendor: {id:x}");
    }
}