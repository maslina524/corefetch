use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static F: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    F.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["fastfetch", "ff"],
                lines: include_str!("f/fastfetch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                    color::FG_MAGENTA,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["fedora"],
                lines: include_str!("f/fedora.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora-asahi-remix"],
                lines: include_str!("a/asahi.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_RED,
                    color::FG_LIGHT_BLACK,
                    color::FG_WHITE,
                    color::FG_CYAN,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["fedora small"],
                lines: include_str!("f/fedora_small.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora2 small"],
                lines: include_str!("f/fedora2_small.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora old"],
                lines: include_str!("f/fedora_old.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora-silverblue"],
                lines: include_str!("f/fedora_silverblue.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora-kinoite"],
                lines: include_str!("f/fedora_kinoite.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora-sericea"],
                lines: include_str!("f/fedora_sericea.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["fedora-coreos"],
                lines: include_str!("f/fedora_coreos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["femboyos"],
                lines: include_str!("f/femboyos.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["feren"],
                lines: include_str!("f/feren.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["flatcar"],
                lines: include_str!("f/flatcar.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["filotimo"],
                lines: include_str!("f/filotimo.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["finnix"],
                lines: include_str!("f/finnix.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["floflis"],
                lines: include_str!("f/floflis.txt"),
                colors: &[
                    color::FG_LIGHT_CYAN,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["freebsd"],
                lines: include_str!("f/freebsd.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["freebsd small"],
                lines: include_str!("f/freebsd_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["freemint"],
                lines: include_str!("f/freemint.txt"),
                colors: &[
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["frugalware", "frugalware-linux"],
                lines: include_str!("f/frugalware.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["funtoo"],
                lines: include_str!("f/funtoo.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}