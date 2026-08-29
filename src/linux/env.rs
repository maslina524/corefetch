use core::slice;

use alloc::{
    string::{String, ToString},
    vec::Vec
};

#[allow(clippy::similar_names, reason = "that's what they're called in C, i don't give a fuck about clippy")]
pub fn args(argc: usize, argv: *const *const u8) -> Vec<String> {
    let mut ret = Vec::new();
    for i in 0..argc {
        // SAFETY: Moving strictly within the allocated memory by Linux
        let start_ptr = unsafe { *argv.add(i as usize) };

        let mut len = 0;
        // SAFETY: Moving until the end of the allocated string
        while unsafe { *start_ptr.add(len) } != 0 {
            len += 1;
        }

        // SAFETY: All parameters were correctly calculated earlier.
        let slice = unsafe { slice::from_raw_parts(start_ptr, len) };
        let string = String::from_utf8_lossy(slice).to_string();
        ret.push(string);
    }

    ret
}