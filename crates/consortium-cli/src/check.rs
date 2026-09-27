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

//! The `check` subcommand: `cargo check` across every endpoint of a manifest.
//!
//! Validates `Consortium.toml`, writes each endpoint's generated module, and
//! type-checks the firmware, TA, and application crates with the same targets and
//! injected environment `build` uses — without linking, post-processing, signing,
//! building the HMI frontend, or assembling `dist/`.

use std::path::PathBuf;

use clap::Args;
use consortium_builder::pipeline::{
    self,
    check::{self, CheckConfig, TaSkip},
    tee::TeeToolchain,
};

use crate::reporter::StickyReporter;

#[derive(Args)]
pub struct CheckArgs {
    /// Path to the `Consortium.toml` manifest.
    #[arg(long, default_value = "Consortium.toml")]
    manifest: PathBuf,

    /// Linux kernel source checkout to render the device tree against. Without
    /// it the device tree is not validated.
    #[arg(long, value_name = "DIR")]
    linux: Option<PathBuf>,

    /// Check the release profile.
    #[arg(long)]
    release: bool,

    /// Check without network access: every cargo invocation runs `--offline`.
    /// Populate the local registry cache first with `csti fetch`.
    #[arg(long)]
    offline: bool,

    /// Downgrade a missing remoteproc DTS node to a warning. Only meaningful
    /// with `--linux`; see `csti build --help`.
    #[arg(long, requires = "linux")]
    skip_remoteproc_check: bool,
}

/// Type-checks every Rust endpoint of the given `Consortium.toml` manifest.
pub async fn run(args: CheckArgs) -> Result<(), String> {
    let session = pipeline::load_session(&args.manifest)
        .await
        .map_err(|e| e.to_string())?;

    let config = CheckConfig {
        manifest_path: args.manifest.clone(),
        linux: args.linux.clone(),
        release: args.release,
        offline: args.offline,
        tee_toolchain: TeeToolchain::from_env(),
        strict_remoteproc: !args.skip_remoteproc_check,
    };

    let plan = check::plan_check(&session, &config)
        .await
        .map_err(|e| e.to_string())?;
    println!(
        "Checking {} for chip {}",
        args.manifest.display(),
        session.database.name
    );
    for fw in &plan.firmware {
        println!("  firmware: {} ({})", fw.core, fw.config.target);
    }
    match plan.ta_skipped {
        Some(TaSkip::NoToolchain) => {
            println!("  tee:      skipped (set TA_DEV_KIT_DIR to check the trusted application)");
        }
        None if plan.ta.is_some() => println!("  tee:      trusted application"),
        None => {}
    }
    println!("  app:      {}", plan.application.config.target);
    for path in &plan.placeholder_elfs {
        println!(
            "  dbg:      {} not built yet; embedding an empty stand-in",
            path.display()
        );
    }
    match &args.linux {
        Some(_) if plan.dts_files.is_empty() => println!("  dts:      nothing to render"),
        Some(_) => println!("  dts:      {} (rendered)", plan.dts_files.join(", ")),
        None => println!("  dts:      skipped (pass --linux <DIR> to validate)"),
    }
    if args.offline {
        println!("  cargo:    --offline (local registry cache only)");
    }

    let reporter = StickyReporter::new();
    match check::run_with_reporter(config, &reporter).await {
        Ok(_) => reporter.finish_success("Check complete"),
        Err(error) => {
            reporter.finish_failure("Check failed");
            return Err(error.to_string());
        }
    }
    Ok(())
}
