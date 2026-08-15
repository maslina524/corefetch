use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static B: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    B.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["bedrock"],
                lines: include_str!("b/bedrock.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bedrock small"],
                lines: include_str!("b/bedrock_small.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["berserkarch"],
                lines: include_str!("b/berserkarch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["biglinux"],
                lines: include_str!("b/biglinux.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["bitrig"],
                lines: include_str!("b/bitrig.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["blackarch"],
                lines: include_str!("b/blackarch.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_LIGHT_RED,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_LIGHT_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["blackmesa", "black-mesa"],
                lines: include_str!("b/blackmesa.txt"),
                colors: &[
                    color::FG_BLACK,
                ],
                color_keys: color::FG_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["blackpanther"],
                lines: include_str!("b/blackpanther.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["blag"],
                lines: include_str!("b/blag.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["blankon"],
                lines: include_str!("b/blankon.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bluelight"],
                lines: include_str!("b/bluelight.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["bodhi"],
                lines: include_str!("b/bodhi.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["bonsai"],
                lines: include_str!("b/bonsai.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_GREEN,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["bredos"],
                lines: include_str!("b/bredos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bsd"],
                lines: include_str!("b/bsd.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_BLUE,
                    color::FG_YELLOW,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bunsenlabs"],
                lines: include_str!("b/bunsenlabs.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}