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

//! Compiler for the HMI (web) frontend.
//!
//! Unlike the firmware/application/OP-TEE compilers, this one does not invoke
//! `cargo`: it runs the frontend project's own build command (`vp build`,
//! `npm run build`, or an explicit override) in the project directory and then
//! verifies the dist output landed. The resolved dist directory feeds the
//! Linux application's [`HmiConfig`](super::env::HmiConfig) so the deployed app
//! knows where to serve assets from.

use std::path::{Path, PathBuf};

use consortium_cfg_common::config::HmiEngine;
use thiserror::Error;
use tokio::process::Command;

use super::Compiler;
use crate::compile::utils::run_command;
use crate::progress::BuildReporter;

/// How the HMI frontend is built.
///
/// Resolved by [`resolve_hmi_build_command`](super::utils::resolve_hmi_build_command)
/// from the `CONSORTIUM_HMI_BUILD_CMD` override, else an available `vp` or `npm`
/// on PATH.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HmiBuildCommand {
    /// Vite+ unified toolchain: `vp build`.
    VpBuild,
    /// Node package manager: `npm run build`.
    NpmRunBuild,
    /// A known tool resolved to an absolute path before any later PATH changes.
    Resolved {
        /// Executable path.
        program: String,
        /// Arguments passed to the executable.
        args: Vec<String>,
    },
    /// An explicit command from `CONSORTIUM_HMI_BUILD_CMD`, split on whitespace;
    /// the first element is the program, the rest its arguments.
    Custom(Vec<String>),
}

impl HmiBuildCommand {
    /// The program and arguments this command runs.
    fn program_args(&self) -> (&str, Vec<&str>) {
        match self {
            Self::VpBuild => ("vp", vec!["build"]),
            Self::NpmRunBuild => ("npm", vec!["run", "build"]),
            Self::Resolved { program, args } => {
                (program.as_str(), args.iter().map(String::as_str).collect())
            }
            Self::Custom(parts) => {
                let program = parts.first().map(String::as_str).unwrap_or_default();
                let args = parts.iter().skip(1).map(String::as_str).collect();
                (program, args)
            }
        }
    }
}

/// Error type for HMI frontend compilation.
#[derive(Error, Debug)]
pub enum HmiCompileError {
    /// The frontend build command could not be resolved.
    #[error("Failed to resolve HMI build command: {0}")]
    CommandResolveError(String),

    /// The frontend project directory does not exist.
    #[error("HMI project directory not found: {0:?}")]
    ProjectDirNotFound(PathBuf),

    /// The frontend build command failed to launch or exited non-zero.
    #[error("HMI build failed: {0}")]
    BuildFailed(String),

    /// The build succeeded but produced no dist directory.
    #[error("HMI dist directory not found after build: {0:?}")]
    DistNotFound(PathBuf),

    /// The dist directory exists but is empty (no assets were produced).
    #[error("HMI dist directory is empty after build: {0:?}")]
    DistEmpty(PathBuf),

    /// A PocketJS build produced no `<app>.js`/`<app>.pak` pair.
    #[error("PocketJS artifact not found after build: {0:?}")]
    PocketArtifactNotFound(PathBuf),

    /// A PocketJS build was planned without an `[hmi].app` entry name.
    #[error("PocketJS build requires `[hmi].app` (the entry name, e.g. \"hero-main\")")]
    PocketAppMissing,

    /// A filesystem error occurred while verifying the dist output.
    #[error("{0}")]
    UtilityError(String),
}

/// Build parameters for the HMI frontend.
#[derive(Clone)]
pub struct HmiConfiguration {
    /// Rendering engine, selecting how the output is verified.
    pub engine: HmiEngine,
    /// PocketJS entry name (`[hmi].app`); `None` for other engines.
    pub app: Option<String>,
    /// Frontend project root (the manifest's `[hmi].source`, resolved absolute).
    pub project_dir: PathBuf,
    /// Dist output directory the build produces (the manifest's `[hmi].build`,
    /// resolved absolute).
    pub dist_dir: PathBuf,
    /// The command that builds the frontend.
    pub command: HmiBuildCommand,
}

/// Compiler for the HMI frontend: runs the frontend build then verifies dist.
pub struct HmiCompiler;

impl HmiCompiler {
    pub async fn build_with_reporter(
        config: HmiConfiguration,
        reporter: Option<&dyn BuildReporter>,
    ) -> Result<PathBuf, HmiCompileError> {
        let (program, args) = config.command.program_args();
        if program.is_empty() {
            return Err(HmiCompileError::CommandResolveError(
                "HMI build command is empty".to_string(),
            ));
        }

        match tokio::fs::metadata(&config.project_dir).await {
            Ok(meta) if meta.is_dir() => {}
            Ok(_) | Err(_) => {
                return Err(HmiCompileError::ProjectDirNotFound(config.project_dir));
            }
        }

        let mut cmd = Command::new(program);
        cmd.args(&args).current_dir(&config.project_dir);

        let status = run_command(&mut cmd, reporter).await.map_err(|e| {
            HmiCompileError::BuildFailed(format!("failed to launch `{program}`: {e}"))
        })?;

        if !status.success() {
            return Err(HmiCompileError::BuildFailed(format!(
                "`{program}` exited with status: {status}"
            )));
        }

        match config.engine {
            HmiEngine::Cog => verify_dist(&config.dist_dir).await?,
            HmiEngine::Pocket => {
                verify_pocket_artifacts(&config.dist_dir, config.app.as_deref()).await?
            }
        }
        Ok(config.dist_dir)
    }
}

impl Compiler for HmiCompiler {
    type Configuration = HmiConfiguration;
    type Error = HmiCompileError;
    type Output = PathBuf;

    async fn build(config: Self::Configuration) -> Result<Self::Output, Self::Error> {
        Self::build_with_reporter(config, None).await
    }

    async fn output_path(config: &Self::Configuration) -> Result<PathBuf, Self::Error> {
        Ok(config.dist_dir.clone())
    }
}

/// Verifies the dist directory exists and holds at least one entry.
///
/// The frontend build is opaque (no single well-known artifact like an ELF), so
/// the strongest cheap check is that the declared dist directory materialized and
/// is non-empty.
async fn verify_dist(dist: &PathBuf) -> Result<(), HmiCompileError> {
    match tokio::fs::metadata(dist).await {
        Ok(meta) if meta.is_dir() => {}
        Ok(_) => return Err(HmiCompileError::DistNotFound(dist.clone())),
        Err(_) => return Err(HmiCompileError::DistNotFound(dist.clone())),
    }

    let mut entries = tokio::fs::read_dir(dist)
        .await
        .map_err(|e| HmiCompileError::UtilityError(format!("reading dist {dist:?}: {e}")))?;
    match entries.next_entry().await {
        Ok(Some(_)) => Ok(()),
        Ok(None) => Err(HmiCompileError::DistEmpty(dist.clone())),
        Err(e) => Err(HmiCompileError::UtilityError(format!(
            "reading dist entries {dist:?}: {e}"
        ))),
    }
}

/// Verifies a PocketJS build produced both `<app>.js` and `<app>.pak`.
///
/// PocketJS emits exactly these two artifacts (bundle + packed assets), so
/// unlike the opaque web dist their names are known and can be checked directly.
async fn verify_pocket_artifacts(dist: &Path, app: Option<&str>) -> Result<(), HmiCompileError> {
    let app = app
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .ok_or(HmiCompileError::PocketAppMissing)?;
    for ext in ["js", "pak"] {
        let artifact = dist.join(format!("{app}.{ext}"));
        match tokio::fs::metadata(&artifact).await {
            Ok(meta) if meta.is_file() => {}
            _ => return Err(HmiCompileError::PocketArtifactNotFound(artifact)),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_dir(tag: &str) -> PathBuf {
        let base = std::env::var("SCRATCH_DIR")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let dir = base.join(format!("consortium-hmi-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn program_args_map_each_command() {
        assert_eq!(
            HmiBuildCommand::VpBuild.program_args(),
            ("vp", vec!["build"])
        );
        assert_eq!(
            HmiBuildCommand::NpmRunBuild.program_args(),
            ("npm", vec!["run", "build"])
        );
        assert_eq!(
            HmiBuildCommand::Resolved {
                program: "/tools/vp".into(),
                args: vec!["build".into()],
            }
            .program_args(),
            ("/tools/vp", vec!["build"])
        );
        assert_eq!(
            HmiBuildCommand::Custom(vec!["pnpm".into(), "build".into()]).program_args(),
            ("pnpm", vec!["build"])
        );
    }

    #[tokio::test]
    async fn build_runs_command_and_verifies_dist() {
        let project = tmp_dir("ok");
        let dist = project.join("dist");

        // A tiny "build command" that materializes the dist tree, standing in for
        // the real `vp build`/`npm run build`.
        let cfg = HmiConfiguration {
            engine: HmiEngine::Cog,
            app: None,
            project_dir: project.clone(),
            dist_dir: dist.clone(),
            command: HmiBuildCommand::Custom(vec![
                "sh".into(),
                "-c".into(),
                "mkdir -p dist && printf '<html></html>' > dist/index.html".into(),
            ]),
        };

        let out = HmiCompiler::build(cfg).await.expect("hmi build succeeds");
        assert_eq!(out, dist);
        assert!(dist.join("index.html").exists());

        let _ = std::fs::remove_dir_all(&project);
    }

    #[tokio::test]
    async fn build_errors_when_project_dir_missing() {
        let root = tmp_dir("missing-project");
        let project = root.join("website");

        let cfg = HmiConfiguration {
            engine: HmiEngine::Cog,
            app: None,
            project_dir: project.clone(),
            dist_dir: project.join("dist"),
            command: HmiBuildCommand::Custom(vec!["true".into()]),
        };

        let err = HmiCompiler::build(cfg)
            .await
            .expect_err("missing project dir is an error");
        assert!(matches!(
            err,
            HmiCompileError::ProjectDirNotFound(path) if path == project
        ));

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn build_errors_when_dist_missing() {
        let project = tmp_dir("nodist");
        let dist = project.join("dist");

        // The command succeeds but never creates the dist directory.
        let cfg = HmiConfiguration {
            engine: HmiEngine::Cog,
            app: None,
            project_dir: project.clone(),
            dist_dir: dist.clone(),
            command: HmiBuildCommand::Custom(vec!["true".into()]),
        };

        let err = HmiCompiler::build(cfg)
            .await
            .expect_err("missing dist is an error");
        assert!(matches!(err, HmiCompileError::DistNotFound(_)));

        let _ = std::fs::remove_dir_all(&project);
    }

    #[tokio::test]
    async fn build_errors_when_command_fails() {
        let project = tmp_dir("fail");
        let cfg = HmiConfiguration {
            engine: HmiEngine::Cog,
            app: None,
            project_dir: project.clone(),
            dist_dir: project.join("dist"),
            command: HmiBuildCommand::Custom(vec!["false".into()]),
        };

        let err = HmiCompiler::build(cfg)
            .await
            .expect_err("failing command is an error");
        assert!(matches!(err, HmiCompileError::BuildFailed(_)));

        let _ = std::fs::remove_dir_all(&project);
    }

    #[tokio::test]
    async fn pocket_build_verifies_bundle_and_pak() {
        let project = tmp_dir("pocket-ok");
        let dist = project.join("dist");

        // Stand in for `bun scripts/build.ts hero-main --outdir=dist`: emit the
        // two artifacts PocketJS produces.
        let cfg = HmiConfiguration {
            engine: HmiEngine::Pocket,
            app: Some("hero-main".into()),
            project_dir: project.clone(),
            dist_dir: dist.clone(),
            command: HmiBuildCommand::Custom(vec![
                "sh".into(),
                "-c".into(),
                "mkdir -p dist && : > dist/hero-main.js && : > dist/hero-main.pak".into(),
            ]),
        };

        let out = HmiCompiler::build(cfg)
            .await
            .expect("pocket build succeeds");
        assert_eq!(out, dist);

        let _ = std::fs::remove_dir_all(&project);
    }

    #[tokio::test]
    async fn pocket_build_errors_when_pak_missing() {
        let project = tmp_dir("pocket-nopak");
        let dist = project.join("dist");

        // Only the JS bundle lands; the pak is missing.
        let cfg = HmiConfiguration {
            engine: HmiEngine::Pocket,
            app: Some("hero-main".into()),
            project_dir: project.clone(),
            dist_dir: dist.clone(),
            command: HmiBuildCommand::Custom(vec![
                "sh".into(),
                "-c".into(),
                "mkdir -p dist && : > dist/hero-main.js".into(),
            ]),
        };

        let err = HmiCompiler::build(cfg)
            .await
            .expect_err("missing pak is an error");
        assert!(matches!(
            err,
            HmiCompileError::PocketArtifactNotFound(path) if path.ends_with("hero-main.pak")
        ));

        let _ = std::fs::remove_dir_all(&project);
    }
}
