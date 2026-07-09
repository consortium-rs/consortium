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

//! Stage 8 (optional): hand the assembled `dist/` tree off to Yocto/bitbake.
//!
//! There is no in-repo Yocto layer to match conventions against, so this stage is
//! deliberately thin and decoupled: either stage `dist/` under a recipe's
//! `files/` directory (for `SRC_URI`), or invoke `bitbake` on a recipe.

use std::path::{Path, PathBuf};

use thiserror::Error;
use tokio::process::Command;

use crate::compile::utils::run_command;
use crate::progress::BuildReporter;

use super::dist::{DistAssemblyError, copy_dir_recursive};

/// What to do with the assembled `dist/` tree.
#[derive(Debug, Clone)]
pub enum BitbakeAction {
    /// Copy `dist/` under a recipe's `files/` directory so a `SRC_URI` picks it up.
    CopyIntoRecipe {
        /// The recipe's `files/` directory to stage into.
        recipe_files_dir: PathBuf,
    },
    /// Invoke `bitbake <recipe>` in a Yocto build directory.
    InvokeBitbake {
        /// The Yocto build directory (where `bitbake` is run).
        build_dir: PathBuf,
        /// The recipe to build.
        recipe: String,
        /// When set, resolve and report the `bitbake` command without running it.
        ///
        /// This supports splitting the build across machines: the Consortium
        /// build assembles `dist/` on one host, and the actual `bitbake` runs on
        /// a separate Yocto host that need not have the artifacts locally yet. The
        /// stage succeeds without requiring `bitbake` on the local PATH.
        dry_run: bool,
    },
}

/// Error type for the bitbake hand-off.
#[derive(Error, Debug)]
pub enum BitbakeError {
    /// Staging `dist/` into the recipe failed.
    #[error(transparent)]
    Copy(#[from] DistAssemblyError),

    /// `bitbake` was not found on PATH.
    #[error("`bitbake` not found on PATH; source a Yocto build environment first")]
    BitbakeNotFound,

    /// `bitbake` exited non-zero.
    #[error("bitbake failed: {0}")]
    Failed(String),

    /// Writing the generated `.bb` recipe failed.
    #[error("failed to write generated recipe {path}: {source}")]
    RecipeWrite {
        /// The recipe path being written.
        path: String,
        /// The underlying I/O error.
        source: std::io::Error,
    },
}

/// The on-target install manifest for a generated `.bb` recipe.
///
/// Built from the pipeline's resolved outputs and used to render a self-contained
/// Yocto recipe that installs the assembled `dist/` tree. All artifact fields are
/// bare file names under the corresponding `dist/` subdirectory (e.g. `app.elf`
/// lives at `dist/app/app.elf`).
#[derive(Debug, Clone)]
pub struct RecipeSpec {
    /// The recipe/package name (`PN`); also the `.bb` file stem.
    pub recipe_name: String,
    /// On-target install prefix from `[profile] deploy_root` (e.g. `/opt/consortium`).
    pub deploy_root: String,
    /// The application binary file name under `dist/app/`.
    pub app_bin: String,
    /// Firmware ELF file names under `dist/firmware/` (e.g. `cm33.elf`).
    pub firmware: Vec<String>,
    /// The signed TA file name under `dist/ta/`, when present.
    pub ta: Option<String>,
    /// Whether an HMI `dist/www/` tree was assembled.
    pub has_www: bool,
    /// The systemd unit file name under `dist/systemd/` (e.g. `consortium-app.service`).
    pub systemd_unit: String,
    /// Device-tree file names under `dist/dts/linux/`: a `.dtso` overlay is
    /// installed into the rootfs (applied at runtime via configfs); a merged
    /// `.dts` is a kernel build input, staged but not installed.
    pub dts_files: Vec<String>,
    /// Boot-args file names under `dist/boot/` (e.g. `boot.cmd`,
    /// `extlinux-uio.cfg`, `README`). These are bootloader/image inputs, staged
    /// for the integrator but not installed into the rootfs.
    pub boot_files: Vec<String>,
}

/// Renders a self-contained Yocto `.bb` recipe that installs the assembled
/// `dist/` tree.
///
/// The recipe is written next to the `dist/` subdirectories (see [`write_recipe`])
/// and reaches them via `FILESEXTRAPATHS:prepend := "${THISDIR}:"`, so dropping
/// the recipe plus the `dist/` tree into a Yocto layer is enough to `bitbake` it.
pub fn render_recipe(spec: &RecipeSpec) -> String {
    let deploy = spec.deploy_root.trim_end_matches('/');
    let mut src_uri: Vec<String> = Vec::new();
    let mut install: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    // Application binary → {deploy_root}/app/<bin> (matches the systemd unit's
    // ExecStart/WorkingDirectory).
    src_uri.push(format!("file://app/{}", spec.app_bin));
    install.push(format!("    install -d ${{D}}{deploy}/app"));
    install.push(format!(
        "    install -m 0755 ${{WORKDIR}}/app/{bin} ${{D}}{deploy}/app/{bin}",
        bin = spec.app_bin
    ));
    files.push(format!("{deploy}/app"));

    // Firmware ELFs → /lib/firmware/<core>.elf for remoteproc's kernel loader.
    if !spec.firmware.is_empty() {
        install.push("    install -d ${D}${nonarch_base_libdir}/firmware".to_string());
        for fw in &spec.firmware {
            src_uri.push(format!("file://firmware/{fw}"));
            install.push(format!(
                "    install -m 0644 ${{WORKDIR}}/firmware/{fw} \
                 ${{D}}${{nonarch_base_libdir}}/firmware/{fw}"
            ));
        }
        files.push("${nonarch_base_libdir}/firmware".to_string());
    }

    // systemd unit → the distro's system unit dir, registered via SYSTEMD_SERVICE.
    src_uri.push(format!("file://systemd/{}", spec.systemd_unit));
    install.push("    install -d ${D}${systemd_system_unitdir}".to_string());
    install.push(format!(
        "    install -m 0644 ${{WORKDIR}}/systemd/{unit} ${{D}}${{systemd_system_unitdir}}/{unit}",
        unit = spec.systemd_unit
    ));
    files.push(format!("${{systemd_system_unitdir}}/{}", spec.systemd_unit));

    // HMI assets → {deploy_root}/www (served by the app).
    if spec.has_www {
        src_uri.push("file://www".to_string());
        install.push(format!("    install -d ${{D}}{deploy}/www"));
        install.push(format!("    cp -R ${{WORKDIR}}/www/. ${{D}}{deploy}/www/"));
        files.push(format!("{deploy}/www"));
    }

    // Signed TA → /lib/optee_armtz/<uuid>.ta (OP-TEE's TA search path).
    if let Some(ta) = &spec.ta {
        src_uri.push(format!("file://ta/{ta}"));
        install.push("    install -d ${D}${nonarch_base_libdir}/optee_armtz".to_string());
        install.push(format!(
            "    install -m 0444 ${{WORKDIR}}/ta/{ta} ${{D}}${{nonarch_base_libdir}}/optee_armtz/{ta}"
        ));
        files.push("${nonarch_base_libdir}/optee_armtz".to_string());
    }

    // Device tree. A `.dtso` overlay is applied at runtime (configfs, enabled by
    // the CONFIG_OF_OVERLAY fragment), so the rootfs is a reasonable home for it.
    // A merged `.dts` is a *kernel build input* (compiled to a `.dtb` by the
    // kernel), so it is staged but not installed here — the integrator adds it to
    // their kernel recipe (see the guidance block below). The kernel config
    // fragment is likewise a build input, merged by a separate kernel `.bbappend`
    // (see `render_kernel_bbappend`).
    let mut guidance: Vec<String> = Vec::new();
    if !spec.dts_files.is_empty() {
        src_uri.push("file://dts".to_string());
        let overlays: Vec<&String> = spec
            .dts_files
            .iter()
            .filter(|n| n.ends_with(".dtso"))
            .collect();
        let merged: Vec<&String> = spec
            .dts_files
            .iter()
            .filter(|n| n.ends_with(".dts"))
            .collect();
        if !overlays.is_empty() {
            install.push(format!("    install -d ${{D}}{deploy}/dts"));
            for name in &overlays {
                install.push(format!(
                    "    install -m 0644 ${{WORKDIR}}/dts/linux/{name} ${{D}}{deploy}/dts/{name}"
                ));
            }
            files.push(format!("{deploy}/dts"));
        }
        for name in &merged {
            guidance.push(format!(
                "# Kernel input: dts/linux/{name} is a full board device tree. Add it to\n\
                 #   your kernel recipe (e.g. copy into the kernel source and extend\n\
                 #   KERNEL_DEVICETREE), then boot that .dtb. It is NOT a rootfs file."
            ));
        }
    }

    // Boot-args files → staged for the bootloader/image, not installed to the
    // rootfs. `dist/boot/README` documents the per-family integration.
    if !spec.boot_files.is_empty() {
        src_uri.push("file://boot".to_string());
        guidance.push(
            "# Bootloader input: dist/boot/ carries the kernel-command-line files\n\
             #   (boot.cmd -> boot.scr for i.MX, or extlinux-uio.cfg for STM32). Wire\n\
             #   them into your image/u-boot recipe as dist/boot/README describes; they\n\
             #   are NOT rootfs files."
                .to_string(),
        );
    }

    let src_uri = src_uri
        .iter()
        .map(|u| format!("    {u} \\"))
        .collect::<Vec<_>>()
        .join("\n");
    let install = install.join("\n");
    let files = files
        .iter()
        .map(|f| format!("    {f} \\"))
        .collect::<Vec<_>>()
        .join("\n");
    // Non-rootfs artifacts (kernel/bootloader inputs) get a commented guidance
    // block rather than a `do_install`, so the recipe stays a pure rootfs bundle.
    let guidance = if guidance.is_empty() {
        String::new()
    } else {
        format!("\n{}\n\n", guidance.join("\n#\n"))
    };

    format!(
        "# Auto-generated by Consortium — do not edit by hand.\n\
         #\n\
         # Installs the assembled `dist/` tree. Copy this recipe together with the\n\
         # sibling `dist/` subdirectories into a Yocto layer, then `bitbake {pn}`.\n\
         {guidance}\
         SUMMARY = \"Consortium application bundle (generated)\"\n\
         LICENSE = \"CLOSED\"\n\
         \n\
         # Artifacts live next to this recipe under the staged `dist/` subdirectories.\n\
         FILESEXTRAPATHS:prepend := \"${{THISDIR}}:\"\n\
         \n\
         SRC_URI = \"\\\n{src_uri}\n\"\n\
         \n\
         S = \"${{WORKDIR}}\"\n\
         \n\
         inherit systemd\n\
         SYSTEMD_SERVICE:${{PN}} = \"{unit}\"\n\
         \n\
         do_install() {{\n{install}\n}}\n\
         \n\
         FILES:${{PN}} += \"\\\n{files}\n\"\n",
        pn = spec.recipe_name,
        unit = spec.systemd_unit,
    )
}

/// Renders and writes `spec`'s recipe to `<dist_root>/<recipe_name>.bb`,
/// returning the written path. Placing it at the `dist/` root lets the recipe's
/// `${THISDIR}`-relative `SRC_URI` reach the staged artifact subdirectories.
pub async fn write_recipe(dist_root: &Path, spec: &RecipeSpec) -> Result<PathBuf, BitbakeError> {
    let path = dist_root.join(format!("{}.bb", spec.recipe_name));
    tokio::fs::write(&path, render_recipe(spec))
        .await
        .map_err(|source| BitbakeError::RecipeWrite {
            path: path.display().to_string(),
            source,
        })?;
    Ok(path)
}

/// Executes `action` against the assembled `dist_root`.
pub async fn run(action: &BitbakeAction, dist_root: &Path) -> Result<(), BitbakeError> {
    run_with_reporter(action, dist_root, None, None).await
}

/// Executes an action against the assembled dist root, streaming command output.
///
/// `recipe` is the install manifest used to emit a generated `.bb` on a dry-run
/// invoke; it is ignored for a real invoke or a copy-into-recipe action.
pub async fn run_with_reporter(
    action: &BitbakeAction,
    dist_root: &Path,
    recipe: Option<&RecipeSpec>,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), BitbakeError> {
    match action {
        BitbakeAction::CopyIntoRecipe { recipe_files_dir } => {
            copy_dir_recursive(dist_root, recipe_files_dir).await?;
            Ok(())
        }
        BitbakeAction::InvokeBitbake {
            build_dir,
            recipe: recipe_name,
            dry_run,
        } => {
            if *dry_run {
                // Emit the recipe and report the deferred command; the actual
                // build runs later, on the (possibly separate) Yocto host.
                if let Some(spec) = recipe {
                    let path = write_recipe(dist_root, spec).await?;
                    if let Some(reporter) = reporter {
                        reporter.command_output(&format!(
                            "[dry-run] wrote recipe {}\n",
                            path.display()
                        ));
                    }
                }
                if let Some(reporter) = reporter {
                    reporter.command_output(&format!(
                        "[dry-run] skipped `bitbake {recipe_name}` in {}\n",
                        build_dir.display()
                    ));
                }
                return Ok(());
            }
            invoke_bitbake_with("bitbake", build_dir, recipe_name, reporter).await
        }
    }
}

/// Invokes `program <recipe>` in `build_dir`. The `program` seam lets tests drive
/// the not-found path deterministically without touching the process PATH.
async fn invoke_bitbake_with(
    program: &str,
    build_dir: &Path,
    recipe: &str,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), BitbakeError> {
    let mut cmd = Command::new(program);
    cmd.arg(recipe).current_dir(build_dir);

    let status = run_command(&mut cmd, reporter)
        .await
        .map_err(|_| BitbakeError::BitbakeNotFound)?;

    if !status.success() {
        return Err(BitbakeError::Failed(format!(
            "`{program} {recipe}` exited with status: {status}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("consortium-bitbake-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch");
        dir
    }

    #[tokio::test]
    async fn copy_into_recipe_stages_dist_tree() {
        let base = scratch("copy");
        let dist = base.join("dist");
        std::fs::create_dir_all(dist.join("app")).unwrap();
        std::fs::write(dist.join("app").join("bin"), b"binary").unwrap();

        let recipe_files = base.join("recipe/files");
        run(
            &BitbakeAction::CopyIntoRecipe {
                recipe_files_dir: recipe_files.clone(),
            },
            &dist,
        )
        .await
        .expect("copy succeeds");

        assert_eq!(
            std::fs::read_to_string(recipe_files.join("app").join("bin")).unwrap(),
            "binary"
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    #[tokio::test]
    async fn invoke_bitbake_reports_not_found() {
        let base = scratch("notfound");
        let err = invoke_bitbake_with("consortium-not-a-real-bitbake", &base, "image", None)
            .await
            .expect_err("missing bitbake is an error");
        assert!(matches!(err, BitbakeError::BitbakeNotFound));
        let _ = std::fs::remove_dir_all(&base);
    }

    /// A recording reporter that captures `command_output` calls.
    #[derive(Default)]
    struct CapturingReporter {
        output: std::sync::Mutex<String>,
    }

    impl BuildReporter for CapturingReporter {
        fn step_started(&self, _step: crate::progress::BuildStep<'_>) {}
        fn step_succeeded(&self, _step: crate::progress::BuildStep<'_>) {}
        fn step_failed(&self, _step: crate::progress::BuildStep<'_>) {}
        fn command_output(&self, output: &str) {
            self.output.lock().unwrap().push_str(output);
        }
    }

    #[tokio::test]
    async fn dry_run_reports_command_without_invoking_bitbake() {
        let base = scratch("dryrun");
        let reporter = CapturingReporter::default();
        // The build_dir need not contain bitbake; a dry run must not spawn it.
        run_with_reporter(
            &BitbakeAction::InvokeBitbake {
                build_dir: base.join("yocto-build"),
                recipe: "consortium-image".to_string(),
                dry_run: true,
            },
            &base,
            None,
            Some(&reporter),
        )
        .await
        .expect("dry run succeeds without bitbake on PATH");

        let out = reporter.output.lock().unwrap().clone();
        assert!(out.contains("[dry-run]"), "output was: {out}");
        assert!(
            out.contains("bitbake consortium-image"),
            "output was: {out}"
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    fn full_spec() -> RecipeSpec {
        RecipeSpec {
            recipe_name: "meta-consortium".to_string(),
            deploy_root: "/opt/consortium".to_string(),
            app_bin: "melt-pot-app".to_string(),
            firmware: vec!["cm33.elf".to_string()],
            ta: Some("uuid.ta".to_string()),
            has_www: true,
            systemd_unit: "consortium-app.service".to_string(),
            dts_files: vec!["consortium-uio.dtso".to_string()],
            boot_files: vec!["boot.cmd".to_string(), "README".to_string()],
        }
    }

    #[test]
    fn render_recipe_covers_every_artifact() {
        let recipe = render_recipe(&full_spec());
        // Install targets for each staged artifact class.
        assert!(recipe.contains("${D}/opt/consortium/app/melt-pot-app"));
        assert!(recipe.contains("${nonarch_base_libdir}/firmware/cm33.elf"));
        assert!(recipe.contains("${systemd_system_unitdir}/consortium-app.service"));
        assert!(recipe.contains("${D}/opt/consortium/www"));
        assert!(recipe.contains("${nonarch_base_libdir}/optee_armtz/uuid.ta"));
        // A `.dtso` overlay is installed into the rootfs (runtime configfs).
        assert!(recipe.contains("${D}/opt/consortium/dts/consortium-uio.dtso"));
        // Boot files are staged but not installed; a guidance comment explains them.
        assert!(recipe.contains("file://boot"));
        assert!(recipe.contains("Bootloader input"));
        assert!(!recipe.contains("/opt/consortium/boot"));
        // The kernel config fragment is a build-time input, never installed to
        // the rootfs by this app recipe.
        assert!(!recipe.contains("/opt/consortium/kernel"));
        // SRC_URI resolves relative to the recipe via THISDIR.
        assert!(recipe.contains("FILESEXTRAPATHS:prepend := \"${THISDIR}:\""));
        assert!(recipe.contains("file://firmware/cm33.elf"));
        // systemd registration.
        assert!(recipe.contains("inherit systemd"));
        assert!(recipe.contains("SYSTEMD_SERVICE:${PN} = \"consortium-app.service\""));
    }

    #[test]
    fn render_recipe_merged_dts_is_a_kernel_input_not_installed() {
        let spec = RecipeSpec {
            dts_files: vec!["consortium-imx95-19x19-evk.dts".to_string()],
            boot_files: Vec::new(),
            ..full_spec()
        };
        let recipe = render_recipe(&spec);
        // A merged `.dts` is staged but not installed into the rootfs.
        assert!(recipe.contains("file://dts"));
        assert!(recipe.contains("Kernel input"));
        assert!(recipe.contains("KERNEL_DEVICETREE"));
        assert!(!recipe.contains("${D}/opt/consortium/dts"));
    }

    #[test]
    fn render_recipe_omits_absent_optionals() {
        let spec = RecipeSpec {
            firmware: Vec::new(),
            ta: None,
            has_www: false,
            dts_files: Vec::new(),
            boot_files: Vec::new(),
            ..full_spec()
        };
        let recipe = render_recipe(&spec);
        assert!(recipe.contains("/opt/consortium/app/melt-pot-app"));
        assert!(!recipe.contains("firmware"));
        assert!(!recipe.contains("optee_armtz"));
        assert!(!recipe.contains("www"));
        assert!(!recipe.contains("/dts"));
        assert!(!recipe.contains("file://boot"));
    }

    #[tokio::test]
    async fn dry_run_writes_recipe_and_skips_bitbake() {
        let base = scratch("recipe");
        let reporter = CapturingReporter::default();
        let spec = full_spec();

        run_with_reporter(
            &BitbakeAction::InvokeBitbake {
                build_dir: base.join("yocto-build"),
                recipe: "meta-consortium".to_string(),
                dry_run: true,
            },
            &base,
            Some(&spec),
            Some(&reporter),
        )
        .await
        .expect("dry run writes recipe without bitbake");

        // The recipe lands at the dist root, next to the staged subdirectories.
        let bb = base.join("meta-consortium.bb");
        assert!(bb.exists(), "recipe should be written");
        assert_eq!(
            std::fs::read_to_string(&bb).unwrap(),
            render_recipe(&spec),
            "written recipe matches the renderer"
        );

        let out = reporter.output.lock().unwrap().clone();
        assert!(out.contains("wrote recipe"), "output was: {out}");
        assert!(out.contains("skipped `bitbake"), "output was: {out}");
        let _ = std::fs::remove_dir_all(&base);
    }
}
