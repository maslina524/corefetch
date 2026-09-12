use doc::Docs;

use crate::{
    format_for_module, 
    imp::env, 
    impl_display_for_module, 
    modules::Module, 
    sync::OnceLock
};

static PROCESSES: OnceLock<Processes> = OnceLock::new();

#[derive(Debug, Docs)]
pub struct Processes {
    #[doc = "Process count"]
    pub result: usize
}

impl Module for Processes {
    fn new() -> Self {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        let result = env::processes_count()
            .unwrap_or_else(|| { 
                crate::warning!("Failed to read /proc"); 
                0 
            });

        #[cfg(target_os = "windows")]
        let result = env::processes_count();

        Self {
            result
        }
    }

    fn get() -> &'static Self {
        PROCESSES.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Processes"
    }

    fn title(&self) -> &'static str {
        "{result}"
    }

    fn string_name(&self) -> &'static str {
        "processes"
    }

    format_for_module!(Processes, result);
}

impl_display_for_module!(Processes);