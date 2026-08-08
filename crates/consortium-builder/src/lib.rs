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

//! Compiles, post-processes, and stages every artifact a `Consortium.toml` describes.
//!
//! `consortium-cfg` answers "what does this system look like, and is it valid?" and emits
//! generated source. This crate answers "now actually build it": four compilers, a
//! post-processing step remoteproc requires, and a staging tree a packaging recipe can
//! consume.
//!
//! It stops short of deployment. Nothing here writes to a live target filesystem — the
//! output is a local `dist/` tree plus an optional BitBake hand-off.
//!
//! # Layout
//!
//! | Module         | Role                                                        |
//! | -------------- | ----------------------------------------------------------- |
//! | [`pipeline`]   | The end-to-end `Consortium.toml` → `dist/` flow, stage by stage. |
//! | `compile`      | One compiler per artifact kind, behind the [`Compiler`] trait. |
//! | [`progress`]   | Reporting hooks the CLI uses to keep a step prompt visible.    |
//!
//! # Fetch, then build offline
//!
//! [`pipeline::fetch`] resolves the same Cargo projects the build plan does and
//! downloads their dependencies (plus each target's `std` and the frontend's
//! packages) up front. A subsequent run with [`PipelineConfig::offline`] passes
//! `--offline` to every cargo invocation, so the build reads only local caches —
//! useful for air-gapped hosts, reproducible CI, and container images that should
//! not re-resolve the registry on each build.
//!
//! # The `Compiler` trait
//!
//! Firmware, the Linux application, the OP-TEE TA, and the HMI frontend are built by
//! very different toolchains, but callers drive them identically: each implementor owns
//! a `Configuration` carrying all its parameters, a typed `Error`, and an `Output`
//! (usually the path to the built artifact). See [`Compiler`].
//!
//! # Plan, then run
//!
//! [`pipeline::plan`] resolves every stage's inputs — paths, env vars, artifact
//! locations — **without running any build**, threading the data flow between stages.
//! [`pipeline::run`] plans, then executes.
//!
//! That split is what makes the pipeline debuggable: a wrong sysroot or a missing
//! firmware ELF surfaces as a planning failure naming the stage, rather than as a cargo
//! error several minutes into a cross-compile.
//!
//! # Compile-time configuration
//!
//! Much of what generated code needs is passed as `CONSORTIUM_*` environment variables
//! read with `option_env!`/`env!`, not as generated source. The carriers in
//! `compile::env` describe what gets injected: [`SharedMemConfig`], [`DbgConfig`],
//! [`HmiConfig`].
//!
//! Numeric values are emitted as **decimal strings**, so a firmware-side consumer that
//! needs const parsing can use `const-str` without depending on the std-oriented schema
//! crates. Note that there is deliberately no `CONSORTIUM_IPC_BASE`: controller firmware
//! gets its region base baked into generated init code, and the Linux application reaches
//! the same memory through its UIO device, so neither side needs the physical address
//! from the environment.
//!
//! # Firmware post-processing
//!
//! Linux remoteproc expects a coprocessor's vector table in a section named
//! `.isr_vectors`, but `cortex-m-rt` emits `.vector_table`. The pipeline renames it, and
//! keeps the debug/original artifacts alongside the deployable ELF.
//!
//! # Staging
//!
//! The assembled tree can contain `app/`, `firmware/`, `ta/`, `www/`, `dts/linux/`,
//! `kernel/`, `boot/`, and `systemd/`, plus a generated BitBake recipe. CLI flags can
//! stage that tree into a recipe's `files/` directory and invoke or dry-run BitBake —
//! the dry-run path exists for the split-machine flow, where `dist/` is produced on one
//! host and the Yocto build runs on another.
//!
//! Keep the planning, compilation, staging, and external BitBake hand-off boundaries
//! distinct when extending this crate; and prefer extending `consortium-cfg` lowering
//! first, then the pipeline.

mod compile;
pub mod pipeline;
pub mod progress;

// Trait
pub use compile::Compiler;
pub use progress::{BuildReporter, BuildStep};

// Deployment artifact rendering
pub use compile::deploy::systemd::{
    SystemdRestartPolicy, SystemdUnitConfig, render_unit as render_systemd_unit,
    write_unit as write_systemd_unit,
};

// End-to-end pipeline
pub use pipeline::{
    CargoOptions, PipelineConfig, PipelineError, PipelinePlan, run as run_pipeline,
};

// Pre-build dependency download
pub use pipeline::fetch::{FetchConfig, FetchPlan, run as run_fetch};

// Compile-time environment-injection carriers
pub use compile::env::{CortexMcuCore, DbgConfig, HmiConfig, SharedMemConfig};

// Backward-compatible builders (String-error API)
pub use compile::application::ApplicationBuilder;
pub use compile::firmware::FirmwareBuilder;
pub use compile::optee::{OpteeBuilder, OpteeEnvironment};

// Typed compiler API
pub use compile::application::{
    ApplicationCompileError, ApplicationCompiler, ApplicationConfiguration,
};
pub use compile::firmware::{FirmwareCompileError, FirmwareCompiler, FirmwareConfiguration};
pub use compile::optee::{OPTEE_TARGET, OpteeCompileError, OpteeCompiler, OpteeConfiguration};
pub use compile::web::{HmiBuildCommand, HmiCompileError, HmiCompiler, HmiConfiguration};
