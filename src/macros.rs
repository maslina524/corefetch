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
macro_rules! warning {
    ($($args:tt)*) => {{
        $crate::eprintln!("\x1b[1;{}mwarning\x1b[0m: ", $crate::color::FG_YELLOW);
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

#[cfg(test)]
mod tests {
    use crate::warning;

    #[test]
    fn warning_test() {
        warning!("Example warn");
        warning!("Formatted msg: 2 + 2 = {}", 2 + 2);
    }
}