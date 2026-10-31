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

//! Coprocessor firmware compilation, plus the remoteproc section rename.
//!
//! Builds a bare-metal ELF with `cargo`, injecting the `CSTI_*` build-host directories,
//! and verifies the result is really an ELF before handing it on.
//!
//! Linux remoteproc loaders use a chip-specific vector section name while
//! `cortex-m-rt` emits `.vector_table`. The deployable artifact is therefore a
//! renamed copy, with the original kept alongside it for debugging.

use consortium_cfg::components::remoteproc::RemoteprocMemoryLayout;
use object::{Object, ObjectSection, ObjectSegment};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::process::Command;

use super::Compiler;
use crate::compile::env::CstiWorkspace;
use crate::compile::utils::{elf_path, inject_csti, resolve_cargo, run_command, verify_elf};
use crate::progress::BuildReporter;

/// Error type for firmware compilation.
#[derive(Error, Debug)]
pub enum FirmwareCompileError {
    #[error("Failed to resolve cargo: {0}")]
    CargoResolveError(String),

    #[error("Cargo failed: {0}")]
    BuildFailed(String),

    #[error("ELF not found at expected path: {0:?}")]
    ElfNotFound(PathBuf),

    #[error("copying firmware ELF to {0:?}: {1}")]
    Copy(PathBuf, std::io::Error),

    #[error("objcopy post-processing failed: {0}")]
    Objcopy(String),

    #[error("remoteproc ELF validation failed: {0}")]
    ElfLayout(String),

    #[error("{0}")]
    UtilityError(String),
}

/// The two firmware ELF images the build emits from one Rust compilation.
#[derive(Clone, Debug)]
pub struct FirmwareArtifacts {
    /// `<name>_original.elf`: an unmodified copy of the Rust-compiled ELF,
    /// retaining its `.defmt` section. This is the image the AP embeds with
    /// `include_bytes!` so it can decode the coprocessor's defmt logs.
    pub original: PathBuf,
    /// `<name>.elf`: the remoteproc-loadable image, produced from the original by
    /// `arm-none-eabi-objcopy` — `.vector_table` renamed to `.isr_vectors` and the
    /// host-only `.defmt` section stripped. This is the image deployed to the
    /// target.
    pub deploy: PathBuf,
}

/// The `<name>_original.elf` sibling of a firmware crate's raw cargo output.
///
/// The raw cargo binary has no extension (e.g. `.../release/melt-pot-mcu`); the
/// original is written alongside it as `.../release/melt-pot-mcu_original.elf`.
pub fn original_elf_path(raw: &Path) -> PathBuf {
    sibling(raw, "_original.elf")
}

/// The `<name>.elf` sibling of a firmware crate's raw cargo output — the
/// remoteproc-loadable, objcopy-processed image (see [`FirmwareArtifacts`]).
pub fn deploy_elf_path(raw: &Path) -> PathBuf {
    sibling(raw, ".elf")
}

fn sibling(raw: &Path, suffix: &str) -> PathBuf {
    let stem = raw
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("firmware");
    raw.with_file_name(format!("{stem}{suffix}"))
}

/// Resolves `arm-none-eabi-objcopy`, honouring a `CONSORTIUM_OBJCOPY` override.
fn resolve_objcopy() -> String {
    std::env::var("CONSORTIUM_OBJCOPY").unwrap_or_else(|_| "arm-none-eabi-objcopy".to_string())
}

/// Post-processes the raw firmware ELF into a remoteproc-loadable image.
///
/// Renames `.vector_table` to `.isr_vectors` (the section name the remoteproc ELF
/// loader expects) and strips the host-only `.defmt` section, writing the result
/// to `dest`.
async fn objcopy_firmware(
    src: &Path,
    dest: &Path,
    layout: Option<&RemoteprocMemoryLayout>,
    strict: bool,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), FirmwareCompileError> {
    let vector_section = layout.map_or(".isr_vectors", |layout| layout.vector_section.as_str());
    if let Some(layout) = layout {
        validate_firmware_elf(src, ".vector_table", layout, strict).await?;
    }
    let objcopy = resolve_objcopy();
    let mut cmd = Command::new(&objcopy);
    cmd.arg("--rename-section")
        .arg(format!(".vector_table={vector_section}"))
        .arg("--remove-section")
        .arg(".defmt")
        .arg(src)
        .arg(dest);

    let status = run_command(&mut cmd, reporter)
        .await
        .map_err(|e| FirmwareCompileError::Objcopy(format!("{objcopy}: {e}")))?;

    if !status.success() {
        return Err(FirmwareCompileError::Objcopy(format!(
            "{objcopy} exited with status: {status}"
        )));
    }
    if let Some(layout) = layout {
        validate_firmware_elf(dest, vector_section, layout, strict).await?;
    }
    Ok(())
}

async fn validate_firmware_elf(
    path: &Path,
    vector_section: &str,
    layout: &RemoteprocMemoryLayout,
    strict: bool,
) -> Result<(), FirmwareCompileError> {
    let bytes = tokio::fs::read(path)
        .await
        .map_err(|error| FirmwareCompileError::ElfLayout(format!("{}: {error}", path.display())))?;
    let file = object::File::parse(bytes.as_slice())
        .map_err(|error| FirmwareCompileError::ElfLayout(format!("{}: {error}", path.display())))?;
    let section = file.section_by_name(vector_section).ok_or_else(|| {
        FirmwareCompileError::ElfLayout(format!(
            "{} has no '{vector_section}' section",
            path.display()
        ))
    })?;
    if section.size() == 0 {
        return Err(FirmwareCompileError::ElfLayout(format!(
            "{} section '{vector_section}' is empty",
            path.display()
        )));
    }
    if section.address() != layout.vector.window.origin {
        if !strict {
            consortium_log::warn!(
                "{} section '{}' starts at {:#x}, not vector origin {:#x}; continuing (--skip-remoteproc-check)",
                path.display(),
                vector_section,
                section.address(),
                layout.vector.window.origin
            );
            return Ok(());
        }
        return Err(FirmwareCompileError::ElfLayout(format!(
            "{} section '{vector_section}' starts at {:#x}, expected vector origin {:#x}",
            path.display(),
            section.address(),
            layout.vector.window.origin
        )));
    }
    if section.size() > layout.vector.window.length {
        return Err(FirmwareCompileError::ElfLayout(format!(
            "{} section '{vector_section}' is {:#x} bytes, exceeding vector region '{}' ({:#x} bytes)",
            path.display(),
            section.size(),
            layout.vector.name,
            layout.vector.window.length
        )));
    }
    let resource_table = file.section_by_name(".resource_table").ok_or_else(|| {
        FirmwareCompileError::ElfLayout(format!(
            "{} has no mandatory '.resource_table' section",
            path.display()
        ))
    })?;
    let resource_window = layout.shmem.window;
    if resource_table.size() == 0
        || !resource_window.contains(resource_table.address(), resource_table.size())
    {
        return Err(FirmwareCompileError::ElfLayout(format!(
            "{} '.resource_table' at {:#x}..{:#x} is outside mandatory region '{}'",
            path.display(),
            resource_table.address(),
            resource_table
                .address()
                .saturating_add(resource_table.size()),
            layout.shmem.name
        )));
    }
    for segment in file.segments().filter(|segment| segment.size() != 0) {
        let address = segment.address();
        let size = segment.size();
        if !layout
            .load_windows()
            .any(|window| window.contains(address, size))
        {
            return Err(FirmwareCompileError::ElfLayout(format!(
                "{} PT_LOAD segment {address:#x}..{:#x} is outside configured remoteproc memory regions",
                path.display(),
                address.saturating_add(size)
            )));
        }
    }
    Ok(())
}

/// Build parameters for bare-metal coprocessor firmware.
#[derive(Clone)]
pub struct FirmwareConfiguration {
    /// Rust target triple to cross-compile for (e.g.
    /// `thumbv8m.main-none-eabihf`). Resolved by the caller from the coprocessor's
    /// core name (`McoreName`/`RcoreName`/`RvCoreName::as_rust_triple`), the
    /// single source of truth for triples.
    pub target: String,
    /// Path to the firmware Cargo project root.
    pub path: PathBuf,
    /// Build in release mode.
    pub release: bool,
    /// Pass `--offline` to cargo: resolve and build from the local registry cache
    /// only. Populate that cache first with `csti fetch`.
    pub offline: bool,
    /// Resolved remoteproc linker/ELF/DTS contract. The pipeline always sets
    /// this; `None` retains the legacy `.isr_vectors` behavior for direct users.
    pub remoteproc: Option<RemoteprocMemoryLayout>,
    /// Require the vector table to start at the remoteproc vector window's
    /// origin. `csti build --skip-remoteproc-check` clears it: with `separate`
    /// vector placement `cortex-m-rt` links `.vector_table` at `ORIGIN(FLASH)`,
    /// so the check cannot pass there yet.
    pub strict_remoteproc: bool,
    /// Build-host directories injected as `CSTI_*` env vars, and the workspace
    /// layout locating `target/`. `None` injects nothing and uses the crate's
    /// own `target/`.
    pub workspace: Option<CstiWorkspace>,
}

/// Compiler for bare-metal Cortex-M firmware.
pub struct FirmwareCompiler;

impl FirmwareCompiler {
    pub async fn build_with_reporter(
        config: FirmwareConfiguration,
        reporter: Option<&dyn BuildReporter>,
    ) -> Result<FirmwareArtifacts, FirmwareCompileError> {
        Self::cargo(&config, "build", reporter).await?;

        let elf = Self::output_path(&config).await?;

        if !elf.exists() {
            return Err(FirmwareCompileError::ElfNotFound(elf));
        }

        verify_elf(&elf)
            .await
            .map_err(FirmwareCompileError::UtilityError)?;

        // `<name>_original.elf`: the untouched Rust-compiled image (keeps `.defmt`
        // for host-side decoding), which the AP embeds via `include_bytes!`.
        let original = original_elf_path(&elf);
        tokio::fs::copy(&elf, &original)
            .await
            .map_err(|e| FirmwareCompileError::Copy(original.clone(), e))?;

        // `<name>.elf`: the remoteproc-loadable image deployed to the target
        // (`.vector_table` → `.isr_vectors`, `.defmt` stripped).
        let deploy = deploy_elf_path(&elf);
        objcopy_firmware(
            &elf,
            &deploy,
            config.remoteproc.as_ref(),
            config.strict_remoteproc,
            reporter,
        )
        .await?;

        Ok(FirmwareArtifacts { original, deploy })
    }

    /// Type-checks the firmware with `cargo check`, under the same target, flags,
    /// and injected environment as [`build_with_reporter`](Self::build_with_reporter).
    /// Produces no ELF and runs no post-processing.
    pub async fn check_with_reporter(
        config: &FirmwareConfiguration,
        reporter: Option<&dyn BuildReporter>,
    ) -> Result<(), FirmwareCompileError> {
        Self::cargo(config, "check", reporter).await
    }

    /// Runs `cargo <subcommand>` for the firmware crate.
    async fn cargo(
        config: &FirmwareConfiguration,
        subcommand: &str,
        reporter: Option<&dyn BuildReporter>,
    ) -> Result<(), FirmwareCompileError> {
        let cargo = resolve_cargo()
            .await
            .map_err(FirmwareCompileError::CargoResolveError)?;

        let mut cmd = Command::new(&cargo);
        cmd.arg(subcommand)
            .arg("--target")
            .arg(&config.target)
            .current_dir(&config.path);

        inject_csti(&mut cmd, config.workspace.as_ref());

        if config.release {
            cmd.arg("--release");
        }

        if config.offline {
            cmd.arg("--offline");
        }

        let status = run_command(&mut cmd, reporter)
            .await
            .map_err(|e| FirmwareCompileError::BuildFailed(e.to_string()))?;

        if !status.success() {
            return Err(FirmwareCompileError::BuildFailed(format!(
                "cargo {subcommand} failed with status: {status}"
            )));
        }
        Ok(())
    }
}

impl Compiler for FirmwareCompiler {
    type Configuration = FirmwareConfiguration;
    type Error = FirmwareCompileError;
    type Output = FirmwareArtifacts;

    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error> {
        Self::build_with_reporter(config, None).await
    }

    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error> {
        elf_path(
            &config.path,
            config.workspace.as_ref(),
            &config.target,
            config.release,
        )
        .await
        .map_err(FirmwareCompileError::UtilityError)
    }
}

/// Backward-compatible builder; delegates to [`FirmwareCompiler`].
///
/// Always builds online (`offline: false`); use [`FirmwareCompiler`] directly to
/// build from the local registry cache.
pub struct FirmwareBuilder {
    /// Rust target triple to cross-compile for.
    pub target: String,
    pub path: PathBuf,
    pub release: bool,
}

impl FirmwareBuilder {
    /// Builds the firmware, returning the path to the deployable (objcopy-processed)
    /// output ELF.
    pub async fn build(&self) -> Result<PathBuf, String> {
        FirmwareCompiler::build(FirmwareConfiguration {
            target: self.target.clone(),
            path: self.path.clone(),
            release: self.release,
            offline: false,
            remoteproc: None,
            strict_remoteproc: true,
            workspace: None,
        })
        .await
        .map(|artifacts| artifacts.deploy)
        .map_err(|e| e.to_string())
    }

    /// Returns the expected output ELF path without building.
    pub async fn elf_path(&self) -> Result<PathBuf, String> {
        FirmwareCompiler::output_path(&FirmwareConfiguration {
            target: self.target.clone(),
            path: self.path.clone(),
            release: self.release,
            offline: false,
            remoteproc: None,
            strict_remoteproc: true,
            workspace: None,
        })
        .await
        .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_paths_derive_from_raw_cargo_output() {
        // The raw cargo binary has no extension; the two emitted images sit beside
        // it as `<name>_original.elf` and `<name>.elf`.
        let raw = Path::new("/proj/firmware/target/thumbv8m.main-none-eabihf/release/melt-pot-mcu");
        assert_eq!(
            original_elf_path(raw),
            Path::new(
                "/proj/firmware/target/thumbv8m.main-none-eabihf/release/melt-pot-mcu_original.elf"
            )
        );
        assert_eq!(
            deploy_elf_path(raw),
            Path::new("/proj/firmware/target/thumbv8m.main-none-eabihf/release/melt-pot-mcu.elf")
        );
    }

    #[test]
    fn objcopy_resolution_honours_override() {
        // SAFETY: single-threaded test; restores the prior value after asserting.
        let prev = std::env::var("CONSORTIUM_OBJCOPY").ok();
        unsafe { std::env::set_var("CONSORTIUM_OBJCOPY", "riscv64-unknown-elf-objcopy") };
        assert_eq!(resolve_objcopy(), "riscv64-unknown-elf-objcopy");
        unsafe { std::env::remove_var("CONSORTIUM_OBJCOPY") };
        assert_eq!(resolve_objcopy(), "arm-none-eabi-objcopy");
        match prev {
            Some(v) => unsafe { std::env::set_var("CONSORTIUM_OBJCOPY", v) },
            None => unsafe { std::env::remove_var("CONSORTIUM_OBJCOPY") },
        }
    }
}
