// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Compiles a generated device tree to a `.dtb`/`.dtbo` with `gcc -E` + `dtc`.
//!
//! The generated `.dts`/`.dtso` still `#include`s vendor `.dtsi` (which
//! Consortium vendors, BSD/MIT) and `dt-bindings/*.h` (which it does **not** —
//! those are GPL-2.0 kernel headers). So compilation is opt-in via `[dts] linux`
//! pointing at the user's own kernel checkout: the vendored closure is
//! materialized into a temp dir, and the kernel's `include/` supplies the
//! `dt-bindings` macros. Consortium never bundles the GPL-2.0 headers.
//!
//! The two-step invocation mirrors the kernel's own device-tree build:
//! `gcc -E -nostdinc -I <tmp> -I <linux>/include -I <linux>/arch/<arch>/boot/dts
//! -undef -D__DTS__ -x assembler-with-cpp` then `dtc -@ -I dts -O dtb`.

use std::path::{Path, PathBuf};

use thiserror::Error;
use tokio::process::Command;

/// Errors from compiling a device tree.
#[derive(Error, Debug)]
pub enum DtcError {
    /// A filesystem operation failed.
    #[error("device-tree compile I/O error at {path}: {source}")]
    Io {
        /// The path being operated on.
        path: String,
        /// The underlying error.
        source: std::io::Error,
    },

    /// The configured `[dts] linux` kernel tree is missing its `include/`
    /// directory (so it cannot supply `dt-bindings` headers).
    #[error(
        "`[dts] linux` = {path} does not look like a kernel tree (no `include/` directory); \
         point it at a Linux source checkout that provides dt-bindings headers"
    )]
    LinuxTreeInvalid {
        /// The configured path.
        path: String,
    },

    /// `gcc` (the preprocessor) was not found on PATH.
    #[error("`gcc` not found on PATH; needed to preprocess the device tree before `dtc`")]
    GccNotFound,

    /// `dtc` (the device-tree compiler) was not found on PATH.
    #[error("`dtc` not found on PATH; install the device-tree compiler to build `.dtb`s")]
    DtcNotFound,

    /// `gcc`/`dtc` rejected the device tree.
    #[error("device tree {name} failed to compile:\n{message}")]
    Compile {
        /// The source file name.
        name: String,
        /// The `gcc`/`dtc` diagnostics.
        message: String,
    },
}

fn io(path: impl AsRef<Path>, source: std::io::Error) -> DtcError {
    DtcError::Io {
        path: path.as_ref().display().to_string(),
        source,
    }
}

/// The kernel `arch/<arch>` directory name for a Linux application target
/// triple (`aarch64-*` → `arm64`, `arm*`/`thumb*` → `arm`).
pub fn arch_dir(app_target: &str) -> &'static str {
    if app_target.starts_with("aarch64") {
        "arm64"
    } else {
        "arm"
    }
}

/// Compiles the device tree `dts_dir/<dts_name>` to a sibling `.dtb` (for a
/// `.dts`) or `.dtbo` (for a `.dtso`), returning the output path.
///
/// `closure` is the vendored base-tree include closure (from
/// [`consortium_data::dts::base_closure`]), materialized into a temp dir so the
/// generated tree's quoted `#include`s resolve; `linux_dir` is the user's kernel
/// tree supplying `dt-bindings`; `arch` selects `arch/<arch>/boot/dts` (see
/// [`arch_dir`]).
pub async fn compile_device_tree(
    dts_dir: &Path,
    dts_name: &str,
    closure: &[(String, String)],
    linux_dir: &Path,
    arch: &str,
) -> Result<PathBuf, DtcError> {
    consortium_log::trace!("compiling device tree {} (arch {})", dts_name, arch);
    let include = linux_dir.join("include");
    if !include.is_dir() {
        consortium_log::trace!("linux include dir missing at {}", include.display());
        return Err(DtcError::LinuxTreeInvalid {
            path: linux_dir.display().to_string(),
        });
    }

    // A private build dir next to the staged tree holds the materialized vendor
    // closure and the preprocessed intermediate; it is removed on the way out.
    let work = dts_dir.join(".dtc-build");
    let _ = tokio::fs::remove_dir_all(&work).await;
    tokio::fs::create_dir_all(&work)
        .await
        .map_err(|e| io(&work, e))?;

    for (name, contents) in closure {
        let dest = work.join(name);
        tokio::fs::write(&dest, contents)
            .await
            .map_err(|e| io(&dest, e))?;
    }
    // The generated tree compiles from the work dir so its quoted includes see
    // the materialized closure.
    let staged = dts_dir.join(dts_name);
    let src = work.join(dts_name);
    tokio::fs::copy(&staged, &src)
        .await
        .map_err(|e| io(&staged, e))?;

    let stem = dts_name
        .rsplit_once('.')
        .map(|(s, _)| s)
        .unwrap_or(dts_name);
    let is_overlay = dts_name.ends_with(".dtso");
    let out_name = format!("{stem}.{}", if is_overlay { "dtbo" } else { "dtb" });

    // Step 1: preprocess with the C preprocessor, resolving the vendored closure
    // (temp dir) and the user's kernel dt-bindings (`<linux>/include`).
    let preprocessed = work.join(format!("{stem}.pp.dts"));
    let boot_dts = linux_dir.join("arch").join(arch).join("boot").join("dts");
    let gcc = Command::new("gcc")
        .args(["-E", "-nostdinc"])
        .arg("-I")
        .arg(&work)
        .arg("-I")
        .arg(&include)
        .arg("-I")
        .arg(&boot_dts)
        .args(["-undef", "-D__DTS__", "-x", "assembler-with-cpp"])
        .arg(&src)
        .arg("-o")
        .arg(&preprocessed)
        .output()
        .await
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => DtcError::GccNotFound,
            _ => io(&src, e),
        })?;
    if !gcc.status.success() {
        consortium_log::trace!("gcc preprocess failed for {}", dts_name);
        let _ = tokio::fs::remove_dir_all(&work).await;
        return Err(DtcError::Compile {
            name: dts_name.to_string(),
            message: String::from_utf8_lossy(&gcc.stderr).into_owned(),
        });
    }

    // Step 2: compile the preprocessed tree. `-@` keeps `__symbols__` so an
    // overlay's phandle fixups resolve when applied.
    let out = dts_dir.join(&out_name);
    let dtc = Command::new("dtc")
        .args(["-@", "-I", "dts", "-O", "dtb", "-o"])
        .arg(&out)
        .arg(&preprocessed)
        .output()
        .await
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => DtcError::DtcNotFound,
            _ => io(&preprocessed, e),
        });

    let _ = tokio::fs::remove_dir_all(&work).await;
    let dtc = dtc?;
    if !dtc.status.success() {
        consortium_log::trace!("dtc compile failed for {}", dts_name);
        return Err(DtcError::Compile {
            name: dts_name.to_string(),
            message: String::from_utf8_lossy(&dtc.stderr).into_owned(),
        });
    }

    consortium_log::trace!("device tree compiled to {}", out.display());
    Ok(out)
}

/// The message printed when the device tree is emitted but not compiled (no
/// `[dts] linux`), telling the user how to produce the `.dtb` themselves.
///
/// `yocto` selects the guidance flavor: a Yocto build compiles the tree through
/// the kernel recipe; otherwise the exact `gcc -E` + `dtc` commands are shown.
pub fn compilation_hint(dts_name: &str, arch: &str, yocto: bool) -> String {
    let stem = dts_name
        .rsplit_once('.')
        .map(|(s, _)| s)
        .unwrap_or(dts_name);
    let is_overlay = dts_name.ends_with(".dtso");
    let out_ext = if is_overlay { "dtbo" } else { "dtb" };

    let license = "  Note: dt-bindings headers are GPL-2.0; Consortium does not bundle them, \
                   so supply your own Linux tree.";

    if yocto {
        format!(
            "device tree dist/dts/linux/{dts_name} was emitted but not compiled.\n\
             Set `[dts] linux = \"…\"` to a kernel checkout to compile it here, or let your\n\
             Yocto kernel build compile it: add the {ext} to your kernel recipe (extend\n\
             KERNEL_DEVICETREE) and `bitbake virtual/kernel`.\n{license}",
            ext = if is_overlay { "overlay" } else { ".dts" },
        )
    } else {
        format!(
            "device tree dist/dts/linux/{dts_name} was emitted but not compiled.\n\
             Set `[dts] linux = \"…\"` to a kernel checkout to compile it here, or run it\n\
             yourself (replace ~/src/linux with your kernel tree):\n\
             \x20 gcc -E -nostdinc -I <vendor-dts-dir> -I ~/src/linux/include \\\n\
             \x20     -I ~/src/linux/arch/{arch}/boot/dts -undef -D__DTS__ \\\n\
             \x20     -x assembler-with-cpp -o {stem}.pp.dts {dts_name}\n\
             \x20 dtc -@ -I dts -O dtb -o {stem}.{out_ext} {stem}.pp.dts\n{license}",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_dir_maps_triples() {
        assert_eq!(arch_dir("aarch64-unknown-linux-gnu"), "arm64");
        assert_eq!(arch_dir("armv7-unknown-linux-gnueabihf"), "arm");
    }

    #[test]
    fn invalid_linux_tree_is_rejected() {
        let dir = std::env::temp_dir().join(format!("dtc-nolinux-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let err = rt
            .block_on(compile_device_tree(
                &dir,
                "x.dts",
                &[],
                &dir.join("not-a-kernel"),
                "arm64",
            ))
            .unwrap_err();
        assert!(matches!(err, DtcError::LinuxTreeInvalid { .. }));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn hint_manual_shows_commands() {
        let hint = compilation_hint("consortium-imx95.dts", "arm64", false);
        assert!(hint.contains("gcc -E -nostdinc"));
        assert!(hint.contains("dtc -@ -I dts -O dtb"));
        assert!(hint.contains("consortium-imx95.dtb"));
        assert!(hint.contains("GPL-2.0"));
    }

    #[test]
    fn hint_yocto_points_at_kernel_build() {
        let hint = compilation_hint("consortium-imx95.dts", "arm64", true);
        assert!(hint.contains("bitbake virtual/kernel"));
        assert!(hint.contains("KERNEL_DEVICETREE"));
    }

    /// Whether `gcc` and `dtc` both resolve on PATH; the real-compile test is
    /// skipped when either is absent.
    fn tools_available() -> bool {
        fn on_path(program: &str) -> bool {
            std::process::Command::new(program)
                .arg("--version")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
        on_path("gcc") && on_path("dtc")
    }

    #[test]
    fn compiles_dts_against_a_synthetic_kernel_tree() {
        if !tools_available() {
            eprintln!("skipping: `gcc`/`dtc` not on PATH");
            return;
        }
        let base = std::env::temp_dir().join(format!("dtc-synth-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let dts_dir = base.join("dts");
        std::fs::create_dir_all(&dts_dir).unwrap();

        // A synthetic "kernel tree" that only supplies one dt-bindings header, so
        // the compile exercises the `-I <linux>/include` path without vendoring
        // any GPL-2.0 content.
        let bindings = base.join("linux/include/dt-bindings/test");
        std::fs::create_dir_all(&bindings).unwrap();
        std::fs::write(bindings.join("macros.h"), "#define TEST_VALUE 0x2a\n").unwrap();

        // The generated tree `#include`s a vendored `.dtsi` (a quoted, local
        // include resolved from the materialized closure) and the synthetic
        // angled dt-bindings header.
        let generated = "/dts-v1/;\n\
             #include <dt-bindings/test/macros.h>\n\
             #include \"vendor.dtsi\"\n\
             / {\n\tconsortium { value = <TEST_VALUE>; };\n};\n";
        std::fs::write(dts_dir.join("consortium-x.dts"), generated).unwrap();
        let closure = vec![(
            "vendor.dtsi".to_string(),
            "/ {\n\tvendor { ok; };\n};\n".to_string(),
        )];

        let rt = tokio::runtime::Runtime::new().unwrap();
        let out = rt
            .block_on(compile_device_tree(
                &dts_dir,
                "consortium-x.dts",
                &closure,
                &base.join("linux"),
                "arm64",
            ))
            .expect("synthetic compile succeeds");

        assert_eq!(out, dts_dir.join("consortium-x.dtb"));
        // A real `.dtb` starts with the flattened-device-tree magic 0xd00dfeed.
        let dtb = std::fs::read(&out).unwrap();
        assert_eq!(&dtb[..4], &[0xd0, 0x0d, 0xfe, 0xed]);
        // The private build dir is cleaned up.
        assert!(!dts_dir.join(".dtc-build").exists());

        let _ = std::fs::remove_dir_all(&base);
    }
}
