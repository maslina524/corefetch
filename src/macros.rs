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
    ($lit:literal) => {{
        eprintln!(
            concat!("\x1b[1;{}mwarning\x1b[0m: ", $lit),
            $crate::color::FG_YELLOW
        );
    }};
    ($lit:literal, $($args:tt)*) => {{
        eprintln!(
            concat!("\x1b[1;{}mwarning\x1b[0m: ", $lit),
            $crate::color::FG_YELLOW,
            $($args)*
        );
    }};
}

#[macro_export]
macro_rules! abort {
    ($lit:literal) => {{
        eprintln!(
            concat!("\x1b[1;{}mabort\x1b[0m: ", $lit),
            $crate::color::FG_RED
        );
        $crate::exit(101)
    }};
    ($lit:literal, $($args:tt)*) => {{
        eprintln!(
            concat!("\x1b[1;{}mabort\x1b[0m: ", $lit),
            $crate::color::FG_RED,
            $($args)*
        );
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