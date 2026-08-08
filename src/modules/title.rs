use alloc::{
    string::String,
    borrow::ToOwned
};

use crate::{
    detect::title, format, format_for_module, format_for_module_wo_key, impl_display_for_module, modules::Module, os::{env, path::Path}, sync::OnceLock
};

static TITLE: OnceLock<Title> = OnceLock::new();

#[derive(Debug)]
pub struct Title {
    pub user_name: String,
    pub host_name: String,
    pub home_dir: Path,
    pub exe_path: String,
    pub user_shell: String,
    pub user_name_colored: String,
    pub at_symbol_colored: &'static str,
    pub host_name_colored: String,
    pub full_user_name: String,
    pub user_id: String,
    pub pid: u32,
    pub cwd: String
}

impl Module for Title {
    fn new() -> Self {
        let ver = env::os_version();
        let pretty_name = format!("{} {} ({})", ver.name, ver.version, ver.codename);
        let id = format!("{} {}", ver.name, ver.version);
        Self { 
            user_name: title::user_name().to_owned(),
            host_name: title::host_name().to_owned(),
            home_dir: title::home_dir(),
            exe_path: title::exe_path(),
            user_shell: title::user_shell(),
            user_name_colored: title::colored_user_name(),
            at_symbol_colored: "@",
            host_name_colored: title::colored_host_name(),
            full_user_name: title::full_user_name(),
            user_id: title::user_id(),
            pid: title::pid(),
            cwd: title::cwd()
        }
    }

    fn get() -> &'static Self {
        TITLE.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Title"
    }

    fn title(&self) -> &'static str {
        "{user-name-colored}{at-symbol-colored}{host-name-colored}"
    }

    fn string_name(&self) -> &'static str {
        "title"
    }

    format_for_module_wo_key!(
        Title,
        user_name, host_name, home_dir, exe_path, 
        user_shell, user_name_colored, at_symbol_colored, host_name_colored, 
        full_user_name, user_id, pid, cwd
    );
}

impl_display_for_module!(Title);

