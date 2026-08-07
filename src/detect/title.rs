use alloc::string::String;

use crate::{
    os::windows::GetUserNameW,
    os::path::Path,
    os::env,
    sync::OnceLock,
    logo::LogoInfo,
    todo_or,
    format
};

static USER_NAME: OnceLock<String> = OnceLock::new();

const UNLEN     : usize            = 256;

pub fn user_name() -> &'static String {
    USER_NAME.get_or_init(|| {
        let mut buf = [0u16; UNLEN + 1];
        let mut size = (UNLEN + 1) as u32;
        
        // SAFETY: Completely safe
        let ret = unsafe {
            GetUserNameW(
                (&raw mut buf).cast(),
                &raw mut size
            )
        };
        if ret == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&buf)
    })
}

pub fn home_dir() -> Path {
    Path::from("C:/Users").join(user_name())
}

pub fn exe_path() -> String {
    env::args()[0].clone()
}

pub fn user_shell() -> String {
    todo_or!("Will be implemented in the future", String::new())
}

pub fn colored_user_name() -> String {
    format!(
        "{}{}\x1b[0m",
        LogoInfo::get().unwrap().color_title,
        user_name()
    )
}

#[cfg(test)]
mod tests {
    use crate::detect::title;

    extern crate std;

    #[test]
    fn exe_path_test() {
        let path = title::exe_path();
        println!("{path}");
        assert!(path.contains("nofetch"));
    }
}