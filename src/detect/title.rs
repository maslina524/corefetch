use alloc::string::String;

use crate::{
    os::windows::GetUserNameW,
    os::path::Path,
    sync::OnceLock
};

static USER_NAME: OnceLock<String> = OnceLock::new();

const UNLEN     : usize            = 256;

pub fn user_name() -> &'static String {
    USER_NAME.get_or_init(|| {
        let mut buf = [0u16; UNLEN + 1];
        let mut size = (UNLEN + 1) as u32;
        
        // SAFETY: Completely safe
        let ret = unsafe {
            GetUserNameW(
                (&raw mut buf).cast(),
                &raw mut size
            )
        };
        if ret == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&buf)
    })
}