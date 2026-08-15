use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static B: OnceLock<[LogoInfo; 16]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 16] {
    B.get_or_init(|| {
        [
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
                names: &["bedrock_small"],
                lines: include_str!("b/bedrock_small.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BerserkArch"],
                lines: include_str!("b/berserkarch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BigLinux"],
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
                names: &["Bitrig"],
                lines: include_str!("b/bitrig.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["Blackarch"],
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
                names: &["BlackMesa", "black-mesa"],
                lines: include_str!("b/blackmesa.txt"),
                colors: &[
                    color::FG_BLACK,
                ],
                color_keys: color::FG_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BlackPanther"],
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
                names: &["BLAG"],
                lines: include_str!("b/blag.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BlankOn"],
                lines: include_str!("b/blankon.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BlueLight"],
                lines: include_str!("b/bluelight.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Bodhi"],
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
                names: &["Bonsai"],
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
                names: &["Bredos"],
                lines: include_str!("b/bredos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["BSD"],
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
                names: &["BunsenLabs"],
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