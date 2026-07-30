use core::fmt::Write;

use alloc::string::String;

use crate::{
    sync::OnceLock,
    os::windows::GetStdHandle
};

const STDOUT: u32 = u32::MAX - 10;
const STDERR: u32 = u32::MAX - 11;

static STDOUT_HANDLE: OnceLock<isize> = OnceLock::new();
static STDERR_HANDLE: OnceLock<isize> = OnceLock::new();

pub struct StringFormatter<'a>(&'a mut String);

impl<'a> Write for StringFormatter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}

impl<'a> StringFormatter<'a> {
    #[inline(always)]
    pub fn new(ptr: &'a mut String) -> Self {
        StringFormatter(ptr)
    }
    
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        self.0.write_str("\n")
    }
}

#[macro_export]
macro_rules! format {
    ($($tt:tt)*) => {{
        let mut string = alloc::string::String::new();
        let mut formatter = $crate::os::io::StringFormatter::new(&mut string);
        let _ = formatter.write_fmt(format_args!($($tt)*));
        string
    }};
}

#[inline(always)]
pub fn stdout() -> isize {
    *STDOUT_HANDLE.get_or_init(|| {
        (unsafe { GetStdHandle(STDOUT) }) as isize
    })
}

#[inline(always)]
pub fn stderr() -> isize {
    *STDERR_HANDLE.get_or_init(|| {
        (unsafe { GetStdHandle(STDERR) }) as isize
    })
}