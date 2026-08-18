use std::fs;

fn main() {
    let mut args = vec![
        "--out", "../src/os/windows.rs",
        "--flat",
        "--sys",
        "--flat",
        "--no-deps",
        "--filter"
    ];

    let binding = fs::read_to_string("../winlinks.txt")
        .unwrap_or_default();
    
    let links: Vec<&str> = binding
        .lines()
        .filter(|s| !s.is_empty())
        .collect();
    
    args.extend(links);

    println!("{args:#?}");
    let warnings = windows_bindgen::bindgen(args);
    println!("Warnings: {warnings}");
}