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

//! Single-slot buffer layout within the DMA-coherent region.
//!
//! ```text
//! [0..4]        len     : u32   - valid payload bytes
//! [4..8]        flags   : u32   - FLAG_VALID | FLAG_CONSUMED | FLAG_INTEGRITY_ERROR
//!                                 | FLAG_ACK_REQUESTED
//! [8..12]       crc32   : u32   - integrity check
//! [12..12+mtu]  payload : [u8]
//! ```
//!
//! Write order: payload → len → flags (ensures receiver sees consistent data).
//! Read order:  check flags → read len → copy payload → write FLAG_CONSUMED.
//!
//! `FLAG_ACK_REQUESTED` is the sender's per-message request for an acknowledgement.
//! It rides in the same single flags store that publishes `FLAG_VALID`, so it adds no
//! ordering constraint, and the receiver reports it back through [`ReadOutcome`] so the
//! transport can ring the ack line for exactly the messages that asked for one.
use consortium_shared_memory::Region;

use crate::utils::{rv_u32, wv_u32};

pub const SLOT_HEADER: usize = 12;

pub const FLAG_VALID: u32 = 1 << 0;
pub const FLAG_CONSUMED: u32 = 1 << 1;
pub const FLAG_INTEGRITY_ERROR: u32 = 1 << 2;
/// Set by the sender when it will park waiting for the peer to drain this slot.
pub const FLAG_ACK_REQUESTED: u32 = 1 << 3;

/// What one [`try_read`] pass found in a slot.
///
/// Both non-delivering variants carry `ack_requested`, because a sender parked on an
/// acknowledgement must be woken even when its message did not survive the CRC check —
/// otherwise it waits for an edge that never comes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadOutcome {
    /// A message was copied into the caller's buffer and the slot marked consumed.
    Delivered { len: usize, ack_requested: bool },
    /// The payload failed its CRC check; the slot is flagged and no bytes are delivered.
    Corrupt { ack_requested: bool },
    /// The slot held nothing readable.
    Empty,
}

impl ReadOutcome {
    /// The delivered payload length, or `None` when nothing was delivered.
    pub const fn delivered(self) -> Option<usize> {
        match self {
            Self::Delivered { len, .. } => Some(len),
            Self::Corrupt { .. } | Self::Empty => None,
        }
    }

    /// Whether the message occupying the slot asked to be acknowledged.
    ///
    /// True on every outcome that drained a slot, delivered or corrupt; false when there
    /// was nothing there.
    pub const fn ack_requested(self) -> bool {
        match self {
            Self::Delivered { ack_requested, .. } | Self::Corrupt { ack_requested } => {
                ack_requested
            }
            Self::Empty => false,
        }
    }
}

/// Write `payload` into the slot and mark it VALID.
///
/// `ack` sets [`FLAG_ACK_REQUESTED`], asking the peer to signal once it has drained the
/// slot. It is published in the same store as `FLAG_VALID`.
///
/// # Safety
/// `base` must point to a slot of at least `SLOT_HEADER + payload.len()` bytes.
pub unsafe fn write(base: *mut u8, payload: &[u8], ack: bool) {
    // SAFETY: caller guarantees the slot header is valid.
    debug_assert!(unsafe { is_consumed(base) });

    // SAFETY: caller guarantees the slot contains room for `SLOT_HEADER + payload.len()`.
    let dst = unsafe { base.add(SLOT_HEADER) };
    // SAFETY: `dst` points to the slot payload and does not overlap `payload`.
    unsafe { core::ptr::copy_nonoverlapping(payload.as_ptr(), dst, payload.len()) };
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { wv_u32(base, 0, payload.len() as u32) };

    let crc32 = crc32fast::hash(payload);
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { wv_u32(base, 8, crc32) };

    let flags = if ack {
        FLAG_VALID | FLAG_ACK_REQUESTED
    } else {
        FLAG_VALID
    };
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { wv_u32(base, 4, flags) };
}

/// Copy the slot payload into `buf` and mark it CONSUMED.
///
/// See [`ReadOutcome`] for the three results; the caller owes the peer a doorbell ring
/// whenever the outcome reports `ack_requested`.
///
/// # Safety
/// `base` must point to a valid slot of at least `SLOT_HEADER + buf.len()` bytes.
pub unsafe fn try_read<R: Region>(base: *mut u8, buf: &mut [u8], region: &R) -> ReadOutcome {
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    let flags = unsafe { rv_u32(base, 4) };
    if flags & FLAG_VALID == 0 {
        return ReadOutcome::Empty;
    }

    // Captured before the flags word is overwritten below.
    let ack_requested = flags & FLAG_ACK_REQUESTED != 0;

    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    let len = unsafe { rv_u32(base, 0) } as usize;
    let n = len.min(buf.len());
    // SAFETY: caller guarantees the slot payload contains at least `n` readable bytes.
    let src = unsafe { base.add(SLOT_HEADER) };
    // SAFETY: `src` points to the slot payload and does not overlap `buf`.
    unsafe { core::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), n) };

    let crc_calculated = crc32fast::hash(&buf[..n]);
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    let crc_stored = unsafe { rv_u32(base, 8) };

    if crc_calculated != crc_stored {
        // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
        unsafe { wv_u32(base, 4, FLAG_INTEGRITY_ERROR) };
        region.flush();
        return ReadOutcome::Corrupt { ack_requested };
    }

    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { wv_u32(base, 4, FLAG_CONSUMED) };
    region.flush();
    ReadOutcome::Delivered {
        len: n,
        ack_requested,
    }
}

/// Read the slot's raw flags word.
///
/// The acknowledged-send path uses this to watch its own slot for `FLAG_CONSUMED` or
/// `FLAG_INTEGRITY_ERROR` without re-reading the payload.
///
/// # Safety
/// `base` must point to a valid slot.
pub unsafe fn flags(base: *const u8) -> u32 {
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { rv_u32(base, 4) }
}

/// True if the message currently occupying the slot asked for an acknowledgement.
///
/// # Safety
/// `base` must point to a valid slot.
pub unsafe fn ack_requested(base: *const u8) -> bool {
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    unsafe { flags(base) & FLAG_ACK_REQUESTED != 0 }
}

/// True if the slot is free for the sender to reuse (consumed or never written).
///
/// `FLAG_ACK_REQUESTED` needs no handling here: it is only ever written OR'd with
/// `FLAG_VALID`, so a slot carrying it correctly reads as not consumed.
///
/// # Safety
/// `base` must point to a valid slot.
pub unsafe fn is_consumed(base: *const u8) -> bool {
    // SAFETY: caller guarantees the slot header is valid and DMA-coherent.
    let flags = unsafe { rv_u32(base, 4) };
    flags == 0 || (flags & FLAG_CONSUMED != 0) || (flags & FLAG_INTEGRITY_ERROR != 0)
}
