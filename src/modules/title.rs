use alloc::string::String;

use crate::{
    detect::title::TitleInfo,
    format_for_module,
    impl_display_for_module,
    modules::Module,
    imp::path::Path,
    sync::OnceLock
};

static TITLE: OnceLock<Title> = OnceLock::new();

#[derive(Debug)]
pub struct Title {
    pub user_name: String,
    pub host_name: String,
    pub home_dir: Path,
    pub exe_path: Path,
    pub user_shell: Path,
    pub user_name_colored: String,
    pub at_symbol_colored: &'static str,
    pub host_name_colored: String,
    pub full_user_name: String,
    pub user_id: String,
    pub pid: u32,
    pub cwd: Path
}

impl Module for Title {
    fn new() -> Self {
        let info = TitleInfo::new();
        Self { 
            user_name: info.user_name,
            host_name: info.host_name,
            home_dir: info.home_dir,
            exe_path: info.exe_path,
            user_shell: info.user_shell,
            user_name_colored: info.user_name_colored,
            at_symbol_colored: "@",
            host_name_colored: info.host_name_colored,
            full_user_name: info.full_user_name,
            user_id: info.user_id,
            pid: info.pid,
            cwd: info.cwd
        }
    }

    fn get() -> &'static Self {
        TITLE.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        ""
    }

    fn title(&self) -> &'static str {
        "{user-name-colored}{at-symbol-colored}{host-name-colored}"
    }

    fn string_name(&self) -> &'static str {
        "title"
    }

    format_for_module!(
        Title,
        user_name, host_name, home_dir, exe_path, 
        user_shell, user_name_colored, at_symbol_colored, host_name_colored, 
        full_user_name, user_id, pid, cwd
    );
}

impl_display_for_module!(Title);