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

mod dispatch;
mod handler;
mod invocation;
mod script;

use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
};

use cogcore::{UserScript, View, glib};
use futures_channel::mpsc::unbounded;
use futures_util::StreamExt;

use self::{
    dispatch::dispatch,
    handler::Handler,
    script::{JS_PRELUDE, resolve_script},
};

/// Tauri-like JS<->Rust command bridge over WebKit's script message channel.
///
/// Register handlers with [`Bridge::handle`], then call [`Bridge::attach`]
/// before the first page load. From JS:
///
/// ```js
/// const result = await window.__BRIDGE__.invoke("my_command", payload);
/// ```
///
/// Handlers receive the raw payload JSON string and must return a JSON value
/// string; that string is passed as-is to the Promise resolve callback.
pub struct Bridge {
    handlers: Arc<Mutex<HashMap<String, Handler>>>,
    runtime: tokio::runtime::Handle,
}

impl Bridge {
    /// Creates a bridge that spawns async handlers on the current Tokio runtime.
    ///
    /// # Panics
    ///
    /// Panics if called outside a Tokio runtime. Use [`Bridge::with_runtime`]
    /// when the HMI is initialized from non-async setup code.
    pub fn new() -> Self {
        Self::with_runtime(tokio::runtime::Handle::current())
    }

    /// Creates a bridge that spawns async handlers on `runtime`.
    pub fn with_runtime(runtime: tokio::runtime::Handle) -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            runtime,
        }
    }

    /// Registers a synchronous command handler.
    pub fn handle(
        &mut self,
        cmd: impl Into<String>,
        f: impl Fn(String) -> String + Send + 'static,
    ) {
        self.handlers
            .lock()
            .unwrap()
            .insert(cmd.into(), Handler::sync(f));
    }

    /// Registers an asynchronous command handler.
    pub fn handle_async<F, Fut>(&mut self, cmd: impl Into<String>, f: F)
    where
        F: Fn(String) -> Fut + Send + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        self.handlers
            .lock()
            .unwrap()
            .insert(cmd.into(), Handler::async_handler(f));
    }

    /// Wires the bridge into `view`.
    ///
    /// Injects the `__BRIDGE__` prelude script and connects the message signal.
    /// Must be called before the first page load.
    pub fn attach(self, view: &View) -> cogcore::Result<()> {
        let ucm = view.user_content_manager().ok_or(cogcore::Error::Null(
            "webkit_web_view_get_user_content_manager",
        ))?;

        ucm.add_script(&UserScript::new_at_document_start(JS_PRELUDE)?);
        ucm.register_script_message_handler("bridge")?;

        let handlers = Arc::clone(&self.handlers);
        let runtime = self.runtime.clone();
        let (responses, mut response_rx) = unbounded::<(u64, String)>();
        let view = view.clone();
        glib::MainContext::default().spawn_local(async move {
            while let Some((id, result)) = response_rx.next().await {
                let _ = view.run_js(&resolve_script(id, &result));
            }
        });

        ucm.connect_script_message("bridge", move |msg| {
            dispatch(&handlers, &runtime, responses.clone(), &msg);
        })?;

        Ok(())
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self::new()
    }
}
