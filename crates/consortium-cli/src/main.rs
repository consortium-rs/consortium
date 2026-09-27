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

//! `csti` — the Consortium command-line interface.
//!
//! A thin `clap` front end over [`consortium_builder`]. `csti build` drives the whole
//! `Consortium.toml` pipeline: compile the Linux application and controller firmware
//! (plus the OP-TEE TA and HMI frontend when the manifest declares them),
//! post-process the firmware for Linux remoteproc, generate the endpoint modules,
//! device tree, Kconfig fragment, boot integration, and systemd unit, then assemble a
//! local `dist/` staging tree with a BitBake recipe.
//!
//! It does **not** deploy: nothing is written to a live target filesystem.
//!
//! `csti check` is its fast loop, the `cargo check` of a whole system: it validates
//! the manifest, writes the generated endpoint modules, and type-checks the
//! firmware, TA, and application crates without linking or staging anything.
//!
//! `csti fetch` is its network half: it resolves the same Cargo projects the
//! build plan does and downloads their dependencies (plus each cross target's
//! `std` and the HMI frontend's packages) ahead of time, so `csti build
//! --linux <DIR> --offline` can then run from local caches alone.
//!
//! What the CLI adds beyond argument parsing is progress reporting — it passes a
//! [`BuildReporter`](consortium_builder::BuildReporter) that keeps a persistent step
//! prompt visible underneath cargo, `vp`, and BitBake output, so a long cross-compile
//! still says which stage it is in.
//!
//! This crate publishes as `csti`, matching the binary: `consortium-cli` and the bare
//! `consortium` name are both taken on crates.io by unrelated projects. The source
//! directory keeps its descriptive name, so cargo invocations use `-p csti`.

mod build;
mod check;
mod fetch;
mod reporter;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::build::BuildArgs;
use crate::check::CheckArgs;
use crate::fetch::FetchArgs;

/// Build and package Consortium applications and runtimes.
#[derive(Parser)]
#[command(name = "csti", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the end-to-end build pipeline for a `Consortium.toml` manifest.
    Build(BuildArgs),

    /// Validate the manifest and type-check every Rust endpoint with
    /// `cargo check`, without building or packaging anything.
    Check(CheckArgs),

    /// Download every dependency that build would need, so it can then run
    /// with `--offline`.
    Fetch(FetchArgs),
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Build(args) => build::run(args).await,
        Command::Check(args) => check::run(args).await,
        Command::Fetch(args) => fetch::run(args).await,
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_requires_linux_checkout() {
        assert!(Cli::try_parse_from(["csti", "build"]).is_err());
        assert!(Cli::try_parse_from(["csti", "build", "--linux", "/src/linux"]).is_ok());
    }

    #[test]
    fn check_needs_no_linux_checkout() {
        assert!(Cli::try_parse_from(["csti", "check"]).is_ok());
        assert!(Cli::try_parse_from(["csti", "check", "--linux", "/src/linux"]).is_ok());
        // The remoteproc downgrade only means something when a DTS is rendered.
        assert!(Cli::try_parse_from(["csti", "check", "--skip-remoteproc-check"]).is_err());
    }
}
