pub struct Version<'a> {
    name: &'a str,
    ver: &'a str,
    arch: &'a str
}

impl<'a> Version<'a> {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_BIN_NAME"), 
            ver: env!("CARGO_PKG_VERSION"), 
            arch: env!("TARGET_ARCH")
        }
    }
}

impl<'a> core::fmt::Display for Version<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {} ({})", self.name, self.ver, self.arch)
    }
}