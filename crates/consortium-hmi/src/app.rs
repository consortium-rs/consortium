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

use cogcore::{Shell, View};

use crate::ipc::IpcBridge;

/// Minimal kiosk application wrapping a Cog shell and a fullscreen web view.
///
/// # Usage
///
/// ```no_run
/// use consortium_hmi::{KioskApp, IpcBridge};
///
/// let mut bridge = IpcBridge::new();
/// bridge.handle("ping", |_| r#""pong""#.into());
///
/// let app = KioskApp::new("my-kiosk", None, None).unwrap();
/// app.attach_ipc(bridge).unwrap();
/// app.view().load_uri("app://ui/index.html").unwrap();
/// ```
pub struct KioskApp {
    _shell: Shell,
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
        Ok(Self {
            _shell: shell,
            view,
        })
    }

    /// Returns a reference to the underlying view.
    pub fn view(&self) -> &View {
        &self.view
    }

    /// Attaches an [`IpcBridge`] to the view.
    ///
    /// Call this before [`View::load_uri`] so the prelude script is in place
    /// for the first page load.
    pub fn attach_ipc(&self, bridge: IpcBridge) -> cogcore::Result<()> {
        bridge.attach(&self.view)
    }
}
