#[cfg(target_os = "linux")]
fn main() {
    use std::{env, path::PathBuf};

    let cogcore = pkg_config::probe_library("cogcore").unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");

    let vendor = PathBuf::from("vendor");

    // Tell Cargo where to find the static library
    println!(
        "cargo:rustc-link-search=native={}",
        vendor.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=cogcore");

    // Re-run if headers change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=vendor/include/cog.h");

    let search_path =
        std::env::var("COGCORE_LIB_DIR").unwrap_or_else(|_| "/usr/local/lib".to_string());

    println!("cargo:rustc-link-search=native={search_path}");
    println!("cargo:rustc-link-lib=dylib=cogcore");

    // Generate bindings
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", vendor.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    for include_path in cogcore.include_paths {
        builder = builder.clang_arg(format!("-I{}", include_path.display()));
    }

    let bindings = builder.generate().expect("failed to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("failed to write bindings");
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("cogcore-sys only supports Linux");
}
