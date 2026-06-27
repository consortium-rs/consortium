#![cfg_attr(not(feature = "std"), no_std)]

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

/// Doorbells commonly paired with the shared-memory transport.
pub mod doorbell;

mod descriptor;
mod error;
#[path = "slot.rs"]
mod slot_internal;
mod transport;
pub(crate) mod utils;

pub use consortium_ipc::{Primary, Secondary, Side};
pub use descriptor::{
    CHANNEL_NUM_MAX, ChannelDirection, ChannelLayout, DESCRIPTOR_LENGTH, DescState, Descriptor,
    MAGIC, RX_UPDATED_ID_OFFSET, VERSION, channel_table_entry_offset, descriptor_complete,
    get_state, set_state,
};

pub use error::{DescriptorError, Error};

/// Low-level slot layout operations over the DMA-coherent shared region.
pub mod slot {
    pub use super::slot_internal::{
        FLAG_CONSUMED, FLAG_INTEGRITY_ERROR, FLAG_VALID, SLOT_HEADER, is_consumed, try_read, write,
    };
}

pub use transport::{
    RecvAccess, RecvFut, RegionSharedMemoryTransportable, SendFut, SharedMemoryTransport,
    SharedMemoryTransportRx, SharedMemoryTransportTx,
};
