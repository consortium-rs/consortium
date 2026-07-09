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

//! `[dts]` section: how the Linux device tree is generated.

/// `[dts]` section.
///
/// Controls how the builder turns the vendor board/SoC device tree plus the
/// lowered UIO layout into the artifact under `dist/dts/linux/`.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dts {
    /// Output form. Defaults to [`DtsMode::Merge`] (a full board `.dts`).
    #[serde(default)]
    pub mode: DtsMode,
    /// Opt into `CONFIG_OF_CONFIGFS=y` in the kernel fragment. Only meaningful in
    /// [`DtsMode::Overlay`], where an overlay may be applied at runtime through
    /// configfs. Ignored in merge mode.
    #[serde(default)]
    pub configfs: bool,
    /// Explicit base device-tree file stem override (no extension), matched
    /// against the vendor DTS database. Wins over `[profile].devkit` and the
    /// generic SoC `.dtsi` fallback. Useful for a board not otherwise selectable
    /// through `devkit`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// Path to a Linux kernel source tree, used only for its `dt-bindings`
    /// headers when compiling the generated device tree to a `.dtb`/`.dtbo`.
    ///
    /// Those headers are GPL-2.0 licensed, so Consortium never bundles them: the
    /// user points at their own kernel checkout (e.g. `~/src/linux`). When set,
    /// the pipeline preprocesses (`gcc -E`) and compiles (`dtc`) the device tree;
    /// when unset, it emits only the `.dts`/`.dtso` and prints how to compile it.
    /// A relative path is resolved against the manifest directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<String>,
}

/// The device-tree output form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DtsMode {
    /// Emit a full, self-contained board `.dts`: the base tree with the UIO
    /// nodes, reserved-memory carveouts, remoteproc fixups, and conflict
    /// disabling merged in. This is the default and the only mode that can strip
    /// a remoteproc node's `mboxes`/`mbox-names`.
    #[default]
    Merge,
    /// Emit a `.dtso` overlay applied on top of an unmodified base tree. Cannot
    /// delete properties, so a remoteproc core's `mboxes`/`mbox-names` cannot be
    /// removed in this mode — use [`DtsMode::Merge`] when a remoteproc core is
    /// present.
    Overlay,
}

impl DtsMode {
    /// Whether this mode emits an overlay (`.dtso`) rather than a full `.dts`.
    pub fn is_overlay(self) -> bool {
        matches!(self, DtsMode::Overlay)
    }
}
