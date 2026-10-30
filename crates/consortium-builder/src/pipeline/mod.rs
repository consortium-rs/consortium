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

//! End-to-end build pipeline: `Consortium.toml` → `dist/`.
//!
//! The eight stages are:
//!
//! 1. compile firmware for remote coprocessors ([`firmware`]);
//! 2. build the defmt "ELF map" the app embeds ([`elf_map`]);
//! 3. build the HMI frontend (`compile::web`);
//! 4. compile + sign the OP-TEE TA ([`tee`]);
//! 5. compile the Linux application ([`application`]);
//! 6. render the systemd unit (`compile::deploy::systemd`);
//! 7. assemble `dist/` ([`dist`]);
//! 8. optionally hand off to bitbake ([`bitbake`]).
//!
//! [`plan`] resolves every stage's inputs without running any build (a dry run
//! that threads the data flow); [`run`] plans, then executes.
//!
//! [`check`] is the `cargo check` of the pipeline: it plans the same way, writes
//! the generated modules, and type-checks every Cargo project without building,
//! linking, or staging anything.
//!
//! [`fetch`] sits beside them rather than inside them: it resolves the same Cargo
//! projects and downloads their dependencies ahead of time, so a later [`run`]
//! with [`PipelineConfig::offline`] never reaches the network.

pub mod application;
pub mod bitbake;
pub mod check;
pub mod dist;
pub mod dtc;
pub mod dts;
pub mod elf_map;
pub mod fetch;
pub mod firmware;
pub mod tee;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg_common::config::{HmiEngine, VisibleCore, Workspace, XcoreName};
use consortium_cfg_common::database::Coprocessor;
use thiserror::Error;

use crate::compile::application::{ApplicationCompileError, ApplicationCompiler};
use crate::compile::deploy::systemd::{self, SystemdRestartPolicy, SystemdUnitConfig};
use crate::compile::env::CstiWorkspace;
use crate::compile::firmware::{FirmwareCompileError, FirmwareCompiler};
use crate::compile::optee::{OpteeCompileError, OpteeCompiler, OpteeConfiguration};
use crate::compile::web::{HmiCompileError, HmiCompiler, HmiConfiguration};
use crate::progress::{BuildReporter, BuildStep};

use application::ApplicationBuildPlan;
use bitbake::{BitbakeAction, BitbakeError, RecipeSpec};
use dist::{DistAssemblyError, DistLayout};
use firmware::FirmwareBuildPlan;
use tee::TeeToolchain;

/// The file name the generated per-core init module is written as inside a crate
/// (`src/<GENERATED_MODULE>`). The crate is expected to `include!`/`mod` it.
const GENERATED_MODULE: &str = "consortium.gen.rs";
const GENERATED_MEMORY_X: &str = "memory.x";

/// Aggregate error for the build pipeline.
#[derive(Error, Debug)]
pub enum PipelineError {
    /// Reading the manifest file failed.
    #[error("reading manifest {path}: {source}")]
    ManifestRead {
        /// The manifest path.
        path: String,
        /// The underlying error.
        source: std::io::Error,
    },

    /// `[profile] workspace = "unified"`, but the manifest directory holds no
    /// Cargo workspace root.
    #[error(
        "[profile] workspace = \"unified\" requires a workspace root Cargo.toml beside the manifest, but {0} {1}"
    )]
    UnifiedWorkspace(String, &'static str),

    /// Parsing/validating the manifest or lowering it failed.
    #[error(transparent)]
    Cfg(#[from] Box<consortium_cfg::CfgError>),

    /// Firmware compilation failed.
    #[error(transparent)]
    Firmware(#[from] FirmwareCompileError),

    /// Application compilation failed.
    #[error(transparent)]
    Application(#[from] ApplicationCompileError),

    /// OP-TEE TA compilation/signing failed.
    #[error(transparent)]
    Optee(#[from] OpteeCompileError),

    /// HMI frontend compilation failed.
    #[error(transparent)]
    Hmi(#[from] HmiCompileError),

    /// Assembling the `dist/` tree failed.
    #[error(transparent)]
    Dist(#[from] DistAssemblyError),

    #[error(transparent)]
    Dtc(#[from] dtc::DtcError),

    /// The bitbake hand-off failed.
    #[error(transparent)]
    Bitbake(#[from] BitbakeError),

    /// Writing a generated module into a crate failed.
    #[error("writing generated module to {path}: {source}")]
    GeneratedWrite {
        /// The generated-module path.
        path: String,
        /// The underlying error.
        source: std::io::Error,
    },

    /// A `[dbg.<core>]` core was not built in Stage 1 (typically a manager core).
    #[error(
        "debug core '{0}' has no compiled firmware to embed: `[dbg.{0}]` names a \
         core that is not a remote firmware target"
    )]
    DbgCoreNotBuilt(String),

    /// A required `[endpoints]` entry is missing.
    #[error("manifest declares no endpoint for the required party '{0}'")]
    MissingEndpoint(String),

    /// A `[tee]` section is present but no OP-TEE toolchain was supplied.
    #[error(
        "manifest has a [tee] section but no OP-TEE toolchain was provided \
         (set TA_DEV_KIT_DIR or pass a toolchain)"
    )]
    MissingTeeToolchain,

    /// A core name could not be resolved.
    #[error("unknown core '{0}'")]
    UnknownCore(String),

    /// Code generation produced invalid tokens or a path could not be resolved.
    #[error("codegen error: {0}")]
    Codegen(String),

    /// A `cargo fetch` invocation failed.
    #[error("fetching dependencies for {label} ({target}): {message}")]
    Fetch {
        /// The artifact whose crate was being fetched (a core name, `ta`, `app`).
        label: String,
        /// The Rust target triple the fetch was scoped to.
        target: String,
        /// The underlying failure.
        message: String,
    },

    /// Installing a target's Rust standard library failed.
    #[error("installing rust target(s) {targets}: {message}")]
    RustTarget {
        /// The space-separated triples that were requested.
        targets: String,
        /// The underlying failure.
        message: String,
    },

    /// Installing the HMI frontend's dependencies failed.
    #[error("installing HMI dependencies in {dir}: {message}")]
    HmiFetch {
        /// The frontend project directory the installer ran in.
        dir: String,
        /// The underlying failure.
        message: String,
    },
}

impl From<consortium_cfg::CfgError> for PipelineError {
    fn from(error: consortium_cfg::CfgError) -> Self {
        Self::Cfg(Box::new(error))
    }
}

/// How cargo is invoked for every Rust artifact the pipeline builds.
///
/// The firmware, TA, and application are three separate cargo projects; these are
/// the flags that must agree across all of them, carried as one value so a caller
/// cannot set them inconsistently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CargoOptions {
    /// Build in release mode.
    pub release: bool,
    /// Pass `--offline`, restricting cargo to its local registry cache. A build
    /// that has not been preceded by [`fetch`] will fail rather than reach the
    /// network, which is the point: it makes the missing dependency loud.
    pub offline: bool,
}

/// All parameters for a pipeline run.
pub struct PipelineConfig {
    /// Path to the `Consortium.toml` manifest.
    pub manifest_path: PathBuf,
    /// Root of the `dist/` staging tree to assemble.
    pub dist_root: PathBuf,
    /// Authoritative Linux kernel checkout. All BSP DTS/DTSI inputs and
    /// dt-bindings are read from this tree; Consortium bundles none of them.
    pub linux: PathBuf,
    /// Build in release mode.
    pub release: bool,
    /// Build from cargo's local registry cache only (`--offline`), never the
    /// network. Populate the cache first with [`fetch::run_with_reporter`].
    pub offline: bool,
    /// HMI frontend build parameters, or `None` to skip the HMI stage. Derive
    /// with [`derive_hmi_configuration`].
    pub hmi: Option<HmiConfiguration>,
    /// OP-TEE toolchain, or `None` (an error if the manifest declares `[tee]`).
    pub tee_toolchain: Option<TeeToolchain>,
    /// Sign the OP-TEE TA after building it. `false` (from `--skip-tee-sign`)
    /// stops at the unsigned ELF, so no signer script or keys are needed.
    pub sign_tee: bool,
    /// Require the selected BSP DTS source to declare every manifest remoteproc
    /// core's node. `false` (from `--skip-remoteproc-check`) downgrades a
    /// missing node to a warning and skips DTS fixups for that core instead of
    /// failing the build — useful when the `--linux` checkout does not yet
    /// carry a vendor's out-of-tree remoteproc binding (the rendered DTS will
    /// not actually be able to load that firmware until it does).
    pub strict_remoteproc: bool,
    /// Bitbake hand-off actions run in order after `dist/` is assembled. Empty
    /// skips the stage. Multiple actions compose: e.g. stage `dist/` into a
    /// recipe's `files/` dir *and* record the deferred `bitbake` command
    /// (dry-run) when the recipe is built later on a separate Yocto host.
    pub bitbake: Vec<BitbakeAction>,
}

impl PipelineConfig {
    /// The cargo flags every Rust stage of this run shares.
    pub fn cargo(&self) -> CargoOptions {
        CargoOptions {
            release: self.release,
            offline: self.offline,
        }
    }
}

/// The fully resolved plan for a pipeline run: every stage's inputs threaded
/// together without running any build.
pub struct PipelinePlan {
    /// Directory containing the manifest (all relative paths resolve against it).
    pub manifest_dir: PathBuf,
    /// Rust path for the shared message-type crate used by codegen.
    pub ty_crate: syn::Path,
    /// One plan per remote coprocessor firmware.
    pub firmware: Vec<FirmwareBuildPlan>,
    /// Expected build-host ELF path per firmware core.
    pub firmware_outputs: HashMap<VisibleCore, PathBuf>,
    /// The debug "ELF map": firmware ELF path per `[dbg.<core>]` core.
    pub dbg_elfs: HashMap<VisibleCore, PathBuf>,
    /// HMI build parameters, when the HMI stage runs.
    pub hmi: Option<HmiConfiguration>,
    /// The HMI dist directory the frontend build produces, when it runs.
    pub hmi_dist: Option<PathBuf>,
    /// TA build parameters, when the manifest declares `[tee]`.
    pub ta: Option<OpteeConfiguration>,
    /// The application build plan and its generated AP init module.
    pub application: ApplicationBuildPlan,
    /// The Linux device tree to write under `dist/dts/linux/`, as
    /// `(file_name, contents)`: a merged `.dts` (default) or a `.dtso` overlay.
    /// Empty when the manifest declares neither UIO nor remoteproc devices.
    pub dts_files: Vec<(String, String)>,
    /// Exact BSP DTS sources used to produce `dts_files`.
    pub dts_source_closure: Vec<(String, String)>,
    /// The kernel-side files to write under `dist/kernel/`, as
    /// `(file_name, contents)`: the UIO config fragment and, when `[profile]
    /// kernel_recipe` is set, the `.bbappend` that merges it. Empty when neither
    /// UIO nor remoteproc needs kernel configuration.
    pub kernel_files: Vec<(String, String)>,
    /// The boot-args files to write under `dist/boot/`, as
    /// `(file_name, contents)`: the family-specific bootloader snippet(s) that
    /// put `uio_pdrv_genirq.of_id=generic-uio` on the kernel command line, plus a
    /// README. Empty when the manifest declares no UIO devices.
    pub boot_files: Vec<(String, String)>,
    /// The systemd unit to render.
    pub systemd: SystemdUnitConfig,
}

/// Loads and validates the manifest at `manifest_path`.
pub async fn load_session(manifest_path: &Path) -> Result<Session, PipelineError> {
    consortium_log::trace!("loading manifest {}", manifest_path.display());
    let toml = tokio::fs::read_to_string(manifest_path)
        .await
        .map_err(|source| {
            consortium_log::trace!("manifest read failed: {}", manifest_path.display());
            PipelineError::ManifestRead {
                path: manifest_path.display().to_string(),
                source,
            }
        })?;
    Ok(Session::from_toml(&toml)?)
}

/// Resolves `[profile] workspace` against the manifest directory into the
/// `CSTI_*` directories every endpoint build is given.
///
/// Both directories are made absolute (lexically). `CSTI_MCU_PATH` is set only
/// when the manifest declares exactly one firmware crate. A unified workspace
/// requires `<manifest dir>/Cargo.toml` to exist and declare `[workspace]`, since
/// every endpoint's output then lands in the manifest directory's `target/`.
pub fn resolve_workspace(
    session: &Session,
    manifest_dir: &Path,
) -> Result<CstiWorkspace, PipelineError> {
    let absolute = |p: &Path| std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf());
    let manifest_dir = absolute(manifest_dir);
    let layout = session.config.profile.workspace;

    if layout == Workspace::Unified {
        let cargo_toml = manifest_dir.join("Cargo.toml");
        let shown = cargo_toml.display().to_string();
        let contents = std::fs::read_to_string(&cargo_toml)
            .map_err(|_| PipelineError::UnifiedWorkspace(shown.clone(), "does not exist"))?;
        let is_workspace = contents
            .parse::<toml::Table>()
            .is_ok_and(|table| table.contains_key("workspace"));
        if !is_workspace {
            return Err(PipelineError::UnifiedWorkspace(
                shown,
                "has no [workspace] table",
            ));
        }
    }

    let firmware = firmware::firmware_endpoints(session)?;
    let mcu_dir = match firmware.as_slice() {
        [(_, _, root)] => Some(absolute(&manifest_dir.join(root))),
        _ => None,
    };

    Ok(CstiWorkspace {
        layout,
        manifest_dir,
        mcu_dir,
    })
}

/// Derives HMI build parameters from the manifest's `[hmi]` section, or `None`.
///
/// Resolves the frontend project (`[hmi].source`) and dist (`[hmi].build`)
/// directories against the manifest directory and the build command from the
/// environment/PATH.
pub fn derive_hmi_configuration(
    session: &Session,
    manifest_dir: &Path,
) -> Result<Option<HmiConfiguration>, PipelineError> {
    let Some(hmi) = &session.config.hmi else {
        return Ok(None);
    };
    hmi.validate().map_err(PipelineError::Codegen)?;

    let project_dir = manifest_dir.join(&hmi.source);
    let dist_dir = manifest_dir.join(&hmi.build);
    let command = match hmi.engine {
        HmiEngine::Cog => {
            crate::compile::utils::resolve_hmi_build_command().map_err(PipelineError::Codegen)?
        }
        HmiEngine::Pocket => {
            // `validate` guaranteed `app` is present and non-empty.
            let app = hmi.app.as_deref().unwrap_or_default();
            crate::compile::utils::resolve_pocket_build_command(app, &dist_dir)
                .map_err(PipelineError::Codegen)?
        }
    };
    Ok(Some(HmiConfiguration {
        engine: hmi.engine,
        app: hmi.app.clone(),
        project_dir,
        dist_dir,
        command,
    }))
}

/// Resolves every stage's inputs without building (a dry run).
pub async fn plan(
    session: &Session,
    config: &PipelineConfig,
) -> Result<PipelinePlan, PipelineError> {
    let manifest_dir = config
        .manifest_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let ty_crate = resolve_ty_crate(session, &manifest_dir).await?;
    let cargo = config.cargo();
    let workspace = resolve_workspace(session, &manifest_dir)?;

    // Stage 1: firmware plans + expected ELF outputs.
    let firmware = firmware::plan_firmware_builds(session, &workspace, cargo, &ty_crate)?;
    let mut firmware_outputs = HashMap::new();
    for plan in &firmware {
        let elf = firmware::firmware_elf_path(&plan.config).await?;
        firmware_outputs.insert(plan.core, elf);
    }

    // Stage 2: the defmt ELF map (fails fast if a dbg core was not built). The AP
    // decodes defmt from the `*_original.elf` image (the objcopy-stripped deploy
    // ELF has no `.defmt` section), so map each raw output to its original sibling.
    let ir = session.lower()?;
    let dbg_sources: HashMap<VisibleCore, PathBuf> = firmware_outputs
        .iter()
        .map(|(core, raw)| (*core, crate::compile::firmware::original_elf_path(raw)))
        .collect();
    let dbg_elfs = elf_map::resolve_dbg_elf_paths(&dbg_sources, ir.dbg.rings().keys().copied())?;

    // Stage 3: HMI dist directory (built later; recorded here).
    let hmi_dist = config.hmi.as_ref().map(|h| h.dist_dir.clone());

    // Stage 4: TA plan.
    let ta = tee::plan_ta_build(
        session,
        &workspace,
        config.tee_toolchain.as_ref(),
        cargo,
        config.sign_tee,
    )?;

    // Stage 5: application plan (AP module embeds the resolved firmware ELFs).
    let application = application::plan_application_build(
        session,
        &workspace,
        hmi_dist.as_deref(),
        &dbg_elfs,
        cargo,
        &ty_crate,
    )?;

    // Stage 6: the Linux device tree (merged `.dts` or overlay `.dtso`, built on
    // the vendor base for `[profile] chip`/`devkit`) plus the kernel-side files
    // enabling its drivers, and the boot-args files — all from the lowered IR.
    let profile = &session.config.profile;
    let dts_cfg = &session.config.dts;
    // The coprocessor cores the build targets, so remoteproc fixups stay scoped.
    let target_cores: Vec<String> = firmware.iter().map(|p| p.core.to_string()).collect();
    let (dts_files, dts_source_closure) = match dts::render_linux_dts(
        &profile.chip,
        profile.devkit.as_deref(),
        dts_cfg.base.as_deref(),
        dts_cfg.mode,
        &ir,
        &target_cores,
        &config.linux,
        config.strict_remoteproc,
    )? {
        Some(rendered) => {
            consortium_log::trace!(
                "rendered linux dts {} with {} warnings",
                rendered.file_name,
                rendered.warnings.len()
            );
            for warning in &rendered.warnings {
                consortium_log::trace!("dts warning: {}", warning);
                eprintln!("warning: {warning}");
            }
            (
                vec![(rendered.file_name, rendered.contents)],
                rendered.source_closure,
            )
        }
        None => {
            consortium_log::trace!("no UIO or remoteproc devices; no linux dts rendered");
            (Vec::new(), Vec::new())
        }
    };

    // The kernel config fragment plus, when the manifest names a kernel recipe, a
    // `.bbappend` that merges it into the kernel build. Both land in `dist/kernel/`.
    let mut kernel_files = dts::render_kernel_kconfig(&profile.chip, &ir)
        .map(|contents| vec![(dts::KCONFIG_NAME.to_string(), contents)])
        .unwrap_or_default();
    if let Some(kernel_recipe) = &session.config.profile.kernel_recipe {
        let fragment_names: Vec<String> = kernel_files.iter().map(|(n, _)| n.clone()).collect();
        if let Some(bbappend) = dts::render_kernel_bbappend(&fragment_names) {
            kernel_files.push((dts::kernel_bbappend_name(kernel_recipe), bbappend));
        }
    }

    // The family-specific boot-args files under `dist/boot/`.
    let boot_files = dts::render_boot(&profile.chip, &ir);

    // Stage 7: systemd unit.
    let systemd = systemd_config(session, &application.config.path).await?;

    Ok(PipelinePlan {
        manifest_dir,
        ty_crate,
        firmware,
        firmware_outputs,
        dbg_elfs,
        hmi: config.hmi.clone(),
        hmi_dist,
        ta,
        application,
        dts_files,
        dts_source_closure,
        kernel_files,
        boot_files,
        systemd,
    })
}

/// Runs the full pipeline, returning the assembled `dist/` root.
pub async fn run(config: PipelineConfig) -> Result<PathBuf, PipelineError> {
    run_inner(config, None).await
}

/// Runs the full pipeline while reporting step progress and subprocess output.
pub async fn run_with_reporter(
    config: PipelineConfig,
    reporter: &dyn BuildReporter,
) -> Result<PathBuf, PipelineError> {
    run_inner(config, Some(reporter)).await
}

async fn run_inner(
    config: PipelineConfig,
    reporter: Option<&dyn BuildReporter>,
) -> Result<PathBuf, PipelineError> {
    consortium_log::trace!("pipeline run: manifest {}", config.manifest_path.display());
    let session = load_session(&config.manifest_path).await?;
    let plan = plan(&session, &config).await?;
    consortium_log::trace!(
        "pipeline plan: {} firmware, ta={}, hmi={}, {} bitbake",
        plan.firmware.len(),
        plan.ta.is_some(),
        config.hmi.is_some(),
        config.bitbake.len()
    );

    let total_steps = plan.firmware.len()
        + usize::from(config.hmi.is_some())
        + usize::from(plan.ta.is_some())
        + 2
        + config.bitbake.len();
    let mut steps = StepRunner::new(reporter, total_steps);

    // The deploy (objcopy-processed) ELF per core, staged into `dist/firmware/`.
    // The AP already embedded the `*_original.elf` at plan time (Stage 2/5); the
    // unstripped original is also staged into `dist/firmware/` so the integrator
    // keeps its `.defmt` section for host-side defmt table parsing.
    let mut firmware_outputs = HashMap::new();
    let mut firmware_originals = HashMap::new();
    for fw in &plan.firmware {
        let detail = format!("{} ({})", fw.core, fw.config.target);
        let (deploy, original) = steps
            .run("Build firmware", Some(&detail), async {
                write_generated_module(&fw.config.path, &fw.generated).await?;
                write_generated_memory_x(&fw.config).await?;
                let artifacts =
                    FirmwareCompiler::build_with_reporter(fw.config.clone(), reporter).await?;
                Ok((artifacts.deploy, artifacts.original))
            })
            .await?;
        firmware_outputs.insert(fw.core, deploy);
        firmware_originals.insert(fw.core, original);
    }

    let hmi_dist = match &config.hmi {
        Some(hmi) => {
            let detail = hmi.project_dir.display().to_string();
            Some(
                steps
                    .run("Build HMI", Some(&detail), async {
                        let dist = HmiCompiler::build_with_reporter(hmi.clone(), reporter).await?;
                        Ok(dist)
                    })
                    .await?,
            )
        }
        None => None,
    };

    let ta = match plan.ta {
        Some(ta) => {
            let detail = if ta.sign {
                "signed trusted application"
            } else {
                "unsigned trusted application"
            };
            Some(
                steps
                    .run("Build TEE", Some(detail), async {
                        let ta = OpteeCompiler::build_with_reporter(ta, reporter).await?;
                        Ok(ta)
                    })
                    .await?,
            )
        }
        None => None,
    };

    let app_detail = plan.application.config.target.clone();
    let application = steps
        .run("Build application", Some(&app_detail), async {
            write_generated_module(&plan.application.config.path, &plan.application.generated)
                .await?;
            let app =
                ApplicationCompiler::build_with_reporter(plan.application.config.clone(), reporter)
                    .await?;
            Ok(app)
        })
        .await?;

    let systemd_units = vec![(
        plan.systemd.file_name(),
        systemd::render_unit(&plan.systemd),
    )];

    let dist_detail = config.dist_root.display().to_string();
    let layout = DistLayout::new(&config.dist_root);
    steps
        .run("Assemble dist", Some(&dist_detail), async {
            let inputs = dist::DistAssemblyInputs {
                firmware: &firmware_outputs,
                firmware_originals: &firmware_originals,
                application: &application,
                ta: ta.as_deref(),
                hmi: config
                    .hmi
                    .as_ref()
                    .zip(hmi_dist.as_deref())
                    .map(|(cfg, dist)| dist::HmiStaging {
                        engine: cfg.engine,
                        app: cfg.app.as_deref(),
                        dist,
                    }),
                dts_files: &plan.dts_files,
                kconfig_fragments: &plan.kernel_files,
                boot_files: &plan.boot_files,
                systemd_units: &systemd_units,
            };
            dist::assemble(&layout, inputs).await?;
            Ok(())
        })
        .await?;

    // Compile the emitted device tree against the same `--linux` checkout from
    // which its BSP sources were read.
    if let Some((dts_name, _)) = plan.dts_files.first() {
        let arch = dtc::arch_dir(&plan.application.config.target);
        let dts_dir = layout.dts_linux_dir();
        let detail = format!("dtc {dts_name}");
        steps
            .run("Compile device tree", Some(&detail), async {
                let out = dtc::compile_device_tree(
                    &dts_dir,
                    dts_name,
                    &plan.dts_source_closure,
                    &config.linux,
                    arch,
                )
                .await?;
                Ok(out)
            })
            .await?;
    }

    for action in &config.bitbake {
        // A dry-run invoke emits a self-contained `.bb` recipe describing how to
        // install the assembled `dist/` tree, so a separate Yocto host can build
        // it without re-running the Consortium pipeline.
        let recipe_spec = match action {
            BitbakeAction::InvokeBitbake {
                recipe, dry_run, ..
            } if *dry_run => Some(recipe_spec(
                recipe,
                &session.config.profile.deploy_root,
                &application,
                &firmware_outputs,
                ta.as_deref(),
                hmi_dist.as_deref(),
                &plan.systemd.file_name(),
                &plan.dts_files,
                &plan.boot_files,
            )?),
            _ => None,
        };

        let (label, detail) = match action {
            BitbakeAction::CopyIntoRecipe { recipe_files_dir } => (
                "Stage into recipe",
                format!("copy to {}", recipe_files_dir.display()),
            ),
            BitbakeAction::InvokeBitbake {
                build_dir,
                recipe,
                dry_run,
            } => {
                let dry = if *dry_run {
                    " (dry-run, recipe only)"
                } else {
                    ""
                };
                (
                    "Run bitbake",
                    format!("{recipe} in {}{dry}", build_dir.display()),
                )
            }
        };
        steps
            .run(label, Some(&detail), async {
                bitbake::run_with_reporter(
                    action,
                    &config.dist_root,
                    recipe_spec.as_ref(),
                    reporter,
                )
                .await?;
                Ok(())
            })
            .await?;
    }

    Ok(config.dist_root)
}

/// Builds the install manifest for a generated `.bb` recipe from the pipeline's
/// resolved outputs. Artifact fields are the bare file names the `dist/` stage
/// writes under each subdirectory.
#[allow(clippy::too_many_arguments)]
fn recipe_spec(
    recipe_name: &str,
    deploy_root: &str,
    application: &Path,
    firmware_outputs: &HashMap<VisibleCore, PathBuf>,
    ta: Option<&Path>,
    hmi_dist: Option<&Path>,
    systemd_unit: &str,
    dts_files: &[(String, String)],
    boot_files: &[(String, String)],
) -> Result<RecipeSpec, PipelineError> {
    let file_name = |path: &Path| -> Result<String, PipelineError> {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(str::to_string)
            .ok_or_else(|| PipelineError::Codegen(format!("no file name for {}", path.display())))
    };

    // Firmware is staged as `firmware/<core>.elf`, matching the dist layout.
    let firmware = firmware_outputs
        .keys()
        .map(|core| format!("{core}.elf"))
        .collect();

    Ok(RecipeSpec {
        recipe_name: recipe_name.to_string(),
        deploy_root: deploy_root.to_string(),
        app_bin: file_name(application)?,
        firmware,
        ta: ta.map(file_name).transpose()?,
        has_www: hmi_dist.is_some(),
        systemd_unit: systemd_unit.to_string(),
        dts_files: dts_files.iter().map(|(name, _)| name.clone()).collect(),
        boot_files: boot_files.iter().map(|(name, _)| name.clone()).collect(),
    })
}

struct StepRunner<'a> {
    reporter: Option<&'a dyn BuildReporter>,
    next: usize,
    total: usize,
}

impl<'a> StepRunner<'a> {
    fn new(reporter: Option<&'a dyn BuildReporter>, total: usize) -> Self {
        Self {
            reporter,
            next: 0,
            total,
        }
    }

    async fn run<T, F>(
        &mut self,
        name: &str,
        detail: Option<&str>,
        future: F,
    ) -> Result<T, PipelineError>
    where
        F: std::future::Future<Output = Result<T, PipelineError>>,
    {
        self.next += 1;
        let step = BuildStep {
            index: self.next,
            total: self.total,
            name,
            detail,
        };
        if let Some(reporter) = self.reporter {
            reporter.step_started(step);
        }

        let result = future.await;
        if let Some(reporter) = self.reporter {
            match &result {
                Ok(_) => reporter.step_succeeded(step),
                Err(_) => reporter.step_failed(step),
            }
        }
        result
    }
}

/// The [`VisibleCore`] a chip-database coprocessor entry names.
pub(crate) fn visible_core_of(cp: &Coprocessor) -> Result<VisibleCore, PipelineError> {
    cp.name
        .parse()
        .map_err(|_| PipelineError::UnknownCore(cp.name.clone()))
}

/// The Rust target triple for a coprocessor core (the single source of truth).
pub(crate) fn core_rust_triple(core: VisibleCore) -> String {
    match core {
        VisibleCore::Mcore(name) => name.rust_triple(),
        VisibleCore::Rcore(name) => name.rust_triple(),
        VisibleCore::Rvcore(name) => name.rust_triple(),
        // The AP is never a firmware target (it is absent from the chip `cp`
        // list); the application stage resolves its Linux triple separately.
        VisibleCore::Ap => "aarch64-unknown-linux-gnu".to_string(),
    }
}

/// Resolves the shared message-type crate's Rust path.
///
/// Prefers the `[package] name` in the shared crate's `Cargo.toml`; falls back to
/// the last component of `[profile] shared` when that file cannot be read (so the
/// dry-run [`plan`] works without every crate present on disk).
async fn resolve_ty_crate(
    session: &Session,
    manifest_dir: &Path,
) -> Result<syn::Path, PipelineError> {
    let shared_dir = manifest_dir.join(&session.config.profile.shared);
    let name = match crate::compile::utils::binary_name(&shared_dir).await {
        Ok(name) => name,
        Err(_) => shared_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("shared")
            .to_string(),
    };
    let ident = name.replace('-', "_");
    syn::parse_str(&format!("::{ident}"))
        .map_err(|e| PipelineError::Codegen(format!("bad shared crate name '{ident}': {e}")))
}

/// Builds the systemd unit for the application, using its binary name for
/// `ExecStart=`/`WorkingDirectory=` under the manifest's deploy root.
async fn systemd_config(
    session: &Session,
    app_path: &Path,
) -> Result<SystemdUnitConfig, PipelineError> {
    let binary = crate::compile::utils::binary_name(app_path)
        .await
        .map_err(PipelineError::Codegen)?;
    let deploy_root = &session.config.profile.deploy_root;
    Ok(SystemdUnitConfig {
        unit_name: "consortium-app".to_string(),
        exec_start: format!("{deploy_root}/app/{binary}"),
        working_directory: format!("{deploy_root}/app"),
        description: None,
        restart: SystemdRestartPolicy::OnFailure,
    })
}

/// Writes a generated init module into a crate at `src/consortium.gen.rs`.
///
/// The crate is expected to bring it in with `include!("consortium.gen.rs")` (or
/// a `mod`), which is how the melt-pot examples wire generated code in.
async fn write_generated_module(
    project_dir: &Path,
    source: &str,
) -> Result<PathBuf, PipelineError> {
    let src_dir = project_dir.join("src");
    tokio::fs::create_dir_all(&src_dir)
        .await
        .map_err(|source| {
            consortium_log::trace!("failed to create src dir {}", src_dir.display());
            PipelineError::GeneratedWrite {
                path: src_dir.display().to_string(),
                source,
            }
        })?;
    let path = src_dir.join(GENERATED_MODULE);
    consortium_log::trace!(
        "writing {} bytes of generated code to {}",
        source.len(),
        path.display()
    );
    tokio::fs::write(&path, source).await.map_err(|source| {
        consortium_log::trace!("failed to write generated module {}", path.display());
        PipelineError::GeneratedWrite {
            path: path.display().to_string(),
            source,
        }
    })?;
    Ok(path)
}

/// Writes the structured remoteproc linker memory map into the firmware crate.
async fn write_generated_memory_x(
    config: &crate::compile::firmware::FirmwareConfiguration,
) -> Result<PathBuf, PipelineError> {
    let layout = config.remoteproc.as_ref().ok_or_else(|| {
        PipelineError::Codegen(format!(
            "firmware endpoint {} has no remoteproc memory contract",
            config.path.display()
        ))
    })?;
    let source = layout
        .linker_memory()
        .map_err(|error| PipelineError::Codegen(error.to_string()))?
        .render();
    let path = config.path.join(GENERATED_MEMORY_X);
    tokio::fs::write(&path, source)
        .await
        .map_err(|source| PipelineError::GeneratedWrite {
            path: path.display().to_string(),
            source,
        })?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    const STM32: &str = include_str!("../../../consortium-cfg/Consortium.example.stm32mp257.toml");

    /// Fabricates a project tree on disk (manifest + per-endpoint crates with
    /// `Cargo.toml`s, no real sources) so the dry-run [`plan`] can resolve crate
    /// names and expected ELF paths without invoking any build.
    fn fabricate_project(tag: &str, manifest: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("consortium-pipeline-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("Consortium.toml"), manifest).unwrap();
        for (dir, pkg) in [
            ("app", "melt-pot-app"),
            ("ta", "melt-pot-ta"),
            ("firmware", "melt-pot-mcu"),
            ("shared", "melt-pot-shared"),
        ] {
            let crate_dir = root.join(dir);
            std::fs::create_dir_all(&crate_dir).unwrap();
            std::fs::write(
                crate_dir.join("Cargo.toml"),
                format!("[package]\nname = \"{pkg}\"\nversion = \"0.0.0\"\n"),
            )
            .unwrap();
        }
        root
    }

    fn unified(manifest: &str) -> String {
        manifest.replace(
            "deploy_root = \"/opt/consortium\"",
            "deploy_root = \"/opt/consortium\"\nworkspace   = \"unified\"",
        )
    }

    #[test]
    fn unified_workspace_requires_a_root_workspace_manifest() {
        let manifest = unified(STM32);
        let root = fabricate_project("unified-missing", &manifest);
        let session = Session::from_toml(&manifest).unwrap();

        let err = resolve_workspace(&session, &root).expect_err("no root Cargo.toml");
        assert!(matches!(
            err,
            PipelineError::UnifiedWorkspace(_, "does not exist")
        ));

        // A plain package manifest is not a workspace root.
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"x\"\nversion = \"0.0.0\"\n",
        )
        .unwrap();
        let err = resolve_workspace(&session, &root).expect_err("no [workspace]");
        assert!(matches!(
            err,
            PipelineError::UnifiedWorkspace(_, "has no [workspace] table")
        ));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn unified_workspace_builds_into_the_manifest_target() {
        let manifest = unified(STM32);
        let root = fabricate_project("unified", &manifest);
        std::fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"app\", \"firmware\"]\n",
        )
        .unwrap();
        let session = Session::from_toml(&manifest).unwrap();

        let workspace = resolve_workspace(&session, &root).expect("workspace root present");
        assert_eq!(workspace.layout, Workspace::Unified);
        let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
        let firmware = firmware::plan_firmware_builds(
            &session,
            &workspace,
            CargoOptions::default(),
            &ty_crate,
        )
        .unwrap();
        let elf = firmware::firmware_elf_path(&firmware[0].config)
            .await
            .unwrap();
        assert_eq!(
            elf,
            root.join("target/thumbv8m.main-none-eabihf/debug/melt-pot-mcu")
        );

        let dbg_elfs = HashMap::from([(
            firmware[0].core,
            crate::compile::firmware::original_elf_path(&elf),
        )]);
        let application = application::plan_application_build(
            &session,
            &workspace,
            None,
            &dbg_elfs,
            CargoOptions::default(),
            &ty_crate,
        )
        .unwrap();
        assert!(
            application
                .generated
                .contains("::core::env!(\"CSTI_MANIFEST_PATH\")")
        );
        assert!(
            application
                .generated
                .contains("\"/target/thumbv8m.main-none-eabihf/debug/melt-pot-mcu_original.elf\"")
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn plan_threads_all_stages() {
        let manifest = format!("{STM32}\n[dts]\nbase = \"test-board\"\nmode = \"merge\"\n");
        let root = fabricate_project("plan", &manifest);
        let vendor = root.join("linux/arch/arm64/boot/dts/st");
        std::fs::create_dir_all(&vendor).unwrap();
        std::fs::write(
            vendor.join("test-board.dts"),
            r#"/dts-v1/;
            / {
                model = "Pipeline test board";
                m33: remoteproc-m33 {
                    compatible = "st,stm32mp2-m33";
                };
            };"#,
        )
        .unwrap();
        let session = Session::from_toml(&manifest).unwrap();
        let config = PipelineConfig {
            manifest_path: root.join("Consortium.toml"),
            dist_root: root.join("dist"),
            linux: root.join("linux"),
            release: true,
            offline: false,
            hmi: Some(HmiConfiguration {
                engine: HmiEngine::Cog,
                app: None,
                project_dir: root.join("website"),
                dist_dir: root.join("website/dist"),
                command: crate::compile::web::HmiBuildCommand::Custom(vec!["true".into()]),
            }),
            tee_toolchain: Some(TeeToolchain {
                ta_dev_kit_dir: PathBuf::from("/opt/optee/export-ta_arm64"),
                optee_signer: PathBuf::from("/opt/optee/scripts/sign_encrypt.py"),
                linker: None,
            }),
            sign_tee: true,
            strict_remoteproc: true,
            bitbake: Vec::new(),
        };

        let plan = plan(&session, &config).await.expect("plan succeeds");

        // ty_crate came from the shared crate's Cargo.toml package name
        // (`melt-pot-shared` → `::melt_pot_shared`).
        assert_eq!(
            plan.ty_crate.segments.last().unwrap().ident.to_string(),
            "melt_pot_shared"
        );

        // Stage 1: exactly cm33 (the remote firmware target) is planned, and its
        // expected ELF path uses the resolved binary name + target triple.
        assert_eq!(plan.firmware.len(), 1);
        let cm33 = VisibleCore::Mcore("cm33".parse().unwrap());
        // The endpoint is `./firmware`, so the resolved path keeps that prefix.
        let expected_elf =
            root.join("./firmware/target/thumbv8m.main-none-eabihf/release/melt-pot-mcu");
        assert_eq!(plan.firmware_outputs[&cm33], expected_elf);

        // Stage 2: the dbg ELF map points cm33's ring at the `*_original.elf`
        // sibling (the defmt-bearing image), absolutized for `include_bytes!`.
        let original_elf =
            std::path::absolute(crate::compile::firmware::original_elf_path(&expected_elf))
                .unwrap();
        assert_eq!(plan.dbg_elfs[&cm33], original_elf);

        // Stage 5: the AP module embeds the `*_original.elf` Stage 1 will produce,
        // and the app targets Linux with the HMI served from the deploy root.
        // (prettyplease may wrap a long `include_bytes!(...)` across lines, so
        // assert on the embedded path literal rather than the whole call.)
        // The path is relative to the lone firmware crate, which the application
        // build receives as `CSTI_MCU_PATH`.
        assert!(plan.application.generated.contains("include_bytes!"));
        assert!(
            plan.application
                .generated
                .contains("::core::env!(\"CSTI_MCU_PATH\")")
        );
        let relative = original_elf.strip_prefix(root.join("firmware")).unwrap();
        assert!(
            plan.application
                .generated
                .contains(&format!("\"/{}\"", relative.display()))
        );
        let workspace = plan.application.config.workspace.as_ref().unwrap();
        assert_eq!(
            workspace.mcu_dir.as_deref(),
            Some(root.join("firmware").as_path())
        );
        assert_eq!(plan.application.config.target, "aarch64-unknown-linux-gnu");
        assert_eq!(
            plan.application.config.hmi.as_ref().unwrap(),
            &crate::compile::env::HmiConfig::Cog {
                dist: "/opt/consortium/www".to_string(),
                debug_url: Some("http://127.0.0.1:5173/".to_string()),
            }
        );

        // Stage 4: TA resolved from the manifest.
        assert!(plan.ta.is_some());

        // Stage 6: systemd ExecStart uses the app binary under the deploy root.
        assert_eq!(plan.systemd.exec_start, "/opt/consortium/app/melt-pot-app");

        // Stage 6: the UIO manifest yields a merged device tree from the
        // configured kernel fixture plus the kernel config fragment enabling
        // its drivers.
        assert_eq!(
            plan.dts_files
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>(),
            vec!["consortium-test-board.dts"]
        );
        // A merged `.dts` re-emits the vendor base plus the injected UIO nodes.
        assert!(plan.dts_files[0].1.contains("compatible = \"generic-uio\""));
        let kconfig = plan
            .kernel_files
            .iter()
            .find(|(n, _)| n == dts::KCONFIG_NAME)
            .expect("consolidated kconfig fragment planned");
        assert!(kconfig.1.contains("CONFIG_UIO_PDRV_GENIRQ=y"));
        assert!(kconfig.1.contains("CONFIG_UIO_DMEM_GENIRQ=y"));
        assert!(kconfig.1.contains("CONFIG_REMOTEPROC=y"));
        assert!(kconfig.1.contains("CONFIG_STM32_RPROC=y"));
        assert_eq!(
            plan.kernel_files
                .iter()
                .filter(|(name, _)| name.ends_with(".cfg"))
                .count(),
            1
        );
        assert!(kconfig.1.contains("CONFIG_OF_OVERLAY=y"));
        assert!(kconfig.1.contains("CONFIG_OF_CONFIGFS=y"));
        // STM32 → an extlinux boot-args fragment lands under `dist/boot/`.
        assert!(plan.boot_files.iter().any(|(n, _)| n == "extlinux-uio.cfg"));
        // No `[profile] kernel_recipe` in the example, so no bbappend is emitted.
        assert!(
            !plan
                .kernel_files
                .iter()
                .any(|(n, _)| n.ends_with(".bbappend"))
        );

        let _ = std::fs::remove_dir_all(&root);
    }
}
