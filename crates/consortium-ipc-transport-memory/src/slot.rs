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
//! [8..12]       crc32   : u32   - integrity check
//! [12..12+mtu]  payload : [u8]
//! ```
//!
//! Write order: payload → len → flags (ensures receiver sees consistent data).
//! Read order:  check flags → read len → copy payload → write FLAG_CONSUMED.
use crate::utils::{rv_u32, wv_u32};

pub const SLOT_HEADER: usize = 12;

pub const FLAG_VALID: u32 = 1 << 0;
pub const FLAG_CONSUMED: u32 = 1 << 1;
pub const FLAG_INTEGRITY_ERROR: u32 = 1 << 2;

/// Write `payload` into the slot and mark it VALID.
///
/// # Safety
/// `base` must point to a slot of at least `SLOT_HEADER + payload.len()` bytes.
pub unsafe fn write(base: *mut u8, payload: &[u8]) {
    unsafe {
        let dst = base.add(SLOT_HEADER);
        core::ptr::copy_nonoverlapping(payload.as_ptr(), dst, payload.len());
        wv_u32(base, 0, payload.len() as u32);

        let crc32 = crc32fast::hash(payload);
        wv_u32(base, 8, crc32);

        wv_u32(base, 4, FLAG_VALID);
    }
}

/// Copy the slot payload into `buf` and mark it CONSUMED.
/// Returns the number of bytes written, or `None` if the slot is not VALID.
///
/// # Safety
/// `base` must point to a valid slot of at least `SLOT_HEADER + buf.len()` bytes.
pub unsafe fn try_read(base: *mut u8, buf: &mut [u8]) -> Option<usize> {
    unsafe {
        let flags = rv_u32(base, 4);
        if flags & FLAG_VALID == 0 {
            return None;
        }

        let len = rv_u32(base, 0) as usize;
        let n = len.min(buf.len());
        core::ptr::copy_nonoverlapping(base.add(SLOT_HEADER), buf.as_mut_ptr(), n);

        let crc_calculated = crc32fast::hash(&buf[..n]);
        let crc_stored = rv_u32(base, 8);

        if crc_calculated != crc_stored {
            wv_u32(base, 4, FLAG_INTEGRITY_ERROR);
            return None;
        }

        wv_u32(base, 4, FLAG_CONSUMED);
        Some(n)
    }
}

/// True if the slot is free for the sender to reuse (consumed or never written).
///
/// # Safety
/// `base` must point to a valid slot.
pub unsafe fn is_consumed(base: *const u8) -> bool {
    unsafe {
        let flags = rv_u32(base, 4);
        flags == 0 || (flags & FLAG_CONSUMED != 0) || (flags & FLAG_INTEGRITY_ERROR != 0)
    }
}
