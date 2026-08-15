use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static A: OnceLock<[LogoInfo; 68]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 68] {
    A.get_or_init(|| {
        [
            LogoInfo {
                names: &["Adélie", "Adelie"],
                lines: include_str!("a/adelie.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["aerOS"],
                lines: include_str!("a/aeros.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Aeon"],
                lines: include_str!("a/aeon.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["AerynOS"],
                lines: include_str!("a/aerynos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["AerynOS_old"],
                lines: include_str!("a/aerynos_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Afterglow"],
                lines: include_str!("a/afterglow.txt"),
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
                lines: include_str!("a/aix.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Almalinux"],
                lines: include_str!("a/almalinux.txt"),
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
                names: &["Alpine"],
                lines: include_str!("a/alpine.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Alpine2"],
                lines: include_str!("a/alpine2.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Alpine_small"],
                lines: include_str!("a/alpine_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine2_small"],
                lines: include_str!("a/alpine2_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_MAGENTA,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["alpine3_small"],
                lines: include_str!("a/alpine3_small.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Alter"],
                lines: include_str!("a/alter.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["ALTLinux"],
                lines: include_str!("a/altlinux.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_BLACK,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_YELLOW,
            },
            LogoInfo {
                names: &["Amazon"],
                lines: include_str!("a/amazon.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Amazon Linux", "amzn"],
                lines: include_str!("a/amazon_linux.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Amiga"],
                lines: include_str!("a/amiga.txt"),
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
                names: &["AmogOS"],
                lines: include_str!("a/amogos.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Anarchy"],
                lines: include_str!("a/anarchy.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["android"],
                lines: include_str!("a/android.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["android_small"],
                lines: include_str!("a/android_small.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_GREEN,
            },
            LogoInfo {
                names: &["anduinos"],
                lines: include_str!("a/anduinos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Antergos"],
                lines: include_str!("a/antergos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["antiX"],
                lines: include_str!("a/antix.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["AnushOS"],
                lines: include_str!("a/anushos.txt"),
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
                names: &["Aosc OS/Retro", "aoscosretro"],
                lines: include_str!("a/aoscosretro.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Aosc OS/Retro_small", "aoscosretro_small"],
                lines: include_str!("a/aoscosretro_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Aosc OS", "aoscos"],
                lines: include_str!("a/aoscos.txt"),
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
                names: &["Aosc OS_old", "aoscos_old"],
                lines: include_str!("a/aoscos_old.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Aperture"],
                lines: include_str!("a/aperture.txt"),
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
                names: &["Apple"],
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
                names: &["Apple_small"],
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
            LogoInfo {
                names: &["Apricity"],
                lines: include_str!("a/apricity.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ArchBox"],
                lines: include_str!("a/archbox.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Archcraft"],
                lines: include_str!("a/archcraft.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Archcraft2"],
                lines: include_str!("a/archcraft2.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["arch", "archmerge"],
                lines: include_str!("a/arch.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch2"],
                lines: include_str!("a/arch2.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch3"],
                lines: include_str!("a/arch3.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch_small"],
                lines: include_str!("a/arch_small.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["arch_old"],
                lines: include_str!("a/arch_old.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ARCHlabs"],
                lines: include_str!("a/archlabs.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_RED,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["ArchStrike"],
                lines: include_str!("a/archstrike.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_BLACK,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["Arkane", "Arkane Linux"],
                lines: include_str!("a/arkane.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Armbian"],
                lines: include_str!("a/armbian.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Armbian2"],
                lines: include_str!("a/armbian2.txt"),
                colors: &[
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["artix"],
                lines: include_str!("a/artix.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["artix_small"],
                lines: include_str!("a/artix_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["artix2_small"],
                lines: include_str!("a/artix2_small.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["arco", "arcolinux"],
                lines: include_str!("a/arco.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["arco_small", "arcolinux_small"],
                lines: include_str!("a/arco_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["arse", "arselinux", "arse-linux"],
                lines: include_str!("a/arselinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Arya"],
                lines: include_str!("a/arya.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                ],
                color_keys: color::FG_GREEN,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["asahi", "asahi-linux"],
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
                names: &["asahi2", "asahi-linux2"],
                lines: include_str!("a/asahi2.txt"),
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
                lines: include_str!("a/aster.txt"),
                colors: &[
                    color::FG_CYAN,
                ],
                color_keys: color::FG_CYAN,
                color_title: color::FG_CYAN,
            },
            LogoInfo {
                names: &["AsteroidOS"],
                lines: include_str!("a/asteroidos.txt"),
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
                names: &["astOS"],
                lines: include_str!("a/astos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Astra", "Astra Linux", "astralinux"],
                lines: include_str!("a/astra_linux.txt"),
                colors: &[
                    color::FG_LIGHT_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Ataraxia"],
                lines: include_str!("j/januslinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
            LogoInfo {
                names: &["AthenaOS"],
                lines: include_str!("a/athenaos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_LIGHT_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_LIGHT_BLUE,
            },
            LogoInfo {
                names: &["AthenaOS_old"],
                lines: include_str!("a/athenaos_old.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Aurora"],
                lines: include_str!("a/aurora.txt"),
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
                names: &["AxOS"],
                lines: include_str!("a/axos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Azos"],
                lines: include_str!("a/azos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["AzureLinux"],
                lines: include_str!("a/azurelinux.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["AzureLinux2"],
                lines: include_str!("a/azurelinux2.txt"),
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