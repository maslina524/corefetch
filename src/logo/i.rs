use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static I: OnceLock<[LogoInfo; 6]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 6] {
    I.get_or_init(|| {
        [
            LogoInfo {
                names: &["Iglunix", "Iglu"],
                lines: include_str!("i/iglunix.txt"),
                colors: &[
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_LIGHT_BLACK,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["InstantOS"],
                lines: include_str!("i/instantos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Interix"],
                lines: include_str!("i/interix.txt"),
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
                names: &["IRIX"],
                lines: include_str!("i/irix.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Ironclad"],
                lines: include_str!("i/ironclad.txt"),
                colors: &[
                    color::FG_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Itc"],
                lines: include_str!("i/itc.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
        ]
    })
}