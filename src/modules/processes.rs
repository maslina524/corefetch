use crate::{
    format_for_module,
    impl_display_for_module,
    modules::Module,
    os::env,
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

    fn key() -> &'static str {
        "Processes"
    }

    fn title() -> &'static str {
        "{result}"
    }

    format_for_module!(Processes, result);
}

impl_display_for_module!(Processes);