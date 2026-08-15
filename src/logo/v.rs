use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static V: OnceLock<[LogoInfo; 13]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 13] {
    V.get_or_init(|| {
        [
            LogoInfo {
                names: &["Valhalla", "valhallaos", "valhalla-linux"],
                lines: include_str!("v/valhalla.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["vanilla"],
                lines: include_str!("v/vanilla.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["vanilla2"],
                lines: include_str!("v/vanilla2.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["vanilla_small"],
                lines: include_str!("v/vanilla_small.txt"),
                colors: &[
                    color::FG_LIGHT_YELLOW,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Venom"],
                lines: include_str!("v/venom.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Venom_small"],
                lines: include_str!("v/venom_small.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["VincentOS"],
                lines: include_str!("v/vincentos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Vnux"],
                lines: include_str!("v/vnux.txt"),
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
                names: &["Vzlinux"],
                lines: include_str!("v/vzlinux.txt"),
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
                lines: include_str!("v/void.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void_small"],
                lines: include_str!("v/void_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void2"],
                lines: include_str!("v/void2.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_DEFAULT,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["void2_small"],
                lines: include_str!("v/void2_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_GREEN,
            },
        ]
    })
}