use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static G: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    G.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["galliumos"],
                lines: include_bytes!("../../temp/g/galliumos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["garuda"],
                lines: include_bytes!("../../temp/g/garuda.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["garudadragon", "garuda-dragon"],
                lines: include_bytes!("../../temp/g/garuda_dragon.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["garuda small", "garuda-linux small"],
                lines: include_bytes!("../../temp/g/garuda_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["gentoo"],
                lines: include_bytes!("../../temp/g/gentoo.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["gentoo small"],
                lines: include_bytes!("../../temp/g/gentoo_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["ghostbsd"],
                lines: include_bytes!("../../temp/g/ghostbsd.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ghostfreak"],
                lines: include_bytes!("../../temp/g/ghostfreak.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["glaucus"],
                lines: include_bytes!("../../temp/g/glaucus.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["gnewsense"],
                lines: include_bytes!("../../temp/g/gnewsense.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["gnome os"],
                lines: include_bytes!("../../temp/g/gnome.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["gnu"],
                lines: include_bytes!("../../temp/g/gnu.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["gobolinux", "gobo"],
                lines: include_bytes!("../../temp/g/gobolinux.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["goldendoglinux"],
                lines: include_bytes!("../../temp/g/goldendoglinux.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["grapheneos"],
                lines: include_bytes!("../../temp/g/grapheneos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["grombyang"],
                lines: include_bytes!("../../temp/g/grombyang.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["guix"],
                lines: include_bytes!("../../temp/g/guix.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["guix small"],
                lines: include_bytes!("../../temp/g/guix_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["gxde"],
                lines: include_bytes!("../../temp/g/gxde.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
        ]
    })
}