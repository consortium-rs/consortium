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

use cogcore::{UserScript, View};

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
        self.handlers
            .lock()
            .unwrap()
            .insert(cmd.into(), Box::new(f));
    }

    /// Wires the bridge into `view`.
    ///
    /// Injects the `__IPC__` prelude script and connects the message signal.
    /// Must be called before the first page load.
    pub fn attach(self, view: &View) -> cogcore::Result<()> {
        let ucm = view.user_content_manager().ok_or(cogcore::Error::Null(
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

#[derive(Debug, PartialEq, Eq)]
struct Invocation {
    id: u64,
    cmd: String,
    payload: String,
}

fn parse_invocation(msg: &str) -> Option<Invocation> {
    let val = serde_json::from_str::<serde_json::Value>(msg).ok()?;

    Some(Invocation {
        id: val.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
        cmd: val
            .get("cmd")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        payload: val
            .get("payload")
            .map(|v| v.to_string())
            .unwrap_or_default(),
    })
}

fn dispatch_handler(
    handlers: &Arc<Mutex<HashMap<String, CommandFn>>>,
    msg: &str,
) -> Option<(u64, String)> {
    let invocation = parse_invocation(msg)?;
    let result = handlers
        .lock()
        .unwrap()
        .get(&invocation.cmd)
        .map(|f| f(invocation.payload))?;

    Some((invocation.id, result))
}

fn resolve_script(id: u64, result: &str) -> String {
    format!("window.__IPC__._resolve({id},{result})")
}

fn dispatch(handlers: &Arc<Mutex<HashMap<String, CommandFn>>>, view: &View, msg: &str) {
    if let Some((id, result)) = dispatch_handler(handlers, msg) {
        let _ = view.run_js(&resolve_script(id, &result));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command_handlers() -> Arc<Mutex<HashMap<String, CommandFn>>> {
        let mut bridge = IpcBridge::new();
        bridge.handle("echo", |payload| payload);
        bridge.handle("answer", |_| "42".into());
        bridge.handlers
    }

    #[test]
    fn parse_invocation_preserves_json_payload() {
        let invocation =
            parse_invocation(r#"{"id":7,"cmd":"echo","payload":{"armed":true,"count":3}}"#)
                .unwrap();

        assert_eq!(invocation.id, 7);
        assert_eq!(invocation.cmd, "echo");
        assert_eq!(invocation.payload, r#"{"armed":true,"count":3}"#);
    }

    #[test]
    fn parse_invocation_defaults_missing_fields() {
        let invocation = parse_invocation(r#"{"payload":null}"#).unwrap();

        assert_eq!(invocation.id, 0);
        assert_eq!(invocation.cmd, "");
        assert_eq!(invocation.payload, "null");
    }

    #[test]
    fn parse_invocation_rejects_invalid_json() {
        assert_eq!(parse_invocation("not json"), None);
    }

    #[test]
    fn dispatch_handler_runs_registered_command() {
        let response = dispatch_handler(
            &command_handlers(),
            r#"{"id":11,"cmd":"echo","payload":["a","b"]}"#,
        );

        assert_eq!(response, Some((11, r#"["a","b"]"#.into())));
    }

    #[test]
    fn dispatch_handler_ignores_unknown_or_invalid_commands() {
        let handlers = command_handlers();

        assert_eq!(
            dispatch_handler(&handlers, r#"{"id":1,"cmd":"missing"}"#),
            None
        );
        assert_eq!(dispatch_handler(&handlers, "not json"), None);
    }

    #[test]
    fn resolve_script_inlines_json_result() {
        assert_eq!(
            resolve_script(5, r#"{"ok":true}"#),
            r#"window.__IPC__._resolve(5,{"ok":true})"#,
        );
    }
}
