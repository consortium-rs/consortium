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

mod handler;
mod invocation;

use std::{
    collections::HashMap,
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use self::{handler::Handler, invocation::parse_invocation};

/// A backend-independent command response.
///
/// Adapters decide where to poll this future and how to deliver its JSON value
/// to their guest runtime.
pub type ResponseFuture = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

/// One parsed invocation and the future producing its raw JSON response.
pub struct Dispatch {
    /// Guest-provided correlation identifier.
    pub id: u64,
    /// Future resolving to a JSON value string.
    pub response: ResponseFuture,
}

/// Failure to parse or route an invocation envelope.
#[derive(Debug)]
pub enum DispatchError {
    /// The message was not a valid invocation JSON object.
    InvalidInvocation(serde_json::Error),
    /// No handler was registered for the requested command.
    UnknownCommand(String),
}

impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInvocation(error) => write!(f, "invalid HMI invocation: {error}"),
            Self::UnknownCommand(command) => write!(f, "unknown HMI command: {command}"),
        }
    }
}

impl Error for DispatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidInvocation(error) => Some(error),
            Self::UnknownCommand(_) => None,
        }
    }
}

/// Registry of application services exposed to an HMI backend.
///
/// `Bridge` contains no runtime or renderer state. It is cheap to clone and
/// can be attached to WebKit, PocketJS, a headless test host, or another HMI
/// adapter. Handlers receive a normalized raw JSON payload and return a raw
/// JSON value string.
#[derive(Clone, Default)]
pub struct Bridge {
    handlers: Arc<Mutex<HashMap<String, Handler>>>,
}

impl Bridge {
    /// Creates an empty command registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a synchronous command handler.
    pub fn handle(
        &mut self,
        cmd: impl Into<String>,
        f: impl Fn(String) -> String + Send + Sync + 'static,
    ) {
        self.handlers
            .lock()
            .unwrap()
            .insert(cmd.into(), Handler::sync(f));
    }

    /// Registers an asynchronous command handler.
    pub fn handle_async<F, Fut>(&mut self, cmd: impl Into<String>, f: F)
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        self.handlers
            .lock()
            .unwrap()
            .insert(cmd.into(), Handler::async_handler(f));
    }

    /// Starts a command directly, returning `None` when it is not registered.
    ///
    /// Calling this method never executes user code. Both synchronous and
    /// asynchronous handlers begin when the returned future is polled by the
    /// backend.
    pub fn invoke(&self, cmd: &str, payload: impl Into<String>) -> Option<ResponseFuture> {
        let handler = self.handlers.lock().unwrap().get(cmd).cloned()?;
        Some(handler.call(payload.into()))
    }

    /// Parses a JSON invocation envelope and routes it to a handler.
    ///
    /// The accepted shape is `{ "id": number, "cmd": string, "payload":
    /// any-json-value }`. Backends use `id` only for response correlation.
    pub fn dispatch(&self, message: &str) -> Result<Dispatch, DispatchError> {
        let invocation = parse_invocation(message).map_err(DispatchError::InvalidInvocation)?;
        let response = self
            .invoke(&invocation.cmd, invocation.payload.to_string())
            .ok_or_else(|| DispatchError::UnknownCommand(invocation.cmd.clone()))?;
        Ok(Dispatch {
            id: invocation.id,
            response,
        })
    }

    /// Returns whether no commands are registered.
    pub fn is_empty(&self) -> bool {
        self.handlers.lock().unwrap().is_empty()
    }

    /// Returns the number of registered commands.
    pub fn len(&self) -> usize {
        self.handlers.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invokes_sync_handler_without_a_runtime_dependency() {
        let mut bridge = Bridge::new();
        bridge.handle("echo", |payload| payload);

        let response = bridge.invoke("echo", r#"{"ok":true}"#).unwrap();
        assert_eq!(pollster(response), r#"{"ok":true}"#);
    }

    #[test]
    fn dispatches_json_envelope() {
        let mut bridge = Bridge::new();
        bridge.handle_async("answer", |_| async { "42".into() });

        let dispatch = bridge
            .dispatch(r#"{"id":7,"cmd":"answer","payload":null}"#)
            .unwrap();
        assert_eq!(dispatch.id, 7);
        assert_eq!(pollster(dispatch.response), "42");
    }

    #[test]
    fn reports_invalid_and_unknown_invocations() {
        let bridge = Bridge::new();
        assert!(matches!(
            bridge.dispatch("not json"),
            Err(DispatchError::InvalidInvocation(_))
        ));
        assert!(matches!(
            bridge.dispatch(r#"{"id":1,"cmd":"missing","payload":null}"#),
            Err(DispatchError::UnknownCommand(command)) if command == "missing"
        ));
    }

    fn pollster(mut future: ResponseFuture) -> String {
        use std::task::{Context, Poll, Waker};

        let mut context = Context::from_waker(Waker::noop());
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly pending"),
        }
    }
}
