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

use futures_channel::mpsc::UnboundedSender;

use super::{handler::Handler, invocation::parse_invocation};

pub(super) fn dispatch(
    handlers: &Arc<Mutex<HashMap<String, Handler>>>,
    runtime: &tokio::runtime::Handle,
    responses: UnboundedSender<(u64, String)>,
    msg: &str,
) {
    let _ = dispatch_handler(handlers, runtime, responses, msg);
}

fn dispatch_handler(
    handlers: &Arc<Mutex<HashMap<String, Handler>>>,
    runtime: &tokio::runtime::Handle,
    responses: UnboundedSender<(u64, String)>,
    msg: &str,
) -> Option<()> {
    let invocation = parse_invocation::<serde_json::Value>(msg)?;
    let handlers = handlers.lock().unwrap();
    let handler = handlers.get(&invocation.cmd)?;

    match handler {
        Handler::Sync(f) => {
            let result = f(invocation.payload.to_string());
            let _ = responses.unbounded_send((invocation.id, result));
        }
        Handler::Async(f) => {
            let id = invocation.id;
            let fut = f(invocation.payload.to_string());
            runtime.spawn(async move {
                let result = fut.await;
                let _ = responses.unbounded_send((id, result));
            });
        }
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use futures_channel::mpsc::unbounded;
    use futures_util::StreamExt;

    use super::*;

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
    }

    fn command_handlers() -> Arc<Mutex<HashMap<String, Handler>>> {
        let mut handlers = HashMap::new();
        handlers.insert("echo".into(), Handler::sync(|payload| payload));
        handlers.insert("answer".into(), Handler::sync(|_| "42".into()));
        handlers.insert(
            "async_echo".into(),
            Handler::async_handler(|payload| async move { payload }),
        );
        Arc::new(Mutex::new(handlers))
    }

    #[test]
    fn dispatch_handler_runs_registered_command() {
        let runtime = runtime();
        let (responses, mut response_rx) = unbounded();
        let dispatched = dispatch_handler(
            &command_handlers(),
            runtime.handle(),
            responses,
            r#"{"id":11,"cmd":"echo","payload":["a","b"]}"#,
        );
        let response = response_rx.try_recv().unwrap();

        assert_eq!(dispatched, Some(()));
        assert_eq!(response, (11, r#"["a","b"]"#.into()));
    }

    #[test]
    fn dispatch_handler_spawns_async_command() {
        let runtime = runtime();
        let (responses, mut response_rx) = unbounded();
        let dispatched = dispatch_handler(
            &command_handlers(),
            runtime.handle(),
            responses,
            r#"{"id":12,"cmd":"async_echo","payload":{"ok":true}}"#,
        );

        assert_eq!(dispatched, Some(()));
        let response = runtime.block_on(async { response_rx.next().await });

        assert_eq!(response, Some((12, r#"{"ok":true}"#.into())));
    }

    #[test]
    fn dispatch_handler_ignores_unknown_or_invalid_commands() {
        let runtime = runtime();
        let handlers = command_handlers();
        let (responses, _response_rx) = unbounded();

        assert_eq!(
            dispatch_handler(
                &handlers,
                runtime.handle(),
                responses.clone(),
                r#"{"id":1,"cmd":"missing"}"#
            ),
            None
        );
        assert_eq!(
            dispatch_handler(&handlers, runtime.handle(), responses, "not json"),
            None
        );
    }
}
