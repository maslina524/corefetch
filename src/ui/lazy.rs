use core::fmt::{self, Debug, Display};

use alloc::boxed::Box;

use crate::sync::OnceLock;

pub struct LazyField<T> {
    value: OnceLock<T>,
    func: Box<dyn Fn() -> T + Send + Sync>,
}

impl<T> LazyField<T> {
    pub fn new(func: Box<dyn Fn() -> T + Send + Sync>) -> Self {
        Self { value: OnceLock::new(), func }
    }

    pub fn get(&self) -> &T {
        self.value.get_or_init(|| (self.func)())
    }
}

impl<T: Display> Display for LazyField<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<T: Debug> Debug for LazyField<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value.get() {
            Some(v) => v.fmt(f),
            None => f.write_str("<lazy>"),
        }
    }
}