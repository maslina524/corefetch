use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static C: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    C.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["cachyos"],
                lines: include_str!("c/cachyos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_CYAN,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["cachyos small"],
                lines: include_str!("c/cachyos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["cachyos old small"],
                lines: include_str!("c/cachyos_old_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["calculate"],
                lines: include_str!("c/calculate.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["calinixos"],
                lines: include_str!("c/calinixos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["calinixos small"],
                lines: include_str!("c/calinixos_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["carbs"],
                lines: include_str!("c/carbs.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["cbl-mariner"],
                lines: include_str!("c/cbl_mariner.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["celos"],
                lines: include_str!("c/celos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["center"],
                lines: include_str!("c/center.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["centos"],
                lines: include_str!("c/centos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["centos small"],
                lines: include_str!("c/centos_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["cereus"],
                lines: include_str!("c/cereus.txt"),
                colors: &[
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
                names: &["chakra"],
                lines: include_str!("c/chakra.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["chaletos"],
                lines: include_str!("c/chaletos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["chapeau"],
                lines: include_str!("c/chapeau.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["chimera"],
                lines: include_str!("c/chimera_linux.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["chimera2"],
                lines: include_str!("c/chimera_linux2.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_RED,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["chimera small"],
                lines: include_str!("c/chimera_linux_small.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_MAGENTA,
                    color::FG_BLUE,
                    color::FG_RED,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["chonkysealos"],
                lines: include_str!("c/chonkysealos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["chrom", "chromeos"],
                lines: include_str!("c/chrom.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["cleanjaro"],
                lines: include_str!("c/cleanjaro.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["cleanjaro small"],
                lines: include_str!("c/cleanjaro_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["clear linux", "clearlinux", "clear linux os"],
                lines: include_str!("c/clear_linux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["clearos"],
                lines: include_str!("c/clearos.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["clover"],
                lines: include_str!("c/clover.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["cobalt"],
                lines: include_str!("c/cobalt.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLUE,
                    color::FG_LIGHT_BLACK,
                    color::FG_LIGHT_BLUE,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["codex linux"],
                lines: include_str!("c/codex.txt"),
                colors: &[
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["condres"],
                lines: include_str!("c/condres.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["coreos", "container linux by coreos"],
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
                names: &["common-torizon"],
                lines: include_str!("t/torizoncore.txt"),
                colors: &[
                    color::FG_LIGHT_WHITE,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["cosmic"],
                lines: include_str!("c/cosmic.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_LIGHT_YELLOW,
                    color::FG_YELLOW,
                    color::FG_LIGHT_RED,
                    color::FG_RED,
                ],
                color_keys: color::FG_LIGHT_RED,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["crux"],
                lines: include_str!("c/crux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["crux small"],
                lines: include_str!("c/crux_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["crystal", "crystal", "crystal-linux", "crystal-linux"],
                lines: include_str!("c/crystal.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["cucumber", "cucumberos"],
                lines: include_str!("c/cucumber.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["cuerdos", "cuerdos gnu/linux"],
                lines: include_str!("c/cuerdos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["cutefishos"],
                lines: include_str!("c/cutefishos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["cuteos"],
                lines: include_str!("c/cuteos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["cyberos"],
                lines: include_str!("c/cyberos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["cycledream"],
                lines: include_str!("c/cycledream.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
        ]
    })
}