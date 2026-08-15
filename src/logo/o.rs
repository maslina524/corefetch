use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static O: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    O.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["obarun"],
                lines: include_str!("o/obarun.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["obrevenge"],
                lines: include_str!("o/obrevenge.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["obsidianos"],
                lines: include_str!("o/obsidianos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_CYAN,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["omnios"],
                lines: include_str!("o/omnios.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_YELLOW,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openkylin"],
                lines: include_str!("o/openkylin.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openbsd"],
                lines: include_str!("o/openbsd.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                    color::FG_CYAN,
                    color::FG_RED,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openbsd_small"],
                lines: include_str!("o/openbsd_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openeuler"],
                lines: include_str!("o/openeuler.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openindiana"],
                lines: include_str!("o/openindiana.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openmamba"],
                lines: include_str!("o/openmamba.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openruyi"],
                lines: include_str!("o/openruyi.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                    color::FG_LIGHT_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openstage"],
                lines: include_str!("o/openstage.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opensuse"],
                lines: include_str!("o/opensuse.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse_small"],
                lines: include_str!("o/opensuse_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-microos"],
                lines: include_str!("o/opensuse_microos.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opensuse-leap"],
                lines: include_str!("o/opensuse_leap.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-leap_old"],
                lines: include_str!("o/opensuse_leap_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed"],
                lines: include_str!("o/opensuse_tumbleweed.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed_small"],
                lines: include_str!("o/opensuse_tumbleweed_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed_old"],
                lines: include_str!("o/opensuse_tumbleweed_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed2"],
                lines: include_str!("o/opensuse_tumbleweed2.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-slowroll", "opensuse-tumbleweed-slowroll"],
                lines: include_str!("o/opensuse_slowroll.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["openmandriva"],
                lines: include_str!("o/openmandriva.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["openwrt"],
                lines: include_str!("o/openwrt.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openwrt_small"],
                lines: include_str!("o/openwrt_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openwrt_old"],
                lines: include_str!("o/openwrt_old.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opnsense"],
                lines: include_str!("o/opnsense.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ol", "oracle"],
                lines: include_str!("o/oracle.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["orchid"],
                lines: include_str!("o/orchid.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["orchid_small"],
                lines: include_str!("o/orchid_small.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["oreon"],
                lines: include_str!("o/oreon.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["origami"],
                lines: include_str!("o/origami.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["origami_small"],
                lines: include_str!("o/origami_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["os2warp"],
                lines: include_str!("o/os2warp.txt"),
                colors: &[
                    color::FG_LIGHT_WHITE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["os elbrus"],
                lines: include_str!("o/os_elbrus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["osmc", "open source media center"],
                lines: include_str!("o/osmc.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["osx"],
                lines: include_str!("m/macos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["osx_small"],
                lines: include_str!("m/macos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
        ]
    })
}