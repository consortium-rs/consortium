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

//! The `fetch` subcommand: download every dependency a build needs, up front.
//!
//! Pairs with `csti build --offline`. A Consortium build spans several
//! independent Cargo projects plus a frontend package manager, so there is no one
//! `cargo fetch` that covers the system; this resolves the same project set the
//! build plan does and fetches each one.

use std::path::PathBuf;

use clap::Args;
use consortium_builder::pipeline::{
    self,
    fetch::{self, FetchConfig},
};

use crate::reporter::StickyReporter;

#[derive(Args)]
pub struct FetchArgs {
    /// Path to the `Consortium.toml` manifest.
    #[arg(long, default_value = "Consortium.toml")]
    manifest: PathBuf,

    /// Skip the HMI frontend's package install, fetching only Rust dependencies.
    #[arg(long)]
    skip_hmi: bool,

    /// Skip `rustup target add` for the targets the build cross-compiles for.
    /// Use when the host manages its toolchain some other way (an SDK, a distro
    /// rustc); a host with no `rustup` at all skips this step on its own.
    #[arg(long)]
    skip_rust_targets: bool,
}

/// Downloads every dependency a build of the given manifest would need.
pub async fn run(args: FetchArgs) -> Result<(), String> {
    let session = pipeline::load_session(&args.manifest)
        .await
        .map_err(|e| e.to_string())?;
    let manifest_dir = args
        .manifest
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let hmi = if args.skip_hmi {
        None
    } else {
        fetch::derive_hmi_fetch(&session, &manifest_dir).map_err(|e| e.to_string())?
    };

    let config = FetchConfig {
        manifest_path: args.manifest.clone(),
        hmi,
        rust_targets: !args.skip_rust_targets,
    };

    // Show what will be downloaded before the (long) network work begins, the
    // same way `build` prints its plan.
    let plan = fetch::plan_fetch(&session, &manifest_dir, config.hmi.clone())
        .map_err(|e| e.to_string())?;
    println!(
        "Fetching dependencies for {} (chip {})",
        args.manifest.display(),
        session.database.name
    );
    for entry in &plan.cargo {
        println!("  cargo:    {} ({})", entry.label, entry.target);
    }
    if let Some(hmi) = &plan.hmi {
        println!("  hmi:      {}", hmi.project_dir.display());
    }
    if !args.skip_rust_targets {
        println!("  targets:  {}", plan.rust_targets().join(" "));
    }

    let reporter = StickyReporter::new();
    match fetch::run_with_reporter(config, &reporter).await {
        Ok(_) => reporter.finish_success("Fetch complete"),
        Err(error) => {
            reporter.finish_failure("Fetch failed");
            return Err(error.to_string());
        }
    }
    println!("Dependencies cached; `csti build --offline` can now run without network access.");
    Ok(())
}
