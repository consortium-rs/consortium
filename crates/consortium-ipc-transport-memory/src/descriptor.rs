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
//! Offset  Size  Owner Field
//! 0x00    6     N/A   header (refer to `DescriptorHeader`)
//! 0x06    1     app   num_channels
//! 0x07    1     N/A   reserved
//! 0x08    1     app   tx_updated_id (0xFF is default and indicates no update)
//! 0x09    1     mcu   rx_updated_id (0xFF is default and indicates no update)
//! 0x0A    2     N/A   reserved
//! 0x0C    4     app   tx_transfer_size
//! 0x10    4     mcu   rx_transfer_size
//! 0x14    4     N/A   reserved
//! 0x18    4     a&m   state           (DescState)
//! 0x1C    4     N/A   reserved ; alignment padding
//! 0x20    12*N  N/A   channel table
//! ```
//!
//! Each channel table entry (12 bytes):
//! ```text
//! +0x00  4   offset      the byte offset of the channel's slot
//! +0x04  4   size        the total size of the channel's slot (must be ≥ SLOT_HEADER + MTU)
//! +0x08  1   direction   0 for Primary→Secondary, 1 for Secondary→Primary
//! +0x09  3   reserved
//! ```
#![allow(dead_code)]
use crate::transport::RegionSharedMemoryTransportable;
use crate::utils::{rv_u8, rv_u32, wv_u8, wv_u32};
use crate::{Error, SharedMemoryTransport};
use consortium_ipc::{Chan, Doorbell, Secondary};
use consortium_ipc::{Primary, Side};
use consortium_shared_memory::descriptor::{DescriptorHeader, DescriptorMagic};
use core::sync::atomic::Ordering;

pub const CHANNEL_NUM_MAX: usize = 32;

pub const MAGIC: DescriptorMagic = DescriptorMagic::new(b"CIPC");
pub const VERSION: u8 = 1;

const CHAN_TABLE_BASE: usize = 0x20;
const CHAN_ENTRY_SIZE: usize = 12;

const MAGIC_OFFSET: usize = 0x00;
const VERSION_OFFSET: usize = 0x04;
pub(crate) const NUM_CHANNELS_OFFSET: usize = 0x06;
const TX_UPDATED_ID_OFFSET: usize = 0x08;
pub const RX_UPDATED_ID_OFFSET: usize = 0x09;
const TX_TRANSFER_SIZE_OFFSET: usize = 0x0C;
const RX_TRANSFER_SIZE_OFFSET: usize = 0x10;
const STATE_OFFSET: usize = 0x18;
const CHANNEL_TABLE_OFFSET: usize = 0x20;
const DESCRIPTOR_LENGTH: usize = core::mem::size_of::<Descriptor>();

#[derive(Debug)]
#[repr(C)]
pub struct Descriptor {
    pub header: DescriptorHeader,
    pub num_channels: u8,
    _reserved1: [u8; 1],
    pub tx_updated_id: u8,
    pub rx_updated_id: u8,
    _reserved2: [u8; 2],
    pub tx_mtu: u32,
    pub rx_mtu: u32,
    _reserved3: [u8; 4],
    pub state: DescState,
    _reserved4: [u8; 4],
    pub channels: [ChannelLayout; CHANNEL_NUM_MAX],
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

impl Descriptor {
    /// Reads a `Descriptor` from the shared-memory region at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `CHANNEL_TABLE_OFFSET + num_channels * CHAN_ENTRY_SIZE`
    /// bytes and the memory must be DMA-coherent.
    pub unsafe fn from_raw_ptr(ptr: *const u8) -> Self {
        // SAFETY: caller guarantees the descriptor header is present at `ptr`.
        let header = unsafe { DescriptorHeader::from_raw_ptr(ptr) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let num_channels = unsafe { rv_u8(ptr, NUM_CHANNELS_OFFSET) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let tx_updated_id = unsafe { rv_u8(ptr, TX_UPDATED_ID_OFFSET) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let rx_updated_id = unsafe { rv_u8(ptr, RX_UPDATED_ID_OFFSET) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let tx_mtu = unsafe { rv_u32(ptr, TX_TRANSFER_SIZE_OFFSET) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let rx_mtu = unsafe { rv_u32(ptr, RX_TRANSFER_SIZE_OFFSET) };
        // SAFETY: caller guarantees the descriptor fixed header is valid and coherent.
        let state = DescState::from_primitive(unsafe { rv_u32(ptr, STATE_OFFSET) });

        let mut channels = [ChannelLayout::default(); CHANNEL_NUM_MAX];
        for (i, channel) in channels.iter_mut().enumerate().take(num_channels as usize) {
            // SAFETY: caller guarantees the channel table covers `num_channels` entries.
            let entry = unsafe { ptr.add(CHANNEL_TABLE_OFFSET + i * CHAN_ENTRY_SIZE) };
            // SAFETY: `entry` points at a valid channel-table entry by the invariant above.
            *channel = unsafe { ChannelLayout::from_raw_ptr(entry) };
        }

        Self {
            header,
            num_channels,
            _reserved1: [0; 1],
            tx_updated_id,
            rx_updated_id,
            _reserved2: [0; 2],
            tx_mtu,
            rx_mtu,
            _reserved3: [0; 4],
            state,
            _reserved4: [0; 4],
            channels,
        }
    }
}

impl ChannelLayout {
    /// Reads a `ChannelLayout` entry from the channel table at `ptr`.
    ///
    /// # Safety
    /// `ptr` must be valid for at least `CHAN_ENTRY_SIZE` (12) bytes.
    pub unsafe fn from_raw_ptr(ptr: *const u8) -> Self {
        // SAFETY: caller guarantees `ptr` covers the channel entry.
        let offset = unsafe { rv_u32(ptr, 0) };
        // SAFETY: caller guarantees `ptr` covers the channel entry.
        let size = unsafe { rv_u32(ptr, 4) };
        // SAFETY: caller guarantees `ptr` covers the channel entry.
        let direction = ChannelDirection::from_primitive(unsafe { rv_u8(ptr, 8) });

        Self {
            offset,
            size,
            direction,
            _reserved: [0; 3],
        }
    }
}

pub const fn channel_table_entry_offset(index: usize) -> usize {
    assert!(index < CHANNEL_NUM_MAX, "channel index out of range");

    CHANNEL_TABLE_OFFSET + index * CHAN_ENTRY_SIZE
}

pub const fn slot_offset(index: usize, mtu: usize) -> usize {
    assert!(index < CHANNEL_NUM_MAX, "channel index out of range");

    (DESCRIPTOR_LENGTH + (SLOT_HEADER + mtu) * index) as usize
}

/// # Safety
///
/// Caller must ensure the base and index is valid.
pub(crate) unsafe fn read_channel(base: *const u8, index: usize) -> ChannelLayout {
    let off = channel_table_entry_offset(index);
    // SAFETY: caller guarantees `base` covers the indexed channel entry.
    let offset = unsafe { rv_u32(base, off) };
    // SAFETY: caller guarantees `base` covers the indexed channel entry.
    let size = unsafe { rv_u32(base, off + 4) };
    // SAFETY: caller guarantees `base` covers the indexed channel entry.
    let direction = ChannelDirection::from_primitive(unsafe { rv_u8(base, off + 8) });

    ChannelLayout {
        offset,
        size,
        direction,
        _reserved: [0; 3],
    }
}

use core::sync::atomic::compiler_fence;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::error::DescriptorError;
use crate::slot_internal::SLOT_HEADER;

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

impl ChannelLayout {
    /// Returns a pointer to the slot data for this channel within the shared-memory region.
    ///
    /// # Safety
    /// `base` must be valid for at least `self.offset + self.size` bytes and the memory
    /// must be DMA-coherent.
    pub unsafe fn slot_addr(&self, base: *const u8) -> *mut u8 {
        // SAFETY: caller guarantees base is valid for at least self.offset + self.size bytes.
        unsafe { base.add(self.offset as usize) as *mut u8 }
    }
}

/// Absolute direction of a channel's slot, expressed in terms of the two
/// processor sides.
///
/// By convention the A-core (Linux) is [`Primary`] and the M-core (firmware) is
/// [`Secondary`], but nothing here is tied to a particular core: a direction is
/// resolved into TX/RX relative to a given [`Side`] by [`is_tx`]/[`is_rx`].
///
/// [`is_tx`]: ChannelDirection::is_tx
/// [`is_rx`]: ChannelDirection::is_rx
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Default)]
pub enum ChannelDirection {
    /// Slot written by [`Primary`] and read by [`Secondary`].
    PrimaryToSecondary = 0,
    /// Slot written by [`Secondary`] and read by [`Primary`].
    SecondaryToPrimary = 1,
    #[default]
    Unrecognized = 0xFF,
}

impl ChannelDirection {
    /// Returns `true` if this direction is a receive channel for side `S`.
    ///
    /// [`Primary`] receives what [`Secondary`] writes, and vice versa.
    pub fn is_rx<S: Side>(&self) -> bool {
        if S::INDEX == Primary::INDEX {
            matches!(self, ChannelDirection::SecondaryToPrimary)
        } else {
            matches!(self, ChannelDirection::PrimaryToSecondary)
        }
    }

    /// Returns `true` if this direction is a transmit channel for side `S`.
    ///
    /// [`Primary`] transmits to [`Secondary`], and vice versa.
    pub fn is_tx<S: Side>(&self) -> bool {
        if S::INDEX == Primary::INDEX {
            matches!(self, ChannelDirection::PrimaryToSecondary)
        } else {
            matches!(self, ChannelDirection::SecondaryToPrimary)
        }
    }
}

/// Read the descriptor state field.
///
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn get_state(base: *const u8) -> DescState {
    // SAFETY: caller guarantees the descriptor state field is valid and coherent.
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

impl<D: Doorbell, R: RegionSharedMemoryTransportable> SharedMemoryTransport<Primary, D, R> {
    /// Write the descriptor header and channel table to the shared memory region.
    ///
    /// # Arguments
    ///
    /// * `base` - pointer to the start of the shared DMA-coherent region
    /// * `transfer_size` - transfer size (written to offset 0x10)
    /// * `channels` - slice of channel descriptors; its length is the channel count
    ///
    /// # Safety
    ///
    /// Caller must ensure:
    /// - `base` is valid for at least `CHAN_TABLE_BASE + channels.len() * CHAN_ENTRY_SIZE` bytes
    /// - Memory is DMA-coherent and not accessed concurrently
    /// - All values in `channels` do not cause out-of-bounds offsets
    /// - Should set a timeout for the doorbell wait to avoid deadlock if the secondary core fails to respond.
    pub async unsafe fn descriptor_init(
        base: *mut u8,
        num_channels: u8,
        transfer_size: usize,
        doorbell: &D,
        doorbell_tx: Chan,
        doorbell_rx: Chan,
    ) -> Result<(), Error<D>> {
        // Write magic and version.
        let header = DescriptorHeader::new(MAGIC, VERSION);
        // SAFETY: caller guarantees `base` covers the descriptor header.
        unsafe { header.write_to_raw_ptr(base) };
        // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
        unsafe { wv_u8(base, NUM_CHANNELS_OFFSET, num_channels) };
        // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
        unsafe { wv_u32(base, RX_TRANSFER_SIZE_OFFSET, transfer_size as u32) };

        for i in 0..num_channels as usize {
            let off = channel_table_entry_offset(i);
            // SAFETY: caller guarantees the channel table covers all `channels` entries.
            unsafe { wv_u32(base, off, slot_offset(i, transfer_size) as u32) };
            // SAFETY: caller guarantees the channel table covers all `channels` entries.
            unsafe { wv_u32(base, off + 4, (SLOT_HEADER + transfer_size) as u32) };
            // SAFETY: caller guarantees the channel table covers all `channels` entries.
            unsafe {
                wv_u8(
                    base,
                    off + 8,
                    match i % 2 {
                        0 => ChannelDirection::PrimaryToSecondary.into(), // TX
                        1 => ChannelDirection::SecondaryToPrimary.into(), // RX
                        _ => unreachable!(),
                    },
                )
            };
        }

        // SAFETY: caller guarantees the descriptor state field is valid and coherent.
        unsafe { wv_u32(base, STATE_OFFSET, DescState::Ready as u32) };

        doorbell.ring(doorbell_tx).map_err(Error::Doorbell)?;
        doorbell.wait(doorbell_rx).await.map_err(Error::Doorbell)?;
        Ok(())
    }
}

impl<D: Doorbell, R: RegionSharedMemoryTransportable> SharedMemoryTransport<Secondary, D, R> {
    /// Parse the descriptor from the start of the mmap'd region.
    ///
    /// # Safety
    /// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes.
    /// All accesses use `read_volatile`; the region must be DMA-coherent.
    pub async unsafe fn descriptor_ack(
        base: *mut u8,
        num_channels: usize,
        transfer_size: usize,
        doorbell: &D,
        doorbell_rx: Chan,
        doorbell_tx: Chan,
    ) -> Result<Descriptor, super::Error<D>> {
        // SAFETY: caller guarantees the descriptor header is present at `base`.
        let header = unsafe { DescriptorHeader::from_raw_ptr(base) };

        header
            .check(MAGIC, VERSION)
            .map_err(|e| Error::Descriptor(DescriptorError::Header(e)))?;

        // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
        let num_ch = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };
        if num_ch as usize != num_channels || num_ch as usize > CHANNEL_NUM_MAX {
            return Err(DescriptorError::InvalidChannelCount {
                expected: num_channels,
                actual: num_ch as usize,
            }
            .into());
        }
        // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
        let rx_transfer_size = unsafe { rv_u32(base, RX_TRANSFER_SIZE_OFFSET) };
        // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid. We write back the (potentially adjusted) transfer size to ensure that both sides agree on the final value.
        unsafe {
            wv_u32(
                base,
                RX_TRANSFER_SIZE_OFFSET,
                rx_transfer_size.max(transfer_size as u32),
            )
        };
        // SAFETY: caller guarantees the descriptor state field is valid and coherent.
        let mut state = DescState::from_primitive(unsafe { rv_u32(base, STATE_OFFSET) });

        if state != DescState::Ready {
            doorbell
                .wait(doorbell_rx)
                .await
                .map_err(super::Error::Doorbell)?;
            state = DescState::from_primitive(unsafe { rv_u32(base, STATE_OFFSET) });
            if state != DescState::Ready {
                return Err(DescriptorError::NotReady(state).into());
            }
        }

        let mut channels = [ChannelLayout::default(); CHANNEL_NUM_MAX];

        for (i, channel) in channels.iter_mut().enumerate().take(num_ch as usize) {
            let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            // SAFETY: caller guarantees the channel table covers `num_ch` entries.
            let offset = unsafe { rv_u32(base, off) };
            // SAFETY: caller guarantees the channel table covers `num_ch` entries.
            let size = unsafe { rv_u32(base, off + 4) };
            // SAFETY: caller guarantees the channel table covers `num_ch` entries.
            let direction = ChannelDirection::from_primitive(unsafe { rv_u8(base, off + 8) });

            *channel = ChannelLayout {
                offset,
                size,
                direction,
                _reserved: [0; 3],
            };
            validate_channel_layout(i, num_ch as usize, *channel)?;
        }

        compiler_fence(Ordering::Release);

        // SAFETY: the caller must ensure that `base` is valid for at least 4 bytes and that the memory is DMA-coherent. Setting the state to Ready signals to the A-core that the descriptor is fully initialized and that it can read the rx_transfer_size and channel table entries safely.
        unsafe {
            wv_u32(base, STATE_OFFSET, DescState::Ready as u32);
        }

        doorbell.ring(doorbell_tx).map_err(Error::Doorbell)?;

        Ok(Descriptor {
            header,
            _reserved1: [0; 1],
            _reserved2: [0; 2],
            _reserved3: [0; 4],
            state: DescState::Ready,
            _reserved4: [0; 4],
            num_channels: num_ch,
            tx_updated_id: 0xFF,
            rx_updated_id: 0xFF,
            tx_mtu: transfer_size as u32,
            rx_mtu: rx_transfer_size,
            channels,
        })
    }
}

fn validate_channel_layout(
    index: usize,
    num_channels: usize,
    channel: ChannelLayout,
) -> Result<(), DescriptorError> {
    let table_end = CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE;
    if channel.offset as usize <= table_end {
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot overlaps descriptor header or channel table",
        });
    }

    if channel.size < SLOT_HEADER as u32 {
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot is smaller than the slot header",
        });
    }

    if matches!(channel.direction, ChannelDirection::Unrecognized) {
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "direction is unrecognized",
        });
    }

    Ok(())
}

/// Read the A-core updated max transfer size from the descriptor.
///
/// # Safety
///
/// `base` must be valid for at least `STATE_OFFSET + 4` bytes and DMA-coherent.
pub unsafe fn descriptor_complete(base: *const u8) -> u32 {
    // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid.
    let state = DescState::from_primitive(unsafe { rv_u32(base, STATE_OFFSET) });

    debug_assert!(
        state == DescState::Ready,
        "descriptor not in Ready state: {:?}",
        state
    );

    // SAFETY: when the base is valid and coherent, the state read will be consistent with the rx_transfer_size read. The A-core must have set the state to Ready before writing rx_transfer_size, so if we see Ready then rx_transfer_size is valid.
    unsafe { rv_u32(base, RX_TRANSFER_SIZE_OFFSET) }
}
