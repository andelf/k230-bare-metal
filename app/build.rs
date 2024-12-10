use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search=native={}", out_dir.display());

    fs::write(out_dir.join("link-cpu1.x"), include_str!("link-cpu1.x")).unwrap();

    // The same effect as .cargo/config.toml, but in build.rs
    println!("cargo::rustc-link-arg=-Tlink-cpu1.x");

    // println!("cargo:rerun-if-changed=link.x");
}
