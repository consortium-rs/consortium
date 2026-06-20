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

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libcogcore::{UserScript, View};

type CommandFn = Box<dyn Fn(String) -> String + Send + 'static>;

const JS_PRELUDE: &str = r#"window.__IPC__ = (() => {
  let _seq = 0;
  const _pending = {};
  return {
    invoke(cmd, payload) {
      return new Promise(resolve => {
        const id = _seq++;
        _pending[id] = resolve;
        window.webkit.messageHandlers.ipc.postMessage(
          JSON.stringify({ id, cmd, payload: payload ?? null })
        );
      });
    },
    _resolve(id, result) {
      const cb = _pending[id];
      if (cb) { delete _pending[id]; cb(result); }
    },
  };
})();"#;

/// Tauri-like JS↔Rust command bridge over WebKit's script message channel.
///
/// Register handlers with [`IpcBridge::handle`], then call [`IpcBridge::attach`]
/// before the first page load. From JS:
///
/// ```js
/// const result = await window.__IPC__.invoke("my_command", payload);
/// ```
///
/// Handlers receive the raw payload JSON string and must return a JSON value
/// string; that string is passed as-is to the Promise resolve callback.
pub struct IpcBridge {
    handlers: Arc<Mutex<HashMap<String, CommandFn>>>,
}

impl IpcBridge {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a synchronous command handler.
    pub fn handle(
        &mut self,
        cmd: impl Into<String>,
        f: impl Fn(String) -> String + Send + 'static,
    ) {
        self.handlers.lock().unwrap().insert(cmd.into(), Box::new(f));
    }

    /// Wires the bridge into `view`.
    ///
    /// Injects the `__IPC__` prelude script and connects the message signal.
    /// Must be called before the first page load.
    pub fn attach(self, view: &View) -> libcogcore::Result<()> {
        let ucm = view
            .user_content_manager()
            .ok_or(libcogcore::Error::Null(
                "webkit_web_view_get_user_content_manager",
            ))?;

        ucm.add_script(&UserScript::new_at_document_start(JS_PRELUDE)?);
        ucm.register_script_message_handler("ipc")?;

        let handlers = Arc::clone(&self.handlers);
        let view = view.clone();

        ucm.connect_script_message("ipc", move |msg| {
            dispatch(&handlers, &view, &msg);
        })?;

        Ok(())
    }
}

impl Default for IpcBridge {
    fn default() -> Self {
        Self::new()
    }
}

fn dispatch(handlers: &Arc<Mutex<HashMap<String, CommandFn>>>, view: &View, msg: &str) {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(msg) else {
        return;
    };

    let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
    let cmd = val
        .get("cmd")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let payload = val
        .get("payload")
        .map(|v| v.to_string())
        .unwrap_or_default();

    let result = handlers.lock().unwrap().get(&cmd).map(|f| f(payload));

    if let Some(result) = result {
        let _ = view.run_js(&format!("window.__IPC__._resolve({id},{result})"));
    }
}
