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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Compile-time environment variable carrying the OP-TEE TA UUID.
pub const TEE_UUID_ENV: &str = "CONSORTIUM_TEE_UUID";

/// Trusted Application metadata declared in `Consortium.toml` under `[tee]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct TeeConfig {
    /// UUID assigned to the OP-TEE Trusted Application.
    pub uuid: String,
}

impl TeeConfig {
    /// Environment variable used by TA build scripts.
    pub fn env_pair(&self) -> (&'static str, &str) {
        (TEE_UUID_ENV, self.uuid.as_str())
    }

    /// Validates that [`uuid`](Self::uuid) is a canonical `8-4-4-4-12` hex UUID.
    ///
    /// The OP-TEE compiler parses this string into a real UUID and bakes it into
    /// both the signed TA filename and the CA-side lookup, so a malformed value
    /// must be caught when the manifest is loaded rather than producing a
    /// mismatched or unsigned TA later. Hex digits of either case are accepted,
    /// matching the parsers on the build side.
    pub fn validate(&self) -> Result<(), String> {
        // Canonical form: five hyphen-separated hex groups of these lengths.
        const GROUPS: [usize; 5] = [8, 4, 4, 4, 12];

        let groups: Vec<&str> = self.uuid.split('-').collect();
        let well_formed = groups.len() == GROUPS.len()
            && groups.iter().zip(GROUPS).all(|(part, expected)| {
                part.len() == expected && part.bytes().all(|b| b.is_ascii_hexdigit())
            });

        if well_formed {
            Ok(())
        } else {
            Err(format!(
                "invalid OP-TEE TA UUID `{}`: expected canonical 8-4-4-4-12 hex form \
                 (e.g. 12345678-1234-1234-1234-123456789012)",
                self.uuid
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_pair_carries_the_uuid_under_the_shared_const() {
        // The key must equal `TEE_UUID_ENV` so the builder's OP-TEE compiler and
        // the TA build script read the same variable name.
        let cfg = TeeConfig {
            uuid: "12345678-1234-1234-1234-123456789012".to_string(),
        };
        assert_eq!(
            cfg.env_pair(),
            (TEE_UUID_ENV, "12345678-1234-1234-1234-123456789012")
        );
        assert_eq!(TEE_UUID_ENV, "CONSORTIUM_TEE_UUID");
    }

    fn cfg(uuid: &str) -> TeeConfig {
        TeeConfig {
            uuid: uuid.to_string(),
        }
    }

    #[test]
    fn validate_accepts_canonical_uuids() {
        assert!(
            cfg("12345678-1234-1234-1234-123456789012")
                .validate()
                .is_ok()
        );
        // Upper-case hex is accepted, matching the build-side UUID parser.
        assert!(
            cfg("ABCDEF01-2345-6789-ABCD-EF0123456789")
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn validate_rejects_malformed_uuids() {
        // Wrong group lengths, non-hex digits, missing/extra groups, and empty.
        for bad in [
            "",
            "not-a-uuid",
            "12345678-1234-1234-1234-1234567890", // last group too short
            "12345678-1234-1234-1234-1234567890123", // last group too long
            "12345678123412341234123456789012",   // no hyphens
            "g2345678-1234-1234-1234-123456789012", // non-hex digit
            "12345678-1234-1234-1234-123456789012-00", // extra group
        ] {
            let err = cfg(bad).validate().unwrap_err();
            assert!(err.contains(bad), "error should echo the bad value: {err}");
            assert!(
                err.contains("8-4-4-4-12"),
                "error should hint the format: {err}"
            );
        }
    }
}
