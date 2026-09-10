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
//! 0xE0    32    N/A   cache-line padding
//! ```
//!
//! Each channel table entry (12 bytes):
//! ```text
//! +0x00  4   offset      the byte offset of the channel's slot
//! +0x04  4   size        the total size of the channel's slot (must be ≥ SLOT_HEADER + MTU)
//! +0x08  1   direction   0 for Primary→Secondary, 1 for Secondary→Primary
//! +0x09  1   flags       FLAG_CHAN_ACK_CAPABLE
//! +0x0A  2   reserved
//! ```
//!
//! The channel `flags` byte carries link *capabilities* agreed once during the handshake,
//! not per-message state — per-message state lives in the slot header. Today the only bit
//! is [`FLAG_CHAN_ACK_CAPABLE`], which says the two ends wired up acknowledgement doorbell
//! lines for this channel. Both sides declare it from their own manifest and
//! the secondary's half of the handshake rejects a disagreement, so a half-configured pair
//! fails at `connect` instead of hanging on the first acknowledged send.
#![allow(dead_code)]
use crate::transport::RegionSharedMemoryTransportable;
use crate::utils::{rv_u8, rv_u32, wv_u8, wv_u32};
use crate::{Error, SharedMemoryTransport};
use consortium_ipc::{Chan, Doorbell, Secondary};
use consortium_ipc::{Primary, Side};
use consortium_shared_memory::descriptor::{DescriptorHeader, DescriptorMagic};
use core::sync::atomic::Ordering;

pub const CHANNEL_NUM_MAX: usize = 16;

/// Channel-entry flag: acknowledgement doorbell lines are wired for this channel.
///
/// This is a capability, not a mode. It says `send_acked` is usable on the channel; it
/// does not make ordinary [`send`](consortium_ipc::SendTransport::send) acknowledge
/// anything. Whether an individual message wants an acknowledgement is recorded in that
/// message's slot header instead.
pub const FLAG_CHAN_ACK_CAPABLE: u8 = 1 << 0;

/// The descriptor geometry the two ends of a shared-memory link agree on during
/// the handshake.
///
/// This is [`Connect::Params`] for [`SharedMemoryTransport`]. It carries only
/// what is *negotiated* — everything identifying the link (channel ids, the
/// doorbell, the region and its base) is fixed when the transport is
/// constructed.
///
/// `mtu` is a proposal, not a verdict. The primary's value sizes the slots it
/// publishes; the secondary raises the descriptor's transfer-size field to its
/// own requirement if that is larger. Once `connect` returns, both sides report
/// the resulting *slot* capacity through
/// [`max_send_size`]/[`max_recv_size`] — which is what `send` enforces, and
/// which a secondary asking for more than the primary published does not
/// enlarge.
///
/// [`Connect::Params`]: consortium_ipc::Connect::Params
/// [`max_send_size`]: consortium_ipc::SendTransport::max_send_size
/// [`max_recv_size`]: consortium_ipc::RecvTransport::max_recv_size
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DescriptorLayout {
    /// Number of channel-table entries; a bidirectional transceiver uses two
    /// (slot 0 primary→secondary, slot 1 secondary→primary).
    pub num_channels: u8,
    /// Payload transfer size this side proposes, in bytes.
    pub mtu: usize,
    /// Whether this side wired acknowledgement doorbell lines for the link.
    ///
    /// Unlike `mtu`, this is not a proposal the peer may adjust: both ends must agree, and
    /// the secondary's half of the handshake rejects a mismatch rather than picking a
    /// winner.
    pub ack: bool,
}

impl DescriptorLayout {
    /// A layout of `num_channels` slots, each proposing an `mtu`-byte payload.
    ///
    /// Acknowledgement lines are off; add them with [`with_ack`](Self::with_ack).
    pub const fn new(num_channels: u8, mtu: usize) -> Self {
        Self {
            num_channels,
            mtu,
            ack: false,
        }
    }

    /// Declare whether this side wired acknowledgement doorbell lines.
    pub const fn with_ack(self, ack: bool) -> Self {
        Self { ack, ..self }
    }
}

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
pub const DESCRIPTOR_LENGTH: usize = core::mem::size_of::<Descriptor>();
/// Cache-line boundary used to isolate slots with different writers.
///
/// This is 64 bytes because it covers both the Cortex-M7's 32-byte data-cache
/// lines and the AArch64 maintenance granule used by `CachedRegion`.
pub const SLOT_ALIGNMENT: usize = 64;

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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(C)]
pub struct ChannelLayout {
    pub offset: u32,
    pub size: u32,
    pub direction: ChannelDirection,
    /// Link capabilities for this channel; see [`FLAG_CHAN_ACK_CAPABLE`].
    pub flags: u8,
    _reserved: [u8; 2],
}

impl ChannelLayout {
    /// Whether both ends wired acknowledgement doorbell lines for this channel.
    pub const fn ack_capable(&self) -> bool {
        self.flags & FLAG_CHAN_ACK_CAPABLE != 0
    }
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
        // SAFETY: caller guarantees `ptr` covers the channel entry.
        let flags = unsafe { rv_u8(ptr, 9) };

        Self {
            offset,
            size,
            direction,
            flags,
            _reserved: [0; 2],
        }
    }
}

pub const fn channel_table_entry_offset(index: usize) -> usize {
    assert!(index < CHANNEL_NUM_MAX, "channel index out of range");

    CHANNEL_TABLE_OFFSET + index * CHAN_ENTRY_SIZE
}

const fn align_slot_boundary(value: usize) -> usize {
    let remainder = value % SLOT_ALIGNMENT;
    if remainder == 0 {
        value
    } else {
        value + (SLOT_ALIGNMENT - remainder)
    }
}

/// Number of bytes reserved for one slot, including cache-line padding.
pub const fn slot_stride(mtu: usize) -> usize {
    align_slot_boundary(SLOT_HEADER + mtu)
}

/// Byte offset of slot `index` in the shared-memory window.
///
/// Both the descriptor-to-slot boundary and every inter-slot boundary are
/// cache-line aligned. This prevents cleaning a dirty line for one direction
/// from writing back stale bytes belonging to the peer-owned direction.
pub const fn slot_offset(index: usize, mtu: usize) -> usize {
    assert!(index < CHANNEL_NUM_MAX, "channel index out of range");

    align_slot_boundary(DESCRIPTOR_LENGTH) + slot_stride(mtu) * index
}

/// Minimum shared-window length containing `num_channels` complete slots.
pub const fn required_region_len(num_channels: usize, mtu: usize) -> usize {
    assert!(num_channels > 0, "at least one channel is required");
    assert!(
        num_channels <= CHANNEL_NUM_MAX,
        "channel count out of range"
    );

    slot_offset(num_channels - 1, mtu) + SLOT_HEADER + mtu
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
    // SAFETY: caller guarantees `base` covers the indexed channel entry.
    let flags = unsafe { rv_u8(base, off + 9) };

    ChannelLayout {
        offset,
        size,
        direction,
        flags,
        _reserved: [0; 2],
    }
}

use core::sync::atomic::compiler_fence;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::error::DescriptorError;
use crate::slot_internal::SLOT_HEADER;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

/// Write the descriptor header, geometry and channel table into the region.
///
/// This is the synchronous half of the primary's handshake: after it returns,
/// the descriptor is fully published and the secondary may observe it. The
/// caller still owes the peer a doorbell ring.
///
/// # Safety
///
/// Caller must ensure:
/// - `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes
/// - Memory is DMA-coherent and not accessed concurrently
/// - `num_channels` and `transfer_size` do not cause out-of-bounds slot offsets
pub(crate) unsafe fn publish_descriptor(base: *mut u8, layout: DescriptorLayout) {
    let DescriptorLayout {
        num_channels,
        mtu: transfer_size,
        ack,
    } = layout;
    let chan_flags = if ack { FLAG_CHAN_ACK_CAPABLE } else { 0 };
    // Write the descriptor body first; the header magic is published last so a
    // matching magic on the secondary implies the whole descriptor is ready.
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
        // SAFETY: caller guarantees the channel table covers all `channels` entries.
        unsafe { wv_u8(base, off + 9, chan_flags) };
    }

    // SAFETY: caller guarantees the descriptor state field is valid and coherent.
    unsafe { wv_u32(base, STATE_OFFSET, DescState::Ready as u32) };

    // Publish the magic last: the release fence ensures every body write above is
    // visible before the secondary can observe a matching magic and read them.
    compiler_fence(Ordering::Release);
    let header = DescriptorHeader::new(MAGIC, VERSION);
    // SAFETY: caller guarantees `base` covers the descriptor header.
    unsafe { header.write_to_raw_ptr(base) };
}

/// Has the primary published a descriptor whose magic and version match?
///
/// The readiness gate for the secondary: the primary writes the magic last, so
/// a match implies the body is complete.
///
/// # Safety
/// `base` must be valid for the descriptor header and DMA-coherent.
pub(crate) unsafe fn descriptor_published(base: *const u8) -> bool {
    // SAFETY: caller guarantees the descriptor header is present at `base`.
    let header = unsafe { DescriptorHeader::from_raw_ptr(base) };
    header.check(MAGIC, VERSION).is_ok()
}

/// Validate the published descriptor, agree the transfer size, and mark the
/// descriptor `Ready`.
///
/// This is the synchronous half of the secondary's handshake, run once the
/// descriptor is known to be published. The caller still owes the peer the ack
/// doorbell ring.
///
/// # Safety
/// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE`
/// bytes. All accesses use `read_volatile`; the region must be DMA-coherent.
pub(crate) unsafe fn accept_descriptor(
    base: *mut u8,
    layout: DescriptorLayout,
) -> Result<Descriptor, DescriptorError> {
    let DescriptorLayout {
        num_channels,
        mtu: transfer_size,
        ack,
    } = layout;
    let num_channels = num_channels as usize;
    // SAFETY: caller guarantees the descriptor header is present at `base`.
    let header = unsafe { DescriptorHeader::from_raw_ptr(base) };
    header.check(MAGIC, VERSION).map_err(|e| {
        consortium_log::trace!("accept_descriptor: header magic/version check failed");
        DescriptorError::Header(e)
    })?;

    // Acquire the body writes published before the magic on the primary side.
    compiler_fence(Ordering::Acquire);

    // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
    let num_ch = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };
    if num_ch as usize != num_channels || num_ch as usize > CHANNEL_NUM_MAX {
        consortium_log::trace!(
            "accept_descriptor: channel count mismatch (expected {}, got {})",
            num_channels,
            num_ch
        );
        return Err(DescriptorError::InvalidChannelCount {
            expected: num_channels,
            actual: num_ch as usize,
        });
    }
    // SAFETY: caller guarantees the fixed descriptor fields are valid and coherent.
    let rx_transfer_size = unsafe { rv_u32(base, RX_TRANSFER_SIZE_OFFSET) };
    // SAFETY: base is valid and coherent, and the matching magic gate above (with the
    // acquire fence) guarantees the primary finished writing rx_transfer_size before we
    // read it here. We write back the (potentially adjusted) transfer size so both sides
    // agree on the final value.
    unsafe {
        wv_u32(
            base,
            RX_TRANSFER_SIZE_OFFSET,
            rx_transfer_size.max(transfer_size as u32),
        )
    };

    let mut channels = [ChannelLayout::default(); CHANNEL_NUM_MAX];

    for (i, channel) in channels.iter_mut().enumerate().take(num_ch as usize) {
        let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
        // SAFETY: caller guarantees the channel table covers `num_ch` entries.
        let offset = unsafe { rv_u32(base, off) };
        // SAFETY: caller guarantees the channel table covers `num_ch` entries.
        let size = unsafe { rv_u32(base, off + 4) };
        // SAFETY: caller guarantees the channel table covers `num_ch` entries.
        let direction = ChannelDirection::from_primitive(unsafe { rv_u8(base, off + 8) });
        // SAFETY: caller guarantees the channel table covers `num_ch` entries.
        let flags = unsafe { rv_u8(base, off + 9) };

        *channel = ChannelLayout {
            offset,
            size,
            direction,
            flags,
            _reserved: [0; 2],
        };
        validate_channel_layout(i, num_ch as usize, rx_transfer_size as usize, *channel)?;

        // The acknowledgement capability is agreed, not negotiated: each side declares what
        // its own build wired up, and a disagreement means one end would ring a line the
        // other never listens on. Rejecting here turns that into a bring-up error instead of
        // a hang on the first acknowledged send.
        if channel.ack_capable() != ack {
            consortium_log::trace!(
                "accept_descriptor: channel {} ack capability mismatch (expected {}, got {})",
                i,
                ack,
                channel.ack_capable()
            );
            return Err(DescriptorError::AckMismatch {
                index: i,
                expected: ack,
                actual: channel.ack_capable(),
            });
        }
    }

    compiler_fence(Ordering::Release);

    // SAFETY: the caller must ensure that `base` is valid for at least 4 bytes and that the memory is DMA-coherent. Setting the state to Ready signals to the A-core that the descriptor is fully initialized and that it can read the rx_transfer_size and channel table entries safely.
    unsafe {
        wv_u32(base, STATE_OFFSET, DescState::Ready as u32);
    }

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

impl<D: Doorbell, R: RegionSharedMemoryTransportable> SharedMemoryTransport<Primary, D, R> {
    /// Write the descriptor header and channel table to the shared memory region.
    ///
    /// This is the low-level, transport-free form of the primary's handshake.
    /// Prefer [`Connect::connect`], which runs the same protocol against a
    /// constructed transport and records the negotiated MTU.
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
    ///
    /// [`Connect::connect`]: consortium_ipc::Connect::connect
    pub async unsafe fn descriptor_init(
        base: *mut u8,
        layout: DescriptorLayout,
        doorbell: &D,
        doorbell_tx: Chan,
        doorbell_rx: Chan,
    ) -> Result<(), Error<D>> {
        consortium_log::trace!(
            "descriptor_init: {} channels, transfer_size {}, ack {}",
            layout.num_channels,
            layout.mtu,
            layout.ack
        );
        // SAFETY: caller guarantees `base` covers the descriptor and channel table.
        unsafe { publish_descriptor(base, layout) };

        consortium_log::trace!("descriptor_init: ready, ringing and awaiting secondary ack");
        doorbell.ring(doorbell_tx).map_err(Error::Doorbell)?;
        doorbell.wait(doorbell_rx).await.map_err(Error::Doorbell)?;
        consortium_log::trace!("descriptor_init: handshake complete");
        Ok(())
    }
}

impl<D: Doorbell, R: RegionSharedMemoryTransportable> SharedMemoryTransport<Secondary, D, R> {
    /// Parse the descriptor from the start of the mmap'd region.
    ///
    /// This is the low-level, transport-free form of the secondary's handshake.
    /// Prefer [`Connect::connect`], which runs the same protocol against a
    /// constructed transport and records the negotiated MTU.
    ///
    /// # Safety
    /// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes.
    /// All accesses use `read_volatile`; the region must be DMA-coherent.
    ///
    /// [`Connect::connect`]: consortium_ipc::Connect::connect
    pub async unsafe fn descriptor_ack(
        base: *mut u8,
        layout: DescriptorLayout,
        doorbell: &D,
        doorbell_rx: Chan,
        doorbell_tx: Chan,
    ) -> Result<Descriptor, super::Error<D>> {
        consortium_log::trace!(
            "descriptor_ack: expecting {} channels, transfer_size {}, ack {}",
            layout.num_channels,
            layout.mtu,
            layout.ack
        );

        // The header magic is the readiness gate: the primary publishes it last, so a
        // matching magic implies the descriptor body is fully written and ready. If it
        // does not match yet, wait for the primary's doorbell and re-check; once it
        // matches, read and validate the descriptor directly.
        // SAFETY: caller guarantees the descriptor header is present at `base`.
        if !unsafe { descriptor_published(base) } {
            consortium_log::trace!(
                "descriptor_ack: descriptor not published, awaiting primary doorbell"
            );
            doorbell
                .wait(doorbell_rx)
                .await
                .map_err(super::Error::Doorbell)?;
        }

        // SAFETY: caller guarantees `base` covers the descriptor and channel table.
        let descriptor = unsafe { accept_descriptor(base, layout) }?;

        consortium_log::trace!(
            "descriptor_ack: {} channels validated, acking primary",
            descriptor.num_channels
        );
        doorbell.ring(doorbell_tx).map_err(Error::Doorbell)?;

        Ok(descriptor)
    }
}

fn validate_channel_layout(
    index: usize,
    num_channels: usize,
    transfer_size: usize,
    channel: ChannelLayout,
) -> Result<(), DescriptorError> {
    let table_end = CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE;
    if channel.offset as usize <= table_end {
        consortium_log::trace!(
            "channel {}: slot offset {} overlaps descriptor header/table",
            index,
            channel.offset
        );
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot overlaps descriptor header or channel table",
        });
    }

    if channel.size < SLOT_HEADER as u32 {
        consortium_log::trace!(
            "channel {}: slot size {} smaller than slot header",
            index,
            channel.size
        );
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot is smaller than the slot header",
        });
    }

    if channel.offset as usize != slot_offset(index, transfer_size) {
        consortium_log::trace!(
            "channel {}: slot offset {} does not match aligned layout",
            index,
            channel.offset
        );
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot offset does not match aligned layout",
        });
    }

    if channel.size as usize != SLOT_HEADER + transfer_size {
        consortium_log::trace!(
            "channel {}: slot size {} does not match transfer size {}",
            index,
            channel.size,
            transfer_size
        );
        return Err(DescriptorError::InvalidChannel {
            index,
            reason: "slot size does not match transfer size",
        });
    }

    if matches!(channel.direction, ChannelDirection::Unrecognized) {
        consortium_log::trace!("channel {}: unrecognized direction", index);
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
