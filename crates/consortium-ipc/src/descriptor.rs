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
//! 0x00    4     kernel  magic           0x434F_4E53 ("CONS")
//! 0x04    1     kernel  version
//! 0x05    1     kernel  num_channels
//! 0x06    2     kernel  reserved
//! 0x08    4     kernel  proposed_mtu
//! 0x0C    4     m-side  accepted_mtu
//! 0x10    4     kernel  effective_mtu
//! 0x14    4     both    state           (DescState)
//! 0x18    8     kernel  reserved
//! 0x20    16*N  kernel  channel table
//! ```
//!
//! Each channel table entry (16 bytes):
//! ```text
//! +0x00  4  tx_offset   A→M slot byte offset in region
//! +0x04  4  tx_size     slot size (SLOT_HEADER + effective_mtu)
//! +0x08  4  rx_offset   M→A slot byte offset in region
//! +0x0C  4  rx_size
//! ```

pub const MAGIC: u32 = 0x434F_4E53;
pub const VERSION: u8 = 1;

const CHAN_TABLE_BASE: usize = 0x20;
const CHAN_ENTRY_SIZE: usize = 16;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::error::DescriptorError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
pub enum DescState {
    KernelInit = 0,
    Proposed = 1,
    MReady = 2,
    Ready = 3,
    Suspended = 4,
    Error = 5,
    #[num_enum(default)]
    Unrecognized = 0xFFFF_FFFF,
}

/// Channel descriptor written to the descriptor table.
#[derive(Clone, Copy, Debug)]
pub struct ChannelLayout {
    pub tx_offset: u32,
    pub tx_size: u32,
    pub rx_offset: u32,
    pub rx_size: u32,
}

#[derive(Debug)]
pub struct Descriptor {
    pub num_channels: u8,
    pub effective_mtu: usize,
    pub channels: Vec<ChannelLayout>,
}

/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
unsafe fn rv_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { (base.add(offset) as *const u32).read_volatile() }
}
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
unsafe fn rv_u8(base: *const u8, offset: usize) -> u8 {
    unsafe { base.add(offset).read_volatile() }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
unsafe fn wv_u32(base: *mut u8, offset: usize, value: u32) {
    // SAFETY: the caller must ensure that `base` is valid for at least `offset + 4` bytes and that the memory is DMA-coherent.
    unsafe {
        (base.add(offset) as *mut u32).write_volatile(value);
    }
}

/// # Safety
///
/// The caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
unsafe fn wv_u8(base: *mut u8, offset: usize, value: u8) {
    // SAFETY: the caller must ensure that `base` is valid for at least `offset + 1` bytes and that the memory is DMA-coherent.
    unsafe {
        base.add(offset).write_volatile(value);
    }
}

/// Read the descriptor state field.
///
/// # Safety
///
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn get_state(base: *const u8) -> DescState {
    let val = unsafe { rv_u32(base, 0x14) };
    DescState::from_primitive(val)
}

/// Set the descriptor state field.
///
/// # Safety
/// Caller must ensure `base` is valid for at least 4 bytes and DMA-coherent.
pub unsafe fn set_state(base: *mut u8, state: DescState) {
    // SAFETY: the caller must ensure that `base` is valid for at least 4 bytes and that the memory is DMA-coherent.
    unsafe {
        wv_u32(base, 0x14, state as u32);
    }
}

/// Parse the descriptor from the start of the mmap'd region.
///
/// # Safety
/// `base` must be valid for at least `CHAN_TABLE_BASE + num_channels * 16` bytes.
/// All accesses use `read_volatile`; the region must be DMA-coherent.
pub unsafe fn read_descriptor(base: *const u8) -> Result<Descriptor, DescriptorError> {
    let magic = unsafe { rv_u32(base, 0x00) };
    let version = unsafe { rv_u8(base, 0x04) };
    let num_ch = unsafe { rv_u8(base, 0x05) };
    let eff_mtu = unsafe { rv_u32(base, 0x10) } as usize;
    let state = DescState::from_primitive(unsafe { rv_u32(base, 0x14) });

    if magic != MAGIC {
        return Err(DescriptorError::BadMagic { got: magic });
    }
    if version != VERSION {
        return Err(DescriptorError::BadVersion(version));
    }
    if state != DescState::Ready {
        return Err(DescriptorError::NotReady(state));
    }

    let channels = (0..num_ch as usize)
        .map(|i| {
            let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            ChannelLayout {
                tx_offset: unsafe { rv_u32(base, off) },
                tx_size: unsafe { rv_u32(base, off + 4) },
                rx_offset: unsafe { rv_u32(base, off + 8) },
                rx_size: unsafe { rv_u32(base, off + 12) },
            }
        })
        .collect();

    Ok(Descriptor {
        num_channels: num_ch,
        effective_mtu: eff_mtu,
        channels,
    })
}

/// Write the descriptor header and channel table to the shared memory region.
///
/// # Arguments
///
/// * `base` — pointer to the start of the shared DMA-coherent region
/// * `num_channels` — total number of channels in the table
/// * `proposed_mtu` — MTU proposed by kernel/A-core (written to offset 0x08)
/// * `accepted_mtu` — MTU accepted by M-core (written to offset 0x0C)
/// * `channels` — slice of channel descriptors (must match `num_channels`)
///
/// # Safety
///
/// Caller must ensure:
/// - `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes
/// - Memory is DMA-coherent and not accessed concurrently
/// - All values in `channels` do not cause out-of-bounds offsets
pub unsafe fn write_descriptor(
    base: *mut u8,
    num_channels: u8,
    proposed_mtu: u32,
    accepted_mtu: u32,
    channels: &[ChannelLayout],
) {
    debug_assert!(
        channels.len() == num_channels as usize,
        "channel count mismatch"
    );

    // Write magic and version
    // SAFETY: the caller must ensure that `base` is valid for at least `CHAN_TABLE_BASE + num_channels * CHAN_ENTRY_SIZE` bytes and that the memory is DMA-coherent.
    unsafe {
        wv_u32(base, 0x00, MAGIC);
        wv_u8(base, 0x04, VERSION);
        wv_u8(base, 0x05, num_channels);

        // Write MTU fields
        wv_u32(base, 0x08, proposed_mtu);
        wv_u32(base, 0x0C, accepted_mtu);

        // Channel table
        for (i, ch) in channels.iter().enumerate() {
            let off = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            wv_u32(base, off, ch.tx_offset);
            wv_u32(base, off + 4, ch.tx_size);
            wv_u32(base, off + 8, ch.rx_offset);
            wv_u32(base, off + 12, ch.rx_size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal in-memory descriptor buffer for testing.
    fn make_desc(
        magic: u32,
        version: u8,
        num_channels: u8,
        effective_mtu: u32,
        state: u32,
        channels: &[(u32, u32, u32, u32)],
    ) -> Vec<u8> {
        let size = CHAN_TABLE_BASE + channels.len() * CHAN_ENTRY_SIZE;
        let mut buf = vec![0u8; size];

        buf[0x00..0x04].copy_from_slice(&magic.to_le_bytes());
        buf[0x04] = version;
        buf[0x05] = num_channels;
        buf[0x10..0x14].copy_from_slice(&effective_mtu.to_le_bytes());
        buf[0x14..0x18].copy_from_slice(&state.to_le_bytes());

        for (i, &(tx_off, tx_sz, rx_off, rx_sz)) in channels.iter().enumerate() {
            let base = CHAN_TABLE_BASE + i * CHAN_ENTRY_SIZE;
            buf[base..base + 4].copy_from_slice(&tx_off.to_le_bytes());
            buf[base + 4..base + 8].copy_from_slice(&tx_sz.to_le_bytes());
            buf[base + 8..base + 12].copy_from_slice(&rx_off.to_le_bytes());
            buf[base + 12..base + 16].copy_from_slice(&rx_sz.to_le_bytes());
        }
        buf
    }

    #[test]
    fn parse_valid_single_channel() {
        let buf = make_desc(
            MAGIC,
            VERSION,
            1,
            512,
            DescState::Ready as u32,
            &[(0x100, 0x208, 0x400, 0x208)],
        );
        let desc = unsafe { read_descriptor(buf.as_ptr()) }.unwrap();
        assert_eq!(desc.num_channels, 1);
        assert_eq!(desc.effective_mtu, 512);
        assert_eq!(desc.channels.len(), 1);
        assert_eq!(desc.channels[0].tx_offset, 0x100);
        assert_eq!(desc.channels[0].tx_size, 0x208);
        assert_eq!(desc.channels[0].rx_offset, 0x400);
        assert_eq!(desc.channels[0].rx_size, 0x208);
    }

    #[test]
    fn parse_valid_multiple_channels() {
        let channels = [(0x100, 0x108, 0x300, 0x108), (0x500, 0x108, 0x700, 0x108)];
        let buf = make_desc(MAGIC, VERSION, 2, 256, DescState::Ready as u32, &channels);
        let desc = unsafe { read_descriptor(buf.as_ptr()) }.unwrap();
        assert_eq!(desc.num_channels, 2);
        assert_eq!(desc.channels.len(), 2);
        assert_eq!(desc.channels[1].tx_offset, 0x500);
        assert_eq!(desc.channels[1].rx_offset, 0x700);
    }

    #[test]
    fn parse_zero_channels() {
        let buf = make_desc(MAGIC, VERSION, 0, 1024, DescState::Ready as u32, &[]);
        let desc = unsafe { read_descriptor(buf.as_ptr()) }.unwrap();
        assert_eq!(desc.num_channels, 0);
        assert!(desc.channels.is_empty());
        assert_eq!(desc.effective_mtu, 1024);
    }

    #[test]
    fn parse_bad_magic() {
        let buf = make_desc(0xDEAD_BEEF, VERSION, 0, 0, DescState::Ready as u32, &[]);
        let err = unsafe { read_descriptor(buf.as_ptr()) }.unwrap_err();
        assert!(matches!(
            err,
            DescriptorError::BadMagic { got: 0xDEAD_BEEF }
        ));
    }

    #[test]
    fn parse_bad_version() {
        let buf = make_desc(MAGIC, 2, 0, 0, DescState::Ready as u32, &[]);
        let err = unsafe { read_descriptor(buf.as_ptr()) }.unwrap_err();
        assert!(matches!(err, DescriptorError::BadVersion(2)));
    }

    #[test]
    fn parse_not_ready_rejects_all_non_ready_states() {
        for state in [
            DescState::KernelInit,
            DescState::Proposed,
            DescState::MReady,
            DescState::Suspended,
            DescState::Error,
        ] {
            let buf = make_desc(MAGIC, VERSION, 0, 0, state as u32, &[]);
            let err = unsafe { read_descriptor(buf.as_ptr()) }.unwrap_err();
            assert!(
                matches!(err, DescriptorError::NotReady(_)),
                "expected NotReady for {state:?}"
            );
        }
    }

    #[test]
    fn desc_state_from_u32_all_variants() {
        assert_eq!(DescState::from_primitive(0), DescState::KernelInit);
        assert_eq!(DescState::from_primitive(1), DescState::Proposed);
        assert_eq!(DescState::from_primitive(2), DescState::MReady);
        assert_eq!(DescState::from_primitive(3), DescState::Ready);
        assert_eq!(DescState::from_primitive(4), DescState::Suspended);
        assert_eq!(DescState::from_primitive(5), DescState::Error);
    }

    #[test]
    fn desc_state_unknown_maps_to_error() {
        assert_eq!(DescState::from_primitive(99), DescState::Error);
        assert_eq!(DescState::from_primitive(u32::MAX), DescState::Error);
    }
}
