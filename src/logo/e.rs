use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static E: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    E.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["elbrus"],
                lines: include_bytes!("../../temp/e/elbrus.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["elementary"],
                lines: include_bytes!("../../temp/e/elementary.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["elementary small"],
                lines: include_bytes!("../../temp/e/elementary_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["elive"],
                lines: include_bytes!("../../temp/e/elive.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["emmabuntus"],
                lines: include_bytes!("../../temp/e/emmabuntus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["enos"],
                lines: include_bytes!("../../temp/e/enos.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_LIGHT_BLUE,
                color_title: color::FG_LIGHT_BLUE,
            },
            LogoInfo {
                names: &["encryptos"],
                lines: include_bytes!("../../temp/e/encryptos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["endeavouros"],
                lines: include_bytes!("../../temp/e/endeavouros.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_RED,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["endeavouros small"],
                lines: include_bytes!("../../temp/e/endeavouros_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["endless"],
                lines: include_bytes!("../../temp/e/endless.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["enso"],
                lines: include_bytes!("../../temp/e/enso.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["eshanizedos"],
                lines: include_bytes!("../../temp/e/eshanizedos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["eurolinux"],
                lines: include_bytes!("../../temp/e/eurolinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["evolutionos"],
                lines: include_bytes!("../../temp/e/evolutionos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["evolutionos small"],
                lines: include_bytes!("../../temp/e/evolutionos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["evolutionos old"],
                lines: include_bytes!("../../temp/e/evolutionos_old.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["eweos"],
                lines: include_bytes!("../../temp/e/eweos.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_LIGHT_BLACK,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["exherbo"],
                lines: include_bytes!("../../temp/e/exherbo.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["exodia"],
                lines: include_bytes!("../../temp/e/exodia_predator.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
        ]
    })
}