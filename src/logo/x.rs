use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static X: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    X.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["xcp-ng", "xenenterprise"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xcp_ng.txt")),
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
                names: &["xenia"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xenia.txt")),
                colors: &[
                    color::FG_RED,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["xenia old"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xenia_old.txt")),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["xeroarch"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xeroarch.txt")),
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
                names: &["xferience"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xferience.txt")),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["xubuntu"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xubuntu.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["xray os"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xray_os.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["xinux"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xinux.txt")),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["xj380"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/xj380.txt")),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ximper"],
                lines: include_bytes!(concat!(env!("LOGO_OUT_DIR"), "/temp/x/ximper.txt")),
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