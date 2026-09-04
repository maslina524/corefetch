use core::{
    fmt::Display,
    ffi::CStr
};

use crate::{
    linux::libc::{errno, strerror},
    abort
};

pub type Result<T> = core::result::Result<T, ErrorCode>;

#[derive(Debug, Clone)]
pub struct ErrorCode(i32);

impl ErrorCode {
    pub const fn new(code: i32) -> Self {
        Self(code)
    }
    
    pub fn last() -> Self {
        let code = errno();
        Self::new(code)
    }

    pub const fn code(&self) -> i32 {
        self.0
    }

    #[cold]
    #[track_caller]
    pub fn panic(&self) -> ! {
        abort!("LibC: {self}")
    }

    #[cold]
    #[track_caller]
    pub fn panic_code(&self) -> ! {
        abort!("LibC: Code {}", self.0)
    }

    pub const fn is_file_not_found(&self) -> bool {
        self.0 == 0x2
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ptr = strerror(self.0);
        // SAFETY: libs are guaranteed to store a valid cstr
        let c_str = unsafe { CStr::from_ptr(ptr) };
        write!(f, "{c_str:?}")
    }
}

impl core::error::Error for ErrorCode {}