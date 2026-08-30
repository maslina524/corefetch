use core::ffi::c_int;

use crate::linux::libc;

pub const fn stdout() -> isize {
    1
}

pub const fn stderr() -> isize {
    2
}

pub fn write(handle: isize, s: &str) {
    // SAFETY: Completley safe
    unsafe { libc::write(handle as c_int, s.as_ptr().cast(), s.len()) };
}

#[macro_export]
macro_rules! print {
    () => {{}};
    ($($tt:tt)*) => {{
        let handle = $crate::linux::io::stdout();
        let s = $crate::format!($($tt)*);
        $crate::linux::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        let handle = $crate::linux::io::stdout();
        $crate::linux::io::write(handle, "\n");
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::linux::io::stdout();
        let s = $crate::formatln!($($tt)*);
        $crate::linux::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! eprint {
    () => {{}};
    ($expr:expr) => {{
        let handle = $crate::linux::io::stderr();
        let s = $crate::format!("{}", $expr);
        $crate::linux::io::write(handle, s.as_str());
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::linux::io::stderr();
        let s = $crate::format!($($tt)*);
        $crate::linux::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        let handle = $crate::linux::io::stderr();
        $crate::linux::io::write(handle, "\n");
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::linux::io::stderr();
        let s = $crate::formatln!($($tt)*);
        $crate::linux::io::write(handle, s.as_str());
    }}
}

#[cfg(test)]
mod tests {
    use crate::{
        linux::io::{stdout, write}
    };

    #[test]
    fn write_test() {
        write(stdout(), "Hello World!\n");
        write(stdout(), "Привет мир!\n");
        write(stdout(), "👋👋👋\n");
    }

    #[test]
    fn print_test() {
        print!("Hello {}\n", "World");
        print!("2 + 8 = {}", 2 + 8);
        print!("\n");
    }

    #[test]
    fn println_test() {
        println!("Hello {}", "World");
        println!("2 + 8 = {}", 2 + 8);
    }

    #[test]
    fn eprints_test() {
        eprintln!("Hello {}", "World");
        eprint!("42 * 55 = {}", 42 * 55);
    }
}