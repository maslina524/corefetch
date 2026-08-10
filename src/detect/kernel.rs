use alloc::string::String;

use crate::{
    os::env,
    format
};

pub const fn sysname() -> &'static str {
    "WIN32_NT"
}

pub fn release() -> String {
    let (major, minor, build) = env::get_version();
    format!("{major}.{minor}.{build}")
}

#[cfg(test)]
mod tests {
    use crate::detect::kernel;


    #[test]
    fn release_test() {
        let release = kernel::release();
        println!("{release}");
    }
}