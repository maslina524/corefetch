use core::{
    mem,
    ffi::{c_void, c_uint, c_char}
};

use crate::{
    abort,
    os::windows::{FreeLibrary, GetProcAddress, HMODULE, LoadLibraryA}
};

pub type WinapiFn = unsafe extern "system" fn() -> isize;
#[allow(non_camel_case_types)]
pub type nvmlReturn_t = i32;
#[allow(non_camel_case_types)]
pub type nvmlTemperatureSensors_t = i32;
#[allow(non_camel_case_types)]
pub type nvmlDevice_t = *mut c_void;
#[allow(non_camel_case_types)]
pub type nvmlInit = unsafe extern "C" fn() -> nvmlReturn_t;
#[allow(non_camel_case_types)]
pub type nvmlShutdown = unsafe extern "C" fn() -> nvmlReturn_t;
#[allow(non_camel_case_types)]
pub type nvmlDeviceGetHandleByIndex = unsafe extern "C" fn(index: c_uint, device: *mut nvmlDevice_t) -> nvmlReturn_t;
#[allow(non_camel_case_types)]
pub type nvmlDeviceGetTemperature = unsafe extern "C" fn(device: nvmlDevice_t, sensor: nvmlTemperatureSensors_t, temp: *mut c_uint) -> nvmlReturn_t;
#[allow(non_camel_case_types)]
pub type nvmlErrorString = unsafe extern "C" fn(result: nvmlReturn_t) -> *const c_char;

macro_rules! get_fn {
    ($handle:tt, $name:expr) => {{
        // SAFETY: Completely safe
        GetProcAddress($handle, $name.as_ptr().cast()).unwrap_or_else(
            || {
                unload($handle);
                $crate::abort!(concat!(stringify!($name), " not found in nvml.dll"));
            }
        )
    }};
}

fn load() -> HMODULE {
    // SAFETY: An ASCII string is always passed, everything is safe
    let lib = unsafe {
        LoadLibraryA(c"nvml.dll".as_ptr().cast())
    };
    if lib.is_null() {
        abort!("Failed to load nvml.dll");
    }
    lib
}

fn unload(lib: HMODULE) {
    // SAFETY: Completely safe
    unsafe {
        FreeLibrary(lib)
    };
}

pub fn gpu_temperature() -> u16 {
    let lib = load();

    // SAFETY: `transmute` fully complies with the documentation
    let nvml_init = unsafe { mem::transmute::<WinapiFn, nvmlInit>(get_fn!(lib, c"nvmlInit")) };
    // SAFETY: `transmute` fully complies with the documentation
    let nvml_shutdown = unsafe { mem::transmute::<WinapiFn, nvmlShutdown>(get_fn!(lib, c"nvmlShutdown")) };
    // SAFETY: `transmute` fully complies with the documentation
    let nvml_device_get_handle_by_index = unsafe { mem::transmute::<WinapiFn, nvmlDeviceGetHandleByIndex>(get_fn!(lib, c"nvmlDeviceGetHandleByIndex")) };
    // SAFETY: `transmute` fully complies with the documentation
    let nvml_device_get_temperature = unsafe { mem::transmute::<WinapiFn, nvmlDeviceGetTemperature>(get_fn!(lib, c"nvmlDeviceGetTemperature")) };

    let mut dev = nvmlDevice_t::default();
    let mut temp = 0u32;

    // SAFETY: Completely safe
    let ret = unsafe { nvml_init() };
    if ret != 0 {
        unload(lib);
        abort!("Failed to initialize nvml");
    }

    // SAFETY: Completely safe
    let ret = unsafe { nvml_device_get_handle_by_index(0, &raw mut dev) };
    if ret != 0 {
        // SAFETY: Completely safe
        unsafe { nvml_shutdown() };
        unload(lib);
        abort!("Failed to get handle by index (nvml)");
    }

    // SAFETY: Completely safe
    let ret = unsafe { nvml_device_get_temperature(dev, 0, &raw mut temp) };
    // SAFETY: Completely safe
    unsafe { nvml_shutdown() };
    unload(lib);

    if ret != 0 {
        return 0;
    }

    temp as u16
}

#[cfg(test)]
mod tests {
    use crate::nvidia::gpu_temperature;

    #[test]
    fn get_temperature_test() {
        let temp = gpu_temperature();
        assert!(temp != 0);
        println!("Temperature: {temp}");
    }
}