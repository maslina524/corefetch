use crate::{
    os::env, sync::OnceLock
};

static PROCESSES: OnceLock<Processes> = OnceLock::new();

pub fn get() -> &'static Processes {
    PROCESSES.get_or_init(|| {
        Processes::new()
    })
}

#[derive(Debug)]
pub struct Processes {
    result: usize
}

impl Processes {
    pub fn new() -> Self {
        Self {
            result: env::processes_count()
        }
    }
}

impl core::fmt::Display for Processes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.result)
    }
}