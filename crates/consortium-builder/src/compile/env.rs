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

use consortium_cfg_common::config::{Hmi, HmiEngine, MemoryRegion};
use serde::{Deserialize, Serialize};

/// Shared-memory region layout for an IPC channel.
///
/// Injected as compile-time environment variables by the builder before invoking
/// `cargo build`. The physical base address is **not** carried here: the IPC
/// transport never resolves its region from the environment. Controller firmware
/// gets the base baked into its generated init code (via the lowered `Address`),
/// and the Linux application reaches the *same* memory through the UIO device
/// named by `uio`; so no side needs a `CONSORTIUM_IPC_BASE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SharedMemConfig {
    /// Length of the region in bytes.
    pub length: u64,
    /// UIO device index the Linux application mmaps to reach the same region.
    pub uio: u32,
    /// Maximum transfer unit (payload bytes per slot) for the channel.
    pub mtu: u32,
}

impl SharedMemConfig {
    /// Builds a resolved config from a `[[ipc.channel]]` shared-memory region
    /// plus the UIO index and MTU the lowering resolved.
    ///
    /// Only the region's `length` is carried; the physical base is resolved by
    /// the generated code, not the environment (see the type docs). `uio` is the
    /// AP-side UIO device index the Linux application mmaps (from `LoweredIr`'s
    /// resolved `ApAddress`), and `mtu` is the channel MTU.
    pub fn from_region(region: MemoryRegion, uio: u32, mtu: u32) -> Self {
        Self {
            length: u64::from(region.length),
            uio,
            mtu,
        }
    }

    /// Environment variable suffixes used when the builder injects this config.
    ///
    /// The builder emits `CONSORTIUM_<PREFIX>_LENGTH`, `_UIO`, and `_MTU`, where
    /// `<PREFIX>` is `IPC`. Consuming crates read these with `option_env!` and
    /// parse them in a `const` context. Numeric values are emitted as decimal
    /// strings even when the source manifest used hexadecimal TOML literals.
    pub fn env_pairs(&self, prefix: &str) -> [(String, String); 3] {
        [
            (
                format!("CONSORTIUM_{prefix}_LENGTH"),
                self.length.to_string(),
            ),
            (format!("CONSORTIUM_{prefix}_UIO"), self.uio.to_string()),
            (format!("CONSORTIUM_{prefix}_MTU"), self.mtu.to_string()),
        ]
    }
}

/// Debug-ring layout injected as `CONSORTIUM_DBG_*` compile-time variables.
///
/// Unlike IPC, the defmt logger *does* read its region from the environment:
/// `consortium-dbg`'s `config` module resolves `CONSORTIUM_DBG_BASE` and
/// `CONSORTIUM_DBG_LENGTH` with `option_env!` in a `const` context (and treats
/// the presence of `BASE` as "debug configured"). So the debug carrier keeps the
/// physical base the IPC carrier drops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct DbgConfig {
    /// Physical base of the debug ring, as seen by the logging core (raw memory).
    pub base: u64,
    /// Length of the ring in bytes.
    pub length: u64,
}

impl DbgConfig {
    /// Builds a debug carrier from a resolved ring base and length.
    pub fn new(base: u64, length: u64) -> Self {
        Self { base, length }
    }

    /// The `CONSORTIUM_DBG_BASE`/`_LENGTH` pairs `consortium-dbg` reads.
    /// Numeric values are emitted as decimal strings.
    pub fn env_pairs(&self) -> [(String, String); 2] {
        [
            ("CONSORTIUM_DBG_BASE".to_string(), self.base.to_string()),
            ("CONSORTIUM_DBG_LENGTH".to_string(), self.length.to_string()),
        ]
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

    const SAMPLE: SharedMemConfig = SharedMemConfig {
        length: 0x1000,
        uio: 0,
        mtu: 64,
    };

    #[test]
    fn env_pairs_emit_the_ipc_layout_the_app_reads() {
        // The IPC carrier no longer carries a base: nothing reads
        // `CONSORTIUM_IPC_BASE` (the transport resolves its region from generated
        // code / UIO). Numeric env-var values are decimal strings.
        let pairs = SAMPLE.env_pairs("IPC");
        assert_eq!(
            pairs,
            [
                ("CONSORTIUM_IPC_LENGTH".to_string(), "4096".to_string()),
                ("CONSORTIUM_IPC_UIO".to_string(), "0".to_string()),
                ("CONSORTIUM_IPC_MTU".to_string(), "64".to_string()),
            ]
        );
        // No base is emitted for IPC.
        assert!(!pairs.iter().any(|(k, _)| k == "CONSORTIUM_IPC_BASE"));
    }

    #[test]
    fn dbg_config_emits_the_base_and_length_the_logger_reads() {
        // The debug carrier keeps base + length, matching exactly what
        // `consortium-dbg`'s `config` module reads with `option_env!`.
        let dbg = DbgConfig::new(0x2040_8000, 0x2000);
        assert_eq!(
            dbg.env_pairs(),
            [
                ("CONSORTIUM_DBG_BASE".to_string(), "541097984".to_string()),
                ("CONSORTIUM_DBG_LENGTH".to_string(), "8192".to_string()),
            ]
        );
    }

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
    fn from_region_widens_schema_region_into_injectable_config() {
        // A resolved `[[ipc.channel]]` region plus the lowered UIO index and MTU
        // produce the config the builder injects; the region's base is dropped
        // (the IPC carrier has no base).
        let region = MemoryRegion {
            base: 0x2040_0000,
            length: 0x4000,
        };
        let cfg = SharedMemConfig::from_region(region, 0, 128);
        assert_eq!(
            cfg,
            SharedMemConfig {
                length: 0x4000,
                uio: 0,
                mtu: 128,
            }
        );
        // The widened length round-trips through the env injection.
        let pairs = cfg.env_pairs("IPC");
        assert_eq!(
            pairs[0],
            ("CONSORTIUM_IPC_LENGTH".to_string(), "16384".to_string())
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
