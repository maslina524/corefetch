use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static K: OnceLock<[LogoInfo; 18]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 18] {
    K.get_or_init(|| {
        [
            LogoInfo {
                names: &["Kaisen"],
                lines: include_str!("k/kaisen.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Kali"],
                lines: include_str!("k/kali.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLACK,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Kali_small"],
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
                names: &["KaOS"],
                lines: include_str!("k/kaos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["KernelOS"],
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
                names: &["KDE Neon"],
                lines: include_str!("k/kdeneon.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["KibaOS"],
                lines: include_str!("k/kibaos.txt"),
                colors: &[
                    color::BG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Kibojoe"],
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
                names: &["KISS"],
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
                names: &["Kogaion"],
                lines: include_str!("k/kogaion.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Korora"],
                lines: include_str!("k/korora.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["KrassOS", "Krass"],
                lines: include_str!("k/krassos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["KSLinux"],
                lines: include_str!("k/kslinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Kubuntu", "kubuntu-linux", "kde-ubuntu"],
                lines: include_str!("k/kubuntu.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Kylin"],
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