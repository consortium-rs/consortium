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

mod error;
mod pad;
mod service;

use std::{fs, path::Path};

use pocket_mod::{Guest, qjs::Object};

use consortium_hmi::Bridge;
use service::PocketService;

pub use consortium_hmi_input::{
    ButtonCode, InputEvent, InputSource, PointerButton, ScrollAxis, SliceSource, TouchPhase,
};
pub use error::{PocketError, Result};
pub use headless::HeadlessRenderer;
pub use pad::PadState;
pub use pocket_ui_wgpu::{UiRenderer, UiSurface};

mod headless;

#[cfg(feature = "shell")]
mod shell;
#[cfg(feature = "shell")]
pub use shell::{ShellConfig, run};

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
    logical_size: (u32, u32),
    pad: PadState,
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
            return Err(PocketError::ZeroViewport);
        }
        if config.host_target.is_empty() || config.host_abi == 0 {
            return Err(PocketError::InvalidHostContract);
        }

        let guest =
            Guest::new().map_err(|e| PocketError::host("creating PocketJS QuickJS guest", e))?;
        let ui = UiSurface::new((config.logical_size.0 as f32, config.logical_size.1 as f32));
        ui.feed_pak(pak);
        ui.mount(&guest)
            .map_err(|e| PocketError::host("mounting PocketJS ui surface", e))?;
        publish_host_contract(&guest, &config)?;

        let service = PocketService::mount(&guest, bridge, runtime)?;
        guest
            .eval(&config.app_label, bundle)
            .map_err(|e| PocketError::host("evaluating PocketJS application bundle", e))?;
        if !guest.has_frame() {
            return Err(PocketError::MissingFrame);
        }

        Ok(Self {
            guest,
            ui,
            service,
            logical_size: config.logical_size,
            pad: PadState::new(),
        })
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
        let bundle = fs::read_to_string(bundle_path).map_err(|source| PocketError::ReadBundle {
            path: bundle_path.to_path_buf(),
            source,
        })?;
        let pak = fs::read(pak_path).map_err(|source| PocketError::ReadPak {
            path: pak_path.to_path_buf(),
            source,
        })?;
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
            .map_err(|e| PocketError::host("running PocketJS guest frame", e))?;
        self.ui.tick();
        Ok(delivered)
    }

    /// Runs one frame with a centered analog input.
    pub fn frame_buttons(&mut self, buttons: u32) -> Result<usize> {
        self.frame(buttons, pad::ANALOG_CENTER)
    }

    /// Folds one backend-neutral [`InputEvent`] into the pending pad state.
    ///
    /// Events accumulate until the next [`frame_pad`](Self::frame_pad) (or
    /// [`frame_from`](Self::frame_from)) delivers the held state to the guest,
    /// so callers apply events as they arrive from any [`InputSource`] and let
    /// the frame boundary do the delivery.
    pub fn apply_input(&mut self, event: &InputEvent) {
        self.pad.apply(event);
    }

    /// Runs one frame using the accumulated input state.
    pub fn frame_pad(&mut self) -> Result<usize> {
        self.frame(self.pad.buttons(), self.pad.analog())
    }

    /// Drains an [`InputSource`] into the pad state, then runs one frame.
    ///
    /// This is the source-agnostic driver: a winit shell, a `libinput` reader,
    /// or an IPC receiver carrying events from a real-time core all present as
    /// an [`InputSource`], and the guest sees only the folded frame.
    pub fn frame_from(&mut self, source: &mut impl InputSource) -> Result<usize> {
        while let Some(event) = source.poll_event() {
            self.pad.apply(&event);
        }
        self.frame_pad()
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
        self.guest
            .eval(label, source)
            .map_err(|e| PocketError::host("evaluating PocketJS script", e))
    }

    /// Returns the native PocketJS UI surface for a window renderer.
    pub fn ui(&self) -> &UiSurface {
        &self.ui
    }

    /// Logical UI size the guest renders at, in PocketJS units.
    ///
    /// A window shell sizes its offscreen render target with this and scales
    /// the result to the physical surface, so the UI is DPI-independent.
    pub fn logical_size(&self) -> (u32, u32) {
        self.logical_size
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
        .map_err(|error| PocketError::host("publishing PocketJS host contract", error.to_string()))
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
