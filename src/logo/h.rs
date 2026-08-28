use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static H: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    H.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["haiku"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/haiku.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["haiku2"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/haiku2.txt")),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["haiku small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/haiku_small.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["hamonikr"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hamonikr.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["hardclanz"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hardclanz.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["hardenedbsd"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/f/freebsd.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["harmonyos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/harmonyos.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["hash"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hash.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["heliumos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/heliumos.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["huawei cloud euleros", "hce"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hce.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["huayra"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/huayra.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["hybrid"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hybrid.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_LIGHT_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["hydroos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hydroos.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["hyperbola"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hyperbola.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["hyperbola small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/h/hyperbola_small.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}