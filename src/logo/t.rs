use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static T: OnceLock<[LogoInfo; 13]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 13] {
    T.get_or_init(|| {
        [
            LogoInfo {
                names: &["T2", "T2 SDE", "T2/Linux"],
                lines: include_str!("t/t2.txt"),
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
                names: &["T2_small", "T2 SDE_small", "T2/Linux_small"],
                lines: include_str!("t/t2_small.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Tails"],
                lines: include_str!("t/tails.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Tatra"],
                lines: include_str!("t/tatra.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["TeArch"],
                lines: include_str!("t/tearch.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["TempleOS"],
                lines: include_str!("t/templeos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["TileOS"],
                lines: include_str!("t/tileos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Torizon OS", "TorizonCore"],
                lines: include_str!("t/torizoncore.txt"),
                colors: &[
                    color::FG_LIGHT_WHITE,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Trisquel"],
                lines: include_str!("t/trisquel.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["TrueNAS-Scale"],
                lines: include_str!("t/truenas_scale.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Turkish"],
                lines: include_str!("t/turkish.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Tuxedo OS", "tuxedo"],
                lines: include_str!("t/tuxedo_os.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Twister"],
                lines: include_str!("t/twister.txt"),
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