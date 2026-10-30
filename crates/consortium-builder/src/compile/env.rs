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

//! Compile-time environment-injection carriers.
//!
//! The builder lowers high-level build parameters into the `CONSORTIUM_*`
//! environment variables that firmware/application/HMI crates read with
//! `option_env!` at compile time. These types describe *what* the builder
//! injects, not the parsed `Consortium.toml` schema (that lives in
//! `consortium-cfg-common`); the mapping from the current schema onto these
//! carriers is still being reconciled, so they stay local to the builder.

use std::path::{Path, PathBuf};

use consortium_cfg_common::config::{
    CSTI_MANIFEST_PATH_ENV, CSTI_MCU_PATH_ENV, Hmi, HmiEngine, Workspace,
};
use serde::{Deserialize, Serialize};

/// The build-host directories a pipeline run exposes to every endpoint `cargo`
/// invocation, plus the `[profile] workspace` layout that decides where each
/// endpoint crate's `target/` lives.
///
/// Injected as `CSTI_MANIFEST_PATH` (always) and `CSTI_MCU_PATH` (when the
/// manifest declares exactly one controller-core firmware crate). The generated
/// AP module names the embedded debug ELFs relative to one of them, so it carries
/// no build-host path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CstiWorkspace {
    /// `[profile] workspace`.
    pub layout: Workspace,
    /// Absolute directory containing `Consortium.toml`.
    pub manifest_dir: PathBuf,
    /// Absolute root of the controller-core firmware crate, when there is
    /// exactly one.
    pub mcu_dir: Option<PathBuf>,
}

impl CstiWorkspace {
    /// The `CSTI_*` variables injected into every endpoint `cargo` invocation.
    pub fn env_pairs(&self) -> Vec<(&'static str, &Path)> {
        let mut pairs = vec![(CSTI_MANIFEST_PATH_ENV, self.manifest_dir.as_path())];
        if let Some(mcu) = &self.mcu_dir {
            pairs.push((CSTI_MCU_PATH_ENV, mcu.as_path()));
        }
        pairs
    }

    /// The directory whose `target/` receives the build output of the crate at
    /// `crate_dir`: the manifest directory in a unified workspace, the crate
    /// itself when nested.
    pub fn target_base<'a>(&'a self, crate_dir: &'a Path) -> &'a Path {
        match self.layout {
            Workspace::Unified => &self.manifest_dir,
            Workspace::Nested => crate_dir,
        }
    }
}

/// On-target install subdirectory (beneath `[profile] deploy_root`) for each
/// engine's assets: a served tree for Cog, a bundle/pak pair for PocketJS.
const WWW_SUBDIR: &str = "www";
const POCKET_SUBDIR: &str = "hmi";

/// Compile-time HMI carrier the builder injects as environment variables before
/// invoking the application `cargo build`. The consuming runtime crate reads
/// them with `option_env!` at compile time, so the *values* are on-target paths
/// resolved beneath `[profile] deploy_root` — the build host never dereferences
/// them.
///
/// The engine tag selects which variables are emitted:
/// - [`Cog`](Self::Cog): `consortium-hmi-webkit` serves a dist directory.
/// - [`Pocket`](Self::Pocket): `consortium-hmi-pocket` loads a bundle + pak.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "engine")]
pub enum HmiConfig {
    /// Cog/WPE WebKit shell serving a static dist directory via `csti://`.
    Cog {
        /// On-target directory the deployed application serves assets from;
        /// becomes `CONSORTIUM_HMI_DIST`.
        dist: String,
        /// Dev-server URL loaded directly in debug builds; when absent, debug
        /// builds fall back to the `csti://` static handler.
        #[serde(skip_serializing_if = "Option::is_none")]
        debug_url: Option<String>,
    },
    /// Native PocketJS shell loading a built bundle + pak from on-target paths.
    Pocket {
        /// On-target path of the PocketJS `<app>.js` bundle; becomes
        /// `CONSORTIUM_HMI_POCKET_BUNDLE`.
        bundle: String,
        /// On-target path of the PocketJS `<app>.pak`; becomes
        /// `CONSORTIUM_HMI_POCKET_PAK`.
        pak: String,
        /// Logical PocketJS viewport width in pixels.
        logical_width: u32,
        /// Logical PocketJS viewport height in pixels.
        logical_height: u32,
    },
}

impl HmiConfig {
    /// Builds the compile-time HMI carrier from the parsed `[hmi]` section.
    ///
    /// Because the build host and the target are different machines, every value
    /// is an *on-target* path a downstream packaging recipe installs assets to.
    /// Relative install locations are resolved beneath `deploy_root` (the
    /// manifest's `[profile] deploy_root` convention): Cog serves
    /// `{deploy_root}/www`, and PocketJS loads `{deploy_root}/hmi/<app>.{js,pak}`.
    ///
    /// Callers must have validated `hmi` (see [`Hmi::validate`]); a PocketJS
    /// section without `app` falls back to an empty name here.
    pub fn from_hmi(hmi: &Hmi, deploy_root: &str) -> Self {
        match hmi.engine {
            HmiEngine::Cog => Self::Cog {
                dist: on_target(deploy_root, WWW_SUBDIR)
                    .to_string_lossy()
                    .into_owned(),
                debug_url: hmi.debug.clone(),
            },
            HmiEngine::Pocket => {
                let app = hmi.app.as_deref().unwrap_or_default();
                let base = on_target(deploy_root, POCKET_SUBDIR);
                Self::Pocket {
                    bundle: base
                        .join(format!("{app}.js"))
                        .to_string_lossy()
                        .into_owned(),
                    pak: base
                        .join(format!("{app}.pak"))
                        .to_string_lossy()
                        .into_owned(),
                    logical_width: hmi.logical.width,
                    logical_height: hmi.logical.height,
                }
            }
        }
    }

    /// Environment variables the builder injects for this config.
    ///
    /// Consuming crates read these with `option_env!` at compile time. For Cog,
    /// `CONSORTIUM_HMI_DEBUG_URL` is emitted only when set, so its absence lets
    /// debug builds fall back to the static `csti://` handler.
    pub fn env_pairs(&self) -> Vec<(String, String)> {
        match self {
            Self::Cog { dist, debug_url } => {
                let mut pairs = vec![("CONSORTIUM_HMI_DIST".to_string(), dist.clone())];
                if let Some(url) = debug_url {
                    pairs.push(("CONSORTIUM_HMI_DEBUG_URL".to_string(), url.clone()));
                }
                pairs
            }
            Self::Pocket {
                bundle,
                pak,
                logical_width,
                logical_height,
            } => vec![
                ("CONSORTIUM_HMI_POCKET_BUNDLE".to_string(), bundle.clone()),
                ("CONSORTIUM_HMI_POCKET_PAK".to_string(), pak.clone()),
                (
                    "CONSORTIUM_HMI_LOGICAL_WIDTH".to_string(),
                    logical_width.to_string(),
                ),
                (
                    "CONSORTIUM_HMI_LOGICAL_HEIGHT".to_string(),
                    logical_height.to_string(),
                ),
            ],
        }
    }
}

/// Resolves an on-target subdirectory beneath `deploy_root`. An absolute
/// `deploy_root` yields an absolute path; the subdir is always relative.
fn on_target(deploy_root: &str, subdir: &str) -> PathBuf {
    Path::new(deploy_root).join(subdir)
}

/// Cortex-M core a firmware crate targets; selects the Rust target triple.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CortexMcuCore {
    CortexM0p,
    CortexM33,
    CortexM7,
    CortexM4,
    CortexM4f,
}

impl CortexMcuCore {
    /// Parses a dash-separated core name string (e.g., `"cortex-m33"`) into a variant.
    pub fn as_key(s: &str) -> Option<Self> {
        match s {
            "cortex-m0p" => Some(Self::CortexM0p),
            "cortex-m33" => Some(Self::CortexM33),
            "cortex-m7" => Some(Self::CortexM7),
            "cortex-m4" => Some(Self::CortexM4),
            "cortex-m4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    /// Returns the short abbreviation for this core (e.g., `"cm33"`).
    pub fn abbr(&self) -> &'static str {
        match self {
            Self::CortexM0p => "cm0p",
            Self::CortexM33 => "cm33",
            Self::CortexM7 => "cm7",
            Self::CortexM4 => "cm4",
            Self::CortexM4f => "cm4f",
        }
    }

    /// Parses a short abbreviation (e.g., `"cm33"`) into a variant.
    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "cm0p" => Some(Self::CortexM0p),
            "cm33" => Some(Self::CortexM33),
            "cm7" => Some(Self::CortexM7),
            "cm4" => Some(Self::CortexM4),
            "cm4f" => Some(Self::CortexM4f),
            _ => None,
        }
    }

    /// Returns the Rust target triple for cross-compiling to this core.
    pub fn to_rust_target(&self) -> &'static str {
        match self {
            Self::CortexM0p => "thumbv6m-none-eabi",
            Self::CortexM33 => "thumbv8m.main-none-eabihf",
            Self::CortexM7 | Self::CortexM4f => "thumbv7em-none-eabihf",
            Self::CortexM4 => "thumbv7em-none-eabi",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmi_env_pairs_emit_dist_and_optional_debug_url() {
        let with_url = HmiConfig::Cog {
            dist: "ui/dist".to_string(),
            debug_url: Some("http://localhost:5173".to_string()),
        };
        assert_eq!(
            with_url.env_pairs(),
            [
                ("CONSORTIUM_HMI_DIST".to_string(), "ui/dist".to_string()),
                (
                    "CONSORTIUM_HMI_DEBUG_URL".to_string(),
                    "http://localhost:5173".to_string()
                ),
            ]
        );

        let no_url = HmiConfig::Cog {
            dist: "ui/dist".to_string(),
            debug_url: None,
        };
        assert_eq!(
            no_url.env_pairs(),
            [("CONSORTIUM_HMI_DIST".to_string(), "ui/dist".to_string())]
        );
    }

    #[test]
    fn pocket_env_pairs_emit_bundle_and_pak_paths() {
        let cfg = HmiConfig::Pocket {
            bundle: "/opt/consortium/hmi/hero-main.js".to_string(),
            pak: "/opt/consortium/hmi/hero-main.pak".to_string(),
            logical_width: 800,
            logical_height: 480,
        };
        assert_eq!(
            cfg.env_pairs(),
            [
                (
                    "CONSORTIUM_HMI_POCKET_BUNDLE".to_string(),
                    "/opt/consortium/hmi/hero-main.js".to_string()
                ),
                (
                    "CONSORTIUM_HMI_POCKET_PAK".to_string(),
                    "/opt/consortium/hmi/hero-main.pak".to_string()
                ),
                (
                    "CONSORTIUM_HMI_LOGICAL_WIDTH".to_string(),
                    "800".to_string()
                ),
                (
                    "CONSORTIUM_HMI_LOGICAL_HEIGHT".to_string(),
                    "480".to_string()
                ),
            ]
        );
    }

    #[test]
    fn from_hmi_places_cog_dist_under_deploy_root() {
        use consortium_cfg_common::config::{Hmi, HmiBackend, HmiEngine};

        let hmi = Hmi {
            engine: HmiEngine::Cog,
            backend: HmiBackend::Wayland,
            debug: Some("http://127.0.0.1:5173/".to_string()),
            source: "website".to_string(),
            build: "website/dist".to_string(),
            app: None,
            logical: Default::default(),
        };

        // The Cog engine serves `{deploy_root}/www`, and the debug URL carries
        // through for dev builds.
        let cfg = HmiConfig::from_hmi(&hmi, "/opt/consortium");
        assert_eq!(
            cfg,
            HmiConfig::Cog {
                dist: "/opt/consortium/www".to_string(),
                debug_url: Some("http://127.0.0.1:5173/".to_string()),
            }
        );
    }

    #[test]
    fn from_hmi_places_pocket_bundle_and_pak_under_deploy_root() {
        use consortium_cfg_common::config::{Hmi, HmiBackend, HmiEngine};

        let hmi = Hmi {
            engine: HmiEngine::Pocket,
            backend: HmiBackend::Wayland,
            debug: None,
            source: "submodules/pocketjs".to_string(),
            build: "dist".to_string(),
            app: Some("hero-main".to_string()),
            logical: consortium_cfg_common::config::HmiLogical {
                width: 800,
                height: 480,
            },
        };

        let cfg = HmiConfig::from_hmi(&hmi, "/opt/consortium");
        assert_eq!(
            cfg,
            HmiConfig::Pocket {
                bundle: "/opt/consortium/hmi/hero-main.js".to_string(),
                pak: "/opt/consortium/hmi/hero-main.pak".to_string(),
                logical_width: 800,
                logical_height: 480,
            }
        );
    }

    #[test]
    fn mcu_core_resolves_rust_target_triples() {
        assert_eq!(
            CortexMcuCore::CortexM33.to_rust_target(),
            "thumbv8m.main-none-eabihf"
        );
        assert_eq!(
            CortexMcuCore::CortexM7.to_rust_target(),
            "thumbv7em-none-eabihf"
        );
        assert_eq!(
            CortexMcuCore::as_key("cortex-m33"),
            Some(CortexMcuCore::CortexM33)
        );
        assert_eq!(
            CortexMcuCore::from_abbr("cm0p"),
            Some(CortexMcuCore::CortexM0p)
        );
        assert_eq!(CortexMcuCore::CortexM4f.abbr(), "cm4f");
    }
}
