use core::fmt::Display;

use crate::{
    os::windows::{FormatMessageW, GetLastError},
    os::encoding::utf16le_to_utf8
};

pub type Result<T> = core::result::Result<T, ErrorCode>;

const FORMAT_MESSAGE_FROM_SYSTEM   : u32 = 0x00001000;
const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub fn new(code: u32) -> Self {
        Self(code)
    }

    pub fn last() -> Self {
        let code = unsafe { GetLastError() };
        Self::new(code)
    }

    #[track_caller]
    pub fn panic(&self) -> ! {
        panic!("WinApi: {self}")
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut buf = [0u16; 256];
        // SAFETY: The buffer size is sufficient to hold any error.
        let len = unsafe {
            FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, 
                core::ptr::null(), 
                self.0, 
                0, 
                buf.as_mut_ptr(), 
                256, 
                core::ptr::null()
            )
        };

        let utf8 = utf16le_to_utf8(&buf, len as isize)
            .expect("WinAPI passed an invalid UTF-16LE string");

        let utf8_str = utf8.trim();
        write!(f, "{utf8_str}")
    }
}

impl core::error::Error for ErrorCode {}

#[cfg(test)]
mod tests {
    extern crate std;

    use crate::os::error::ErrorCode;

    #[test]
    fn panic_msg_test() {
        let err = ErrorCode::new(5);
        let fmt = std::format!("{err}");
        std::println!("Error msg: `{fmt}`");
        assert!(fmt.len() > "WinApi: ".len())
    }
}