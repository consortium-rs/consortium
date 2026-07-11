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

/// Errors raised by the IPCC doorbell.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpccError {
    /// Chan id is not a valid IPCC channel (`>= IPCC_CHAN_COUNT`).
    #[error("IPCC channel id out of range")]
    ChanOutOfRange,
    /// Chan id falls in the reserved channel range and cannot be used for user
    /// channels.
    #[error("IPCC channel id is reserved")]
    ChanReserved,
    /// The outgoing channel is still occupied by the previous signal.
    #[error("IPCC channel is still occupied by the previous signal")]
    ChannelOccupied,
}
