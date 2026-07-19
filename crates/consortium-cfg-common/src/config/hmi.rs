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
}

/// HMI rendering engine supported by the current builder pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HmiEngine {
    /// Cog/WPE WebKit shell.
    Cog,
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
