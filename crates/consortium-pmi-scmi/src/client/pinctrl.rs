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

//! The PINCTRL protocol (0x19), as [`PinctrlProvider`].
//!
//! Both operations are `PINCTRL_SETTINGS_CONFIGURE` on a single pin (selector
//! type `PIN`): a function select sets attribute bit 10 and carries no
//! configurations; a configure sends `(type, value)` pairs with the function id
//! `0xFFFF_FFFF` ("unchanged"), split across as many messages as the channel
//! needs.

use consortium_core::{Doorbell, MaybeSend, Region};
use consortium_pmi::{FunctionId, PinConfig, PinId, PinctrlProvider};

use super::{Client, words};
use crate::{Error, ProtocolId};

/// `PINCTRL_SETTINGS_CONFIGURE`.
pub const SETTINGS_CONFIGURE: u8 = 0x6;

/// Selector type `PIN` (attributes bits 1:0).
const SELECTOR_PIN: u32 = 0;
/// Attributes bit 10: the function id is valid and should be applied.
const FUNCTION_VALID: u32 = 1 << 10;
/// Function id meaning "leave the function as it is".
const FUNCTION_UNCHANGED: u32 = 0xffff_ffff;
/// identifier, function id, attributes.
const FIXED_WORDS: usize = 3;
/// Attributes bits 9:2 carry the configuration count.
const MAX_CONFIGS_PER_MESSAGE: usize = 0xff;

/// The SCMI `(type, value)` pair for one setting, per the spec's configuration
/// type table (Linux `enum scmi_pinctrl_conf_type`). Flag-like settings send `1`.
fn encode(config: PinConfig) -> (u32, u32) {
    match config {
        PinConfig::BiasDisable => (2, 1),
        PinConfig::PullUp => (4, 1),
        PinConfig::PullDown => (6, 1),
        PinConfig::OpenDrain => (7, 1),
        PinConfig::PushPull => (9, 1),
        PinConfig::DriveStrength(v) => (10, v),
        PinConfig::InputSchmitt(on) => (15, u32::from(on)),
        PinConfig::SlewRate(v) => (20, v),
        PinConfig::Vendor { kind, value } => (u32::from(kind), value),
        // `PinConfig` is `#[non_exhaustive]`; a new variant needs an encoding here.
        #[allow(unreachable_patterns)]
        _ => unreachable!("unencoded PinConfig variant"),
    }
}

impl<R, D> PinctrlProvider for Client<R, D>
where
    R: Region + MaybeSend,
    D: Doorbell + MaybeSend,
{
    type Error = Error<D::Error>;

    async fn set_function(&mut self, pin: PinId, function: FunctionId) -> Result<(), Self::Error> {
        let req = words::<12>(&[pin.0, function.0, SELECTOR_PIN | FUNCTION_VALID]);
        self.call(ProtocolId::PINCTRL, SETTINGS_CONFIGURE, &req, &mut [])
            .await?;
        Ok(())
    }

    async fn set_config(&mut self, pin: PinId, configs: &[PinConfig]) -> Result<(), Self::Error> {
        let room = (self.payload_capacity() / 4).saturating_sub(FIXED_WORDS) / 2;
        let per_message = room.clamp(1, MAX_CONFIGS_PER_MESSAGE);
        // Largest message: the fixed words plus `MAX_CONFIGS_PER_MESSAGE` pairs.
        let mut buf = [0u8; (FIXED_WORDS + 2 * MAX_CONFIGS_PER_MESSAGE) * 4];
        for chunk in configs.chunks(per_message) {
            let attributes = SELECTOR_PIN | ((chunk.len() as u32) << 2);
            let fixed = words::<12>(&[pin.0, FUNCTION_UNCHANGED, attributes]);
            buf[..12].copy_from_slice(&fixed);
            for (i, config) in chunk.iter().enumerate() {
                let (kind, value) = encode(*config);
                let at = 12 + i * 8;
                buf[at..at + 4].copy_from_slice(&kind.to_le_bytes());
                buf[at + 4..at + 8].copy_from_slice(&value.to_le_bytes());
            }
            let len = 12 + chunk.len() * 8;
            self.call(
                ProtocolId::PINCTRL,
                SETTINGS_CONFIGURE,
                &buf[..len],
                &mut [],
            )
            .await?;
        }
        Ok(())
    }
}
