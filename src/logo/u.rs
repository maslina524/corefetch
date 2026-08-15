use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static U: OnceLock<[LogoInfo; 25]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 25] {
    U.get_or_init(|| {
        [
            LogoInfo {
                names: &["UBLinux"],
                lines: include_str!("u/ublinux.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["UBLinux_small"],
                lines: include_str!("u/ublinux_small.txt"),
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
                lines: include_str!("u/ubuntu.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu_small"],
                lines: include_str!("u/ubuntu_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu_old"],
                lines: include_str!("u/ubuntu_old.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu_old2"],
                lines: include_str!("u/ubuntu_old2.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu_old2_small"],
                lines: include_str!("u/ubuntu_old2_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ubuntu budgie", "ubuntu-budgie"],
                lines: include_str!("u/ubuntu_budgie.txt"),
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
                lines: include_str!("u/ubuntu_cinnamon.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu gnome", "ubuntu-gnome"],
                lines: include_str!("u/ubuntu_gnome.txt"),
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
                lines: include_str!("u/ubuntu_kylin.txt"),
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
                lines: include_str!("u/ubuntu_mate.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu studio", "ubuntu-studio"],
                lines: include_str!("u/ubuntu_studio.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu sway", "ubuntu-sway"],
                lines: include_str!("u/ubuntu_sway.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu touch", "ubuntu-touch"],
                lines: include_str!("u/ubuntu_touch.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ubuntu unity", "ubuntu-unity"],
                lines: include_str!("u/ubuntu_unity.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Ultramarine"],
                lines: include_str!("u/ultramarine.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Ultramarine_small"],
                lines: include_str!("u/ultramarine_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Unifi"],
                lines: include_str!("u/unifi.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Univalent"],
                lines: include_str!("u/univalent.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Univention"],
                lines: include_str!("u/univention.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["UOS"],
                lines: include_str!("u/uos.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["UrukOS"],
                lines: include_str!("u/urukos.txt"),
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
                lines: include_str!("u/uwuntu.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Uzbek"],
                lines: include_str!("u/uzbek.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}