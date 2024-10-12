use std::env;
use std::path::PathBuf;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

fn submodules() -> PathBuf {
    manifest_dir().join("submodules")
}

fn main() {
    let manifestdir = manifest_dir();
    let manifestdir_str = manifestdir.to_string_lossy();
    let submodules = submodules();
    let submodules_str = submodules.to_string_lossy();

    //println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-arg=/usr/lib/aarch64-linux-gnu/libboost_program_options.so.1.74.0");

    println!("cargo:rustc-link-arg=/usr/local/lib/aarch64-linux-gnu/rpicam_app.so");
    println!("cargo:rustc-link-lib=camera");
    println!("cargo:rustc-link-lib=camera-base");
    println!("cargo:rustc-link-lib=camera-base");

    // Tell cargo to look for shared libraries in the specified directory
    println!("{}", format!("cargo:rustc-link-search=native={submodules_str}/libcamlite/build"));
    println!("cargo:rustc-link-lib=camlite");
    //println!("{}", format!("cargo:rustc-link-arg={submodules_str}/libcamlite/build/libcamlite.a"));

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("cxx/wrap.h")
        .enable_cxx_namespaces()
        .allowlist_type("libcamlite::StreamFormat")
        .allowlist_type("libcamlite::StreamParams")
        .allowlist_type("libcamlite::H264Params")
        .allowlist_type("libcamlite::LowResParams")
        .allowlist_type("libcamlite::H264Callback")
        .allowlist_type("libcamlite::LowResCallback")
        .allowlist_function("libcamlite::setupH264Stream")
        .allowlist_function("libcamlite::setupLowresStream")
        .allowlist_function("libcamlite::start")
        .allowlist_function("libcamlite::stop")
        .blocklist_type("std")
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{submodules_str}/libcamlite/src"))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");


    cpp_build::Config::new()
        .include(submodules.join("libcamlite/src"))
        .flag("-fPIC")
        .flag("-std=c++20")
        .debug(true)
        .build("src/lib.rs");

}
