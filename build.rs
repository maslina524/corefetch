use std::{
    fs, 
    path::PathBuf, 
    process::Command
};

use zlib_rs::{
    ReturnCode,
    DeflateConfig, 
    compress_bound, 
    compress_slice,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const VALID_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_'
];

fn main() {
    // Bypasses caching, runs every time during compilation
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();
    
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let trigger_file = format!("{out_dir}/build_trigger_{timestamp}");
    fs::write(&trigger_file, &timestamp).unwrap();
    
    println!("cargo:rerun-if-changed={trigger_file}");
    println!("cargo:rerun-if-env-changed=BUILD_TIMESTAMP_{timestamp}");
    
    // ENV: TARGET_ARCH
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:rustc-env=TARGET_ARCH={target_arch}");

    // ENV: BUILD_TIME
    let build_time = chrono::Local::now().format("%b %d %Y, %H:%M:%S").to_string();
    println!("cargo:rustc-env=COMPILE_TIME={build_time}");

    // ENV: TARGET_OS
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:rustc-env=TARGET_OS={target_os}");

    // ENV: RUSTC_VERSION
    let mut rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());
    
    if let Some(idx) = rustc_version.find('(') {
        rustc_version = rustc_version[..idx - 1].trim().to_string();
    }

    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version.trim());

    // Compress logos
    let base_path = PathBuf::from("src/logo");
    let mut all_paths = Vec::new();
    for letter in VALID_CHARS {
        let letter_path = base_path.join(letter.to_string());
        if let Ok(entries) = fs::read_dir(letter_path) {
            for entry in entries.flatten() {
                all_paths.push(entry.path());
            }
        }
    }

    all_paths.par_iter().for_each(|path| {
        let content = fs::read(path).unwrap();
        let mut compressed_buf = vec![0u8; compress_bound(content.len())];
        let (compressed, rc) = compress_slice(&mut compressed_buf, &content, DeflateConfig::default());
        assert_eq!(rc, ReturnCode::Ok);
        
        let letter = path.parent().and_then(|p| p.file_name()).unwrap().to_str().unwrap();
        let dest_dir = format!("temp/{letter}");
        fs::create_dir_all(&dest_dir).ok();
        let dest_path = PathBuf::from(dest_dir).join(path.file_name().unwrap());
        fs::write(dest_path, &*compressed).ok();
        
        #[allow(clippy::cast_precision_loss)]
        {
            println!(
                "cargo:warning={}: {}b -> {}b = {:.02}%",
                path.display(),
                content.len(), 
                compressed.len(),
                (compressed.len() as f64 / content.len() as f64).mul_add(-100.0, 100.0)
            );
        }
    });
}