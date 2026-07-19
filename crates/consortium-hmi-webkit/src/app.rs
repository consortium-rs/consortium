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

use crate::{Bridge, bridge::attach};

/// Compiled-in web dist path injected by `consortium-builder`.
const DIST: &str = match option_env!("CONSORTIUM_HMI_DIST") {
    Some(dist) => dist,
    None => "dist",
};

/// Compiled-in dev-server URL injected by `consortium-builder`.
const DEBUG_URL: Option<&str> = option_env!("CONSORTIUM_HMI_DEBUG_URL");

/// Minimal kiosk application wrapping a Cog shell and fullscreen web view.
///
/// Register application services in the backend-neutral [`Bridge`], attach it
/// before loading, and then enter the GLib main loop owned by the application.
///
/// ```no_run
/// use consortium_hmi_webkit::{Bridge, KioskApp};
///
/// # async fn run() {
/// let mut bridge = Bridge::new();
/// bridge.handle("ping", |_| r#""pong""#.into());
///
/// let app = KioskApp::new("my-kiosk", None, None).unwrap();
/// app.attach_bridge(bridge).unwrap();
/// app.load().unwrap();
/// # }
/// ```
pub struct KioskApp {
    shell: Shell,
    view: View,
}

impl KioskApp {
    /// Initializes Cog and creates a shell and fullscreen view.
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

    /// Returns the underlying WebKit view.
    pub fn view(&self) -> &View {
        &self.view
    }

    /// Attaches services using the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics when called outside a Tokio runtime. Use
    /// [`attach_bridge_with_runtime`](Self::attach_bridge_with_runtime) when
    /// the GLib setup code does not execute inside one.
    pub fn attach_bridge(&self, bridge: Bridge) -> cogcore::Result<()> {
        self.attach_bridge_with_runtime(bridge, tokio::runtime::Handle::current())
    }

    /// Attaches services using an explicit Tokio runtime for handler futures.
    pub fn attach_bridge_with_runtime(
        &self,
        bridge: Bridge,
        runtime: tokio::runtime::Handle,
    ) -> cogcore::Result<()> {
        attach(bridge, runtime, &self.view)
    }

    /// Loads the debug URL or the packaged `csti://app/index.html` frontend.
    pub fn load(&self) -> cogcore::Result<()> {
        #[cfg(debug_assertions)]
        if let Some(url) = DEBUG_URL {
            return self.view.load_uri(url);
        }

        let security = self.shell.security_manager()?;
        security.register_secure("csti")?;
        security.register_cors_enabled("csti")?;

        let routes = HostRoutesHandler::new(None)?;
        routes.add_path("app", DIST)?;
        self.shell
            .set_request_handler("csti", routes.as_request_handler())?;
        self.view.load_uri("csti://app/index.html")
    }
}
