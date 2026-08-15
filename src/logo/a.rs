use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static A: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    A.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["adélie", "adelie"],
                lines: include_bytes!("../../temp/a/adelie.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aeros"],
                lines: include_bytes!("../../temp/a/aeros.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aeon"],
                lines: include_bytes!("../../temp/a/aeon.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aerynos"],
                lines: include_bytes!("../../temp/a/aerynos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aerynos old"],
                lines: include_bytes!("../../temp/a/aerynos_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["afterglow"],
                lines: include_bytes!("../../temp/a/afterglow.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aix"],
                lines: include_bytes!("../../temp/a/aix.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["almalinux"],
                lines: include_bytes!("../../temp/a/almalinux.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_LIGHT_YELLOW,
                    color::FG_BLUE,
                    color::FG_LIGHT_GREEN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["alpine"],
                lines: include_bytes!("../../temp/a/alpine.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine2"],
                lines: include_bytes!("../../temp/a/alpine2.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine small"],
                lines: include_bytes!("../../temp/a/alpine_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine2 small"],
                lines: include_bytes!("../../temp/a/alpine2_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine3 small"],
                lines: include_bytes!("../../temp/a/alpine3_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alter"],
                lines: include_bytes!("../../temp/a/alter.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["altlinux"],
                lines: include_bytes!("../../temp/a/altlinux.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_BLACK,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["amazon"],
                lines: include_bytes!("../../temp/a/amazon.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["amazon linux", "amzn"],
                lines: include_bytes!("../../temp/a/amazon_linux.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["amiga"],
                lines: include_bytes!("../../temp/a/amiga.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_LIGHT_RED,
                    color::FG_YELLOW,
                    color::FG_BLUE,
                    color::FG_CYAN,
                    color::FG_LIGHT_YELLOW,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["amogos"],
                lines: include_bytes!("../../temp/a/amogos.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["anarchy"],
                lines: include_bytes!("../../temp/a/anarchy.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["android"],
                lines: include_bytes!("../../temp/a/android.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["android small"],
                lines: include_bytes!("../../temp/a/android_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["anduinos"],
                lines: include_bytes!("../../temp/a/anduinos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["antergos"],
                lines: include_bytes!("../../temp/a/antergos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["antix"],
                lines: include_bytes!("../../temp/a/antix.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["anushos"],
                lines: include_bytes!("../../temp/a/anushos.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLACK,
                    color::FG_YELLOW,
                    color::FG_CYAN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aosc os/retro", "aoscosretro"],
                lines: include_bytes!("../../temp/a/aoscosretro.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aosc os/retro small", "aoscosretro small"],
                lines: include_bytes!("../../temp/a/aoscosretro_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aosc os", "aoscos"],
                lines: include_bytes!("../../temp/a/aoscos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_BLACK,
                    color::FG_RED,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aosc os old", "aoscos old"],
                lines: include_bytes!("../../temp/a/aoscos_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aperture"],
                lines: include_bytes!("../../temp/a/aperture.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["apple"],
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
                names: &["apple small"],
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
            LogoInfo {
                names: &["apricity"],
                lines: include_bytes!("../../temp/a/apricity.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["archbox"],
                lines: include_bytes!("../../temp/a/archbox.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["archcraft"],
                lines: include_bytes!("../../temp/a/archcraft.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["archcraft2"],
                lines: include_bytes!("../../temp/a/archcraft2.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["arch", "archmerge"],
                lines: include_bytes!("../../temp/a/arch.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch2"],
                lines: include_bytes!("../../temp/a/arch2.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch3"],
                lines: include_bytes!("../../temp/a/arch3.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch small"],
                lines: include_bytes!("../../temp/a/arch_small.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch old"],
                lines: include_bytes!("../../temp/a/arch_old.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["archlabs"],
                lines: include_bytes!("../../temp/a/archlabs.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_RED,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["archstrike"],
                lines: include_bytes!("../../temp/a/archstrike.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["arkane", "arkane linux"],
                lines: include_bytes!("../../temp/a/arkane.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["armbian"],
                lines: include_bytes!("../../temp/a/armbian.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["armbian2"],
                lines: include_bytes!("../../temp/a/armbian2.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["artix"],
                lines: include_bytes!("../../temp/a/artix.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["artix small"],
                lines: include_bytes!("../../temp/a/artix_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["artix2 small"],
                lines: include_bytes!("../../temp/a/artix2_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["arco", "arcolinux"],
                lines: include_bytes!("../../temp/a/arco.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["arco small", "arcolinux small"],
                lines: include_bytes!("../../temp/a/arco_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["arse", "arselinux", "arse-linux"],
                lines: include_bytes!("../../temp/a/arselinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arya"],
                lines: include_bytes!("../../temp/a/arya.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["asahi", "asahi-linux"],
                lines: include_bytes!("../../temp/a/asahi.txt"),
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
                names: &["asahi2", "asahi-linux2"],
                lines: include_bytes!("../../temp/a/asahi2.txt"),
                colors: &[
                    color::FG_LIGHT_YELLOW,
                    color::FG_CYAN,
                    color::FG_RED,
                    color::FG_LIGHT_RED,
                    color::FG_WHITE,
                    color::FG_BLACK,
                    color::FG_LIGHT_CYAN,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["aster"],
                lines: include_bytes!("../../temp/a/aster.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["asteroidos"],
                lines: include_bytes!("../../temp/a/asteroidos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["astos"],
                lines: include_bytes!("../../temp/a/astos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["astra", "astra linux", "astralinux"],
                lines: include_bytes!("../../temp/a/astra_linux.txt"),
                colors: &[
                    color::FG_LIGHT_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ataraxia"],
                lines: include_bytes!("../../temp/j/januslinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["athenaos"],
                lines: include_bytes!("../../temp/a/athenaos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_LIGHT_BLUE,
            },
            LogoInfo {
                names: &["athenaos old"],
                lines: include_bytes!("../../temp/a/athenaos_old.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aurora"],
                lines: include_bytes!("../../temp/a/aurora.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["axos"],
                lines: include_bytes!("../../temp/a/axos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["azos"],
                lines: include_bytes!("../../temp/a/azos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["azurelinux"],
                lines: include_bytes!("../../temp/a/azurelinux.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["azurelinux2"],
                lines: include_bytes!("../../temp/a/azurelinux2.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}