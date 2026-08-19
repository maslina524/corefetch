use core::{
    mem,
    ffi::{c_void, c_uint, c_char},
    sync::atomic::{AtomicPtr, Ordering},
};

use crate::{
    abort,
    os::windows::{FreeLibrary, GetProcAddress, HMODULE, LoadLibraryA},
    sync::OnceLock
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

pub struct NvidiaLib {
    handle: AtomicPtr<HMODULE>,
    nvml_init: nvmlInit,
    nvml_shutdown: nvmlShutdown,
    nvml_device_get_handle_by_index: nvmlDeviceGetHandleByIndex,
    nvml_device_get_temperature: nvmlDeviceGetTemperature,
}

impl NvidiaLib {
    pub fn get() -> &'static Self {
        NVIDIA.get_or_init(|| {
            // Load library
            let mut lib = load();
            let atomic_ptr = AtomicPtr::new(&raw mut lib);

            // Load fns
            // SAFETY: `transmute` fully complies with the documentation
            let nvml_init = unsafe { get_fn!(lib, c"nvmlInit", nvmlInit) };
            // SAFETY: `transmute` fully complies with the documentation
            let nvml_shutdown = unsafe { get_fn!(lib, c"nvmlShutdown", nvmlShutdown) };
            // SAFETY: `transmute` fully complies with the documentation
            let nvml_device_get_handle_by_index = unsafe { get_fn!(lib, c"nvmlDeviceGetHandleByIndex", nvmlDeviceGetHandleByIndex) };
            // SAFETY: `transmute` fully complies with the documentation
            let nvml_device_get_temperature = unsafe { get_fn!(lib, c"nvmlDeviceGetTemperature", nvmlDeviceGetTemperature) };

            // SAFETY: Completely safe
            let ret = unsafe { nvml_init() };
            if ret != 0 {
                abort!("Failed to initialize nvml");
            }

            Self {
                handle: atomic_ptr, 
                nvml_init, nvml_shutdown, 
                nvml_device_get_handle_by_index, nvml_device_get_temperature 
            }
        })
    }
    
    pub fn drop_nvidia() {
        if let Some(lib) = NVIDIA.get() {
            // SAFETY: Completely safe
            unsafe { (lib.nvml_shutdown)() };
            // SAFETY: A guaranteed non-null pointer is loaded once and
            // is not changed until that moment
            unload(unsafe { *lib.handle.load(Ordering::Acquire) });
        }
    }

    pub fn gpu_temperature(&self) -> u16 {
        let mut dev = nvmlDevice_t::default();
        let mut temp = 0u32;

        // SAFETY: Completely safe
        let ret = unsafe { (self.nvml_device_get_handle_by_index)(0, &raw mut dev) };
        if ret != 0 {
            // SAFETY: Completely safe
            unsafe { (self.nvml_shutdown)() };
            abort!("Failed to get handle by index (nvml)");
        }

        // SAFETY: Completely safe
        let ret = unsafe { (self.nvml_device_get_temperature)(dev, 0, &raw mut temp) };

        if ret != 0 {
            return 0;
        }

        temp as u16
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