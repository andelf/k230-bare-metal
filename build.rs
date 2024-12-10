use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=link.x");

    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("OUT_DIR").unwrap()
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("link.x", out_dir.join("link.x")).unwrap();

    // The same effect as .cargo/config.toml
    println!("cargo::rustc-link-arg=-Tlink.x");
}
