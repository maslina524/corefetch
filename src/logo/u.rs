use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static U: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    U.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["ublinux"],
                lines: include_bytes!("../../temp/u/ublinux.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ublinux small"],
                lines: include_bytes!("../../temp/u/ublinux_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu"],
                lines: include_bytes!("../../temp/u/ubuntu.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu small"],
                lines: include_bytes!("../../temp/u/ubuntu_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu old"],
                lines: include_bytes!("../../temp/u/ubuntu_old.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu old2"],
                lines: include_bytes!("../../temp/u/ubuntu_old2.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu old2 small"],
                lines: include_bytes!("../../temp/u/ubuntu_old2_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu budgie", "ubuntu-budgie"],
                lines: include_bytes!("../../temp/u/ubuntu_budgie.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu cinnamon", "ubuntu-cinnamon"],
                lines: include_bytes!("../../temp/u/ubuntu_cinnamon.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu gnome", "ubuntu-gnome"],
                lines: include_bytes!("../../temp/u/ubuntu_gnome.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["ubuntu kylin", "ubuntu-kylin"],
                lines: include_bytes!("../../temp/u/ubuntu_kylin.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu mate", "ubuntu-mate"],
                lines: include_bytes!("../../temp/u/ubuntu_mate.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu studio", "ubuntu-studio"],
                lines: include_bytes!("../../temp/u/ubuntu_studio.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu sway", "ubuntu-sway"],
                lines: include_bytes!("../../temp/u/ubuntu_sway.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu touch", "ubuntu-touch"],
                lines: include_bytes!("../../temp/u/ubuntu_touch.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu unity", "ubuntu-unity"],
                lines: include_bytes!("../../temp/u/ubuntu_unity.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ultramarine"],
                lines: include_bytes!("../../temp/u/ultramarine.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ultramarine small"],
                lines: include_bytes!("../../temp/u/ultramarine_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["unifi"],
                lines: include_bytes!("../../temp/u/unifi.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["univalent"],
                lines: include_bytes!("../../temp/u/univalent.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["univention"],
                lines: include_bytes!("../../temp/u/univention.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["uos"],
                lines: include_bytes!("../../temp/u/uos.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["urukos"],
                lines: include_bytes!("../../temp/u/urukos.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                    color::FG_LIGHT_BLUE,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["uwuntu"],
                lines: include_bytes!("../../temp/u/uwuntu.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["uzbek"],
                lines: include_bytes!("../../temp/u/uzbek.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}