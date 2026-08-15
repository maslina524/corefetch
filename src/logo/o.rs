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
                lines: include_bytes!("../../temp/o/obarun.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["obrevenge"],
                lines: include_bytes!("../../temp/o/obrevenge.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["obsidianos"],
                lines: include_bytes!("../../temp/o/obsidianos.txt"),
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
                lines: include_bytes!("../../temp/o/omnios.txt"),
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
                lines: include_bytes!("../../temp/o/openkylin.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openbsd"],
                lines: include_bytes!("../../temp/o/openbsd.txt"),
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
                names: &["openbsd small"],
                lines: include_bytes!("../../temp/o/openbsd_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openeuler"],
                lines: include_bytes!("../../temp/o/openeuler.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openindiana"],
                lines: include_bytes!("../../temp/o/openindiana.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openmamba"],
                lines: include_bytes!("../../temp/o/openmamba.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openruyi"],
                lines: include_bytes!("../../temp/o/openruyi.txt"),
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
                lines: include_bytes!("../../temp/o/openstage.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opensuse"],
                lines: include_bytes!("../../temp/o/opensuse.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse small"],
                lines: include_bytes!("../../temp/o/opensuse_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-microos"],
                lines: include_bytes!("../../temp/o/opensuse_microos.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opensuse-leap"],
                lines: include_bytes!("../../temp/o/opensuse_leap.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-leap old"],
                lines: include_bytes!("../../temp/o/opensuse_leap_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed"],
                lines: include_bytes!("../../temp/o/opensuse_tumbleweed.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed small"],
                lines: include_bytes!("../../temp/o/opensuse_tumbleweed_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed old"],
                lines: include_bytes!("../../temp/o/opensuse_tumbleweed_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-tumbleweed2"],
                lines: include_bytes!("../../temp/o/opensuse_tumbleweed2.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["opensuse-slowroll", "opensuse-tumbleweed-slowroll"],
                lines: include_bytes!("../../temp/o/opensuse_slowroll.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["openmandriva"],
                lines: include_bytes!("../../temp/o/openmandriva.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["openwrt"],
                lines: include_bytes!("../../temp/o/openwrt.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openwrt small"],
                lines: include_bytes!("../../temp/o/openwrt_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["openwrt old"],
                lines: include_bytes!("../../temp/o/openwrt_old.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["opnsense"],
                lines: include_bytes!("../../temp/o/opnsense.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ol", "oracle"],
                lines: include_bytes!("../../temp/o/oracle.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["orchid"],
                lines: include_bytes!("../../temp/o/orchid.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["orchid small"],
                lines: include_bytes!("../../temp/o/orchid_small.txt"),
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
                lines: include_bytes!("../../temp/o/oreon.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["origami"],
                lines: include_bytes!("../../temp/o/origami.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["origami small"],
                lines: include_bytes!("../../temp/o/origami_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["os2warp"],
                lines: include_bytes!("../../temp/o/os2warp.txt"),
                colors: &[
                    color::FG_LIGHT_WHITE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["os elbrus"],
                lines: include_bytes!("../../temp/o/os_elbrus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["osmc", "open source media center"],
                lines: include_bytes!("../../temp/o/osmc.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["osx"],
                lines: include_bytes!("../../temp/m/macos.txt"),
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
                names: &["osx small"],
                lines: include_bytes!("../../temp/m/macos_small.txt"),
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