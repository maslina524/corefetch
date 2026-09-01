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

#[cfg(target_os = "windows")]
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
        core::mem::transmute::<ApiBaseFn, $typ>(addr)
    }};
}

#[cfg(target_os = "linux")]
#[macro_export]
macro_rules! get_fn {
    ($handle:tt, $name:expr, $typ:ident) => {{
        // SAFETY: Completely safe
        let addr = $crate::linux::libc::dlsym($handle, $name.as_ptr().cast());
        if addr.is_null() {
            unload($handle);
            $crate::abort!(concat!(stringify!($name), " not found in library"));
        }
        core::mem::transmute::<ApiBaseFn, $typ>(addr)
    }};
}


#[macro_export]
/// Copied from `cfg_if`
/// 
/// Source link: <https://docs.rs/cfg-if/latest/src/cfg_if/lib.rs.html#1-212>
macro_rules! cfg_if {
    (
        if #[cfg( $($i_meta:tt)+ )] { $( $i_tokens:tt )* }
        $(
            else if #[cfg( $($ei_meta:tt)+ )] { $( $ei_tokens:tt )* }
        )*
        $(
            else { $( $e_tokens:tt )* }
        )?
    ) => {
        $crate::cfg_if! {
            @__items () ;
            (( $($i_meta)+ ) ( $( $i_tokens )* )),
            $(
                (( $($ei_meta)+ ) ( $( $ei_tokens )* )),
            )*
            $(
                (() ( $( $e_tokens )* )),
            )?
        }
    };

    (@__items ( $( ($($_:tt)*) , )* ) ; ) => {};
    (
        @__items ( $( ($($no:tt)+) , )* ) ;
        (( $( $($yes:tt)+ )? ) ( $( $tokens:tt )* )),
        $( $rest:tt , )*
    ) => {
        #[cfg(all(
            $( $($yes)+ , )?
            not(any( $( $($no)+ ),* ))
        ))]

        $crate::cfg_if! { @__temp_group $( $tokens )* }

        $crate::cfg_if! {
            @__items ( $( ($($no)+) , )* $( ($($yes)+) , )? ) ;
            $( $rest , )*
        }
    };

    (@__temp_group $( $tokens:tt )* ) => {
        $( $tokens )*
    };
}

// ------------- PRINTS -------------
#[macro_export]
macro_rules! print {
    () => {{}};
    ($($tt:tt)*) => {{
        let handle = $crate::imp::io::stdout();
        let s = $crate::format!($($tt)*);
        $crate::imp::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        let handle = $crate::imp::io::stdout();
        $crate::imp::io::write(handle, "\n");
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::imp::io::stdout();
        let s = $crate::formatln!($($tt)*);
        $crate::imp::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! eprint {
    () => {{}};
    ($expr:expr) => {{
        let handle = $crate::imp::io::stderr();
        let s = $crate::format!("{}", $expr);
        $crate::imp::io::write(handle, s.as_str());
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::imp::io::stderr();
        let s = $crate::format!($($tt)*);
        $crate::imp::io::write(handle, s.as_str());
    }}
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        let handle = $crate::imp::io::stderr();
        $crate::imp::io::write(handle, "\n");
    }};
    ($($tt:tt)*) => {{
        let handle = $crate::imp::io::stderr();
        let s = $crate::formatln!($($tt)*);
        $crate::imp::io::write(handle, s.as_str());
    }}
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! dbg {
    () => {
        $crate::eprintln!("[{}:{}:{}]", core::file!(), core::line!(), core::column!())
    };
    ($flag:ident) => {
        if $crate::imp::env::contains_in_dbg_args(stringify!($flag)) {
            $crate::dbg!();
        }
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    column!(),
                    stringify!($val),
                    &&tmp as &dyn core::fmt::Debug,
                );
                tmp
            }
        }
    };
    ($val:expr $(,)?, $flag:ident) => {
        if $crate::imp::env::contains_in_dbg_args(stringify!($flag)) {
            $crate::dbg!($val)
        } else {
            $val
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+)
    };
    ($($val:expr),+ $(,)?, $flag:ident) => {
        if $crate::imp::env::contains_in_dbg_args(stringify!($flag)) {
            ($($crate::dbg!($val)),+)
        } else {
            ($($val),+)
        }
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! dbg {
    () => {};
    ($flag:ident) => {};
    ($val:expr $(,)?) => {};
    ($val:expr $(,)?, $flag:ident) => {};
    ($($val:expr),+ $(,)?) => {};
    ($($val:expr),+ $(,)?, $flag:ident) => {};
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