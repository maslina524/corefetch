use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static M: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    M.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["macaroni"],
                lines: include_str!("m/macaronios.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos"],
                lines: include_str!("m/macos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos small"],
                lines: include_str!("m/macos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos2"],
                lines: include_str!("m/macos2.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos2 small"],
                lines: include_str!("m/macos2_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos3"],
                lines: include_str!("m/macos3.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["mainsailos"],
                lines: include_str!("m/mainsailos.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mainsailos small"],
                lines: include_str!("m/mainsailos_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mageia"],
                lines: include_str!("m/mageia.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mageia small"],
                lines: include_str!("m/mageia_small.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mandriva", "mandrake"],
                lines: include_str!("m/mandriva.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["manjaro", "manjaro-arm"],
                lines: include_str!("m/manjaro.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["manjaro small", "manjaro-arm small"],
                lines: include_str!("m/manjaro_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["massos"],
                lines: include_str!("m/massos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["matuusos", "matuus"],
                lines: include_str!("m/matuusos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_RED,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["maui"],
                lines: include_str!("m/maui.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mauna"],
                lines: include_str!("m/mauna.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["meowix"],
                lines: include_str!("m/meowix.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["mer"],
                lines: include_str!("m/mer.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midnightbsd"],
                lines: include_str!("m/midnightbsd.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midos"],
                lines: include_str!("m/midos.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midos old"],
                lines: include_str!("m/midos_old.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["minimal system"],
                lines: include_str!("m/minimal.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["minix"],
                lines: include_str!("m/minix.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["miraclelinux"],
                lines: include_str!("m/miracle_linux.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mos"],
                lines: include_str!("m/mos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["msys2"],
                lines: include_str!("m/msys2.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["mx"],
                lines: include_str!("m/mx.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["mx small"],
                lines: include_str!("m/mx_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["mx2"],
                lines: include_str!("m/mx2.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
        ]
    })
}