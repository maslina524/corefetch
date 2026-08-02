use crate::sync::OnceLock;

static COLORS: OnceLock<Colors> = OnceLock::new();

#[derive(Debug)]
pub struct Colors;

impl Colors {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get() -> &'static Colors {
        COLORS.get_or_init(|| {
            Colors::new()
        })
    }
}

impl core::fmt::Display for Colors {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 40..=47 {
            let _ = write!(f, "\x1b[{i}m   ");
        }
        let _ = writeln!(f, "\x1b[0m");
        for i in 100..=107 {
            let _ = write!(f, "\x1b[{i}m   ");
        }
        write!(f, "\x1b[0m")
    }
}