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

//! `consortium` command-line interface.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use consortium_builder::pipeline::{
    self, PipelineConfig, bitbake::BitbakeAction, tee::TeeToolchain,
};

/// Build and package Consortium applications and runtimes.
#[derive(Parser)]
#[command(name = "consortium", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the end-to-end build pipeline for a `Consortium.toml` manifest.
    Build(BuildArgs),
}

#[derive(Args)]
struct BuildArgs {
    /// Path to the `Consortium.toml` manifest.
    #[arg(long, default_value = "Consortium.toml")]
    manifest: PathBuf,

    /// Directory to assemble the `dist/` staging tree into.
    #[arg(long, default_value = "dist")]
    dist: PathBuf,

    /// Build in release mode.
    #[arg(long)]
    release: bool,

    /// Stage the assembled `dist/` tree under a bitbake recipe's `files/` dir.
    #[arg(long, value_name = "DIR")]
    bitbake_files_dir: Option<PathBuf>,

    /// Invoke bitbake on this recipe (requires `--bitbake-build-dir`).
    #[arg(long, value_name = "RECIPE")]
    bitbake_recipe: Option<String>,

    /// The Yocto build directory `bitbake` is invoked in.
    #[arg(long, value_name = "DIR")]
    bitbake_build_dir: Option<PathBuf>,
}

/// Resolves the optional bitbake hand-off from the CLI flags.
fn resolve_bitbake(args: &BuildArgs) -> Result<Option<BitbakeAction>, String> {
    match (
        &args.bitbake_files_dir,
        &args.bitbake_recipe,
        &args.bitbake_build_dir,
    ) {
        (None, None, None) => Ok(None),
        (Some(dir), None, None) => Ok(Some(BitbakeAction::CopyIntoRecipe {
            recipe_files_dir: dir.clone(),
        })),
        (None, Some(recipe), Some(build_dir)) => Ok(Some(BitbakeAction::InvokeBitbake {
            build_dir: build_dir.clone(),
            recipe: recipe.clone(),
        })),
        (None, Some(_), None) => Err("--bitbake-recipe requires --bitbake-build-dir".to_string()),
        (None, None, Some(_)) => Err("--bitbake-build-dir requires --bitbake-recipe".to_string()),
        _ => Err(
            "choose either --bitbake-files-dir or (--bitbake-recipe + --bitbake-build-dir), \
             not both"
                .to_string(),
        ),
    }
}

async fn build(args: BuildArgs) -> Result<(), String> {
    let session = pipeline::load_session(&args.manifest)
        .await
        .map_err(|e| e.to_string())?;
    let manifest_dir = args
        .manifest
        .parent()
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let hmi =
        pipeline::derive_hmi_configuration(&session, &manifest_dir).map_err(|e| e.to_string())?;
    let tee_toolchain = TeeToolchain::from_env();
    let bitbake = resolve_bitbake(&args)?;

    let config = PipelineConfig {
        manifest_path: args.manifest.clone(),
        dist_root: args.dist.clone(),
        release: args.release,
        hmi,
        tee_toolchain,
        bitbake,
    };

    // Print a plan summary so the user sees what will be built before the (long)
    // cross-compilation begins.
    let plan = pipeline::plan(&session, &config)
        .await
        .map_err(|e| e.to_string())?;
    println!(
        "Building {} for chip {}",
        args.manifest.display(),
        session.database.name
    );
    for fw in &plan.firmware {
        println!("  firmware: {} ({})", fw.core, fw.config.target);
    }
    if plan.hmi.is_some() {
        let source = session
            .config
            .hmi
            .as_ref()
            .map(|h| h.source.as_str())
            .unwrap_or("-");
        println!("  hmi:      {source}");
    }
    if plan.ta.is_some() {
        println!("  tee:      trusted application");
    }
    println!("  app:      {}", plan.application.config.target);

    let dist = pipeline::run(config).await.map_err(|e| e.to_string())?;
    println!("Assembled dist tree at {}", dist.display());
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Build(args) => build(args).await,
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
