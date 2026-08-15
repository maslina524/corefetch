use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static Z: OnceLock<[LogoInfo; 4]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 4] {
    Z.get_or_init(|| {
        [
            LogoInfo {
                names: &["Zerene"],
                lines: include_str!("z/zerene.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["Zorin"],
                lines: include_str!("z/zorin.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
            LogoInfo {
                names: &["z/OS", "zos"],
                lines: include_str!("z/zos.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
            LogoInfo {
                names: &["Zraxyl"],
                lines: include_str!("z/zraxyl.txt"),
                colors: &[
                    color::FG_BLUE,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_BLUE,
            },
        ]
    })
}