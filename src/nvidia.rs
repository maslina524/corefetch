use core::{
    mem,
    ffi::{c_void, c_uint, c_char},
    sync::atomic::{AtomicPtr, Ordering},
};

use alloc::boxed::Box;

use crate::{
    abort, 
    sync::OnceLock, 
    warning, 
    windows::link::{FreeLibrary, GetProcAddress, HMODULE, LoadLibraryA}
};

pub type WinapiFn = unsafe extern "system" fn() -> isize;

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

macro_rules! get_fn {
    ($handle:tt, $name:expr, $typ:ident) => {{
        // SAFETY: Completely safe
        let addr = GetProcAddress($handle, $name.as_ptr().cast()).unwrap_or_else(
            || {
                unload($handle);
                $crate::abort!(concat!(stringify!($name), " not found in nvml.dll"));
            }
        );
        mem::transmute::<WinapiFn, $typ>(addr)
    }};
}

static NVIDIA: OnceLock<NvidiaLib> = OnceLock::new();

const NVML_CLOCK_GRAPHICS: u32 = 0;

pub struct NvidiaLib {
    handle: AtomicPtr<HMODULE>,
    device: AtomicPtr<nvmlDevice>,
    init: nvmlInit,
    shutdown: nvmlShutdown,
    device_get_handle_by_index: nvmlDeviceGetHandleByIndex,
    device_get_temperature: nvmlDeviceGetTemperature,
    get_clock_info: nvmlDeviceGetClockInfo
}

impl NvidiaLib {
    pub fn get() -> &'static Self {
        NVIDIA.get_or_init(|| {
            // Load library
            let mut lib = load();
            let lib_atomic = AtomicPtr::new(&raw mut lib);

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

            let device_atomic = AtomicPtr::new(&raw mut device);

            Self {
                handle: lib_atomic,
                device: device_atomic,
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
            unload(unsafe { *lib.handle.load(Ordering::Acquire) });
        }
    }

    pub fn gpu_temperature(&self) -> u16 {
        let mut temp = 0u32;
        let dev = unsafe { *self.device.load(Ordering::Relaxed) };

        // SAFETY: Completely safe
        let ret = unsafe { (self.device_get_temperature)(dev, 0, &raw mut temp) };
        if ret != 0 {
            return 0;
        }

        temp as u16
    }

    pub fn get_frequency_ghz(&self) -> f64 {
        let mut clock_mhz = 0;
        // SAFETY: Device is always valid
        let dev = unsafe { *self.device.load(Ordering::Relaxed) };

        // SAFETY: Completely safe
        let ret = unsafe {
            (self.get_clock_info)(dev, NVML_CLOCK_GRAPHICS, &raw mut clock_mhz)
        };
        if ret == 0 {
            clock_mhz as f64 / 1000.0
        } else {
            warning!("Failed to get GPU frequency (nvml)");
            0.0
        }
    }
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