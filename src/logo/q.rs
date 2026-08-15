use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static Q: OnceLock<[LogoInfo; 8]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 8] {
    Q.get_or_init(|| {
        [
            LogoInfo {
                names: &["qts"],
                lines: include_str!("q/qts.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Quasar"],
                lines: include_str!("q/quasar.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Q4OS"],
                lines: include_str!("q/q4os.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Qubes"],
                lines: include_str!("q/qubes.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Qubes_small"],
                lines: include_str!("q/qubes_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Qubyt"],
                lines: include_str!("q/qubyt.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Quibian"],
                lines: include_str!("q/quibian.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Quirinux"],
                lines: include_str!("q/quirinux.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}