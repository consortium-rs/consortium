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

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: DeserializeOwned"))]
pub(super) struct Invocation<T>
where
    T: Serialize + DeserializeOwned,
{
    #[serde(default)]
    pub(super) id: u64,
    #[serde(default)]
    pub(super) cmd: String,
    pub(super) payload: T,
}

pub(super) fn parse_invocation<T>(msg: &str) -> Option<Invocation<T>>
where
    T: Serialize + DeserializeOwned,
{
    serde_json::from_str(msg).ok()
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
        let invocation = parse_invocation::<Payload>(
            r#"{"id":7,"cmd":"echo","payload":{"armed":true,"count":3}}"#,
        )
        .unwrap();

        assert_eq!(invocation.id, 7);
        assert_eq!(invocation.cmd, "echo");
        assert_eq!(
            invocation.payload,
            Payload {
                armed: true,
                count: 3
            }
        );
    }

    #[test]
    fn parse_invocation_defaults_missing_metadata() {
        let invocation = parse_invocation::<serde_json::Value>(r#"{"payload":null}"#).unwrap();

        assert_eq!(invocation.id, 0);
        assert_eq!(invocation.cmd, "");
        assert_eq!(invocation.payload, serde_json::Value::Null);
    }

    #[test]
    fn parse_invocation_rejects_invalid_json() {
        assert_eq!(parse_invocation::<serde_json::Value>("not json"), None);
    }

    #[test]
    fn invocation_serializes_typed_payload() {
        let invocation = Invocation {
            id: 9,
            cmd: "status".into(),
            payload: Payload {
                armed: false,
                count: 2,
            },
        };

        assert_eq!(
            serde_json::to_string(&invocation).unwrap(),
            r#"{"id":9,"cmd":"status","payload":{"armed":false,"count":2}}"#
        );
    }
}
