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

//! Mounting a [`Bridge`] into a WebKit web view.
//!
//! This is where the backend-neutral bridge meets a concrete guest runtime, and the
//! interesting part is the thread discipline.
//!
//! A JS prelude (`bridge_prelude.js`) is injected as a document-start user script and
//! registers a `bridge` script-message handler, giving page code a `postMessage`-shaped
//! call site. Invocations arrive on GLib's main context — WebKit's thread — but a handler's
//! [`ResponseFuture`](consortium_hmi::ResponseFuture) generally needs a Tokio runtime to
//! make progress.
//!
//! So the two are bridged explicitly: the future is spawned onto the Tokio handle the
//! caller supplies, its `(id, json)` result is sent back over an unbounded channel, and a
//! task on GLib's main context drains that channel and evaluates a resolve script in the
//! view. **Every WebKit call therefore happens on GLib's thread**, which is the invariant
//! WebKit requires and the reason `consortium-hmi` hands back a future rather than polling
//! one itself.
//!
//! Responses are resolved by correlation id, which is the guest's — the prelude keeps the
//! pending-promise map, so concurrent calls are the page's business rather than this
//! module's.

use cogcore::{UserScript, View, glib};
use consortium_hmi::Bridge;
use futures_channel::mpsc::unbounded;
use futures_util::StreamExt;

const JS_PRELUDE: &str = include_str!("bridge_prelude.js");

pub(super) fn attach(
    bridge: Bridge,
    runtime: tokio::runtime::Handle,
    view: &View,
) -> cogcore::Result<()> {
    let manager = view.user_content_manager().ok_or(cogcore::Error::Null(
        "webkit_web_view_get_user_content_manager",
    ))?;

    manager.add_script(&UserScript::new_at_document_start(JS_PRELUDE)?);
    manager.register_script_message_handler("bridge")?;

    let (responses, mut response_rx) = unbounded::<(u64, String)>();
    let response_view = view.clone();
    glib::MainContext::default().spawn_local(async move {
        while let Some((id, result)) = response_rx.next().await {
            let _ = response_view.run_js(&resolve_script(id, &result));
        }
    });

    manager.connect_script_message("bridge", move |message| {
        let Ok(dispatch) = bridge.dispatch(&message) else {
            return;
        };
        let responses = responses.clone();
        let task = runtime.spawn(async move {
            let result = dispatch.response.await;
            let _ = responses.unbounded_send((dispatch.id, result));
        });
        drop(task);
    })?;

    Ok(())
}

fn resolve_script(id: u64, result: &str) -> String {
    format!("window.__BRIDGE__._resolve({id},{result})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_script_preserves_raw_json() {
        assert_eq!(
            resolve_script(5, r#"{"ok":true}"#),
            r#"window.__BRIDGE__._resolve(5,{"ok":true})"#,
        );
    }
}
