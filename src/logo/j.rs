use alloc::{
    vec::Vec,
    vec
};
use crate::{
    color,
    logo::LogoInfo,
    sync::OnceLock
};

static J: OnceLock<Vec<LogoInfo>> = OnceLock::new();

pub fn get() -> &'static Vec<LogoInfo> {
    J.get_or_init(|| {
        vec![
            LogoInfo {
                names: &["januslinux"],
                lines: include_bytes!("../../temp/j/januslinux.txt"),
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