use std::{env, path::PathBuf};

fn main() {
    pkg_config::probe_library("cogc`ore").unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");

    let vendor = PathBuf::from("vendor");

    // Tell Cargo where to find the static library
    println!("cargo:rustc-link-search=native={}", vendor.join("lib").display());
    println!("cargo:rustc-link-lib=static=cogcore");

    // Re-run if headers change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=vendor/include/cogcore.h");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", vendor.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("failed to write bindings");
}