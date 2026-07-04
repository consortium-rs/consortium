#[cfg(target_os = "linux")]
fn main() {
    use std::{env, path::PathBuf};

    let vendor = PathBuf::from("vendor/include");

    // Native libraries Cog Core is built against. Probing each one emits the
    // `cargo:rustc-link-lib`/`rustc-link-search` directives the final binary
    // needs and yields the include paths we compile the vendored sources with.
    // These are the `Requires:`/`dependencies` of Cog's own `meson.build`.
    let deps = [
        "wpe-webkit-2.0",
        "wpe-1.0",
        "gio-2.0",
        "gobject-2.0",
        "gmodule-2.0",
        "glib-2.0",
        "libsoup-3.0",
    ];
    let mut include_paths = vec![vendor.clone()];
    for dep in deps {
        let lib = pkg_config::probe_library(dep)
            .unwrap_or_else(|e| panic!("pkg-config could not find `{dep}`: {e}"));
        include_paths.extend(lib.include_paths);
    }

    // Compile the vendored Cog Core C sources into a static archive we own,
    // rather than linking a system `libcogcore` whose version/API need not match
    // these headers (e.g. the crate calls `cog_init`/`cog_view_new`, which the
    // packaged Cog does not export). The source list mirrors `cogcore_sources`
    // in the vendored `meson.build`; `cog-gamepad-manette.c` is omitted because
    // the checked-in `cog-config.h` sets `COG_ENABLE_GAMEPAD_MANETTE 0`.
    let sources = [
        "cog-directory-files-handler.c",
        "cog-host-routes-handler.c",
        "cog-modules.c",
        "cog-platform.c",
        "cog-fallback-platform.c",
        "cog-prefix-routes-handler.c",
        "cog-request-handler.c",
        "cog-shell.c",
        "cog-utils.c",
        "cog-webkit-utils.c",
        "cog-gamepad.c",
        "cog-view.c",
        "cog-viewport.c",
    ];

    let mut build = cc::Build::new();
    build
        .define("G_LOG_DOMAIN", "\"Cog-Core\"")
        // Cog's internal headers refuse direct inclusion unless this is set; the
        // library build (unlike public consumers) compiles the sources with it.
        .define("COG_INSIDE_COG__", "1")
        // Select the WPE WebKit 2.x (`wpe-webkit-2.0`) API paths. Upstream meson
        // derives this from the detected WebKit version; the probed dependency is
        // the 2.0 API, whose errors use `WEBKIT_MEDIA_ERROR` (the pre-WPE2
        // `WEBKIT_PLUGIN_ERROR` was removed).
        .define("COG_USE_WPE2", "1")
        .warnings(false);
    for inc in &include_paths {
        build.include(inc);
    }
    for src in sources {
        let path = vendor.join(src);
        println!("cargo:rerun-if-changed={}", path.display());
        build.file(path);
    }
    // A distinct archive name so this never resolves against a system
    // `-lcogcore` on the link line.
    build.compile("cogcore_bundled");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=vendor/include/cog.h");

    // Generate bindings from the same vendored headers we just compiled.
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for inc in &include_paths {
        builder = builder.clang_arg(format!("-I{}", inc.display()));
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
