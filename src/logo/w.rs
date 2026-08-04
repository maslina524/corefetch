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
            // Windows2025
            LogoInfo {
                names: vec!["Windows Server 2025"],
                lines: include_str!("w/windows_2025.txt"),
                colors: vec![
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            // Windows11
            LogoInfo {
                names: vec!["Windows 11", "Windows Server 2022"],
                lines: include_str!("w/windows_11.txt"),
                colors: vec![
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            // Windows11Small
            LogoInfo {
                names: vec!["Windows 11_small"],
                lines: include_str!("w/windows_11_small.txt"),
                colors: vec![
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_BLUE
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            // Windows8
            LogoInfo {
                names: vec![
                    "Windows 8", "Windows 8.1", "Windows 10", "Windows Server 2012",
                    "Windows Server 2012 R2", "Windows Server 2016", "Windows Server 2019"
                ],
                lines: include_str!("w/windows_8.txt"),
                colors: vec![
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            // Windows
            LogoInfo {
                names: vec![
                    "Windows", "Windows 7",
                    "Windows Server 2008", "Windows Server 2008 R2"
                ],
                lines: include_str!("w/windows.txt"),
                colors: vec![
                    color::FG_RED,
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_YELLOW
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_GREEN,
            },
            // Windows95
            LogoInfo {
                names: vec!["Windows 95", "Windows 9x"],
                lines: include_str!("w/windows_95.txt"),
                colors: vec![
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
        ]
    })
}