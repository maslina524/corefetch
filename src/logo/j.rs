use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static J: OnceLock<[LogoInfo; 1]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 1] {
    J.get_or_init(|| {
        [
            LogoInfo {
                names: &["januslinux"],
                lines: include_str!("j/januslinux.txt"),
                colors: &[
                    color::FG_BLUE,
                    color::FG_MAGENTA,
                ],
                color_keys: color::FG_BLUE,
                color_title: color::FG_MAGENTA,
            },
        ]
    })
}