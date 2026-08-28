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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macaronios.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["macos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macos.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macos_small.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macos2.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macos2_small.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/macos3.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mainsailos.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mainsailos small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mainsailos_small.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mageia"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mageia.txt")),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mageia small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mageia_small.txt")),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mandriva", "mandrake"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mandriva.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["manjaro", "manjaro-arm"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/manjaro.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["manjaro small", "manjaro-arm small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/manjaro_small.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["massos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/massos.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["matuusos", "matuus"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/matuusos.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_RED,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["maui"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/maui.txt")),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mauna"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mauna.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["meowix"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/meowix.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mer.txt")),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midnightbsd"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/midnightbsd.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/midos.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["midos old"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/midos_old.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["minimal system"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/minimal.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["minix"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/minix.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/miracle_linux.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["mos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mos.txt")),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["msys2"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/msys2.txt")),
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
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mx.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["mx small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mx_small.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["mx2"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/m/mx2.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
        ]
    })
}