use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static V: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    V.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["valhalla", "valhallaos", "valhalla-linux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/valhalla.txt")),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["vanilla"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vanilla.txt")),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["vanilla2"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vanilla2.txt")),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["vanilla small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vanilla_small.txt")),
                colors: &[
                    color::FG_LIGHT_YELLOW,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["venom"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/venom.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["venom small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/venom_small.txt")),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["vincentos"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vincentos.txt")),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["vnux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vnux.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["vzlinux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/vzlinux.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["void"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/void.txt")),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/void_small.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void2"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/void2.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_DEFAULT,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void2 small"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/v/void2_small.txt")),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
        ]
    })
}