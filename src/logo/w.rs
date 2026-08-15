use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static W: OnceLock<[LogoInfo; 8]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 8] {
    W.get_or_init(|| {
        [
            LogoInfo {
                names: &["WiiLinuxNgx", "WiiLinux", "Wii-Linux", "Wii Linux"],
                lines: include_str!("w/wii_linux.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Windows Server 2025"],
                lines: include_str!("w/windows_2025.txt"),
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
                names: &["Windows 11", "Windows Server 2022"],
                lines: include_str!("w/windows_11.txt"),
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
                names: &["Windows 11_small"],
                lines: include_str!("w/windows_11_small.txt"),
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
                names: &["Windows 8", "Windows 8.1", "Windows 10", "Windows Server 2012", "Windows Server 2012 R2", "Windows Server 2016", "Windows Server 2019"],
                lines: include_str!("w/windows_8.txt"),
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
                names: &["Windows", "Windows 7", "Windows Server 2008", "Windows Server 2008 R2"],
                lines: include_str!("w/windows.txt"),
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
                names: &["Windows 95", "Windows 9x"],
                lines: include_str!("w/windows_95.txt"),
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
                names: &["WolfOS"],
                lines: include_str!("w/wolfos.txt"),
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