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

//! Stage 7: assemble the local `dist/` staging tree.
//!
//! `dist/` is a local staging directory a downstream bitbake recipe consumes; the
//! pipeline never writes to a live target filesystem. Firmware ELFs land under
//! `dist/firmware/<core>.elf` purely so a firmware-packaging recipe can install
//! them to `/lib/firmware/<core>` for remoteproc's kernel-side loader — this is
//! unrelated to the defmt embedding, which happens in the application binary.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use consortium_cfg_common::config::{HmiEngine, VisibleCore};
use thiserror::Error;
use tokio::process::Command;

/// Preprocessor defines that resolve the `dt-bindings` interrupt macros the UIO
/// overlay emits (`GIC_SPI`, `IRQ_TYPE_*`), so `dtc` can verify the overlay
/// without the kernel `dt-bindings` headers on disk. Values match
/// `dt-bindings/interrupt-controller/{arm-gic,irq}.h`.
const DT_BINDING_DEFINES: &[&str] = &[
    "-DGIC_SPI=0",
    "-DGIC_PPI=1",
    "-DIRQ_TYPE_NONE=0",
    "-DIRQ_TYPE_EDGE_RISING=1",
    "-DIRQ_TYPE_EDGE_FALLING=2",
    "-DIRQ_TYPE_EDGE_BOTH=3",
    "-DIRQ_TYPE_LEVEL_HIGH=4",
    "-DIRQ_TYPE_LEVEL_LOW=8",
];

/// The `dist/` staging tree layout.
pub struct DistLayout {
    /// The staging root; children are `firmware/`, `app/`, `ta/`, `www/`,
    /// `systemd/`.
    pub root: PathBuf,
}

impl DistLayout {
    /// A layout rooted at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The firmware subdirectory (`<root>/firmware`).
    pub fn firmware_dir(&self) -> PathBuf {
        self.root.join("firmware")
    }

    /// The application subdirectory (`<root>/app`).
    pub fn app_dir(&self) -> PathBuf {
        self.root.join("app")
    }

    /// The TA subdirectory (`<root>/ta`).
    pub fn ta_dir(&self) -> PathBuf {
        self.root.join("ta")
    }

    /// The Cog HMI assets subdirectory (`<root>/www`).
    pub fn www_dir(&self) -> PathBuf {
        self.root.join("www")
    }

    /// The PocketJS HMI subdirectory (`<root>/hmi`) holding the bundle + pak.
    pub fn hmi_dir(&self) -> PathBuf {
        self.root.join("hmi")
    }

    /// The Linux device-tree overlay subdirectory (`<root>/dts/linux`).
    pub fn dts_linux_dir(&self) -> PathBuf {
        self.root.join("dts").join("linux")
    }

    /// The kernel config-fragment subdirectory (`<root>/kernel`).
    pub fn kernel_dir(&self) -> PathBuf {
        self.root.join("kernel")
    }

    /// The boot-args subdirectory (`<root>/boot`).
    pub fn boot_dir(&self) -> PathBuf {
        self.root.join("boot")
    }

    /// The systemd unit subdirectory (`<root>/systemd`).
    pub fn systemd_dir(&self) -> PathBuf {
        self.root.join("systemd")
    }
}

/// Error type for assembling the `dist/` tree.
#[derive(Error, Debug)]
pub enum DistAssemblyError {
    /// A filesystem operation failed.
    #[error("dist assembly I/O error at {path}: {source}")]
    Io {
        /// The path being operated on.
        path: String,
        /// The underlying error.
        source: std::io::Error,
    },

    /// The C preprocessor is needed to resolve the overlay's `dt-bindings`
    /// macros before `dtc`, but was not found on PATH.
    #[error("`cpp` not found on PATH; needed to preprocess device-tree overlays before `dtc`")]
    CppNotFound,

    /// `dtc` (the device-tree compiler) was not found on PATH.
    #[error("`dtc` not found on PATH; install the device-tree compiler to verify UIO overlays")]
    DtcNotFound,

    /// `dtc` (or `cpp`) rejected a generated overlay.
    #[error("device-tree overlay {name} failed verification:\n{message}")]
    DtsVerify {
        /// The overlay file name.
        name: String,
        /// The `cpp`/`dtc` diagnostics.
        message: String,
    },
}

fn io(path: impl AsRef<Path>, source: std::io::Error) -> DistAssemblyError {
    DistAssemblyError::Io {
        path: path.as_ref().display().to_string(),
        source,
    }
}

/// Assembles the `dist/` tree from the pipeline's build outputs.
///
/// - each firmware ELF → `firmware/<core>.elf`;
/// - each firmware ELF's unstripped original → `firmware/<core>_original.elf`
///   (retains the `.defmt` section for host-side defmt table parsing);
/// - the application binary → `app/<file_name>`;
/// - the signed TA (when present) → `ta/<file_name>`;
/// - the HMI assets (when present): a Cog dist tree → `www/` (copied
///   recursively), or a PocketJS `<app>.js`/`<app>.pak` pair → `hmi/`;
/// - each `(name, contents)` Linux device tree → `dts/linux/<name>`; a `.dtso`
///   overlay is verified with `dtc` (see [`verify_dts`]), a merged `.dts` is
///   not (see below);
/// - each `(name, contents)` kernel config fragment → `kernel/<name>`;
/// - each `(name, contents)` boot-args file → `boot/<name>`;
/// - each `(name, contents)` systemd unit → `systemd/<name>`.
///
/// A `.dtso` overlay uses only the interrupt macros this module defines, so it is
/// self-contained enough to `dtc`-verify here (before the systemd units, so a
/// rejection fails early). A merged `.dts` `#include`s vendor `.dtsi`/`dt-bindings`
/// headers that are not on disk offline, so it is written but not compiled here;
/// the kernel build performs the final `dtc`.
pub struct DistAssemblyInputs<'a> {
    pub firmware: &'a HashMap<VisibleCore, PathBuf>,
    /// The unstripped `<core>_original.elf` per core (keeps the `.defmt` section),
    /// staged alongside the deployable ELF for host-side defmt table parsing.
    pub firmware_originals: &'a HashMap<VisibleCore, PathBuf>,
    pub application: &'a Path,
    pub ta: Option<&'a Path>,
    pub hmi: Option<HmiStaging<'a>>,
    pub dts_files: &'a [(String, String)],
    pub kconfig_fragments: &'a [(String, String)],
    pub boot_files: &'a [(String, String)],
    pub systemd_units: &'a [(String, String)],
}

/// How the built HMI assets are staged, selected by engine.
#[derive(Clone, Copy)]
pub struct HmiStaging<'a> {
    /// Rendering engine, selecting the staging layout.
    pub engine: HmiEngine,
    /// PocketJS entry name (`[hmi].app`); `None` for other engines.
    pub app: Option<&'a str>,
    /// Build-host dist directory the frontend build produced.
    pub dist: &'a Path,
}

pub async fn assemble(
    layout: &DistLayout,
    inputs: DistAssemblyInputs<'_>,
) -> Result<(), DistAssemblyError> {
    let DistAssemblyInputs {
        firmware,
        firmware_originals,
        application,
        ta,
        hmi,
        dts_files,
        kconfig_fragments,
        boot_files,
        systemd_units,
    } = inputs;
    // Firmware ELFs, named by core for the packaging recipe.
    let firmware_dir = layout.firmware_dir();
    tokio::fs::create_dir_all(&firmware_dir)
        .await
        .map_err(|e| io(&firmware_dir, e))?;
    for (core, elf) in firmware {
        let dest = firmware_dir.join(format!("{core}.elf"));
        copy_file(elf, &dest).await?;
    }
    // Unstripped originals (keep `.defmt`) for host-side defmt table parsing.
    for (core, elf) in firmware_originals {
        let dest = firmware_dir.join(format!("{core}_original.elf"));
        copy_file(elf, &dest).await?;
    }

    // Application binary.
    let app_dir = layout.app_dir();
    tokio::fs::create_dir_all(&app_dir)
        .await
        .map_err(|e| io(&app_dir, e))?;
    let app_dest = app_dir.join(file_name(application)?);
    copy_file(application, &app_dest).await?;

    // Signed TA, when the manifest declared one.
    if let Some(ta) = ta {
        let ta_dir = layout.ta_dir();
        tokio::fs::create_dir_all(&ta_dir)
            .await
            .map_err(|e| io(&ta_dir, e))?;
        let ta_dest = ta_dir.join(file_name(ta)?);
        copy_file(ta, &ta_dest).await?;
    }

    // HMI assets. Cog serves a whole dist tree from `www/`; PocketJS installs
    // just the built `<app>.js`/`<app>.pak` pair under `hmi/`.
    if let Some(hmi) = hmi {
        match hmi.engine {
            HmiEngine::Cog => {
                let www = layout.www_dir();
                copy_dir_recursive(hmi.dist, &www).await?;
            }
            HmiEngine::Pocket => {
                let app = hmi.app.unwrap_or_default();
                let hmi_dir = layout.hmi_dir();
                tokio::fs::create_dir_all(&hmi_dir)
                    .await
                    .map_err(|e| io(&hmi_dir, e))?;
                for ext in ["js", "pak"] {
                    let name = format!("{app}.{ext}");
                    copy_file(&hmi.dist.join(&name), &hmi_dir.join(&name)).await?;
                }
            }
        }
    }

    // Linux device tree, written before the systemd units. A `.dtso` overlay is
    // `dtc`-verified here; a merged `.dts` pulls in vendor headers we don't have
    // offline, so it is left for the kernel build to compile.
    if !dts_files.is_empty() {
        let dts_dir = layout.dts_linux_dir();
        tokio::fs::create_dir_all(&dts_dir)
            .await
            .map_err(|e| io(&dts_dir, e))?;
        for (name, contents) in dts_files {
            let dest = dts_dir.join(name);
            tokio::fs::write(&dest, contents)
                .await
                .map_err(|e| io(&dest, e))?;
            if name.ends_with(".dtso") {
                verify_dts(name, &dest).await?;
            }
        }
    }

    // Kernel config fragments (plain text; no verification tooling).
    if !kconfig_fragments.is_empty() {
        let kernel_dir = layout.kernel_dir();
        tokio::fs::create_dir_all(&kernel_dir)
            .await
            .map_err(|e| io(&kernel_dir, e))?;
        for (name, contents) in kconfig_fragments {
            let dest = kernel_dir.join(name);
            tokio::fs::write(&dest, contents)
                .await
                .map_err(|e| io(&dest, e))?;
        }
    }

    // Boot-args files (plain text; wired into the bootloader by the integrator).
    if !boot_files.is_empty() {
        let boot_dir = layout.boot_dir();
        tokio::fs::create_dir_all(&boot_dir)
            .await
            .map_err(|e| io(&boot_dir, e))?;
        for (name, contents) in boot_files {
            let dest = boot_dir.join(name);
            tokio::fs::write(&dest, contents)
                .await
                .map_err(|e| io(&dest, e))?;
        }
    }

    // systemd units.
    if !systemd_units.is_empty() {
        let systemd_dir = layout.systemd_dir();
        tokio::fs::create_dir_all(&systemd_dir)
            .await
            .map_err(|e| io(&systemd_dir, e))?;
        for (name, contents) in systemd_units {
            let dest = systemd_dir.join(name);
            tokio::fs::write(&dest, contents)
                .await
                .map_err(|e| io(&dest, e))?;
        }
    }

    Ok(())
}

/// The final path component of `path` as a string.
fn file_name(path: &Path) -> Result<String, DistAssemblyError> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.to_string())
        .ok_or_else(|| {
            io(
                path,
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "no file name"),
            )
        })
}

/// Copies a single file, creating the parent directory if needed.
async fn copy_file(src: &Path, dest: &Path) -> Result<(), DistAssemblyError> {
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| io(parent, e))?;
    }
    tokio::fs::copy(src, dest).await.map_err(|e| io(src, e))?;
    Ok(())
}

/// Recursively copies the directory tree at `src` into `dest`.
///
/// Boxed because async fns cannot recurse directly. No extra dependency: the walk
/// is a hand-rolled [`tokio::fs::read_dir`] traversal.
pub(crate) fn copy_dir_recursive<'a>(
    src: &'a Path,
    dest: &'a Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DistAssemblyError>> + Send + 'a>>
{
    Box::pin(async move {
        tokio::fs::create_dir_all(dest)
            .await
            .map_err(|e| io(dest, e))?;
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| io(src, e))?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| io(src, e))? {
            let file_type = entry.file_type().await.map_err(|e| io(entry.path(), e))?;
            let child_dest = dest.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_recursive(&entry.path(), &child_dest).await?;
            } else {
                copy_file(&entry.path(), &child_dest).await?;
            }
        }
        Ok(())
    })
}

/// Verifies a written device-tree overlay compiles.
///
/// The overlay carries `dt-bindings` interrupt macros (`GIC_SPI`, `IRQ_TYPE_*`),
/// so it is first run through `cpp` with those macros defined (see
/// [`DT_BINDING_DEFINES`]) — keeping the on-disk overlay header-free — and the
/// preprocessed result is then compiled with `dtc`. Only a non-zero exit is an
/// error; `dtc`'s overlay-context warnings (missing base-tree `#address-cells`,
/// unresolved `reg` unit addresses, etc.) go to stderr with a zero exit and are
/// expected.
async fn verify_dts(name: &str, dts: &Path) -> Result<(), DistAssemblyError> {
    // `cpp` resolves the macros into a sibling `<name>.pp` next to the overlay.
    let preprocessed = dts.with_extension("dts.pp");
    let cpp = Command::new("cpp")
        .args(["-nostdinc", "-undef", "-x", "assembler-with-cpp"])
        .args(DT_BINDING_DEFINES)
        .arg(dts)
        .arg("-o")
        .arg(&preprocessed)
        .output()
        .await
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => DistAssemblyError::CppNotFound,
            _ => io(dts, e),
        })?;
    if !cpp.status.success() {
        return Err(DistAssemblyError::DtsVerify {
            name: name.to_string(),
            message: String::from_utf8_lossy(&cpp.stderr).into_owned(),
        });
    }

    // `dtc` compiles the preprocessed overlay; the `.dtb` output is discarded to
    // stdout (`-o -`) since only the exit status matters here.
    let dtc = Command::new("dtc")
        .args(["-I", "dts", "-O", "dtb", "-o", "-"])
        .arg(&preprocessed)
        .output()
        .await
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => DistAssemblyError::DtcNotFound,
            _ => io(&preprocessed, e),
        });

    let _ = tokio::fs::remove_file(&preprocessed).await;
    let dtc = dtc?;

    if !dtc.status.success() {
        return Err(DistAssemblyError::DtsVerify {
            name: name.to_string(),
            message: String::from_utf8_lossy(&dtc.stderr).into_owned(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("consortium-dist-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch");
        dir
    }

    #[tokio::test]
    async fn assembles_full_tree() {
        let base = scratch("assemble");
        let sources = base.join("src");
        std::fs::create_dir_all(&sources).unwrap();

        // Fake build outputs: ELF-magic stand-ins, an app binary, a .ta, a www tree.
        let cm33_elf = sources.join("cm33.elf");
        std::fs::write(&cm33_elf, [0x7f, b'E', b'L', b'F']).unwrap();
        let cm33_original = sources.join("cm33_original.elf");
        std::fs::write(&cm33_original, [0x7f, b'E', b'L', b'F', b'O']).unwrap();
        let app = sources.join("melt-pot-app");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();
        let ta = sources.join("uuid.ta");
        std::fs::write(&ta, b"signed-ta").unwrap();
        let www = sources.join("dist");
        std::fs::create_dir_all(www.join("assets")).unwrap();
        std::fs::write(www.join("index.html"), b"<html>").unwrap();
        std::fs::write(www.join("assets").join("app.js"), b"//js").unwrap();

        let layout = DistLayout::new(base.join("dist"));
        let firmware = HashMap::from([(
            VisibleCore::Mcore("cm33".parse().unwrap()),
            cm33_elf.clone(),
        )]);
        let firmware_originals = HashMap::from([(
            VisibleCore::Mcore("cm33".parse().unwrap()),
            cm33_original.clone(),
        )]);
        let units = vec![("consortium-app.service".to_string(), "[Unit]\n".to_string())];
        let kconfig = vec![("uio-enable.cfg".to_string(), "CONFIG_UIO=y\n".to_string())];
        let boot = vec![("boot.cmd".to_string(), "setenv bootargs\n".to_string())];

        assemble(
            &layout,
            DistAssemblyInputs {
                firmware: &firmware,
                firmware_originals: &firmware_originals,
                application: &app,
                ta: Some(&ta),
                hmi: Some(HmiStaging {
                    engine: HmiEngine::Cog,
                    app: None,
                    dist: &www,
                }),
                dts_files: &[],
                kconfig_fragments: &kconfig,
                boot_files: &boot,
                systemd_units: &units,
            },
        )
        .await
        .expect("assemble succeeds");

        assert!(layout.firmware_dir().join("cm33.elf").exists());
        assert!(layout.firmware_dir().join("cm33_original.elf").exists());
        assert!(layout.app_dir().join("melt-pot-app").exists());
        assert!(layout.ta_dir().join("uuid.ta").exists());
        assert!(layout.www_dir().join("index.html").exists());
        assert!(layout.www_dir().join("assets").join("app.js").exists());
        assert_eq!(
            std::fs::read_to_string(layout.kernel_dir().join("uio-enable.cfg")).unwrap(),
            "CONFIG_UIO=y\n"
        );
        assert_eq!(
            std::fs::read_to_string(layout.boot_dir().join("boot.cmd")).unwrap(),
            "setenv bootargs\n"
        );
        assert_eq!(
            std::fs::read_to_string(layout.systemd_dir().join("consortium-app.service")).unwrap(),
            "[Unit]\n"
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[tokio::test]
    async fn skips_optional_ta_and_hmi() {
        let base = scratch("minimal");
        let app = base.join("app-bin");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();

        let layout = DistLayout::new(base.join("dist"));
        let firmware = HashMap::new();
        assemble(
            &layout,
            DistAssemblyInputs {
                firmware: &firmware,
                firmware_originals: &firmware,
                application: &app,
                ta: None,
                hmi: None,
                dts_files: &[],
                kconfig_fragments: &[],
                boot_files: &[],
                systemd_units: &[],
            },
        )
        .await
        .expect("assemble succeeds");

        assert!(layout.app_dir().join("app-bin").exists());
        assert!(!layout.ta_dir().exists());
        assert!(!layout.www_dir().exists());
        assert!(!layout.dts_linux_dir().exists());
        assert!(!layout.kernel_dir().exists());
        assert!(!layout.boot_dir().exists());

        let _ = std::fs::remove_dir_all(&base);
    }

    #[tokio::test]
    async fn stages_pocket_bundle_and_pak_into_hmi_dir() {
        let base = scratch("pocket");
        let app = base.join("app-bin");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();

        // A PocketJS build-host dist with the two artifacts (plus a stray file
        // that must NOT be staged — only <app>.js/<app>.pak are).
        let src = base.join("pocket-dist");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("hero-main.js"), b"//bundle").unwrap();
        std::fs::write(src.join("hero-main.pak"), b"PAK").unwrap();
        std::fs::write(src.join("hero.js"), b"//component-only").unwrap();

        let layout = DistLayout::new(base.join("dist"));
        let firmware = HashMap::new();
        assemble(
            &layout,
            DistAssemblyInputs {
                firmware: &firmware,
                firmware_originals: &firmware,
                application: &app,
                ta: None,
                hmi: Some(HmiStaging {
                    engine: HmiEngine::Pocket,
                    app: Some("hero-main"),
                    dist: &src,
                }),
                dts_files: &[],
                kconfig_fragments: &[],
                boot_files: &[],
                systemd_units: &[],
            },
        )
        .await
        .expect("assemble succeeds");

        // Only the mounted pair lands under `hmi/`; no `www/`, no stray file.
        assert_eq!(
            std::fs::read_to_string(layout.hmi_dir().join("hero-main.js")).unwrap(),
            "//bundle"
        );
        assert!(layout.hmi_dir().join("hero-main.pak").exists());
        assert!(!layout.hmi_dir().join("hero.js").exists());
        assert!(!layout.www_dir().exists());

        let _ = std::fs::remove_dir_all(&base);
    }

    /// Whether both `cpp` and `dtc` resolve on PATH; the overlay-verification
    /// tests are skipped when either is absent (they exercise the real tools).
    fn dt_tools_available() -> bool {
        fn on_path(program: &str) -> bool {
            std::process::Command::new(program)
                .arg("--version")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        }
        on_path("cpp") && on_path("dtc")
    }

    /// A minimal overlay carrying the `dt-bindings` interrupt macros, so the
    /// `cpp` preprocessing step is actually exercised.
    const SAMPLE_OVERLAY: &str = "/dts-v1/;\n/plugin/;\n\n\
        / {\n\
        \treserved-memory {\n\
        \t\t#address-cells = <0x2>;\n\
        \t\t#size-cells = <0x2>;\n\
        \t\tranges;\n\
        \t\tipc_shm: ipc-shm@20400000 {\n\
        \t\t\treg = <0x0 0x20400000 0x0 0x8000>;\n\
        \t\t\tno-map;\n\
        \t\t};\n\
        \t};\n\
        \thsem@6240000 {\n\
        \t\tcompatible = \"generic-uio\";\n\
        \t\tinterrupts = <GIC_SPI 199 IRQ_TYPE_LEVEL_HIGH>;\n\
        \t\tstatus = \"okay\";\n\
        \t};\n\
        };\n";

    #[tokio::test]
    async fn writes_and_verifies_overlay_before_systemd() {
        if !dt_tools_available() {
            eprintln!("skipping: `cpp`/`dtc` not on PATH");
            return;
        }
        let base = scratch("dts-ok");
        let app = base.join("app-bin");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();

        let layout = DistLayout::new(base.join("dist"));
        let dts = vec![(
            "consortium-uio.dtso".to_string(),
            SAMPLE_OVERLAY.to_string(),
        )];
        let units = vec![("consortium-app.service".to_string(), "[Unit]\n".to_string())];

        let firmware = HashMap::new();
        assemble(
            &layout,
            DistAssemblyInputs {
                firmware: &firmware,
                firmware_originals: &firmware,
                application: &app,
                ta: None,
                hmi: None,
                dts_files: &dts,
                kconfig_fragments: &[],
                boot_files: &[],
                systemd_units: &units,
            },
        )
        .await
        .expect("assemble + dtc verify succeeds");

        // The overlay lands under dts/linux/, and the preprocessing temp is gone.
        let overlay = layout.dts_linux_dir().join("consortium-uio.dtso");
        assert!(overlay.exists());
        assert!(!overlay.with_extension("dts.pp").exists());
        assert!(layout.systemd_dir().join("consortium-app.service").exists());

        let _ = std::fs::remove_dir_all(&base);
    }

    #[tokio::test]
    async fn rejects_malformed_overlay() {
        if !dt_tools_available() {
            eprintln!("skipping: `cpp`/`dtc` not on PATH");
            return;
        }
        let base = scratch("dts-bad");
        let app = base.join("app-bin");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();

        let layout = DistLayout::new(base.join("dist"));
        // Missing closing brace: `dtc` rejects it.
        let dts = vec![("broken.dtso".to_string(), "/dts-v1/;\n/ {\n".to_string())];

        let firmware = HashMap::new();
        let err = assemble(
            &layout,
            DistAssemblyInputs {
                firmware: &firmware,
                firmware_originals: &firmware,
                application: &app,
                ta: None,
                hmi: None,
                dts_files: &dts,
                kconfig_fragments: &[],
                boot_files: &[],
                systemd_units: &[],
            },
        )
        .await
        .expect_err("malformed overlay is rejected");
        assert!(matches!(
            err,
            DistAssemblyError::DtsVerify { name, .. } if name == "broken.dtso"
        ));

        let _ = std::fs::remove_dir_all(&base);
    }
}
