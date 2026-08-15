use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static P: OnceLock<[LogoInfo; 33]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 33] {
    P.get_or_init(|| {
        [
            LogoInfo {
                names: &["PacBSD"],
                lines: include_str!("p/pacbsd.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Panwah"],
                lines: include_str!("p/panwah.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Parabola"],
                lines: include_str!("p/parabola.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Parabola_small"],
                lines: include_str!("p/parabola_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Parabola2_small"],
                lines: include_str!("p/parabola2_small.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Parch"],
                lines: include_str!("p/parch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Pardus"],
                lines: include_str!("p/pardus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Parrot"],
                lines: include_str!("p/parrot.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Parsix"],
                lines: include_str!("p/parsix.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PCBSD", "TrueOS"],
                lines: include_str!("p/pcbsd.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PCLinuxOS"],
                lines: include_str!("p/pclinuxos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PearOS"],
                lines: include_str!("p/pearos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Pengwin"],
                lines: include_str!("p/pengwin.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_LIGHT_MAGENTA,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Pentoo"],
                lines: include_str!("p/pentoo.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Peppermint"],
                lines: include_str!("p/peppermint.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Peropesis"],
                lines: include_str!("p/peropesis.txt"),
                colors: &[
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PhyOS"],
                lines: include_str!("p/phyos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PikaOS"],
                lines: include_str!("p/pikaos.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PisiLinux"],
                lines: include_str!("p/pisi.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PNM Linux"],
                lines: include_str!("p/pnm_linux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["pop", "popos"],
                lines: include_str!("p/pop.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["pop_small", "popos_small"],
                lines: include_str!("p/pop_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Porteus"],
                lines: include_str!("p/porteus.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PostMarketOS"],
                lines: include_str!("p/postmarketos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PostMarketOS2"],
                lines: include_str!("p/postmarketos2.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PostMarketOS_small"],
                lines: include_str!("p/postmarketos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Proxmox"],
                lines: include_str!("p/proxmox.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PuffOS"],
                lines: include_str!("p/puffos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Puppy"],
                lines: include_str!("p/puppy.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PureOS"],
                lines: include_str!("p/pureos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PureOS_small"],
                lines: include_str!("p/pureos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PrismLinux"],
                lines: include_str!("p/prismlinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["PrismLinux_small"],
                lines: include_str!("p/prismlinux_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}