use std::{
    fs, path::PathBuf, process::Command, str::FromStr, sync::atomic::{AtomicUsize, Ordering}
};

use zlib_rs::{
    ReturnCode,
    DeflateConfig, 
    compress_bound, 
    compress_slice,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{Client, Method, Request, Url};
use serde_json::Value;

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
    pub async fn new() -> Self {
        let response = Self::request().await;
        let root = response.as_object().expect("Incorrect response data");

        Self {
            author, email, date, date_small,
            sha, sha_short, message,
            files, added, deleted, total
        }
    }

    async fn request() -> Value {
        let req = Request::new(
            Method::GET, 
            Url::from_str("https://api.github.com/repos/maslina524/nofetch/commits/main").unwrap()
        );

        let client = Client::new();
        let resp = client
            .execute(req)
            .await
            .expect("Failed to call Github Api");

        resp
            .json()
            .await
            .expect("Failed to parse json response from GitHub")
    }
}

#[tokio::main]
async fn main() {
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
    let commit = Commit::new().await;
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