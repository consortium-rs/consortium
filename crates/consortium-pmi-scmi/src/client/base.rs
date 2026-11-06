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

//! The BASE protocol (0x10): discovery.

use consortium_core::{Doorbell, Region};

use super::{Client, PROTOCOL_VERSION, word};
use crate::{Error, ProtocolId};

/// `BASE_DISCOVER_VENDOR`.
pub const DISCOVER_VENDOR: u8 = 0x3;

/// A protocol version: major in the high half, minor in the low half.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Version(pub u32);

impl Version {
    /// The major revision.
    pub const fn major(self) -> u16 {
        (self.0 >> 16) as u16
    }

    /// The minor revision.
    pub const fn minor(self) -> u16 {
        self.0 as u16
    }
}

impl<R: Region, D: Doorbell> Client<R, D> {
    /// The version of `protocol` the platform implements.
    pub async fn protocol_version(
        &mut self,
        protocol: ProtocolId,
    ) -> Result<Version, Error<D::Error>> {
        let mut out = [0u8; 4];
        self.call_exact(protocol, PROTOCOL_VERSION, &[], &mut out)
            .await?;
        Ok(Version(word(&out, 0)))
    }

    /// The platform vendor's name, NUL-padded to 16 bytes.
    pub async fn discover_vendor(&mut self) -> Result<[u8; 16], Error<D::Error>> {
        let mut out = [0u8; 16];
        self.call_exact(ProtocolId::BASE, DISCOVER_VENDOR, &[], &mut out)
            .await?;
        Ok(out)
    }
}
