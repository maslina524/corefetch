use core::{
    ptr,
    ffi::c_void,
    fmt::Write,
    sync::atomic::{AtomicBool, Ordering}
};

use alloc::string::String;

use crate::{
    os::{error::ErrorCode, windows::{GetStdHandle, SetConsoleOutputCP, WriteFile}}, sync::OnceLock
};

const STDOUT        : u32             = u32::MAX - 10;
const STDERR        : u32             = u32::MAX - 11;
const CP_UTF8       : u32             = 65001;

static STDOUT_HANDLE: OnceLock<isize> = OnceLock::new();
static STDERR_HANDLE: OnceLock<isize> = OnceLock::new();
static IS_UTF8      : AtomicBool      = AtomicBool::new(false);

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

pub fn write(handle: isize, s: &str) {
    if IS_UTF8.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
        // SAFETY: The function always receives a valid constant from the documentation,
        // and possible errors are checked
        let ret = unsafe { SetConsoleOutputCP(CP_UTF8) };
        if ret == 0 { ErrorCode::last().panic(); }
    }
    let mut written = 0;
    let len = s.len() as u32;
    
    // SAFETY: The handle is always correct, 
    // errors are checked, the function is safe
    let ret = unsafe {
        WriteFile(
            handle as *mut c_void, 
            s.as_ptr(), 
            len, 
            &mut written, 
            ptr::null_mut()
        )
    };
    if ret == 0 || written != len { ErrorCode::last().panic(); }
}

#[macro_export]
macro_rules! format {
    ($lit:literal, $($tt:tt)*) => {{
        let mut string = alloc::string::String::new();
        let mut formatter = $crate::os::io::StringFormatter::new(&mut string);
        let _ = formatter.write_fmt(format_args!($lit, $($tt)*));
        string
    }};
}

#[macro_export]
macro_rules! formatln {
    ($lit:literal, $($tt:tt)*) => {{
        let mut string = alloc::string::String::new();
        let mut formatter = $crate::os::io::StringFormatter::new(&mut string);
        let _ = formatter.write_fmt(format_args!($lit, $($tt)*));
        let _ = formatter.write_nl();
        string
    }};
}

#[macro_export]
macro_rules! print {
    () => {{}};
    ($expr:expr) => {{
        let handle = $crate::os::io::stdout();
        let s = $crate::format!("{}", $expr);
        $crate::os::io::write(handle, s.as_str());
    }};
    ($lit:literal, $($tt:tt)*) => {{
        let handle = $crate::os::io::stdout();
        let s = $crate::format!($lit, $($tt)*);
        $crate::os::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        let handle = $crate::os::io::stdout();
        $crate::os::io::write(handle, "\n");
    }};
    ($expr:expr) => {{
        let handle = $crate::os::io::stdout();
        let s = $crate::formatln!("{}", $expr);
        $crate::os::io::write(handle, s.as_str());
    }};
    ($lit:literal, $($tt:tt)*) => {{
        let handle = $crate::os::io::stdout();
        let s = $crate::formatln!($lit, $($tt)*);
        $crate::os::io::write(handle, s.as_str());
    }}
}

#[cfg(test)]
mod tests {
    use crate::{
        os::io::{stdout, write}
    };

    #[test]
    fn write_test() {
        write(stdout(), "Hello World!\n");
    }

    #[test]
    fn print_test() {
        print!("Hello {}\n", "World");
        print!("2 + 8 = ");
        print!(2 + 8);
        print!("\n");
    }

    #[test]
    fn println_test() {
        println!("Hello {}", "World");
        println!("2 + 8 = {}", 2 + 8);
    }
}