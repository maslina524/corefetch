use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static N: OnceLock<[LogoInfo; 21]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 21] {
    N.get_or_init(|| {
        [
            LogoInfo {
                names: &["Namib"],
                lines: include_str!("n/namib.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NebiOS"],
                lines: include_str!("n/nebios.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Nekos"],
                lines: include_str!("n/nekos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Neptune"],
                lines: include_str!("n/neptune.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NetRunner"],
                lines: include_str!("n/netrunner.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nexalinux"],
                lines: include_str!("n/nexalinux.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Nitrux"],
                lines: include_str!("n/nitrux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NixOS"],
                lines: include_str!("n/nixos.txt"),
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
                names: &["NixOS_small"],
                lines: include_str!("n/nixos_small.txt"),
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
                names: &["nixos_old"],
                lines: include_str!("n/nixos_old.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nixos_old_small"],
                lines: include_str!("n/nixos_old_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NixOS2"],
                lines: include_str!("n/nixos2.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NetBSD"],
                lines: include_str!("n/netbsd.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NetBSD2"],
                lines: include_str!("n/netbsd2.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NetBSD_small"],
                lines: include_str!("n/netbsd_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nobara"],
                lines: include_str!("n/nobara.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["nomadbsd"],
                lines: include_str!("n/nomadbsd.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NurOS"],
                lines: include_str!("n/nuros.txt"),
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
                names: &["Nurunner"],
                lines: include_str!("n/nurunner.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NuTyX"],
                lines: include_str!("n/nutyx.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["NetHydra"],
                lines: include_str!("n/nethydra.txt"),
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