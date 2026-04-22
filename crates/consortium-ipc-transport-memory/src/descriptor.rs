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

//! Descriptor layout in the DMA-coherent shared region.
//!
//! ```text
//! Offset  Size  Owner   Field
//! 0x00    4     kernel  magic           0x4353_5449 ("CSTI" = Consortium) + endianness check
//! 0x04    1     kernel  version (compatibility check)
//! 0x05    1             reserved
//! 0x06    1     kernel  num_channels
//! 0x07    1             reserved
//! 0x08    1     kernel  tx_updated_id (0xFF is default and indicates no update)
//! 0x09    1     m-core  rx_updated_id (0xFF is default and indicates no update)
//! 0x0A    2             reserved
//! 0x0C    4     kernel  tx_transfer_size
//! 0x10    4     m-core  rx_transfer_size
//! 0x14    4             reserved
//! 0x18    4     both    state           (DescState)
//! 0x1C    4     kernel  reserved ; alignment padding
//! 0x20    12*N  kernel  channel table
//! ```
//!
//! Each channel table entry (12 bytes):
//! ```text
//! +0x00  4   offset      the byte offset of the channel's slot
//! +0x04  4   size        the total size of the channel's slot (must be ≥ SLOT_HEADER + MTU)
//! +0x08  1   direction   0 for A→M, 1 for M→A
//! +0x09  3   reserved
//! ```
#![allow(dead_code)]
use core::sync::atomic::Ordering;
use crate::utils::{rv_u32, rv_u8, wv_u32, wv_u8};

pub const CHANNEL_NUM_MAX: usize = 32;

pub const MAGIC: u32 = 0x4353_5449; // "CSTI" = Consortium
pub const VERSION: u8 = 1;

const CHAN_TABLE_BASE: usize = 0x20;
const CHAN_ENTRY_SIZE: usize = 12;

pub const MAGIC_OFFSET: usize = 0x00;
pub const VERSION_OFFSET: usize = 0x04;
pub const NUM_CHANNELS_OFFSET: usize = 0x05;
pub const TX_UPDATED_ID_OFFSET: usize = 0x08;
pub const RX_UPDATED_ID_OFFSET: usize = 0x09;
pub const TX_TRANSFER_SIZE_OFFSET: usize = 0x0C;
pub const RX_TRANSFER_SIZE_OFFSET: usize = 0x10;
pub const STATE_OFFSET: usize = 0x18;
pub const CHANNEL_TABLE_OFFSET: usize = 0x20;

pub const fn channel_table_entry_offset(index: usize) -> usize {
    assert!(index < CHANNEL_NUM_MAX, "channel index out of range");
    
    CHANNEL_TABLE_OFFSET + index * CHAN_ENTRY_SIZE
}

/// # Safety
/// 
/// Caller must ensure the base and index is valid.
pub(crate) unsafe fn read_channel(base: *const u8, index: usize) -> ChannelLayout {
    let off = channel_table_entry_offset(index);
    // SAFETY: Checked the base and index correct.
    unsafe {
        ChannelLayout {
            offset: rv_u32(base, off),
            size: rv_u32(base, off + 4),
            direction: ChannelDirection::from_primitive(rv_u8(base, off + 8)),
            _reserved: [0; 3],
        }
    }
}

use std::sync::atomic::compiler_fence;

use num_enum::{FromPrimitive, IntoPrimitive};

pub use crate::error::DescriptorError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
pub enum DescState {
    KernelInit,
    Proposed,
    MReady,
    Ready,
    Suspended,
    Error,
    #[num_enum(default)]
    Unrecognized = 0xFFFF_FFFF,
}

/// Channel descriptor written to the descriptor table.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct ChannelLayout {
    pub offset: u32,
    pub size: u32,
    pub direction: ChannelDirection,
    _reserved: [u8; 3],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Default)]
pub enum ChannelDirection {
    AToM = 0,
    MToA = 1,
    #[default]
    Unrecognized = 0xFF,
}

#[derive(Debug)]
#[repr(C)]
pub struct Descriptor {
    pub num_channels: u8,
    pub tx_updated_id: u8,
    pub rx_updated_id: u8,
    pub tx_mtu: u32,
    pub rx_mtu: u32,
    pub channels: [ChannelLayout; CHANNEL_NUM_MAX],
}

/// Read the descriptor state field.
///
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn get_state(base: *const u8) -> DescState {
    let val = unsafe { rv_u32(base, STATE_OFFSET) };
    DescState::from_primitive(val)
}

/// Set the descriptor state field.
///
/// # Safety
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn set_state(base: *mut u8, state: DescState) {
    // SAFETY: the caller must ensure that `base` is valid for at least 4 bytes and that the memory is DMA-coherent.
    unsafe {
        wv_u32(base, STATE_OFFSET, state as u32);
    }
}

/// Write the descriptor header and channel table to the shared memory region.
///
/// # Arguments
///
/// * `base` — pointer to the start of the shared DMA-coherent region
/// * `num_channels` — total number of channels in the table
/// * `transfer_size` — transfer size (written to offset 0x10)
/// * `channels` — slice of channel descriptors (must match `num_channels`)
///
/// # Safety
///
/// Caller must ensure:
/// - `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes
/// - Memory is DMA-coherent and not accessed concurrently
/// - All values in `channels` do not cause out-of-bounds offsets
pub unsafe fn descriptor_init(
    base: *mut u8,
    num_channels: u8,
    transfer_size: usize,
    channels: &[ChannelLayout],
) {
    debug_assert!(
        channels.len() == num_channels as usize,
        "channel count mismatch"
    );

    // Write magic and version
    // SAFETY: the caller must ensure that `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes and that the memory is DMA-coherent.
    unsafe {
        wv_u32(base, MAGIC_OFFSET, MAGIC);
        wv_u8(base, VERSION_OFFSET, VERSION);
        wv_u8(base, NUM_CHANNELS_OFFSET, num_channels);

        wv_u32(base, RX_TRANSFER_SIZE_OFFSET, transfer_size as u32);

        // Channel table
        for (i, ch) in channels.iter().enumerate() {
            let off = channel_table_entry_offset(i);
            wv_u32(base, off, ch.offset);
            wv_u32(base, off + 4, ch.size);
            wv_u8(base, off + 8, ch.direction as u8);
            // The remaining 3 bytes in the entry are reserved and set to 0 by default
        }

        wv_u32(base, STATE_OFFSET, DescState::MReady as u32);
    }
}

/// Parse the descriptor from the start of the mmap'd region.
///
/// # Safety
/// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * 16` bytes.
/// All accesses use `read_volatile`; the region must be DMA-coherent.
pub unsafe fn descriptor_ack(base: *mut u8, num_channels: usize, transfer_size: usize) -> Result<Descriptor, DescriptorError> {
    let magic = unsafe { rv_u32(base, 0x00) };
    let version = unsafe { rv_u8(base, 0x04) };
    let num_ch = unsafe { rv_u8(base, 0x05) };
    debug_assert!(
        num_ch as usize == num_channels,
        "num_channels mismatch: expected {}, got {}",
        num_channels,
        num_ch
    );
    let rx_transfer_size = unsafe { rv_u32(base, RX_TRANSFER_SIZE_OFFSET) };
    // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid. We write back the (potentially adjusted) transfer size to ensure that both sides agree on the final value.
    unsafe { wv_u32(base, RX_TRANSFER_SIZE_OFFSET, rx_transfer_size.max(transfer_size as u32)) };
    let state = DescState::from_primitive(unsafe { rv_u32(base, STATE_OFFSET) });

    if magic != MAGIC {
        return Err(DescriptorError::BadMagic { got: magic });
    }
    if version != VERSION {
        return Err(DescriptorError::BadVersion(version));
    }
    if state != DescState::MReady {
        return Err(DescriptorError::NotReady(state));
    }

    let mut channels = [ChannelLayout::default(); CHANNEL_NUM_MAX];

    for i in 0..num_ch as usize {
        let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
        // SAFETY: When the `base` is valid and coherent, the reads is consistent with the fields.
        unsafe {
            channels[i] = ChannelLayout {
                offset: rv_u32(base, off),
                size: rv_u32(base, off + 4),
                direction: ChannelDirection::from_primitive(rv_u8(base, off + 8)),
                _reserved: [0; 3],
            };
        }
    }

    compiler_fence(Ordering::Release);

    // SAFETY: the caller must ensure that `base` is valid for at least 4 bytes and that the memory is DMA-coherent. Setting the state to Ready signals to the A-core that the descriptor is fully initialized and that it can read the rx_transfer_size and channel table entries safely.
    unsafe {
        wv_u32(base, STATE_OFFSET, DescState::Ready as u32);
    }

    Ok(Descriptor {
        num_channels: num_ch,
        tx_updated_id: 0xFF,
        rx_updated_id: 0xFF,
        tx_mtu: transfer_size as u32,
        rx_mtu: rx_transfer_size,
        channels,
    })
}

/// Read the A-core updated max transfer size from the descriptor.
/// 
/// # Safety
/// 
/// `base` must be valid for at least 0x14 bytes and DMA-coherent.
pub unsafe fn descriptor_complete(base: *const u8) -> u32 {
    // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid.
    let state = DescState::from_primitive(unsafe { rv_u32(base, 0x14) });

    debug_assert!(
        state == DescState::Ready,
        "descriptor not in Ready state: {:?}",
        state
    );

    // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid.
    unsafe { rv_u32(base, RX_TRANSFER_SIZE_OFFSET) }
}
