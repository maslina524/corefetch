use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static T: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    T.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["t2", "t2 sde", "t2/linux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/t2.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["t2 small", "t2 sde small", "t2/linux small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/t2_small.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["tails"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/tails.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["tatra"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/tatra.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["tearch"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/tearch.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["templeos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/templeos.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["tileos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/tileos.txt")),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["torizon os", "torizoncore"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/torizoncore.txt")),
                colors: &[
                    color::FG_LIGHT_WHITE,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["trisquel"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/trisquel.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["truenas-scale"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/truenas_scale.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["turkish"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/turkish.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["tuxedo os", "tuxedo"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/tuxedo_os.txt")),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["twister"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/t/twister.txt")),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}