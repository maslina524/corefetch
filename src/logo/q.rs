use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static Q: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    Q.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["qts"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/qts.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["quasar"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/quasar.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["q4os"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/q4os.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["qubes"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/qubes.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["qubes small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/qubes_small.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["qubyt"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/qubyt.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["quibian"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/quibian.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["quirinux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/q/quirinux.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}