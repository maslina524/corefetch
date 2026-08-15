use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static D: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    D.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["dahliaos"],
                lines: include_str!("d/dahlia.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["darkos"],
                lines: include_str!("d/darkos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_CYAN,
                    color::FG_MAGENTA,
                    color::FG_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["debian"],
                lines: include_str!("d/debian.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["debian small"],
                lines: include_str!("d/debian_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["deepin"],
                lines: include_str!("d/deepin.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["desaos"],
                lines: include_str!("d/desaos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["devuan"],
                lines: include_str!("d/devuan.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["devuan small"],
                lines: include_str!("d/devuan_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["dietpi"],
                lines: include_str!("d/dietpi.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["dracos"],
                lines: include_str!("d/dracos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["dragonfly"],
                lines: include_str!("d/dragonfly.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["dragonfly small"],
                lines: include_str!("d/dragonfly_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["dragonfly old"],
                lines: include_str!("d/dragonfly_old.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["draugeros", "drauger"],
                lines: include_str!("d/drauger.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["droidian"],
                lines: include_str!("d/droidian.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_LIGHT_GREEN,
            },
        ]
    })
}