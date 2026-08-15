use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static N: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    N.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["namib"],
                lines: include_bytes!("../../temp/n/namib.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nebios"],
                lines: include_bytes!("../../temp/n/nebios.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nekos"],
                lines: include_bytes!("../../temp/n/nekos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["neptune"],
                lines: include_bytes!("../../temp/n/neptune.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["netrunner"],
                lines: include_bytes!("../../temp/n/netrunner.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nexalinux"],
                lines: include_bytes!("../../temp/n/nexalinux.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nitrux"],
                lines: include_bytes!("../../temp/n/nitrux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos"],
                lines: include_bytes!("../../temp/n/nixos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos small"],
                lines: include_bytes!("../../temp/n/nixos_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos old"],
                lines: include_bytes!("../../temp/n/nixos_old.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos old small"],
                lines: include_bytes!("../../temp/n/nixos_old_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos2"],
                lines: include_bytes!("../../temp/n/nixos2.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["netbsd"],
                lines: include_bytes!("../../temp/n/netbsd.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["netbsd2"],
                lines: include_bytes!("../../temp/n/netbsd2.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["netbsd small"],
                lines: include_bytes!("../../temp/n/netbsd_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nobara"],
                lines: include_bytes!("../../temp/n/nobara.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nomadbsd"],
                lines: include_bytes!("../../temp/n/nomadbsd.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nuros"],
                lines: include_bytes!("../../temp/n/nuros.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nurunner"],
                lines: include_bytes!("../../temp/n/nurunner.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nutyx"],
                lines: include_bytes!("../../temp/n/nutyx.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nethydra"],
                lines: include_bytes!("../../temp/n/nethydra.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}