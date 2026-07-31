use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // ENV: TARGET_ARCH
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:rustc-env=TARGET_ARCH={}", target_arch);

    // ENV: BUILD_TIME
    let build_time = chrono::Local::now().format("%b %d %Y, %H:%M:%S").to_string();
    println!("cargo:rustc-env=COMPILE_TIME={}", build_time);

    // ENV: TARGET_OS
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:rustc-env=TARGET_OS={}", target_os);

    // ENV: RUSTC_VERSION
    let mut rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());
    
    if let Some(idx) = rustc_version.find("(") {
        rustc_version = rustc_version[..idx - 1].trim().to_string()
    } 

    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version.trim());
}