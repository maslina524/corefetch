use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static K: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    K.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["kaisen"],
                lines: include_str!("k/kaisen.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kali"],
                lines: include_str!("k/kali.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kali small"],
                lines: include_str!("k/kali_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kalpa-desktop"],
                lines: include_str!("k/kalpa_desktop.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kaos"],
                lines: include_str!("k/kaos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kernelos"],
                lines: include_str!("k/kernelos.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kdelinux", "kde-linux"],
                lines: include_str!("k/kdelinux.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kde neon"],
                lines: include_str!("k/kdeneon.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kibaos"],
                lines: include_str!("k/kibaos.txt"),
                colors: &[
                    color::BG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kibojoe"],
                lines: include_str!("k/kibojoe.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kiss"],
                lines: include_str!("k/kiss.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["kiss2"],
                lines: include_str!("k/kiss2.txt"),
                colors: &[
                    color::FG_BLACK,
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["kogaion"],
                lines: include_str!("k/kogaion.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["korora"],
                lines: include_str!("k/korora.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["krassos", "krass"],
                lines: include_str!("k/krassos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kslinux"],
                lines: include_str!("k/kslinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["kubuntu", "kubuntu-linux", "kde-ubuntu"],
                lines: include_str!("k/kubuntu.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["kylin"],
                lines: include_str!("k/kylin.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
        ]
    })
}