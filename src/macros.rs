#[macro_export]
macro_rules! todo_or_default {
    ($msg:literal, $default:expr) => {{
        #[deprecated(note = $msg)]
        const fn _warn() {}
        _warn();
        $default
    }};
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {{
        $crate::print!("\x1b[1;{}minfo\x1b[0m: ", $crate::color::FG_BLUE);
        $crate::println!($($args)*);
    }};
}

#[macro_export]
macro_rules! warning {
    ($($args:tt)*) => {{
        $crate::eprint!("\x1b[1;{}mwarning\x1b[0m: ", $crate::color::FG_YELLOW);
        $crate::eprintln!($($args)*);
    }};
}

#[macro_export]
macro_rules! abort {
    ($($args:tt)*) => {{
        $crate::eprint!("\x1b[1;{}mabort\x1b[0m: ", $crate::color::FG_RED);
        $crate::eprintln!($($args)*);
        $crate::exit(101)
    }};
}

#[macro_export]
macro_rules! colored {
    ($string:tt) => {{
        $string
            .replace("<bold>", "\x1b[1m")
            .replace("<italic>", "\x1b[3m")
            .replace("<underline>", "\x1b[4m")
            .replace("<reset>", "\x1b[0m")
    }};
}

#[macro_export]
macro_rules! multi_string {
    ($($s:literal),* $(,)?) => {{
        const COUNT: usize = [$(multi_string!(@count $s)),*].len();
        
        let strings: [&str; COUNT] = [$($s),*];
        
        strings.join("\n")
    }};
    
    (@count $s:literal) => { 1 };
}

#[macro_export]
macro_rules! get_fn {
    ($handle:tt, $name:expr, $typ:ident) => {{
        // SAFETY: Completely safe
        let addr = $crate::windows::link::GetProcAddress($handle, $name.as_ptr().cast()).unwrap_or_else(
            || {
                unload($handle);
                $crate::abort!(concat!(stringify!($name), " not found in dll"));
            }
        );
        core::mem::transmute::<WinapiFn, $typ>(addr)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn warning_test() {
        warning!("Example warn");
        warning!("Formatted msg: 2 + 2 = {}", 2 + 2);
    }

    #[test]
    fn colored_test() {
        let colored = colored!("<bold><italic>String<reset>");
        let ansi = "\x1b[1m\x1b[3mString\x1b[0m";
        assert_eq!(colored, ansi);
    }

    #[test]
    fn multi_string_test() {
        let multi = multi_string!(
            "Multi",
            "String",
            "Test"
        );
        let string = "Multi\nString\nTest";
        assert_eq!(multi, string);
    }
}