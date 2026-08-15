use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static R: OnceLock<[LogoInfo; 23]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 23] {
    R.get_or_init(|| {
        [
            LogoInfo {
                names: &["Radix"],
                lines: include_str!("r/radix.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["raspbian"],
                lines: include_str!("r/raspbian.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["raspbian_small"],
                lines: include_str!("r/raspbian_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["RavynOS"],
                lines: include_str!("r/ravynos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["RebornOS"],
                lines: include_str!("r/rebornos.txt"),
                colors: &[
                    color::FG_BLACK,
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["RebornOS_small"],
                lines: include_str!("r/rebornos_small.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["RedCore"],
                lines: include_str!("r/redcore.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rhel", "redhat"],
                lines: include_str!("r/rhel.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["rhel_small", "redhat_small"],
                lines: include_str!("r/rhel_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["rhel_old", "redhat_old"],
                lines: include_str!("r/rhel_old.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["RedOS"],
                lines: include_str!("r/redos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["RedOS_small"],
                lines: include_str!("r/redos_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Redstar"],
                lines: include_str!("r/redstar.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Redrose"],
                lines: include_str!("r/redrose.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Refracta"],
                lines: include_str!("r/refracta.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Regata"],
                lines: include_str!("r/regata.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Regolith"],
                lines: include_str!("r/regolith.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["RhaymOS"],
                lines: include_str!("r/rhaymos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rocky"],
                lines: include_str!("r/rocky.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["rocky_small"],
                lines: include_str!("r/rocky_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["ROSA"],
                lines: include_str!("r/rosa.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Rhino Linux"],
                lines: include_str!("r/rhino.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_LIGHT_BLUE,
                    color::FG_LIGHT_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["RengeOS"],
                lines: include_str!("r/rengeos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}