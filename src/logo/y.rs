use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static Y: OnceLock<[LogoInfo; 1]> = OnceLock::new();

pub fn get() -> &'static [LogoInfo; 1] {
    Y.get_or_init(|| {
        [
            LogoInfo {
                names: &["YiffOS"],
                lines: include_str!("y/yiffos.txt"),
                colors: &[
                    color::FG_DEFAULT,
                    color::FG_DEFAULT,
                ],
                color_keys: color::FG_DEFAULT,
                color_title: color::FG_DEFAULT,
            },
        ]
    })
}