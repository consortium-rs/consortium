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

use cogcore::{HostRoutesHandler, Shell, View};

use crate::bridge::Bridge;

/// Compiled-in dist path. Set by `ApplicationCompiler` via `CONSORTIUM_HMI_DIST`.
/// Falls back to `"dist"` so the crate compiles outside the Consortium pipeline.
const DIST: &str = match option_env!("CONSORTIUM_HMI_DIST") {
    Some(d) => d,
    None => "dist",
};

/// Compiled-in dev-server URL. Set by `ApplicationCompiler` via `CONSORTIUM_HMI_DEBUG_URL`.
/// `None` means debug builds also use the `csti://` static handler.
const DEBUG_URL: Option<&str> = option_env!("CONSORTIUM_HMI_DEBUG_URL");

/// Minimal kiosk application wrapping a Cog shell and a fullscreen web view.
///
/// Call [`KioskApp::attach_bridge`] before [`KioskApp::load`] so the prelude
/// script is in place before the first navigation.
///
/// # URI scheme
///
/// In release builds (and debug builds without `CONSORTIUM_HMI_DEBUG_URL`),
/// [`load`] registers the `csti://` scheme backed by the directory at [`DIST`]
/// and navigates to `csti://app/index.html`.
///
/// In debug builds with `CONSORTIUM_HMI_DEBUG_URL` set, [`load`] navigates
/// directly to that URL, bypassing the static handler.
///
/// # Usage
///
/// ```no_run
/// use consortium_hmi::{KioskApp, Bridge};
///
/// let mut bridge = Bridge::new();
/// bridge.handle("ping", |_| r#""pong""#.into());
///
/// let app = KioskApp::new("my-kiosk", None, None).unwrap();
/// app.attach_bridge(bridge).unwrap();
/// app.load().unwrap();
/// ```
pub struct KioskApp {
    shell: Shell,
    view: View,
}

impl KioskApp {
    /// Initializes Cog and creates a shell and a fullscreen view.
    ///
    /// `platform` and `module_path` are forwarded to [`cogcore::init`].
    pub fn new(
        name: &str,
        platform: Option<&str>,
        module_path: Option<&str>,
    ) -> cogcore::Result<Self> {
        cogcore::init(platform, module_path)?;
        let shell = Shell::new(name, false)?;
        let view = View::new()?;
        Ok(Self { shell, view })
    }

    /// Returns a reference to the underlying view.
    pub fn view(&self) -> &View {
        &self.view
    }

    /// Attaches a [`Bridge`] to the view.
    ///
    /// Must be called before [`load`] so the prelude script is in place for
    /// the first page load.
    pub fn attach_bridge(&self, bridge: Bridge) -> cogcore::Result<()> {
        bridge.attach(&self.view)
    }

    /// Navigates the view to the application UI.
    ///
    /// In **debug builds** with `CONSORTIUM_HMI_DEBUG_URL` set at compile time,
    /// loads that URL directly (dev-server hot-reload path).
    ///
    /// Otherwise registers the `csti://` URI scheme backed by [`DIST`] and
    /// loads `csti://app/index.html`.
    pub fn load(&self) -> cogcore::Result<()> {
        #[cfg(debug_assertions)]
        if let Some(url) = DEBUG_URL {
            return self.view.load_uri(url);
        }

        let sec = self.shell.security_manager()?;
        sec.register_secure("csti")?;
        sec.register_cors_enabled("csti")?;

        let routes = HostRoutesHandler::new(None)?;
        routes.add_path("app", DIST)?;
        self.shell
            .set_request_handler("csti", routes.as_request_handler())?;
        self.view.load_uri("csti://app/index.html")
    }
}
