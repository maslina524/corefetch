use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static H: OnceLock<[LogoInfo; 15]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 15] {
    H.get_or_init(|| {
        [
            LogoInfo {
                names: &["Haiku"],
                lines: include_str!("h/haiku.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["Haiku2"],
                lines: include_str!("h/haiku2.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["Haiku_small"],
                lines: include_str!("h/haiku_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["HamoniKR"],
                lines: include_str!("h/hamonikr.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["HarDClanZ"],
                lines: include_str!("h/hardclanz.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["HardenedBSD"],
                lines: include_str!("f/freebsd.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["HarmonyOS"],
                lines: include_str!("h/harmonyos.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Hash"],
                lines: include_str!("h/hash.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["HeliumOS"],
                lines: include_str!("h/heliumos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Huawei Cloud EulerOS", "hce"],
                lines: include_str!("h/hce.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Huayra"],
                lines: include_str!("h/huayra.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Hybrid"],
                lines: include_str!("h/hybrid.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_LIGHT_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["HydroOS"],
                lines: include_str!("h/hydroos.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["Hyperbola"],
                lines: include_str!("h/hyperbola.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Hyperbola_small"],
                lines: include_str!("h/hyperbola_small.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}