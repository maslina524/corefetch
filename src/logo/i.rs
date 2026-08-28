use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static I: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    I.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["iglunix", "iglu"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/iglunix.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["instantos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/instantos.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["interix"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/interix.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_BLUE,
                    color::FG_BLACK,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["irix"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/irix.txt")),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ironclad"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/ironclad.txt")),
                colors: &[
                    color::FG_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["itc"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/i/itc.txt")),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
        ]
    })
}