use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    imp::env,
    sync::OnceLock
};

static PROCESSES: OnceLock<Processes> = OnceLock::new();

#[derive(Debug)]
pub struct Processes {
    pub result: usize
}

impl Module for Processes {
    fn new() -> Self {
        Self {
            result: env::processes_count()
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