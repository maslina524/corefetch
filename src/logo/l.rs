use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static L: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    L.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["lainos"],
                lines: include_str!("l/lainos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["langitketujuh", "l7"],
                lines: include_str!("l/langitketujuh.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["laxeros"],
                lines: include_str!("l/laxeros.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["lede"],
                lines: include_str!("l/lede.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["libreelec"],
                lines: include_str!("l/libreelec.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_WHITE,
                    color::FG_CYAN,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["lilidog"],
                lines: include_str!("l/lilidog.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["lingmo", "lingmoos"],
                lines: include_str!("l/lingmo.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["limeos"],
                lines: include_str!("l/limeos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["linspire", "lindows"],
                lines: include_str!("l/linspire.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["linux", "linux-generic"],
                lines: include_str!("l/linux.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["linuxfromscratch", "lfs"],
                lines: include_str!("l/lfs.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLACK,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLACK,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["linux_small", "linux-generic_small"],
                lines: include_str!("l/linux_small.txt"),
                colors: &[
                    color::FG_BLACK,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["linuxlite"],
                lines: include_str!("l/linuxlite.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["linuxlite_small"],
                lines: include_str!("l/linuxlite_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["linuxmint"],
                lines: include_str!("l/linuxmint.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["linuxmint_small"],
                lines: include_str!("l/linuxmint_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["linuxmint2"],
                lines: include_str!("l/linuxmint2.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["linuxmint2_small"],
                lines: include_str!("l/linuxmint2_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["linuxmint_old"],
                lines: include_str!("l/linuxmint_old.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["live raizo", "live_raizo"],
                lines: include_str!("l/live_raizo.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["lliurex"],
                lines: include_str!("l/lliurex.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["lmde"],
                lines: include_str!("l/lmde.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["locos", "loc-os", "loc-os linux"],
                lines: include_str!("l/locos.txt"),
                colors: &[
                    color::FG_BLACK,
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["lubuntu"],
                lines: include_str!("l/lubuntu.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["lunar"],
                lines: include_str!("l/lunar.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}