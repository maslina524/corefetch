use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static D: OnceLock<[LogoInfo; 15]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 15] {
    D.get_or_init(|| {
        [
            LogoInfo {
                names: &["dahliaOS"],
                lines: include_str!("d/dahlia.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["DarkOS"],
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
                names: &["Debian"],
                lines: include_str!("d/debian.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Debian_small"],
                lines: include_str!("d/debian_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Deepin"],
                lines: include_str!("d/deepin.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["DesaOS"],
                lines: include_str!("d/desaos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Devuan"],
                lines: include_str!("d/devuan.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Devuan_small"],
                lines: include_str!("d/devuan_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["DietPi"],
                lines: include_str!("d/dietpi.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["DracOS"],
                lines: include_str!("d/dracos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["DragonFly"],
                lines: include_str!("d/dragonfly.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["DragonFly_small"],
                lines: include_str!("d/dragonfly_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["DragonFly_old"],
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
                names: &["DraugerOS", "Drauger"],
                lines: include_str!("d/drauger.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Droidian"],
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