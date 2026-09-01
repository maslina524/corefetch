use std::{
    fs,
    env,
    path::PathBuf,
    process::Command,
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use chrono::{DateTime, FixedOffset};
use zlib_rs::{
    ReturnCode,
    DeflateConfig,
    compress_bound,
    compress_slice,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::{Client, Method, Request, Url};
use serde_json::{Value, Map};
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
    pub sha_small: String,
    pub message: String,
    pub files: u64,
    pub added: u64,
    pub deleted: u64,
    pub total: u64,
}

impl Commit {
    pub fn new_git() -> Self {
        let log = Command::new("git")
            .args([
                "log",
                "--format=%an%n%ae%n%ad%n%H%n%h%n%s",
                "--date=format:%b %d %Y, %H:%M:%S",
                "-1",
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
        let sha_small = log_parts[4].to_owned();
        let message = log_parts[5].to_owned();

        let numstat = Command::new("git")
            .args(["log", "--format=", "--shortstat", "-1"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .expect("Failed to call Git shortstat");

        let re = Regex::new(r"(\d+)").unwrap();
        let nums: Vec<u64> = re
            .find_iter(&numstat)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();

        assert!(nums.len() >= 3, "Failed to call Git shortstat");

        let files = nums[0];
        let added = nums[1];
        let deleted = nums[2];
        let total = added + deleted;

        Self {
            author,
            email,
            date,
            date_small,
            sha,
            sha_small,
            message,
            files,
            added,
            deleted,
            total,
        }
    }

    pub async fn new_github() -> Self {
        let response = Self::request().await;
        let root = response.as_object().expect("Incorrect response data");

        let sha = get_string(root, "sha");
        let sha_small = sha[..7].to_owned();

        let commit_obj = get_object(root, "commit");
        let message = get_string(&commit_obj, "message");

        let author_obj = get_object(&commit_obj, "author");
        let author = get_string(&author_obj, "name");
        let email = get_string(&author_obj, "email");

        let date_raw = get_string(&author_obj, "date");
        let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&date_raw)
            .expect("Incorrect date format");

        let date = dt.format("%b %d %Y, %H:%M:%S").to_string();
        let date_small = dt.format("%b %d %Y").to_string();

        let stats_obj = get_object(root, "stats");
        let added = get_u64(&stats_obj, "additions");
        let deleted = get_u64(&stats_obj, "deletions");
        let total = get_u64(&stats_obj, "total");

        let files_array = get_array(root, "files");
        let files = files_array.len() as u64;

        Self {
            author,
            email,
            date,
            date_small,
            sha,
            sha_small,
            message,
            files,
            added,
            deleted,
            total,
        }
    }

    async fn request() -> Value {
        let req = Request::new(
            Method::GET,
            Url::from_str("https://api.github.com/repos/maslina524/corefetch/commits/main?per_page=1").unwrap(),
        );

        let client = Client::builder()
            .user_agent("corefetch-build/1.0")
            .build()
            .expect("Failed to build http client");

        let resp = client
            .execute(req)
            .await
            .expect("Failed to call Github Api");

        println!("{resp:#?}");
        resp.json().await.expect("Failed to parse json response from GitHub")
    }
}

fn get_object(obj: &Map<String, Value>, key: &str) -> Map<String, Value> {
    obj.get(key)
        .unwrap_or_else(|| panic!("Key `{key}` not found"))
        .as_object()
        .expect("Incorrect response data")
        .clone()
}

fn get_array(obj: &Map<String, Value>, key: &str) -> Vec<Value> {
    obj.get(key)
        .unwrap_or_else(|| panic!("Key `{key}` not found"))
        .as_array()
        .expect("Incorrect response data")
        .clone()
}

fn get_string(obj: &Map<String, Value>, key: &str) -> String {
    obj.get(key)
        .unwrap_or_else(|| panic!("Key `{key}` not found"))
        .as_str()
        .expect("Incorrect response data")
        .to_owned()
}

fn get_u64(obj: &Map<String, Value>, key: &str) -> u64 {
    obj.get(key)
        .unwrap_or_else(|| panic!("Key `{key}` not found"))
        .as_u64()
        .expect("Incorrect response data")
}

fn git_initialized() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .is_some_and(|s| s.trim() == "true")
}

fn github_actions() -> bool {
    env::var("GITHUB_ACTIONS")
        .is_ok_and(|s| s.trim() == "true")
}

#[cfg(target_os = "linux")]
fn get_libc_version() -> String {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let c_file = out_dir.join("version.c");
    let exe = out_dir.join("version");

    fs::write(&c_file, r#"
        #include <stdio.h>

        int main() {
            printf("%d.%d\n", __GLIBC__, __GLIBC_MINOR__);
            return 0;
        }
    "#).unwrap();

    let status = Command::new("gcc")
        .arg(&c_file)
        .arg("-o")
        .arg(&exe)
        .status()
        .expect("failed to compile C program");

    assert!(status.success(), "Compilation failed");
    
    let output = Command::new(&exe)
        .output()
        .expect("failed to run program");

    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() {
    // Bypasses caching, runs every time during compilation
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-env=LOGO_OUT_DIR={}", out_dir.display());
    let trigger_file = out_dir.join(format!("build_trigger_{timestamp}"));
    fs::write(&trigger_file, &timestamp).unwrap();

    println!("cargo:rerun-if-changed={}", trigger_file.display());
    println!("cargo:rerun-if-env-changed=BUILD_TIMESTAMP_{timestamp}");

    // ENV: TARGET_ARCH
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:rustc-env=TARGET_ARCH={target_arch}");

    // ENV: BUILD_TIME
    let build_time = chrono::Local::now().format("%b %d %Y, %H:%M:%S").to_string();
    println!("cargo:rustc-env=COMPILE_TIME={build_time}");

    // ENV: TARGET_OS
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
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
    let commit = if !git_initialized() || github_actions() {
        Commit::new_github().await
    } else {
        Commit::new_git()
    };
    println!("cargo:rustc-env=COMMIT_AUTHOR={}", commit.author);
    println!("cargo:rustc-env=COMMIT_EMAIL={}", commit.email);
    println!("cargo:rustc-env=COMMIT_DATE={}", commit.date);
    println!("cargo:rustc-env=COMMIT_DATE_SMALL={}", commit.date_small);
    println!("cargo:rustc-env=COMMIT_SHA={}", commit.sha);
    println!("cargo:rustc-env=COMMIT_SHA_SMALL={}", commit.sha_small);
    println!("cargo:rustc-env=COMMIT_MESSAGE={}", commit.message);
    println!("cargo:rustc-env=COMMIT_FILES={}", commit.files);
    println!("cargo:rustc-env=COMMIT_ADDED={}", commit.added);
    println!("cargo:rustc-env=COMMIT_DELETED={}", commit.deleted);
    println!("cargo:rustc-env=COMMIT_TOTAL={}", commit.total);

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

    // ENV: LIBC_VERSION
    #[cfg(target_os = "windows")]
    let ver = String::new();
    #[cfg(target_os = "linux")]
    let ver = get_libc_version();
    println!("cargo:rustc-env=LIBC_VERSION={ver}");


    // COMPRESS LOGOS
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
        let dest_dir = out_dir.join("temp").join(letter);
        fs::create_dir_all(&dest_dir).ok();
        let dest_path = dest_dir.join(path.file_name().unwrap());
        fs::write(dest_path, &*compressed).ok();
    });

    let raw = raw_bytes_len.load(Ordering::Relaxed);
    let encoded = encoded_bytes_len.load(Ordering::Relaxed);

    #[allow(clippy::cast_precision_loss)]
    {
        println!(
            "cargo:warning={}b -> {}b = {:.02}%",
            raw,
            encoded,
            (encoded as f64 / raw as f64).mul_add(-100.0, 100.0)
        );
    }

    let is_nightly = rustc_version.contains("nightly") || rustc_version.contains("dev");
    if !is_nightly {
        println!("cargo:warning=The project is not being built in the nightly version, this will not affect the result");
        println!("cargo:warning=in any way, but the local `todo` crate will not work; if you are going to work");
        println!("cargo:warning=on the project (contribute), it is better to \x1b[4minstall the nightly version\x1b[0m:");
        println!("cargo:warning=");
        println!("cargo:warning=\x1b[36m$ rustup install nightly\x1b[0m");
        println!("cargo:warning=\x1b[36m$ rustup override set nightly\x1b[0m");
        println!("cargo:warning=");
        println!("cargo:warning=\x1b[36m$ rustup component add rustfmt --toolchain nightly\x1b[0m");
        println!("cargo:warning=\x1b[36m$ rustup component add clippy --toolchain nightly\x1b[0m");
    }
}