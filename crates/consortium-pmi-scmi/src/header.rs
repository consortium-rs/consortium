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

//! The SCMI message header.
//!
//! Every message opens with one little-endian `u32`:
//!
//! | Bits    | Field       |
//! | ------- | ----------- |
//! | 7:0     | message id  |
//! | 9:8     | message type |
//! | 17:10   | protocol id |
//! | 27:18   | token       |
//! | 31:28   | reserved    |

/// An SCMI protocol identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProtocolId(pub u8);

impl ProtocolId {
    /// Base protocol: discovery and version negotiation.
    pub const BASE: Self = Self(0x10);
    /// Power domain management.
    pub const POWER: Self = Self(0x11);
    /// System power management.
    pub const SYSTEM: Self = Self(0x12);
    /// Performance domain management.
    pub const PERF: Self = Self(0x13);
    /// Clock management.
    pub const CLOCK: Self = Self(0x14);
    /// Sensor management.
    pub const SENSOR: Self = Self(0x15);
    /// Reset domain management.
    pub const RESET: Self = Self(0x16);
    /// Voltage domain management.
    pub const VOLTAGE: Self = Self(0x17);
    /// Power capping.
    pub const POWERCAP: Self = Self(0x18);
    /// Pin control.
    pub const PINCTRL: Self = Self(0x19);
}

/// The kind of message a header introduces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MessageType {
    /// A command and its synchronous response.
    Command,
    /// The delayed response to an asynchronous command.
    DelayedResponse,
    /// A platform-initiated notification.
    Notification,
}

impl MessageType {
    const fn bits(self) -> u32 {
        match self {
            Self::Command => 0,
            Self::DelayedResponse => 2,
            Self::Notification => 3,
        }
    }
}

/// The largest token value; tokens wrap within `0..=TOKEN_MAX`.
pub const TOKEN_MAX: u16 = 0x3ff;

const ID_SHIFT: u32 = 0;
const TYPE_SHIFT: u32 = 8;
const PROTOCOL_SHIFT: u32 = 10;
const TOKEN_SHIFT: u32 = 18;

/// A packed SCMI message header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MessageHeader(u32);

impl MessageHeader {
    /// Pack a header. `token` is truncated to its 10 bits.
    pub const fn new(protocol: ProtocolId, message: u8, kind: MessageType, token: u16) -> Self {
        Self(
            ((message as u32) << ID_SHIFT)
                | (kind.bits() << TYPE_SHIFT)
                | ((protocol.0 as u32) << PROTOCOL_SHIFT)
                | (((token & TOKEN_MAX) as u32) << TOKEN_SHIFT),
        )
    }

    /// Wrap a raw header word as read from the channel.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// The raw header word.
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// The message id within its protocol.
    pub const fn message(self) -> u8 {
        (self.0 >> ID_SHIFT) as u8
    }

    /// The message type, or `None` for the reserved encoding `1`.
    pub const fn kind(self) -> Option<MessageType> {
        match (self.0 >> TYPE_SHIFT) & 0b11 {
            0 => Some(MessageType::Command),
            2 => Some(MessageType::DelayedResponse),
            3 => Some(MessageType::Notification),
            _ => None,
        }
    }

    /// The protocol id.
    pub const fn protocol(self) -> ProtocolId {
        ProtocolId((self.0 >> PROTOCOL_SHIFT) as u8)
    }

    /// The token the agent chose for this command.
    pub const fn token(self) -> u16 {
        ((self.0 >> TOKEN_SHIFT) as u16) & TOKEN_MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_each_field_at_its_bit_position() {
        let h = MessageHeader::new(ProtocolId::CLOCK, 0x07, MessageType::Command, 0x155);
        assert_eq!(h.raw(), 0x07 | (0x14 << 10) | (0x155 << 18));
        assert_eq!(h.message(), 0x07);
        assert_eq!(h.protocol(), ProtocolId::CLOCK);
        assert_eq!(h.kind(), Some(MessageType::Command));
        assert_eq!(h.token(), 0x155);
    }

    #[test]
    fn message_types_round_trip() {
        for kind in [
            MessageType::Command,
            MessageType::DelayedResponse,
            MessageType::Notification,
        ] {
            let h = MessageHeader::new(ProtocolId::BASE, 0, kind, 0);
            assert_eq!(h.kind(), Some(kind));
        }
        assert_eq!(MessageHeader::from_raw(1 << 8).kind(), None);
    }

    #[test]
    fn token_is_truncated_to_ten_bits() {
        let h = MessageHeader::new(ProtocolId::BASE, 0, MessageType::Command, 0xffff);
        assert_eq!(h.token(), TOKEN_MAX);
        assert_eq!(h.raw() >> 28, 0, "reserved bits stay clear");
    }
}
