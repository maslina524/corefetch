use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static G: OnceLock<[LogoInfo; 19]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 19] {
    G.get_or_init(|| {
        [
            LogoInfo {
                names: &["GalliumOS"],
                lines: include_str!("g/galliumos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Garuda"],
                lines: include_str!("g/garuda.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["GarudaDragon", "garuda-dragon"],
                lines: include_str!("g/garuda_dragon.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Garuda_small", "garuda-linux_small"],
                lines: include_str!("g/garuda_small.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Gentoo"],
                lines: include_str!("g/gentoo.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Gentoo_small"],
                lines: include_str!("g/gentoo_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["GhostBSD"],
                lines: include_str!("g/ghostbsd.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["GhostFreak"],
                lines: include_str!("g/ghostfreak.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Glaucus"],
                lines: include_str!("g/glaucus.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["gNewSense"],
                lines: include_str!("g/gnewsense.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["GNOME OS"],
                lines: include_str!("g/gnome.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["GNU"],
                lines: include_str!("g/gnu.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["GoboLinux", "Gobo"],
                lines: include_str!("g/gobolinux.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["GoldenDogLinux"],
                lines: include_str!("g/goldendoglinux.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["GrapheneOS"],
                lines: include_str!("g/grapheneos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Grombyang"],
                lines: include_str!("g/grombyang.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["Guix"],
                lines: include_str!("g/guix.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Guix_small"],
                lines: include_str!("g/guix_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["GXDE"],
                lines: include_str!("g/gxde.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
        ]
    })
}