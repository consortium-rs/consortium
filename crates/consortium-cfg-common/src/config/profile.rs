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

/// `[profile]` section.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    /// Full chip part number, e.g. `stm32mp257fak3`. Resolved against the chip
    /// database to pick the matching family definition.
    pub chip: String,
    /// Optional development-board device-tree file stem, matched against the
    /// vendor DTS database (e.g. `stm32mp257f-dk`, `imx95-15x15-frdm`). When set,
    /// the Linux device tree is built on top of that board `.dts`; when unset,
    /// the generic SoC `.dtsi` for the chip family is used instead.
    ///
    /// This is the *full file stem* (no `.dts` extension), not a short board
    /// token, so the match is unambiguous.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devkit: Option<String>,
    /// Shared crate location
    pub shared: String,
    /// On-target root under which the build pipeline's artifacts are expected to
    /// be installed by a downstream packaging recipe (e.g. `/opt/consortium`).
    ///
    /// This is a *naming convention only*: the pipeline never writes to a live
    /// target filesystem. It is used to compose the HMI dist path, the systemd
    /// unit's `ExecStart=`/`WorkingDirectory=`, and the firmware install
    /// location a bitbake recipe consumes.
    pub deploy_root: String,
    /// Optional Yocto/OpenSTLinux target sysroot used when cross-compiling the
    /// Linux application. When set, it takes priority over the `SDKTARGETSYSROOT`
    /// / `OECORE_TARGET_SYSROOT` environment variables, so a build does not need
    /// an SDK environment-setup script to be sourced. A relative path is resolved
    /// against the manifest directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysroot: Option<String>,
    /// Optional kernel recipe name (`PN`) the build targets, e.g.
    /// `linux-stm32mp` or `linux-imx`. When set and the manifest declares UIO
    /// devices, the pipeline emits a `<kernel_recipe>_%.bbappend` under
    /// `dist/kernel/` that merges the generated UIO config fragment into the
    /// kernel build. Without it, the fragment is still staged but no bbappend is
    /// generated (the integrator wires it into their kernel recipe manually).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_recipe: Option<String>,
}
