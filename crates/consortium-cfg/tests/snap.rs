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

//! Snapshot coverage for the compile-time environment a `Consortium.toml`
//! produces.
//!
//! The builder injects each manifest's `[ipc]`, `[dbg]`, `[hmi]`, and `[tee]`
//! sections as `CONSORTIUM_*` environment variables before invoking
//! `cargo build`; the consuming crates read them with `option_env!`.
//!
//! Each case is a self-contained fixture directory under `tests/snapshots/`:
//!
//! ```text
//! tests/snapshots/<case>/Consortium.toml   # the input manifest
//! tests/snapshots/<case>/artifacts.snap    # the snapshotted env export
//! ```
//!
//! The test below lists those directories, parses each `Consortium.toml`,
//! exports the `KEY=VALUE` pairs it would inject, and snapshots them next to the
//! manifest as `artifacts.snap`. Add a case by dropping in a new directory with
//! a `Consortium.toml`; the snapshot is generated on the next
//! `INSTA_UPDATE=always` run.

use std::fs;
use std::path::{Path, PathBuf};

use consortium_common::config::{HmiConfig, SharedMemConfig, TeeConfig};

/// The env-contributing sections of a `Consortium.toml`.
///
/// Deliberately permissive: there is no `deny_unknown_fields`, so unrelated
/// top-level sections (`[platform]`, `[project]`, `[example]`, ...) are ignored
/// and every manifest shape parses.
#[derive(serde::Deserialize, Default)]
struct Manifest {
    ipc: Option<SharedMemConfig>,
    dbg: Option<SharedMemConfig>,
    hmi: Option<HmiConfig>,
    tee: Option<TeeConfig>,
}

/// Directory holding the fixture cases (`tests/snapshots/<case>/`).
fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots")
}

/// Immediate subdirectories of the fixtures root that contain a
/// `Consortium.toml`, sorted for a deterministic order.
fn fixture_cases(root: &Path) -> Vec<PathBuf> {
    let mut cases: Vec<PathBuf> = fs::read_dir(root)
        .expect("fixtures root should exist")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.join("Consortium.toml").is_file())
        .collect();
    cases.sort();
    cases
}

/// Every `KEY=VALUE` the builder would inject from a manifest, sorted for a
/// stable snapshot.
///
/// TEE UUIDs are validated here: a valid one contributes `CONSORTIUM_TEE_UUID`,
/// while an invalid one is reported as a `# [tee] uuid rejected: ...`
/// diagnostic instead of panicking, so the rejection path is itself snapshotted.
fn env_export(content: &str) -> String {
    let manifest: Manifest = match toml::from_str(content) {
        Ok(manifest) => manifest,
        Err(err) => return format!("error: manifest failed to parse: {err}"),
    };

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut diagnostics: Vec<String> = Vec::new();

    if let Some(ipc) = &manifest.ipc {
        pairs.extend(ipc.env_pairs("IPC"));
    }
    if let Some(dbg) = &manifest.dbg {
        pairs.extend(dbg.env_pairs("DBG"));
    }
    if let Some(hmi) = &manifest.hmi {
        pairs.extend(hmi.env_pairs());
    }
    if let Some(tee) = &manifest.tee {
        match tee.validate() {
            Ok(()) => {
                let (key, value) = tee.env_pair();
                pairs.push((key.to_string(), value.to_string()));
            }
            Err(err) => diagnostics.push(format!("# [tee] uuid rejected: {err}")),
        }
    }

    pairs.sort();
    let mut lines: Vec<String> = pairs
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();
    lines.extend(diagnostics);

    if lines.is_empty() {
        "(no environment variables)".to_string()
    } else {
        lines.join("\n")
    }
}

/// Walk the fixture directories and snapshot each manifest's injected env into
/// `<case>/artifacts.snap`.
#[test]
fn fixture_manifests_export_expected_env() {
    let root = fixtures_root();
    let cases = fixture_cases(&root);
    assert!(
        !cases.is_empty(),
        "expected fixture cases under {}",
        root.display()
    );

    for case in cases {
        let content = fs::read_to_string(case.join("Consortium.toml"))
            .expect("fixture manifest should be readable");

        // Pin the snapshot next to its manifest as `artifacts.snap` rather than
        // the default flat `snap__*.snap`, keeping each case self-contained.
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(&case);
        settings.set_prepend_module_to_snapshot(false);
        settings.set_input_file(case.join("Consortium.toml"));
        settings.bind(|| {
            insta::assert_snapshot!("artifacts", env_export(&content));
        });
    }
}
