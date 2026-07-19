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

//! Native PocketJS backend for [`consortium_hmi`].
//!
//! The prototype hosts a PocketJS bundle in QuickJS, mounts the retained UI
//! surface, mounts Consortium application services as a separate native
//! surface, and provides deterministic headless wgpu rendering. A production
//! Wayland/DRM event shell can build on the same [`PocketApp`] frame contract.

mod service;

use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use pocket_mod::{Guest, qjs::Object};

use consortium_hmi::Bridge;
use service::PocketService;

pub use headless::HeadlessRenderer;
pub use pocket_ui_wgpu::{UiRenderer, UiSurface};

mod headless;

/// PocketJS host identity and logical display configuration.
#[derive(Clone, Debug)]
pub struct PocketConfig {
    /// Label used in QuickJS evaluation errors and logs.
    pub app_label: String,
    /// Logical UI size used by `pocketjs-core`.
    pub logical_size: (u32, u32),
    /// Target identity published as `ui.__host`.
    pub host_target: String,
    /// Host ABI published as `ui.__hostAbi`.
    pub host_abi: u32,
}

impl Default for PocketConfig {
    fn default() -> Self {
        Self {
            app_label: "consortium-pocket".into(),
            logical_size: (480, 272),
            host_target: "desktop".into(),
            host_abi: 1,
        }
    }
}

/// A PocketJS guest, retained UI surface, and Consortium service adapter.
///
/// `PocketApp` is intentionally single-threaded. The display/event-loop thread
/// owns it; Tokio handlers run elsewhere and return through a channel drained
/// by [`frame`](Self::frame) or [`pump_bridge`](Self::pump_bridge).
pub struct PocketApp {
    guest: Guest,
    ui: UiSurface,
    service: PocketService,
}

impl PocketApp {
    /// Creates an application from an in-memory PocketJS bundle and pak.
    pub fn new(
        bundle: &str,
        pak: &[u8],
        bridge: Bridge,
        runtime: tokio::runtime::Handle,
        config: PocketConfig,
    ) -> Result<Self> {
        if config.logical_size.0 == 0 || config.logical_size.1 == 0 {
            bail!("PocketJS logical viewport dimensions must be non-zero");
        }
        if config.host_target.is_empty() || config.host_abi == 0 {
            bail!("PocketJS host target and ABI must identify a real contract");
        }

        let guest = Guest::new().context("creating PocketJS QuickJS guest")?;
        let ui = UiSurface::new((config.logical_size.0 as f32, config.logical_size.1 as f32));
        ui.feed_pak(pak);
        ui.mount(&guest).context("mounting PocketJS ui surface")?;
        publish_host_contract(&guest, &config)?;

        let service = PocketService::mount(&guest, bridge, runtime)?;
        guest
            .eval(&config.app_label, bundle)
            .context("evaluating PocketJS application bundle")?;
        if !guest.has_frame() {
            bail!("PocketJS bundle evaluated but did not install globalThis.frame");
        }

        Ok(Self { guest, ui, service })
    }

    /// Loads a UTF-8 bundle and binary pak from disk.
    pub fn from_files(
        bundle: impl AsRef<Path>,
        pak: impl AsRef<Path>,
        bridge: Bridge,
        runtime: tokio::runtime::Handle,
        config: PocketConfig,
    ) -> Result<Self> {
        let bundle_path = bundle.as_ref();
        let pak_path = pak.as_ref();
        let bundle = fs::read_to_string(bundle_path)
            .with_context(|| format!("reading PocketJS bundle {}", bundle_path.display()))?;
        let pak = fs::read(pak_path)
            .with_context(|| format!("reading PocketJS pak {}", pak_path.display()))?;
        Self::new(&bundle, &pak, bridge, runtime, config)
    }

    /// Runs one guest turn and advances the retained UI core once.
    ///
    /// Completed Consortium commands are delivered before `frame()`, so
    /// PocketJS's effect shell observes them at the next frame boundary.
    pub fn frame(&mut self, buttons: u32, analog: u32) -> Result<usize> {
        let delivered = self.pump_bridge()?;
        self.guest
            .frame_with_analog(buttons, analog)
            .context("running PocketJS guest frame")?;
        self.ui.tick();
        Ok(delivered)
    }

    /// Runs one frame with a centered analog input.
    pub fn frame_buttons(&mut self, buttons: u32) -> Result<usize> {
        self.frame(buttons, 0x8080)
    }

    /// Delivers every currently completed Consortium command to QuickJS.
    ///
    /// Window hosts normally use [`frame`](Self::frame). Headless tests may
    /// call this directly when they need to observe an async completion without
    /// advancing virtual time.
    pub fn pump_bridge(&mut self) -> Result<usize> {
        self.service.pump(&self.guest)
    }

    /// Evaluates an additional global script, useful for diagnostics and hot
    /// reload prototypes.
    pub fn eval(&self, label: &str, source: &str) -> Result<()> {
        self.guest.eval(label, source)
    }

    /// Returns the native PocketJS UI surface for a window renderer.
    pub fn ui(&self) -> &UiSurface {
        &self.ui
    }
}

fn publish_host_contract(guest: &Guest, config: &PocketConfig) -> Result<()> {
    guest
        .with(|ctx| -> pocket_mod::qjs::Result<()> {
            let ui: Object = ctx.globals().get("ui")?;
            ui.set("__host", config.host_target.as_str())?;
            ui.set("__hostAbi", config.host_abi)?;
            Ok(())
        })
        .map_err(|error| anyhow::anyhow!("publishing PocketJS host contract: {error}"))
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, time::Duration};

    use super::*;

    #[test]
    fn routes_bridge_response_into_quickjs() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .build()
            .unwrap();
        let (called_tx, called_rx) = mpsc::channel();
        let mut bridge = Bridge::new();
        bridge.handle("echo", move |payload| {
            called_tx.send(()).unwrap();
            payload
        });

        let mut app = PocketApp::new(
            r#"
              globalThis.answer = null;
              globalThis.frame = () => {};
              globalThis.__BRIDGE__.invoke("echo", { ok: true })
                .then(value => { globalThis.answer = value; });
            "#,
            &[],
            bridge,
            runtime.handle().clone(),
            PocketConfig::default(),
        )
        .unwrap();

        called_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        assert_eq!(app.pump_bridge().unwrap(), 1);
        app.eval(
            "assert-response",
            r#"if (answer?.ok !== true) throw new Error("missing bridge response");"#,
        )
        .unwrap();
        app.eval(
            "assert-contract",
            r#"if (ui.__host !== "desktop" || ui.__hostAbi !== 1) throw new Error("bad host contract");"#,
        )
        .unwrap();
    }

    #[test]
    fn validates_host_configuration() {
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        let error = PocketApp::new(
            "globalThis.frame = () => {};",
            &[],
            Bridge::new(),
            runtime.handle().clone(),
            PocketConfig {
                logical_size: (0, 272),
                ..PocketConfig::default()
            },
        )
        .err()
        .unwrap();
        assert!(error.to_string().contains("viewport"));
    }
}
