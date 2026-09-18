use alloc::string::String;

use crate::{
    format,
    base64,
    imp::io::{stdout, write}
};

pub fn print_png(png: &[u8], cols: Option<usize>, rows: Option<usize>, image_id: u32) {
    let b64 = base64::encode(png);

    let mut ctrl = String::from("a=T,f=100,C=1");
    if image_id != 0 {
        ctrl.push_str(&format!(",i={}", image_id));
    }
    if let Some(c) = cols {
        ctrl.push_str(&format!(",c={}", c));
    }
    if let Some(r) = rows {
        ctrl.push_str(&format!(",r={}", r));
    }

    const CHUNK: usize = 4096;
    let bytes = b64.as_bytes();

    let mut first = true;
    let mut offset = 0usize;
    while offset < bytes.len() {
        let end = (offset + CHUNK).min(bytes.len());
        let more = end < bytes.len();
        let m = if more { 1 } else { 0 };

        if first {
            let ctrl_with_m = format!("{},m={}", ctrl, m);
            write(stdout(), format!("\x1b_G{};", ctrl_with_m).as_bytes());
            first = false;
        } else {
            write(stdout(), format!("\x1b_Gm={};", m).as_bytes());
        }

        write(stdout(), &bytes[offset..end]);
        write(stdout(), b"\x1b\\");

        offset = end;
    }
}