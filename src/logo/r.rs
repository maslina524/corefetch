use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static R: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    R.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["radix"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/radix.txt")),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["raspbian"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/raspbian.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["raspbian small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/raspbian_small.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["ravynos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/ravynos.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rebornos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rebornos.txt")),
                colors: &[
                    color::FG_BLACK,
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["rebornos small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rebornos_small.txt")),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["redcore"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/redcore.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rhel", "redhat"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rhel.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["rhel small", "redhat small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rhel_small.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["rhel old", "redhat old"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rhel_old.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["redos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/redos.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["redos small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/redos_small.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["redstar"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/redstar.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["redrose"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/redrose.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["refracta"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/refracta.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["regata"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/regata.txt")),
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
                names: &["regolith"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/regolith.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rhaymos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rhaymos.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rocky"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rocky.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["rocky small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rocky_small.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["rosa"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rosa.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["rhino linux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rhino.txt")),
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
                names: &["rengeos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/r/rengeos.txt")),
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