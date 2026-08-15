use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static C: OnceLock<[LogoInfo; 41]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 41] {
    C.get_or_init(|| {
        [
            LogoInfo {
                names: &["CachyOS"],
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
                names: &["CachyOS_small"],
                lines: include_str!("c/cachyos_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["CachyOS_old_small"],
                lines: include_str!("c/cachyos_old_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["Calculate"],
                lines: include_str!("c/calculate.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["CalinixOS"],
                lines: include_str!("c/calinixos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["CalinixOS_small"],
                lines: include_str!("c/calinixos_small.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Carbs"],
                lines: include_str!("c/carbs.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["CBL-Mariner"],
                lines: include_str!("c/cbl_mariner.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["CelOS"],
                lines: include_str!("c/celos.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Center"],
                lines: include_str!("c/center.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["CentOS"],
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
                names: &["CentOS_small"],
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
                names: &["Cereus"],
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
                names: &["Chakra"],
                lines: include_str!("c/chakra.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["ChaletOS"],
                lines: include_str!("c/chaletos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Chapeau"],
                lines: include_str!("c/chapeau.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Chimera"],
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
                names: &["Chimera2"],
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
                names: &["Chimera_small"],
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
                names: &["ChonkySealOS"],
                lines: include_str!("c/chonkysealos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Chrom", "ChromeOS"],
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
                names: &["Cleanjaro"],
                lines: include_str!("c/cleanjaro.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Cleanjaro_small"],
                lines: include_str!("c/cleanjaro_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Clear Linux", "clearlinux", "Clear Linux OS"],
                lines: include_str!("c/clear_linux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["ClearOS"],
                lines: include_str!("c/clearos.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["Clover"],
                lines: include_str!("c/clover.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Cobalt"],
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
                names: &["Codex Linux"],
                lines: include_str!("c/codex.txt"),
                colors: &[
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Condres"],
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
                names: &["CoreOS", "Container Linux by CoreOS"],
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
                names: &["Cosmic"],
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
                names: &["CRUX"],
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
                names: &["CRUX_small"],
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
                names: &["Crystal", "Crystal", "crystal-linux", "Crystal-Linux"],
                lines: include_str!("c/crystal.txt"),
                colors: &[
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["Cucumber", "CucumberOS"],
                lines: include_str!("c/cucumber.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["CuerdOS", "CuerdOS GNU/Linux"],
                lines: include_str!("c/cuerdos.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["CutefishOS"],
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
                names: &["CuteOS"],
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
                names: &["CyberOS"],
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