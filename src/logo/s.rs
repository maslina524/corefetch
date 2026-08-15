use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static S: OnceLock<[LogoInfo; 45]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 45] {
    S.get_or_init(|| {
        [
            LogoInfo {
                names: &["Sabayon"],
                lines: include_str!("s/sabayon.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Sabotage"],
                lines: include_str!("s/sabotage.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Sailfish"],
                lines: include_str!("s/sailfish.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SalentOS"],
                lines: include_str!("s/salentos.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_RED,
                    color::FG_YELLOW,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Salient OS", "SalientOS"],
                lines: include_str!("s/salientos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Salix"],
                lines: include_str!("s/salix.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SambaBOX"],
                lines: include_str!("s/sambabox.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Sasanqua"],
                lines: include_str!("s/sasanqua.txt"),
                colors: &[
                    color::FG_MAGENTA,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Scientific"],
                lines: include_str!("s/scientific.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["secureblue"],
                lines: include_str!("s/secureblue.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Serpent OS"],
                lines: include_str!("s/serpent_os.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["semc"],
                lines: include_str!("s/semc.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_LIGHT_BLACK,
                    color::FG_RED,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Septor"],
                lines: include_str!("s/septor.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Serene"],
                lines: include_str!("s/serene.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_CYAN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SharkLinux"],
                lines: include_str!("s/sharklinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["ShastraOS"],
                lines: include_str!("s/shastraos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Shebang"],
                lines: include_str!("s/shebang.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Siduction"],
                lines: include_str!("s/siduction.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SkiffOS"],
                lines: include_str!("s/skiffos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SleeperOS"],
                lines: include_str!("s/sleeperos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SleeperOS_small"],
                lines: include_str!("s/sleeperos_small.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Slitaz"],
                lines: include_str!("s/slitaz.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SpoinkOS", "spoink-os"],
                lines: include_str!("s/spoinkos.txt"),
                colors: &[
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Slackel"],
                lines: include_str!("s/slackel.txt"),
                colors: &[
                    color::FG_YELLOW,
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Slackware"],
                lines: include_str!("s/slackware.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Slackware_small"],
                lines: include_str!("s/slackware_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SmartOS"],
                lines: include_str!("s/smartos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SnigdhaOS", "Snigdha"],
                lines: include_str!("s/snigdhaos.txt"),
                colors: &[
                    color::FG_CYAN,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Soda"],
                lines: include_str!("s/soda.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Source Mage", "Source Mage GNU/Linux", "source_mage", "sourcemage"],
                lines: include_str!("s/source_mage.txt"),
                colors: &[
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["solaris", "sunos"],
                lines: include_str!("s/solaris.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["solaris_small", "sunos_small"],
                lines: include_str!("s/solaris_small.txt"),
                colors: &[
                    color::FG_YELLOW,
                ],
                color_keys: color::FG_YELLOW,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Solus", "solus-linux"],
                lines: include_str!("s/solus.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Sparky"],
                lines: include_str!("s/sparky.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Star"],
                lines: include_str!("s/star.txt"),
                colors: &[
                    color::FG_WHITE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Stock Linux"],
                lines: include_str!("s/stock_linux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SteamOS"],
                lines: include_str!("s/steamos.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["SteamDeck"],
                lines: include_str!("s/steamdeck.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["SteamDeck_small"],
                lines: include_str!("s/steamdeck_small.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["SteamDeckOled"],
                lines: include_str!("s/steamdeck.txt"),
                colors: &[
                    color::FG_RED,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_RED,
                color_title: color::FG_RED,
            },
            LogoInfo {
                names: &["Sulin"],
                lines: include_str!("s/sulin.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["SummitOS"],
                lines: include_str!("s/summitos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["suse", "suse-linux"],
                lines: include_str!("s/suse.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["suse_small", "suse-linux_small"],
                lines: include_str!("o/opensuse_small.txt"),
                colors: &[
                    color::FG_GREEN,
                    color::FG_GREEN,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Swagarch"],
                lines: include_str!("s/swagarch.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_WHITE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}