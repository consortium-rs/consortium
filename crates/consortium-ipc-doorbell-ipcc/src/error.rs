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

#[derive(Debug, PartialEq, Eq)]
pub enum IpccError {
    /// Chan id is out of the valid 0..=15 range.
    ChanOutOfRange,
    /// Chan id is in the reserved range (0..=1) and cannot be used for user channels.
    ChanReserved,
    /// The outgoing channel is still occupied by the previous signal.
    ChannelOccupied,
    /// `IpccDoorbell` was instantiated for a processor id other than 1 or 2.
    InvalidProcessor,
}
