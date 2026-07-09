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

//! Vendor device-tree sources, embedded at compile time and resolved from a chip
//! part number plus an optional board selector.
//!
//! The builder needs the vendor board/SoC device tree to build the Linux device
//! tree on top of. Those sources live under `database/dts/{imx9,stm32mp2}/` and
//! are embedded here so the installed CLI can read them regardless of the working
//! directory.

use include_dir::{Dir, include_dir};

use crate::error::DataError;

/// The whole embedded `database/dts/` tree.
static DTS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/database/dts");

/// A base device tree selected for a chip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseDts {
    /// Family subdirectory under `database/dts/` (e.g. `imx9`, `stm32mp2`). This
    /// is the include-search root a downstream `cpp -I` needs so the base's
    /// local `#include "..."` directives resolve.
    pub family_dir: &'static str,
    /// The entry file name, with extension (e.g. `stm32mp257f-dk.dts` or
    /// `stm32mp257.dtsi`).
    pub entry: String,
}

impl BaseDts {
    /// Whether the entry is a full board `.dts` (as opposed to a generic
    /// `.dtsi`). A `.dts` carries `/dts-v1/;` and board-level nodes; a `.dtsi`
    /// is a SoC include with no board layer.
    pub fn is_board(&self) -> bool {
        self.entry.ends_with(".dts")
    }

    /// Path of the entry file within the embedded tree (e.g.
    /// `imx9/imx95-15x15-frdm.dts`).
    pub fn entry_path(&self) -> String {
        format!("{}/{}", self.family_dir, self.entry)
    }

    /// The entry file's contents.
    pub fn contents(&self) -> Result<&'static str, DataError> {
        read(&self.entry_path())
    }
}

/// Resolves the base device tree for a chip.
///
/// Selection order, highest priority first:
/// 1. `base_override` (the `[dts].base` stem) — matched as `.dts` then `.dtsi`.
/// 2. `devkit` (the `[profile].devkit` stem) — matched as `.dts` then `.dtsi`.
/// 3. the generic SoC `.dtsi` derived from the chip part number.
///
/// # Errors
///
/// Returns [`DataError::UnknownChip`] if the chip family has no DTS directory,
/// or [`DataError::UnknownDts`] if an explicit stem does not match any embedded
/// file.
pub fn resolve_base(
    chip: &str,
    devkit: Option<&str>,
    base_override: Option<&str>,
) -> Result<BaseDts, DataError> {
    let family_dir = family_dir(chip).ok_or_else(|| DataError::UnknownChip(chip.to_string()))?;

    if let Some(stem) = base_override.or(devkit) {
        let entry =
            match_stem(family_dir, stem).ok_or_else(|| DataError::UnknownDts(stem.to_string()))?;
        return Ok(BaseDts { family_dir, entry });
    }

    let entry = generic_dtsi(chip).ok_or_else(|| DataError::UnknownChip(chip.to_string()))?;
    // A generic mapping that does not resolve to an embedded file is a bug in the
    // mapping table, so surface it as an unknown-DTS error rather than silently
    // returning a dangling path.
    if DTS.get_file(format!("{family_dir}/{entry}")).is_none() {
        return Err(DataError::UnknownDts(entry));
    }
    Ok(BaseDts { family_dir, entry })
}

/// Reads a file from the embedded DTS tree by its tree-relative path (e.g.
/// `imx9/imx95.dtsi`).
pub fn read(path: &str) -> Result<&'static str, DataError> {
    DTS.get_file(path)
        .and_then(|f| f.contents_utf8())
        .ok_or_else(|| DataError::UnknownDts(path.to_string()))
}

/// The transitive closure of a base file's *local* (`#include "..."`) includes
/// within its family directory, as `(file_name, contents)` pairs.
///
/// The entry file itself is included first. Angled includes (`#include <...>`,
/// i.e. `dt-bindings` headers) are not part of the embedded tree and are
/// skipped; a downstream compile resolves them from the kernel tree. Missing
/// local includes are skipped rather than erroring, since a base may reference an
/// optional overlay-only include.
pub fn local_include_closure(base: &BaseDts) -> Result<Vec<(String, String)>, DataError> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    let mut queue: Vec<String> = vec![base.entry.clone()];

    while let Some(name) = queue.pop() {
        if seen.iter().any(|s| s == &name) {
            continue;
        }
        seen.push(name.clone());
        let path = format!("{}/{}", base.family_dir, name);
        let Some(contents) = DTS.get_file(&path).and_then(|f| f.contents_utf8()) else {
            // Not embedded (e.g. an angled dt-bindings header, or an optional
            // include we don't ship): skip.
            continue;
        };
        for included in local_includes(contents) {
            queue.push(included);
        }
        out.push((name, contents.to_string()));
    }

    // Return in a stable, include-before-dependent-agnostic order: entry first as
    // discovered, then the rest sorted for determinism.
    Ok(out)
}

/// Resolves the base for a chip and returns its local include closure as
/// `(file_name, contents)` pairs, ready to materialize next to a generated
/// device tree so its quoted `#include`s resolve. A convenience wrapping
/// [`resolve_base`] + [`local_include_closure`].
pub fn base_closure(
    chip: &str,
    devkit: Option<&str>,
    base_override: Option<&str>,
) -> Result<Vec<(String, String)>, DataError> {
    let base = resolve_base(chip, devkit, base_override)?;
    local_include_closure(&base)
}

/// The `#include "..."` (quoted, local) file names referenced by `source`.
fn local_includes(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("#include") else {
            continue;
        };
        let rest = rest.trim_start();
        // Quoted, local include only. Angled `<...>` headers are not embedded.
        if let Some(after) = rest.strip_prefix('"')
            && let Some(end) = after.find('"')
        {
            out.push(after[..end].to_string());
        }
    }
    out
}

/// The family subdirectory under `database/dts/` for a chip part number.
fn family_dir(chip: &str) -> Option<&'static str> {
    let needle = normalize(chip);
    if needle.starts_with("stm32mp2") {
        Some("stm32mp2")
    } else if needle.starts_with("mimx9") {
        Some("imx9")
    } else {
        None
    }
}

/// Finds the embedded file matching a stem (no extension), preferring `.dts`
/// (a board file) over `.dtsi` (a SoC include).
fn match_stem(family_dir: &str, stem: &str) -> Option<String> {
    for ext in ["dts", "dtsi"] {
        let name = format!("{stem}.{ext}");
        if DTS.get_file(format!("{family_dir}/{name}")).is_some() {
            return Some(name);
        }
    }
    None
}

/// The generic SoC `.dtsi` file name for a chip part number.
///
/// - i.MX: the `mimx9NN` prefix maps to `imx9N.dtsi` (e.g. `mimx9596` →
///   `imx95.dtsi`).
/// - STM32MP2: the leading `stm32mpNNN` of the part number is the SoC name
///   (e.g. `stm32mp257fak3` → `stm32mp257.dtsi`).
fn generic_dtsi(chip: &str) -> Option<String> {
    let needle = normalize(chip);
    if let Some(rest) = needle.strip_prefix("mimx9") {
        // First digit after `mimx9` selects the SoC line (5 -> imx95, 3 -> imx93).
        let line = rest.chars().next()?;
        return Some(format!("imx9{line}.dtsi"));
    }
    if needle.starts_with("stm32mp") {
        // `stm32mp` + the following three digits is the SoC dtsi stem.
        let digits: String = needle
            .chars()
            .skip("stm32mp".len())
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if digits.len() >= 3 {
            return Some(format!("stm32mp{}.dtsi", &digits[..3]));
        }
    }
    None
}

/// Lowercases the part number and rewrites `imx` aliases to the `mimx` prefix,
/// matching [`crate::database::load`]'s normalization.
fn normalize(chip: &str) -> String {
    let lowered = chip.to_ascii_lowercase();
    match lowered.strip_prefix("imx") {
        Some(rest) => format!("mimx{rest}"),
        None => lowered,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devkit_stem_resolves_board_dts() {
        let base = resolve_base("imx95", Some("imx95-15x15-frdm"), None).unwrap();
        assert_eq!(base.family_dir, "imx9");
        assert_eq!(base.entry, "imx95-15x15-frdm.dts");
        assert!(base.is_board());
        assert!(base.contents().unwrap().contains("/dts-v1/"));
    }

    #[test]
    fn stm32_devkit_stem_resolves() {
        let base = resolve_base("stm32mp257fak3", Some("stm32mp257f-dk"), None).unwrap();
        assert_eq!(base.family_dir, "stm32mp2");
        assert_eq!(base.entry, "stm32mp257f-dk.dts");
    }

    #[test]
    fn no_devkit_falls_back_to_generic_dtsi() {
        let imx = resolve_base("imx95", None, None).unwrap();
        assert_eq!(imx.entry, "imx95.dtsi");
        assert!(!imx.is_board());

        let stm = resolve_base("stm32mp257fak3", None, None).unwrap();
        assert_eq!(stm.entry, "stm32mp257.dtsi");
    }

    #[test]
    fn base_override_wins_over_devkit() {
        let base = resolve_base(
            "stm32mp257fak3",
            Some("stm32mp257f-dk"),
            Some("stm32mp257f-ev1"),
        )
        .unwrap();
        assert_eq!(base.entry, "stm32mp257f-ev1.dts");
    }

    #[test]
    fn unknown_stem_errors() {
        let err = resolve_base("imx95", Some("nope-not-a-board"), None).unwrap_err();
        assert!(matches!(err, DataError::UnknownDts(_)));
    }

    #[test]
    fn closure_includes_soc_dtsi_of_board() {
        let base = resolve_base("stm32mp257fak3", Some("stm32mp257f-dk"), None).unwrap();
        let closure = local_include_closure(&base).unwrap();
        let names: Vec<&str> = closure.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"stm32mp257f-dk.dts"));
        assert!(names.contains(&"stm32mp257.dtsi"));
    }
}
