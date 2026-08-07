#[macro_export]
macro_rules! todo_or {
    ($msg:literal, $default:expr) => {{
        #[deprecated(note = $msg)]
        const fn _warn() {}
        _warn();
        $default
    }};
}