use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static E: OnceLock<[LogoInfo; 19]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 19] {
    E.get_or_init(|| {
        [
            LogoInfo {
                names: &["elbrus"],
                lines: include_str!("e/elbrus.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Elementary"],
                lines: include_str!("e/elementary.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Elementary_small"],
                lines: include_str!("e/elementary_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Elive"],
                lines: include_str!("e/elive.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Emmabuntus"],
                lines: include_str!("e/emmabuntus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ENOS"],
                lines: include_str!("e/enos.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_LIGHT_BLUE,
                color_title: color::FG_LIGHT_BLUE,
            },
            LogoInfo {
                names: &["EncryptOS"],
                lines: include_str!("e/encryptos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["EndeavourOS"],
                lines: include_str!("e/endeavouros.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_RED,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["EndeavourOS_small"],
                lines: include_str!("e/endeavouros_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Endless"],
                lines: include_str!("e/endless.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Enso"],
                lines: include_str!("e/enso.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["EshanizedOS"],
                lines: include_str!("e/eshanizedos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["EuroLinux"],
                lines: include_str!("e/eurolinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["EvolutionOS"],
                lines: include_str!("e/evolutionos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["EvolutionOS_small"],
                lines: include_str!("e/evolutionos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["EvolutionOS_old"],
                lines: include_str!("e/evolutionos_old.txt"),
                colors: &[
                    color::FG_LIGHT_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["eweOS"],
                lines: include_str!("e/eweos.txt"),
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
                names: &["Exherbo"],
                lines: include_str!("e/exherbo.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Exodia"],
                lines: include_str!("e/exodia_predator.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
        ]
    })
}