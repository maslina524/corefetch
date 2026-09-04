use core::{
    ffi::{CStr, c_char}
};

use alloc::{
    string::{String, ToString},
    vec
};

use crate::{
    abort,
    warning,
    format,
    logo::LogoInfo,
    detect::title::TitleInfo, 
    imp::libc::{getpwuid, getuid, gethostname, getpid, getcwd},
    imp::path::Path,
    imp::fs,
    linux::error::ErrorCode
};

const HOST_NAME_MAX: usize = 64;
const PATH_MAX     : usize = 4096;

impl TitleInfo {
    pub fn new() -> Self {
        let uid = getuid();
        let pw_ptr = getpwuid(uid);
        if pw_ptr.is_null() {
            abort!("Failed to call `getpwuid`: {}", ErrorCode::last());
        }
        // SAFETY: Libc always returns a valid pointer
        let pw = unsafe { &*pw_ptr };

        // SAFETY: libs are guaranteed to store a valid cstr
        let user_name_c_str = unsafe { CStr::from_ptr(pw.pw_name) };
        let user_name = user_name_c_str.to_string_lossy().into_owned();

        let user_id = pw.pw_uid.to_string();
        let pid = getpid() as u32;

        // SAFETY: libs are guaranteed to store a valid cstr
        let home_dir_c_str = unsafe { CStr::from_ptr(pw.pw_dir) };
        let home_dir_string = home_dir_c_str.to_string_lossy().into_owned();
        let home_dir = Path::from(home_dir_string);

        // SAFETY: libs are guaranteed to store a valid cstr
        let user_shell_c_str = unsafe { CStr::from_ptr(pw.pw_shell) };
        let user_shell_string = user_shell_c_str.to_string_lossy().into_owned();
        let user_shell = Path::from(user_shell_string);

        let host_name = Self::host_name();
        let exe_path = Self::exe_path();

        let user_name_colored = Self::colored(&user_name);
        let host_name_colored = Self::colored(&host_name);

        let cwd = Self::cwd();

        Self { 
            user_name, 
            host_name, 
            home_dir, 
            exe_path, 
            user_shell, 
            user_name_colored,
            host_name_colored, 
            full_user_name: String::new(), 
            user_id, 
            pid, cwd 
        }
    }

    fn cwd() -> Path {
        let mut buf = vec![0i8; PATH_MAX];
        let ret = getcwd(buf.as_mut_ptr(), PATH_MAX);
        if ret.is_null() {
            warning!("Failed to call `getcwd`");
            return Path::new();
        }

        // SAFETY: libs are guaranteed to store a valid cstr
        let c_str = unsafe { CStr::from_ptr(buf.as_ptr()) };
        let string = c_str.to_string_lossy().into_owned();
        Path::from(string)
    }

    fn colored(s: &str) -> String {
        format!(
            "\x1b[1;{}m{}\x1b[0m",
            LogoInfo::get().unwrap().color_title,
            s
        )
    }

    fn host_name() -> String {
        let mut buf = [c_char::default(); HOST_NAME_MAX + 1];
        let ret = gethostname(buf.as_mut_ptr(), HOST_NAME_MAX + 1);
        if ret == 0 {
            // SAFETY: libs are guaranteed to store a valid cstr
            let c_str = unsafe { CStr::from_ptr(buf.as_mut_ptr()) };
            c_str.to_string_lossy().into_owned()
        } else {
            warning!("Failed to get host name (gethostname)");
            String::new()
        }
    }

    fn exe_path() -> Path {
        let Some(path_string) = fs::read_link("/proc/self/exe", PATH_MAX) else {
            warning!("Failed to get exe path");
            return Path::new()
        };
        Path::from(path_string)
    } 
}

#[cfg(test)]
mod tests {
    use crate::detect::title::TitleInfo;

    extern crate std;

    #[test]
    fn cwd_test() {
        let cwd = TitleInfo::cwd();
        println!("{cwd}");
    }

    #[test]
    fn host_test() {
        let cwd = TitleInfo::host_name();
        println!("{cwd}");
    }

}