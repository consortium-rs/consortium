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

//! The invocation envelope a guest sends across the bridge.
//!
//! `{ "id": u64, "cmd": String, "payload": <any JSON> }`.
//!
//! `id` and `cmd` both `#[serde(default)]`, so a malformed or partial envelope parses into
//! a dispatch that fails on lookup rather than failing to parse at all. That keeps the
//! failure reportable back to the guest with an id where one was supplied.
//!
//! `payload` stays a `serde_json::Value` and is never inspected here — the bridge hands it
//! to a handler as a string, and the handler owns its own type.

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Invocation {
    #[serde(default)]
    pub(super) id: u64,
    #[serde(default)]
    pub(super) cmd: String,
    pub(super) payload: serde_json::Value,
}

pub(super) fn parse_invocation(msg: &str) -> serde_json::Result<Invocation> {
    serde_json::from_str(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Payload {
        armed: bool,
        count: u8,
    }

    #[test]
    fn parse_invocation_deserializes_typed_payload() {
        let invocation = serde_json::from_str::<Invocation>(
            r#"{"id":7,"cmd":"echo","payload":{"armed":true,"count":3}}"#,
        )
        .unwrap();

        assert_eq!(invocation.id, 7);
        assert_eq!(invocation.cmd, "echo");
        assert_eq!(
            serde_json::from_value::<Payload>(invocation.payload).unwrap(),
            Payload {
                armed: true,
                count: 3
            }
        );
    }

    #[test]
    fn parse_invocation_defaults_missing_metadata() {
        let invocation = parse_invocation(r#"{"payload":null}"#).unwrap();

        assert_eq!(invocation.id, 0);
        assert_eq!(invocation.cmd, "");
        assert_eq!(invocation.payload, serde_json::Value::Null);
    }

    #[test]
    fn parse_invocation_rejects_invalid_json() {
        assert!(parse_invocation("not json").is_err());
    }

    #[test]
    fn invocation_serializes_typed_payload() {
        let invocation = Invocation {
            id: 9,
            cmd: "status".into(),
            payload: serde_json::json!({"armed": false, "count": 2}),
        };

        assert_eq!(
            serde_json::to_string(&invocation).unwrap(),
            r#"{"id":9,"cmd":"status","payload":{"armed":false,"count":2}}"#
        );
    }
}
