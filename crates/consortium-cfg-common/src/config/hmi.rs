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

/// `[hmi]` section: packaged human-machine interface configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hmi {
    /// Rendering engine hosting the UI.
    pub engine: HmiEngine,
    /// Windowing/display backend the engine renders to.
    pub backend: HmiBackend,
    /// Dev-server URL loaded in debug builds; absent in release builds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<String>,
    /// Frontend project root the build pipeline compiles (manifest-relative),
    /// e.g. `website`. The pipeline runs the frontend build command here.
    pub source: String,
    /// Resolved dist output directory the frontend build produces and the
    /// application serves in release builds (manifest-relative), e.g.
    /// `website/dist`.
    pub build: String,
    /// PocketJS entry/app name to build (the mounted `-main` entry that installs
    /// the frame contract), e.g. `hero-main`. Required for [`HmiEngine::Pocket`]
    /// and ignored by other engines; it names the `<app>.js`/`<app>.pak` pair.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    /// Logical UI viewport configured by Consortium and passed to the HMI
    /// runtime. PocketJS lays out and clips the retained UI in these pixels.
    #[serde(default)]
    pub logical: HmiLogical,
}

/// Logical HMI viewport in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HmiLogical {
    /// Logical viewport width in pixels.
    pub width: u32,
    /// Logical viewport height in pixels.
    pub height: u32,
}

impl Default for HmiLogical {
    fn default() -> Self {
        Self {
            width: 480,
            height: 272,
        }
    }
}

impl Hmi {
    /// Validates engine-specific field requirements.
    ///
    /// PocketJS needs an [`app`](Self::app) entry name and does not yet have a
    /// DRM/KMS shell, so those combinations are rejected before the build runs.
    pub fn validate(&self) -> Result<(), String> {
        if self.logical.width == 0 || self.logical.height == 0 {
            return Err("[hmi.logical] width and height must be non-zero".to_string());
        }
        match self.engine {
            HmiEngine::Cog => Ok(()),
            HmiEngine::Pocket => {
                if self.app.as_deref().unwrap_or_default().trim().is_empty() {
                    return Err(
                        "[hmi] engine = \"pocket\" requires `app` (the PocketJS entry name, \
                         e.g. \"hero-main\")"
                            .to_string(),
                    );
                }
                if self.backend == HmiBackend::Drm {
                    return Err(
                        "[hmi] engine = \"pocket\" does not yet support backend = \"drm\"; \
                         use \"wayland\" or \"headless\""
                            .to_string(),
                    );
                }
                Ok(())
            }
        }
    }
}

/// HMI rendering engine supported by the current builder pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HmiEngine {
    /// Cog/WPE WebKit shell.
    Cog,
    /// Native PocketJS shell (`consortium-hmi-pocket`).
    Pocket,
}

/// HMI display backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HmiBackend {
    /// Wayland compositor.
    Wayland,
    /// Direct DRM/KMS rendering.
    Drm,
    /// Headless/offscreen rendering.
    Headless,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pocket(app: Option<&str>, backend: HmiBackend) -> Hmi {
        Hmi {
            engine: HmiEngine::Pocket,
            backend,
            debug: None,
            source: "submodules/pocketjs".to_string(),
            build: "dist".to_string(),
            app: app.map(str::to_string),
            logical: HmiLogical::default(),
        }
    }

    #[test]
    fn pocket_requires_app_entry() {
        assert!(
            pocket(Some("hero-main"), HmiBackend::Wayland)
                .validate()
                .is_ok()
        );
        assert!(pocket(None, HmiBackend::Wayland).validate().is_err());
        assert!(pocket(Some("  "), HmiBackend::Wayland).validate().is_err());
    }

    #[test]
    fn pocket_rejects_drm_backend_for_now() {
        assert!(
            pocket(Some("hero-main"), HmiBackend::Drm)
                .validate()
                .is_err()
        );
        assert!(
            pocket(Some("hero-main"), HmiBackend::Headless)
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn cog_ignores_pocket_only_fields() {
        let hmi = Hmi {
            engine: HmiEngine::Cog,
            backend: HmiBackend::Wayland,
            debug: None,
            source: "website".to_string(),
            build: "website/dist".to_string(),
            app: None,
            logical: HmiLogical::default(),
        };
        assert!(hmi.validate().is_ok());
    }

    #[test]
    fn rejects_zero_logical_dimensions() {
        let mut hmi = pocket(Some("hero-main"), HmiBackend::Headless);
        hmi.logical.width = 0;
        assert!(hmi.validate().is_err());
    }
}
