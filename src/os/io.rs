use core::fmt::Write;

use alloc::string::String;

pub struct StringFormatter<'a>(&'a mut String);

impl<'a> Write for StringFormatter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}

impl<'a> StringFormatter<'a> {
    #[inline(always)]
    pub fn new(ptr: &'a mut String) -> Self {
        StringFormatter(ptr)
    }
    
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }

    pub fn write_nl(&mut self) -> core::fmt::Result {
        self.0.write_str("\n")
    }
}