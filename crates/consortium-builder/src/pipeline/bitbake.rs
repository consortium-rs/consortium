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
}

/// Executes `action` against the assembled `dist_root`.
pub async fn run(action: &BitbakeAction, dist_root: &Path) -> Result<(), BitbakeError> {
    run_with_reporter(action, dist_root, None).await
}

/// Executes an action against the assembled dist root, streaming command output.
pub async fn run_with_reporter(
    action: &BitbakeAction,
    dist_root: &Path,
    reporter: Option<&dyn BuildReporter>,
) -> Result<(), BitbakeError> {
    match action {
        BitbakeAction::CopyIntoRecipe { recipe_files_dir } => {
            copy_dir_recursive(dist_root, recipe_files_dir).await?;
            Ok(())
        }
        BitbakeAction::InvokeBitbake { build_dir, recipe } => {
            invoke_bitbake_with("bitbake", build_dir, recipe, reporter).await
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
}
