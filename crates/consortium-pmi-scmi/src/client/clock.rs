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

//! The CLOCK protocol (0x14), as [`ClockProvider`].
//!
//! `CLOCK_CONFIG_SET` grew an OEM word in protocol version 3.0; the client asks
//! the platform's clock version once and sends the matching layout, as Linux's
//! `clock.c` does.

use consortium_core::{Doorbell, MaybeSend, Region};
use consortium_pmi::{ClockId, ClockProvider};

use super::{Client, word, words};
use crate::{Error, ProtocolId};

/// `CLOCK_RATE_SET`.
pub const RATE_SET: u8 = 0x5;
/// `CLOCK_RATE_GET`.
pub const RATE_GET: u8 = 0x6;
/// `CLOCK_CONFIG_SET`.
pub const CONFIG_SET: u8 = 0x7;

/// `CLOCK_CONFIG_SET` attributes: state `ENABLE` (bits 1:0).
const STATE_ENABLE: u32 = 1;
/// `CLOCK_CONFIG_SET` attributes: state `DISABLE`.
const STATE_DISABLE: u32 = 0;

impl<R: Region, D: Doorbell> Client<R, D> {
    async fn clock_config_set(&mut self, id: ClockId, state: u32) -> Result<(), Error<D::Error>> {
        let major = match self.clock_major {
            Some(major) => major,
            None => {
                let major = self.protocol_version(ProtocolId::CLOCK).await?.major();
                self.clock_major = Some(major);
                major
            }
        };
        if major >= 3 {
            // id, attributes, oem_config_val (no OEM type)
            let req = words::<12>(&[id.0, state, 0]);
            self.call(ProtocolId::CLOCK, CONFIG_SET, &req, &mut [])
                .await?;
        } else {
            let req = words::<8>(&[id.0, state]);
            self.call(ProtocolId::CLOCK, CONFIG_SET, &req, &mut [])
                .await?;
        }
        Ok(())
    }
}

impl<R, D> ClockProvider for Client<R, D>
where
    R: Region + MaybeSend,
    D: Doorbell + MaybeSend,
{
    type Error = Error<D::Error>;

    fn enable(&mut self, id: ClockId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.clock_config_set(id, STATE_ENABLE)
    }

    fn disable(
        &mut self,
        id: ClockId,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.clock_config_set(id, STATE_DISABLE)
    }

    async fn set_rate(&mut self, id: ClockId, hz: u64) -> Result<(), Self::Error> {
        // flags (synchronous, exact), id, rate low, rate high
        let req = words::<16>(&[0, id.0, hz as u32, (hz >> 32) as u32]);
        self.call(ProtocolId::CLOCK, RATE_SET, &req, &mut [])
            .await?;
        Ok(())
    }

    async fn rate(&mut self, id: ClockId) -> Result<u64, Self::Error> {
        let mut out = [0u8; 8];
        self.call_exact(ProtocolId::CLOCK, RATE_GET, &words::<4>(&[id.0]), &mut out)
            .await?;
        Ok(u64::from(word(&out, 0)) | (u64::from(word(&out, 1)) << 32))
    }
}
