use core::{
    ffi::c_void,
    ptr,
    mem::size_of
};

use alloc::{
    vec::Vec,
    vec
};

use crate::{
    windows::link::{
        GetRawInputDeviceList, RAWINPUTDEVICELIST
    },
    windows::error::ErrorCode,
    warning
};

const RIM_TYPEMOUSE: u32 = 0; 
const RIM_TYPEKEYBOARD: u32 = 1; 
const RIM_TYPEHID: u32 = 2; 

#[derive(Debug)]
pub enum Device {
    Mouse,
    Keyboard,
    Hid
}

impl Device {
    pub const fn from_type(typ: u32) -> Option<Self> {
        match typ {
            RIM_TYPEMOUSE    => Some(Self::Mouse),
            RIM_TYPEKEYBOARD => Some(Self::Keyboard),
            RIM_TYPEHID      => Some(Self::Hid),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct DeviceManager {
    devices: Vec<Device>
}

impl DeviceManager {
    pub fn new() -> Self {
        let mut count = 0;
        // SAFETY: Completely safe
        let ret = unsafe {
            GetRawInputDeviceList(
                ptr::null_mut(), 
                &raw mut count, 
                size_of::<RAWINPUTDEVICELIST>() as u32
            )
        };
        if ret == u32::MAX {
            warning!("GetRawInputDeviceList failed: {} (size)", ErrorCode::last());
            return Self { devices: Vec::new() };
        }
        if count == 0 {
            warning!("No devices found");
            return Self { devices: Vec::new() };
        }

        let mut raw_devices = vec![RAWINPUTDEVICELIST::default(); count as usize];
        // SAFETY: Completely safe
        let ret = unsafe {
            GetRawInputDeviceList(
                raw_devices.as_mut_ptr(), 
                &raw mut count, 
                size_of::<RAWINPUTDEVICELIST>() as u32
            )
        };
        if ret == u32::MAX {
            warning!("GetRawInputDeviceList failed: {} (data)", ErrorCode::last());
            return Self { devices: Vec::new() };
        }

        let mut devices = Vec::with_capacity(count as usize);
        for d in raw_devices {
            if let Some(device) = Device::from_type(d.dwType) {
                devices.push(device);
            }
        }

        Self { devices }
    }

    pub const fn as_devices(&self) -> &Vec<Device> {
        &self.devices
    }
}

#[cfg(test)]
mod tests {
    use crate::windows::device::DeviceManager;

    #[test]
    fn get_hid_test() {
        let hid = DeviceManager::new();
        println!("{hid:#?}");
    }
}