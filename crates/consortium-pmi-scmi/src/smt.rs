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

//! The SCMI Shared Memory Transport (SMT) channel.
//!
//! One channel is one window, laid out as the spec (and Linux's `shmem.c`)
//! define it. Every field is a little-endian `u32`:
//!
//! | Offset | Field            | Notes                                         |
//! | ------ | ---------------- | --------------------------------------------- |
//! | 0x00   | reserved         |                                               |
//! | 0x04   | `channel_status` | bit 0 free, bit 1 error                        |
//! | 0x08   | reserved (8 B)   |                                               |
//! | 0x10   | `flags`          | bit 0: platform raises a completion interrupt |
//! | 0x14   | `length`         | header + payload bytes                        |
//! | 0x18   | `msg_header`     | [`MessageHeader`](crate::MessageHeader)       |
//! | 0x1c   | `msg_payload`    | request parameters / status + return values   |
//!
//! Ownership passes by the free bit. The agent writes a request only into a free
//! channel, clears the bit to hand it over, and rings; the platform answers in
//! place and sets the bit again. A response's payload opens with the status
//! word, so its return values start at `0x20`.
//!
//! Every access is volatile, and the region's [`flush`](Region::flush) and
//! [`invalidate`](Region::invalidate) bracket each hand-over, so the channel
//! works in cached and uncached windows alike.

use consortium_core::Region;

/// Offset of `channel_status`.
pub const CHANNEL_STATUS: usize = 0x04;
/// Offset of `flags`.
pub const FLAGS: usize = 0x10;
/// Offset of `length`.
pub const LENGTH: usize = 0x14;
/// Offset of `msg_header`.
pub const MSG_HEADER: usize = 0x18;
/// Offset of `msg_payload`.
pub const MSG_PAYLOAD: usize = 0x1c;

/// `channel_status`: the agent may write the channel.
pub const STATUS_FREE: u32 = 1 << 0;
/// `channel_status`: the platform reported a channel error.
pub const STATUS_ERROR: u32 = 1 << 1;
/// `flags`: ask the platform to signal completion with an interrupt.
pub const FLAG_COMPLETION_IRQ: u32 = 1 << 0;

/// Smallest window that holds the layout plus a response's status word.
pub const MIN_LEN: usize = MSG_PAYLOAD + 4;

/// A region that cannot back an SMT channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtError {
    /// The window is shorter than [`MIN_LEN`].
    TooSmall {
        /// The window's length.
        len: usize,
    },
    /// The window is not 4-byte aligned.
    Misaligned,
}

/// A response read back from the channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Response {
    /// The header the platform wrote.
    pub header: u32,
    /// The raw status word.
    pub status: i32,
    /// Bytes of return values the platform wrote (after the status word).
    pub len: usize,
}

/// An SMT channel over a shared-memory window.
pub struct Smt<R> {
    region: R,
}

impl<R: Region> Smt<R> {
    /// Use `region` as an SMT channel.
    pub fn new(region: R) -> Result<Self, SmtError> {
        if region.len() < MIN_LEN {
            return Err(SmtError::TooSmall { len: region.len() });
        }
        if !(base(&region) as usize).is_multiple_of(4) {
            return Err(SmtError::Misaligned);
        }
        Ok(Self { region })
    }

    /// The window backing this channel.
    pub fn region(&self) -> &R {
        &self.region
    }

    /// Give the window back.
    pub fn into_region(self) -> R {
        self.region
    }

    /// Bytes of request payload the channel holds.
    pub fn payload_capacity(&self) -> usize {
        self.region.len() - MSG_PAYLOAD
    }

    /// The current `channel_status`, after invalidating the window.
    pub fn status(&self) -> u32 {
        self.region.invalidate();
        self.read_u32(CHANNEL_STATUS)
    }

    /// Write a request and hand the channel to the platform.
    ///
    /// The caller has checked the channel is free. `completion_irq` sets the
    /// flag asking the platform to ring back on completion. The window is
    /// flushed before returning, so the caller can ring at once.
    pub fn write_request(
        &mut self,
        header: u32,
        payload: &[u8],
        completion_irq: bool,
    ) -> Result<(), crate::ProtocolError> {
        let capacity = self.payload_capacity();
        if payload.len() > capacity {
            return Err(crate::ProtocolError::RequestTooLarge {
                len: payload.len(),
                capacity,
            });
        }
        // Same order as Linux: take the channel, then fill it.
        self.write_u32(CHANNEL_STATUS, 0);
        self.write_u32(
            FLAGS,
            if completion_irq {
                FLAG_COMPLETION_IRQ
            } else {
                0
            },
        );
        self.write_u32(LENGTH, 4 + payload.len() as u32);
        self.write_u32(MSG_HEADER, header);
        for (i, byte) in payload.iter().enumerate() {
            self.write_u8(MSG_PAYLOAD + i, *byte);
        }
        self.region.flush();
        Ok(())
    }

    /// Read the platform's response, copying up to `out.len()` return-value
    /// bytes into `out`.
    pub fn read_response(&self, out: &mut [u8]) -> Result<Response, crate::ProtocolError> {
        self.region.invalidate();
        let length = self.read_u32(LENGTH) as usize;
        // header (4) + status (4)
        if length < 8 {
            return Err(crate::ProtocolError::ShortResponse { len: 0 });
        }
        if length - 4 > self.payload_capacity() {
            return Err(crate::ProtocolError::OversizedResponse { len: length });
        }
        let len = length - 8;
        for (i, slot) in out.iter_mut().take(len).enumerate() {
            *slot = self.read_u8(MSG_PAYLOAD + 4 + i);
        }
        Ok(Response {
            header: self.read_u32(MSG_HEADER),
            status: self.read_u32(MSG_PAYLOAD) as i32,
            len,
        })
    }

    fn read_u32(&self, offset: usize) -> u32 {
        debug_assert!(offset + 4 <= self.region.len() && offset.is_multiple_of(4));
        let ptr = base(&self.region).wrapping_add(offset).cast::<u32>();
        // SAFETY: `offset` is a fixed, 4-aligned field offset below `MIN_LEN`
        // (checked in `new`) and the base is 4-aligned, so `ptr` is aligned and
        // inside the window `Region` guarantees valid for reads.
        u32::from_le(unsafe { ptr.read_volatile() })
    }

    fn write_u32(&mut self, offset: usize, value: u32) {
        debug_assert!(offset + 4 <= self.region.len() && offset.is_multiple_of(4));
        let ptr = base(&self.region).wrapping_add(offset).cast::<u32>();
        // SAFETY: as in `read_u32`; `Region` guarantees the window is valid for
        // writes, and `&mut self` makes this the only agent-side writer.
        unsafe { ptr.write_volatile(value.to_le()) }
    }

    fn read_u8(&self, offset: usize) -> u8 {
        assert!(offset < self.region.len());
        let ptr = base(&self.region).wrapping_add(offset);
        // SAFETY: `offset` is inside the window (asserted), which `Region`
        // guarantees valid for reads.
        unsafe { ptr.read_volatile() }
    }

    fn write_u8(&mut self, offset: usize, value: u8) {
        assert!(offset < self.region.len());
        let ptr = base(&self.region).wrapping_add(offset);
        // SAFETY: `offset` is inside the window (asserted), which `Region`
        // guarantees valid for writes; `&mut self` makes this the only writer.
        unsafe { ptr.write_volatile(value) }
    }
}

fn base<R: Region>(region: &R) -> *mut u8 {
    // SAFETY: `Region::as_ptr` only obliges the caller to stay within `len`
    // bytes, which every accessor above checks before dereferencing.
    unsafe { region.as_ptr() }
}
