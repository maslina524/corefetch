use alloc::{
    borrow::ToOwned,
    string::String
};

use crate::{
    imp::parser::LinuxInfo,
    imp::fs,
    detect::os::OsInfo
};

impl OsInfo {
    pub fn new() -> Self {
        let sysname = "Linux";

        let os_release = LinuxInfo::parse_os_release().unwrap();

        let name = os_release.get("NAME", "Unknown".to_owned());
        let codename = os_release.get("VERSION_CODENAME", "".to_owned());
        let variant = os_release.get("VARIANT", "".to_owned());
        let variant_id = os_release.get("VARIANT_ID", "".to_owned());
        let id = os_release.get("ID", "Unknown".to_owned());
        let version = Self::get_version(&id).unwrap_or_else(|| {
            os_release.get("VERSION_ID", "Unknown")
        });
        let nerd = Self::nerd(&id);

        Self { 
            sysname,
            name,
            id: id.clone(),
            id_like: id,
            version: version.clone(),
            version_id: version,
            codename,
            variant,
            variant_id,
            nerd
        }
    }

    fn nerd(id: &str) -> char {
        match id {
            "debian" => '\u{f08da}',
            "kali" => '\u{f327}',
            "ubuntu" => '\u{ef72}',
            "linuxmint" => '\u{f08ed}',
            "fedora" => '\u{e7d9}',
            "opensuse" | "sle" => '\u{ef6d}',
            "arch" => '\u{f08c7}',
            "manjaro" => '\u{f312}',
            "popos" => '\u{f32a}',
            "mxlinux" => '\u{f33f}',
            "almalinux" => '\u{e8f3}',
            "rockylinux" => '\u{e891}',
            "rhel" => '\u{ef5d}',
            "garuda" => '\u{f337}',
            "gentoo" => '\u{e7e6}',
            "slackware" => '\u{f318}',
            "nixos" => '\u{e843}',
            "raspios" => '\u{e722}',
            "zorin" => '\u{f32f}',
            "cachyos" => '\u{f385}',
            "void" => '\u{f32e}',
            _ => '\u{ebc6}'
        }
    }
    
    fn get_version(id: &str) -> Option<String> {
        match id {
            "debian" => fs::read_to_string("/etc/debian_version").ok(),
            "alpine" => fs::read_to_string("/etc/alpine-version").ok(),
            "rhel" | "centos" | "fedora" | "rocky" | "almalinux" => {
                let content = fs::read_to_string("/etc/redhat-release").ok()?;
                Some(Self::extract_version(&content))
            },
            "gentoo" => {
                let content = fs::read_to_string("/etc/gentoo-release").ok()?;
                Some(Self::extract_version(&content))
            },
            "arch" => Some("Rolling".to_owned()),
            _ => None // os-release -> VERSION_ID
        }
    }

    fn extract_version(s: &str) -> String {
        let mut ret = String::with_capacity(8);
        let mut in_ret = false;

        for ch in s.chars() {
            if ch.is_numeric() && !in_ret {
                in_ret = true;
                ret.push(ch);
                continue;
            }

            if in_ret && (ch.is_numeric() || ch == '.') {
                ret.push(ch);
                continue;
            }
            break;
        }
        
        ret
    }
}