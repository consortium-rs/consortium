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

//! Stage 7: assemble the local `dist/` staging tree.
//!
//! `dist/` is a local staging directory a downstream bitbake recipe consumes; the
//! pipeline never writes to a live target filesystem. Firmware ELFs land under
//! `dist/firmware/<core>.elf` purely so a firmware-packaging recipe can install
//! them to `/lib/firmware/<core>` for remoteproc's kernel-side loader — this is
//! unrelated to the defmt embedding, which happens in the application binary.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use consortium_cfg_common::config::VisibleCore;
use thiserror::Error;

/// The `dist/` staging tree layout.
pub struct DistLayout {
    /// The staging root; children are `firmware/`, `app/`, `ta/`, `www/`,
    /// `systemd/`.
    pub root: PathBuf,
}

impl DistLayout {
    /// A layout rooted at `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// The firmware subdirectory (`<root>/firmware`).
    pub fn firmware_dir(&self) -> PathBuf {
        self.root.join("firmware")
    }

    /// The application subdirectory (`<root>/app`).
    pub fn app_dir(&self) -> PathBuf {
        self.root.join("app")
    }

    /// The TA subdirectory (`<root>/ta`).
    pub fn ta_dir(&self) -> PathBuf {
        self.root.join("ta")
    }

    /// The HMI assets subdirectory (`<root>/www`).
    pub fn www_dir(&self) -> PathBuf {
        self.root.join("www")
    }

    /// The systemd unit subdirectory (`<root>/systemd`).
    pub fn systemd_dir(&self) -> PathBuf {
        self.root.join("systemd")
    }
}

/// Error type for assembling the `dist/` tree.
#[derive(Error, Debug)]
pub enum DistAssemblyError {
    /// A filesystem operation failed.
    #[error("dist assembly I/O error at {path}: {source}")]
    Io {
        /// The path being operated on.
        path: String,
        /// The underlying error.
        source: std::io::Error,
    },
}

fn io(path: impl AsRef<Path>, source: std::io::Error) -> DistAssemblyError {
    DistAssemblyError::Io {
        path: path.as_ref().display().to_string(),
        source,
    }
}

/// Assembles the `dist/` tree from the pipeline's build outputs.
///
/// - each firmware ELF → `firmware/<core>.elf`;
/// - the application binary → `app/<file_name>`;
/// - the signed TA (when present) → `ta/<file_name>`;
/// - the HMI dist tree (when present) → `www/` (copied recursively);
/// - each `(name, contents)` systemd unit → `systemd/<name>`.
pub async fn assemble(
    layout: &DistLayout,
    firmware: &HashMap<VisibleCore, PathBuf>,
    application: &Path,
    ta: Option<&Path>,
    hmi_dist: Option<&Path>,
    systemd_units: &[(String, String)],
) -> Result<(), DistAssemblyError> {
    // Firmware ELFs, named by core for the packaging recipe.
    let firmware_dir = layout.firmware_dir();
    tokio::fs::create_dir_all(&firmware_dir)
        .await
        .map_err(|e| io(&firmware_dir, e))?;
    for (core, elf) in firmware {
        let dest = firmware_dir.join(format!("{core}.elf"));
        copy_file(elf, &dest).await?;
    }

    // Application binary.
    let app_dir = layout.app_dir();
    tokio::fs::create_dir_all(&app_dir)
        .await
        .map_err(|e| io(&app_dir, e))?;
    let app_dest = app_dir.join(file_name(application)?);
    copy_file(application, &app_dest).await?;

    // Signed TA, when the manifest declared one.
    if let Some(ta) = ta {
        let ta_dir = layout.ta_dir();
        tokio::fs::create_dir_all(&ta_dir)
            .await
            .map_err(|e| io(&ta_dir, e))?;
        let ta_dest = ta_dir.join(file_name(ta)?);
        copy_file(ta, &ta_dest).await?;
    }

    // HMI assets, copied recursively.
    if let Some(hmi_dist) = hmi_dist {
        let www = layout.www_dir();
        copy_dir_recursive(hmi_dist, &www).await?;
    }

    // systemd units.
    if !systemd_units.is_empty() {
        let systemd_dir = layout.systemd_dir();
        tokio::fs::create_dir_all(&systemd_dir)
            .await
            .map_err(|e| io(&systemd_dir, e))?;
        for (name, contents) in systemd_units {
            let dest = systemd_dir.join(name);
            tokio::fs::write(&dest, contents)
                .await
                .map_err(|e| io(&dest, e))?;
        }
    }

    Ok(())
}

/// The final path component of `path` as a string.
fn file_name(path: &Path) -> Result<String, DistAssemblyError> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.to_string())
        .ok_or_else(|| {
            io(
                path,
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "no file name"),
            )
        })
}

/// Copies a single file, creating the parent directory if needed.
async fn copy_file(src: &Path, dest: &Path) -> Result<(), DistAssemblyError> {
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| io(parent, e))?;
    }
    tokio::fs::copy(src, dest).await.map_err(|e| io(src, e))?;
    Ok(())
}

/// Recursively copies the directory tree at `src` into `dest`.
///
/// Boxed because async fns cannot recurse directly. No extra dependency: the walk
/// is a hand-rolled [`tokio::fs::read_dir`] traversal.
pub(crate) fn copy_dir_recursive<'a>(
    src: &'a Path,
    dest: &'a Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), DistAssemblyError>> + Send + 'a>>
{
    Box::pin(async move {
        tokio::fs::create_dir_all(dest)
            .await
            .map_err(|e| io(dest, e))?;
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| io(src, e))?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| io(src, e))? {
            let file_type = entry.file_type().await.map_err(|e| io(entry.path(), e))?;
            let child_dest = dest.join(entry.file_name());
            if file_type.is_dir() {
                copy_dir_recursive(&entry.path(), &child_dest).await?;
            } else {
                copy_file(&entry.path(), &child_dest).await?;
            }
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("consortium-dist-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create scratch");
        dir
    }

    #[tokio::test]
    async fn assembles_full_tree() {
        let base = scratch("assemble");
        let sources = base.join("src");
        std::fs::create_dir_all(&sources).unwrap();

        // Fake build outputs: ELF-magic stand-ins, an app binary, a .ta, a www tree.
        let cm33_elf = sources.join("cm33.elf");
        std::fs::write(&cm33_elf, [0x7f, b'E', b'L', b'F']).unwrap();
        let app = sources.join("melt-pot-app");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();
        let ta = sources.join("uuid.ta");
        std::fs::write(&ta, b"signed-ta").unwrap();
        let www = sources.join("dist");
        std::fs::create_dir_all(www.join("assets")).unwrap();
        std::fs::write(www.join("index.html"), b"<html>").unwrap();
        std::fs::write(www.join("assets").join("app.js"), b"//js").unwrap();

        let layout = DistLayout::new(base.join("dist"));
        let firmware =
            HashMap::from([(VisibleCore::Mcore("cm33".parse().unwrap()), cm33_elf.clone())]);
        let units = vec![(
            "consortium-app.service".to_string(),
            "[Unit]\n".to_string(),
        )];

        assemble(&layout, &firmware, &app, Some(&ta), Some(&www), &units)
            .await
            .expect("assemble succeeds");

        assert!(layout.firmware_dir().join("cm33.elf").exists());
        assert!(layout.app_dir().join("melt-pot-app").exists());
        assert!(layout.ta_dir().join("uuid.ta").exists());
        assert!(layout.www_dir().join("index.html").exists());
        assert!(layout.www_dir().join("assets").join("app.js").exists());
        assert_eq!(
            std::fs::read_to_string(layout.systemd_dir().join("consortium-app.service")).unwrap(),
            "[Unit]\n"
        );

        let _ = std::fs::remove_dir_all(&base);
    }

    #[tokio::test]
    async fn skips_optional_ta_and_hmi() {
        let base = scratch("minimal");
        let app = base.join("app-bin");
        std::fs::write(&app, [0x7f, b'E', b'L', b'F']).unwrap();

        let layout = DistLayout::new(base.join("dist"));
        assemble(&layout, &HashMap::new(), &app, None, None, &[])
            .await
            .expect("assemble succeeds");

        assert!(layout.app_dir().join("app-bin").exists());
        assert!(!layout.ta_dir().exists());
        assert!(!layout.www_dir().exists());

        let _ = std::fs::remove_dir_all(&base);
    }
}
