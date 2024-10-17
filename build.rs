use std::env;
use std::path::PathBuf;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn submodules() -> PathBuf {
    manifest_dir().join("submodules")
}

fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("cxx/wrap.cxx")
        .std("c++20")
        .debug(true)
        .compile("rslibcamlite");

    let submodules = submodules();
    let submodules_str = submodules.to_string_lossy();

    println!("cargo:rustc-link-arg=/usr/lib/aarch64-linux-gnu/libboost_program_options.so.1.74.0");
    println!("cargo:rustc-link-arg=/usr/local/lib/aarch64-linux-gnu/rpicam_app.so");
    println!("cargo:rustc-link-lib=camera");
    println!("cargo:rustc-link-lib=camera-base");
    println!("{}", format!("cargo:rustc-link-search=native={submodules_str}/libcamlite/build"));
    println!("cargo:rustc-link-lib=dylib=camlite");

    println!("cargo:rerun-if-changed=cxx/wrap.h");
    println!("cargo:rerun-if-changed=cxx/wrap.cxx");
    println!("cargo:rerun-if-changed=src/main.rs");

}




