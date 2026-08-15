use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static X: OnceLock<[LogoInfo; 10]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 10] {
    X.get_or_init(|| {
        [
            LogoInfo {
                names: &["XCP-ng", "xenenterprise"],
                lines: include_str!("x/xcp_ng.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_RED,
                    color::FG_BLACK,
                    color::FG_BLACK,
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Xenia"],
                lines: include_str!("x/xenia.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Xenia_old"],
                lines: include_str!("x/xenia_old.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["XeroArch"],
                lines: include_str!("x/xeroarch.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Xferience"],
                lines: include_str!("x/xferience.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Xubuntu"],
                lines: include_str!("x/xubuntu.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Xray_OS"],
                lines: include_str!("x/xray_os.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Xinux"],
                lines: include_str!("x/xinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["XJ380"],
                lines: include_str!("x/xj380.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Ximper"],
                lines: include_str!("x/ximper.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}