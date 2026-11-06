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

//! SCMI response status codes.

/// The status word that opens every response payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status {
    /// `SUCCESS` (0).
    Success,
    /// `NOT_SUPPORTED` (-1).
    NotSupported,
    /// `INVALID_PARAMETERS` (-2).
    InvalidParameters,
    /// `DENIED` (-3): this agent lacks permission.
    Denied,
    /// `NOT_FOUND` (-4): no such resource.
    NotFound,
    /// `OUT_OF_RANGE` (-5).
    OutOfRange,
    /// `BUSY` (-6).
    Busy,
    /// `COMMS_ERROR` (-7).
    CommsError,
    /// `GENERIC_ERROR` (-8).
    GenericError,
    /// `HARDWARE_ERROR` (-9).
    HardwareError,
    /// `PROTOCOL_ERROR` (-10).
    ProtocolError,
    /// A code this crate does not name, carried raw.
    Unknown(i32),
}

impl Status {
    /// Decode a raw status word.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Success,
            -1 => Self::NotSupported,
            -2 => Self::InvalidParameters,
            -3 => Self::Denied,
            -4 => Self::NotFound,
            -5 => Self::OutOfRange,
            -6 => Self::Busy,
            -7 => Self::CommsError,
            -8 => Self::GenericError,
            -9 => Self::HardwareError,
            -10 => Self::ProtocolError,
            other => Self::Unknown(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_every_named_code() {
        let named = [
            Status::Success,
            Status::NotSupported,
            Status::InvalidParameters,
            Status::Denied,
            Status::NotFound,
            Status::OutOfRange,
            Status::Busy,
            Status::CommsError,
            Status::GenericError,
            Status::HardwareError,
            Status::ProtocolError,
        ];
        for (i, status) in named.into_iter().enumerate() {
            assert_eq!(Status::from_raw(-(i as i32)), status);
        }
    }

    #[test]
    fn keeps_unknown_codes() {
        assert_eq!(Status::from_raw(-11), Status::Unknown(-11));
        assert_eq!(Status::from_raw(7), Status::Unknown(7));
    }
}
