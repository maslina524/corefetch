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
                lines: include_bytes!("../../temp/b/bedrock.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bedrock small"],
                lines: include_bytes!("../../temp/b/bedrock_small.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["berserkarch"],
                lines: include_bytes!("../../temp/b/berserkarch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["biglinux"],
                lines: include_bytes!("../../temp/b/biglinux.txt"),
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
                lines: include_bytes!("../../temp/b/bitrig.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["blackarch"],
                lines: include_bytes!("../../temp/b/blackarch.txt"),
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
                lines: include_bytes!("../../temp/b/blackmesa.txt"),
                colors: &[
                    color::FG_BLACK,
                ],
                color_keys: color::FG_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["blackpanther"],
                lines: include_bytes!("../../temp/b/blackpanther.txt"),
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
                lines: include_bytes!("../../temp/b/blag.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["blankon"],
                lines: include_bytes!("../../temp/b/blankon.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bluelight"],
                lines: include_bytes!("../../temp/b/bluelight.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["bodhi"],
                lines: include_bytes!("../../temp/b/bodhi.txt"),
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
                lines: include_bytes!("../../temp/b/bonsai.txt"),
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
                lines: include_bytes!("../../temp/b/bredos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["bsd"],
                lines: include_bytes!("../../temp/b/bsd.txt"),
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
                lines: include_bytes!("../../temp/b/bunsenlabs.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}