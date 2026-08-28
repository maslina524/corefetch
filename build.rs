use std::{
    fs, 
    path::PathBuf, 
    process::Command,
    sync::atomic::{AtomicUsize, Ordering}
};

use zlib_rs::{
    ReturnCode,
    DeflateConfig, 
    compress_bound, 
    compress_slice,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

const VALID_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_'
];

struct Commit {
    pub author: String,
    pub email: String,
    pub date: String,
    pub date_small: String,
    pub sha: String,
    pub sha_short: String,
    pub message: String,
    pub files: usize,
    pub added: usize,
    pub deleted: usize,
    pub total: usize
}

impl Commit {
    pub fn new() -> Self {
        let log = Command::new("git")
            .args([
                "log",
                "--format=%an%n%ae%n%ad%n%H%n%h%n%s",
                "--date=format:%b %d %Y, %H:%M:%S", 
                "-1"
            ])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .expect("Failed to call Git log");
        
        let log_parts: Vec<&str> = log.split('\n').collect();
        
        assert!(log_parts.len() >= 6, "Failed to call Git log: {log_parts:?}");
        
        let author = log_parts[0].to_owned();
        let email = log_parts[1].to_owned();
        let date = log_parts[2].to_owned();
        let date_small = date[..date.find(',').unwrap()].to_owned();
        let sha = log_parts[3].to_owned();
        let sha_short = log_parts[4].to_owned();
        let message = log_parts[5].to_owned();

        let numstat = Command::new("git")
            .args([
                "log",
                "--format=",
                "--shortstat",
                "-1"
            ])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .expect("Failed to call Git shortstat");
        
        let re = Regex::new(r"(\d+)").unwrap();
        let nums: Vec<usize> = re
            .find_iter(&numstat)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        assert!(nums.len() >= 3, "Failed to call Git shortstat");

        let files = nums[0];
        let added = nums[1];
        let deleted = nums[2];
        let total = added + deleted;

        Self {
            author, email, date, date_small,
            sha, sha_short, message,
            files, added, deleted, total
        }
    }
}

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

    // ENV: CARGO_VERSION
    let mut cargo_version = Command::new("cargo")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string());
    
    if let Some(idx) = cargo_version.find('(') {
        cargo_version = cargo_version[..idx - 1].trim().to_string();
    }
    println!("cargo:rustc-env=CARGO_VERSION={}", cargo_version.trim());

    // ENV: Commit
    let commit = Commit::new();
    println!("cargo:rustc-env=COMMIT_AUTHOR={}",     commit.author);
    println!("cargo:rustc-env=COMMIT_EMAIL={}",      commit.email);
    println!("cargo:rustc-env=COMMIT_DATE={}",       commit.date);
    println!("cargo:rustc-env=COMMIT_DATE_SMALL={}", commit.date_small);
    println!("cargo:rustc-env=COMMIT_SHA={}",        commit.sha);
    println!("cargo:rustc-env=COMMIT_SHA_SHORT={}",  commit.sha_short);
    println!("cargo:rustc-env=COMMIT_MESSAGE={}",    commit.message);
    println!("cargo:rustc-env=COMMIT_FILES={}",      commit.files);
    println!("cargo:rustc-env=COMMIT_ADDED={}",      commit.added);
    println!("cargo:rustc-env=COMMIT_DELETED={}",    commit.deleted);
    println!("cargo:rustc-env=COMMIT_TOTAL={}",      commit.total);

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

    let raw_bytes_len = AtomicUsize::new(0);
    let encoded_bytes_len = AtomicUsize::new(0);

    all_paths.par_iter().for_each(|path| {
        let content = fs::read(path).unwrap();
        raw_bytes_len.fetch_add(content.len(), Ordering::Relaxed);

        let mut compressed_buf = vec![0u8; compress_bound(content.len())];
        let (compressed, rc) = compress_slice(&mut compressed_buf, &content, DeflateConfig::default());
        encoded_bytes_len.fetch_add(compressed.len(), Ordering::Relaxed);
        assert_eq!(rc, ReturnCode::Ok);
        
        let letter = path.parent().and_then(|p| p.file_name()).unwrap().to_str().unwrap();
        let dest_dir = format!("temp/{letter}");
        fs::create_dir_all(&dest_dir).ok();
        let dest_path = PathBuf::from(dest_dir).join(path.file_name().unwrap());
        fs::write(dest_path, &*compressed).ok();
        
        // #[allow(clippy::cast_precision_loss)]
        // {
        //     println!(
        //         "cargo:warning={}: {}b -> {}b = {:.02}%",
        //         path.display(),
        //         content.len(), 
        //         compressed.len(),
        //         (compressed.len() as f64 / content.len() as f64).mul_add(-100.0, 100.0)
        //     );
        // }
    });

    let raw = raw_bytes_len.load(Ordering::Relaxed);
    let encoded = encoded_bytes_len.load(Ordering::Relaxed);
    
    #[allow(clippy::cast_precision_loss)]
    {
        println!("cargo:warning={}b -> {}b = {:.02}%",
            raw, 
            encoded,
            (encoded as f64 / raw as f64).mul_add(-100.0, 100.0)
        );
    }
}