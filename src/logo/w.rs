use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static W: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    W.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["wiilinuxngx", "wiilinux", "wii-linux", "wii linux"],
                lines: include_bytes!("../../temp/w/wii_linux.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["windows server 2025"],
                lines: include_bytes!("../../temp/w/windows_2025.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["windows 11", "windows server 2022"],
                lines: include_bytes!("../../temp/w/windows_11.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["windows 11 small"],
                lines: include_bytes!("../../temp/w/windows_11_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["windows 8", "windows 8.1", "windows 10", "windows server 2012", "windows server 2012 r2", "windows server 2016", "windows server 2019"],
                lines: include_bytes!("../../temp/w/windows_8.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["windows", "windows 7", "windows server 2008", "windows server 2008 r2"],
                lines: include_bytes!("../../temp/w/windows.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["windows 95", "windows 9x"],
                lines: include_bytes!("../../temp/w/windows_95.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLUE,
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_RED,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["wolfos"],
                lines: include_bytes!("../../temp/w/wolfos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}