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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CortexAppCore {
    CortexA7,
    CortexA9,
    CortexA53,
    CortexA35,
    CortexA55,
    CortexA72,
}

impl CortexAppCore {
    pub fn as_key(s: &str) -> Option<Self> {
        match s {
            "cortex-a7" => Some(Self::CortexA7),
            "cortex-a9" => Some(Self::CortexA9),
            "cortex-a53" => Some(Self::CortexA53),
            "cortex-a35" => Some(Self::CortexA35),
            "cortex-a55" => Some(Self::CortexA55),
            "cortex-a72" => Some(Self::CortexA72),
            _ => None,
        }
    }

    pub fn abbr(&self) -> &'static str {
        match self {
            Self::CortexA7 => "ca7",
            Self::CortexA9 => "ca9",
            Self::CortexA53 => "ca53",
            Self::CortexA35 => "ca35",
            Self::CortexA55 => "ca55",
            Self::CortexA72 => "ca72",
        }
    }

    pub fn from_abbr(s: &str) -> Option<Self> {
        match s {
            "ca7" => Some(Self::CortexA7),
            "ca9" => Some(Self::CortexA9),
            "ca53" => Some(Self::CortexA53),
            "ca35" => Some(Self::CortexA35),
            "ca55" => Some(Self::CortexA55),
            "ca72" => Some(Self::CortexA72),
            _ => None,
        }
    }

    pub fn to_rust_target(&self) -> &'static str {
        match self {
            // ARMv7-A (32-bit)
            Self::CortexA7 | Self::CortexA9 => "armv7-unknown-linux-gnueabihf",
            // ARMv8-A (64-bit)
            Self::CortexA53 | Self::CortexA35 | Self::CortexA55 | Self::CortexA72 => {
                "aarch64-unknown-linux-gnu"
            }
        }
    }
}
