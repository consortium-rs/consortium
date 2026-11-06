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

//! The POWER protocol (0x11), as [`PowerProvider`].

use consortium_core::{Doorbell, MaybeSend, Region};
use consortium_pmi::{PowerDomainId, PowerProvider, PowerState};

use super::{Client, word, words};
use crate::{Error, ProtocolId};

/// `POWER_STATE_SET`.
pub const STATE_SET: u8 = 0x4;
/// `POWER_STATE_GET`.
pub const STATE_GET: u8 = 0x5;

/// Generic "on": state type 0, id 0.
const STATE_ON: u32 = 0;
/// Generic "off": state type 1 (bit 30), id 0.
const STATE_OFF: u32 = 1 << 30;

impl<R, D> PowerProvider for Client<R, D>
where
    R: Region + MaybeSend,
    D: Doorbell + MaybeSend,
{
    type Error = Error<D::Error>;

    async fn set_state(&mut self, id: PowerDomainId, state: PowerState) -> Result<(), Self::Error> {
        let raw = match state {
            PowerState::On => STATE_ON,
            PowerState::Off => STATE_OFF,
            PowerState::Other(raw) => raw,
            #[allow(unreachable_patterns)]
            _ => unreachable!("unencoded PowerState variant"),
        };
        // flags (synchronous), domain, state
        let req = words::<12>(&[0, id.0, raw]);
        self.call(ProtocolId::POWER, STATE_SET, &req, &mut [])
            .await?;
        Ok(())
    }

    async fn state(&mut self, id: PowerDomainId) -> Result<PowerState, Self::Error> {
        let mut out = [0u8; 4];
        self.call_exact(ProtocolId::POWER, STATE_GET, &words::<4>(&[id.0]), &mut out)
            .await?;
        Ok(match word(&out, 0) {
            STATE_ON => PowerState::On,
            STATE_OFF => PowerState::Off,
            raw => PowerState::Other(raw),
        })
    }
}
