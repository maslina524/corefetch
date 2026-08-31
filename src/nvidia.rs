use core::ffi::{c_void, c_uint, c_char};

use crate::{
    abort, 
    sync::OnceLock, 
    warning,
    get_fn,
    cfg_if
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        use crate::windows::link::{FreeLibrary, HMODULE, LoadLibraryA, HMODULE};

        type ApiBaseFn = unsafe extern "system" fn() -> isize;
        type LibHandle = HMODULE;
    } else if #[cfg(target_os = "linux")] {
        use crate::linux::libc::{dlopen, dlclose};

        type ApiBaseFn = *mut c_void;
        type LibHandle = *mut c_void;
    }
}



#[allow(non_camel_case_types)]
pub type nvmlReturn = i32;
#[allow(non_camel_case_types)]
pub type nvmlTemperatureSensors = i32;
#[allow(non_camel_case_types)]
pub type nvmlDevice = *mut c_void;
#[allow(non_camel_case_types)]
pub type nvmlClockType = u32;

#[allow(non_camel_case_types)]
pub type nvmlInit = unsafe extern "C" fn() -> nvmlReturn;
#[allow(non_camel_case_types)]
pub type nvmlShutdown = unsafe extern "C" fn() -> nvmlReturn;
#[allow(non_camel_case_types)]
pub type nvmlDeviceGetHandleByIndex = unsafe extern "C" fn(index: c_uint, device: *mut nvmlDevice) -> nvmlReturn;
#[allow(non_camel_case_types)]
pub type nvmlDeviceGetTemperature = unsafe extern "C" fn(device: nvmlDevice, sensor: nvmlTemperatureSensors, temp: *mut c_uint) -> nvmlReturn;
#[allow(non_camel_case_types)]
pub type nvmlDeviceGetClockInfo = unsafe extern "C" fn(device: nvmlDevice, typ: nvmlClockType, clock: *mut u32 ) -> nvmlReturn;
#[allow(non_camel_case_types)]
pub type nvmlErrorString = unsafe extern "C" fn(result: nvmlReturn) -> *const c_char;

static NVIDIA: OnceLock<NvidiaLib> = OnceLock::new();

const NVML_CLOCK_SM: u32 = 1;

pub struct NvidiaLib {
    handle: LibHandle,
    device: nvmlDevice,
    init: nvmlInit,
    shutdown: nvmlShutdown,
    device_get_handle_by_index: nvmlDeviceGetHandleByIndex,
    device_get_temperature: nvmlDeviceGetTemperature,
    get_clock_info: nvmlDeviceGetClockInfo
}

// SAFETY: THE STRUCTURE IS NOT THREAD-SAFE;
// We are not going to modify non-thread-safe fields,
// using the structure from different threads will not cause problems.
unsafe impl Sync for NvidiaLib {}

impl NvidiaLib {
    pub fn get() -> &'static Self {
        NVIDIA.get_or_init(|| {
            // Load library
            let lib = load();

            // Load fns
            // SAFETY: `transmute` fully complies with the documentation
            let init = unsafe { get_fn!(lib, c"nvmlInit", nvmlInit) };
            // SAFETY: `transmute` fully complies with the documentation
            let shutdown = unsafe { get_fn!(lib, c"nvmlShutdown", nvmlShutdown) };
            // SAFETY: `transmute` fully complies with the documentation
            let device_get_handle_by_index = unsafe { get_fn!(lib, c"nvmlDeviceGetHandleByIndex", nvmlDeviceGetHandleByIndex) };
            // SAFETY: `transmute` fully complies with the documentation
            let device_get_temperature = unsafe { get_fn!(lib, c"nvmlDeviceGetTemperature", nvmlDeviceGetTemperature) };
            // SAFETY: `transmute` fully complies with the documentation
            let get_clock_info = unsafe { get_fn!(lib, c"nvmlDeviceGetClockInfo", nvmlDeviceGetClockInfo) };

            // SAFETY: Completely safe
            let ret = unsafe { init() };
            if ret != 0 {
                abort!("Failed to initialize nvml");
            }

            let mut device = nvmlDevice::default();
            // SAFETY: Completely safe
            let ret = unsafe { (device_get_handle_by_index)(0, &raw mut device) };
            if ret != 0 {
                // SAFETY: Completely safe
                unsafe { (shutdown)() };
                abort!("Failed to get handle by index (nvml)");
            }

            Self {
                handle: lib,
                device,
                init, shutdown, 
                device_get_handle_by_index, device_get_temperature,
                get_clock_info
            }
        })
    }
    
    pub fn drop_nvidia() {
        if let Some(lib) = NVIDIA.get() {
            // SAFETY: Completely safe
            unsafe { (lib.shutdown)() };
            // SAFETY: A guaranteed non-null pointer is loaded once and
            // is not changed until that moment
            unload(lib.handle);
        }
    }

    pub fn gpu_temperature(&self) -> u16 {
        let mut temp = 0u32;

        // SAFETY: Completely safe
        let ret = unsafe { (self.device_get_temperature)(self.device, 0, &raw mut temp) };
        if ret != 0 {
            return 0;
        }

        temp as u16
    }

    pub fn get_frequency_ghz(&self) -> f64 {
        let mut clock = 0;
        let dev = self.device;

        // Warm-up
        for _ in 0..5 {
            // SAFETY: Completely safe
            unsafe {
                (self.get_clock_info)(dev, NVML_CLOCK_SM, &raw mut clock);
            }
        }
        
        // SAFETY: Completely safe
        let ret = unsafe {
            (self.get_clock_info)(dev, NVML_CLOCK_SM, &raw mut clock)
        };
        
        if ret == 0 {
            clock as f64 / 100.0
        } else {
            warning!("Failed to get GPU frequency (nvml)");
            0.0
        }
    }
}

cfg_if! {
    if #[cfg(target_os = "windows")] {
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
    } else if #[cfg(target_os = "linux")] {
        fn load() -> LibHandle {
            let lib_names = [c"libnvidia-ml.so.1", c"libnvidia-ml.so"];

            for name in &lib_names {
                // SAFETY: An ASCII string is always passed, everything is safe
                let lib = dlopen(name.as_ptr().cast(), 0);
                if !lib.is_null() {
                    return lib;
                }
            }
            abort!("Failed to load nvml library")
        }

        fn unload(lib: LibHandle) {
            dlclose(lib);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nvidia::NvidiaLib;

    #[test]
    fn get_temperature_test() {
        let temp = NvidiaLib::get().gpu_temperature();
        assert!(temp != 0);
        println!("Temperature: {temp}");
    }
}