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

//! The `CUART` wire frame: COBS-delimited, CRC-protected message framing.
//!
//! A UART is a raw byte stream with no message boundaries, so the transport
//! frames each payload as:
//!
//! ```text
//! COBS( chan: u8 | payload[0..=MTU] | crc32: u32 LE )  0x00
//! ```
//!
//! - `chan` is the logical channel byte (the [`Chan`](consortium_ipc::Chan)
//!   contract for serial transports).
//! - `crc32` (crc32fast, IEEE) covers `chan | payload`.
//! - COBS encoding removes every `0x00` from the frame body, so the trailing
//!   `0x00` delimiter lets a receiver resynchronize after line noise or a
//!   mid-frame peer reboot by hunting for the next zero byte.

/// Bytes of framing around the payload inside the COBS region: the channel
/// byte plus the CRC32 trailer.
pub const FRAME_OVERHEAD: usize = 1 + 4;

/// Length of the un-encoded frame body (`chan | payload | crc32`) for a
/// payload of `mtu` bytes.
pub const fn inner_len(mtu: usize) -> usize {
    mtu + FRAME_OVERHEAD
}

/// Maximum encoded length of one frame on the wire, including the trailing
/// `0x00` delimiter. COBS overhead is one byte per 254 encoded, plus one.
pub const fn max_frame_len(mtu: usize) -> usize {
    let inner = inner_len(mtu);
    inner + inner / 254 + 1 + 1
}

/// Required size of the scratch buffer passed to
/// [`UartTransportTx::new`](crate::UartTransportTx::new): staging space for
/// the frame body plus the encoded frame.
pub const fn tx_buf_len(mtu: usize) -> usize {
    inner_len(mtu) + max_frame_len(mtu)
}

/// Size of the read-chunk region at the front of the receive buffer.
pub(crate) const RX_CHUNK: usize = 64;

/// Required size of the buffer passed to
/// [`UartTransportRx::new`](crate::UartTransportRx::new): a read-chunk region
/// plus an accumulator able to hold one maximally encoded frame (without its
/// delimiter).
pub const fn rx_buf_len(mtu: usize) -> usize {
    RX_CHUNK + max_frame_len(mtu) - 1
}

/// Encode one frame.
///
/// `inner` must hold at least [`inner_len`]`(payload.len())` bytes and `out`
/// at least [`max_frame_len`]`(payload.len())` bytes; both are checked by the
/// transport constructors against the MTU.
///
/// Returns the number of encoded bytes written to `out`, including the
/// trailing delimiter.
pub(crate) fn encode_frame(chan: u8, payload: &[u8], inner: &mut [u8], out: &mut [u8]) -> usize {
    let body_len = payload.len() + FRAME_OVERHEAD;
    inner[0] = chan;
    inner[1..1 + payload.len()].copy_from_slice(payload);
    let crc = crc(chan, payload);
    inner[1 + payload.len()..body_len].copy_from_slice(&crc.to_le_bytes());

    let encoded = cobs::encode(&inner[..body_len], out);
    out[encoded] = 0x00;
    encoded + 1
}

fn crc(chan: u8, payload: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&[chan]);
    hasher.update(payload);
    hasher.finalize()
}

/// Outcome of feeding one received byte to [`Deframer::push`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Push {
    /// Byte consumed; frame still in progress.
    Consumed,
    /// A delimiter closed a frame; `acc[..len]` holds the encoded body.
    Frame { len: usize },
    /// A delimiter closed a frame that had overflowed or was empty; nothing
    /// to decode.
    Skipped,
}

/// Incremental frame boundary scanner.
///
/// Owns only indices; the accumulator storage is the transport's caller
/// provided buffer, passed to every call so the deframer itself stays
/// trivially movable.
#[derive(Debug, Default)]
pub(crate) struct Deframer {
    len: usize,
    overflow: bool,
}

impl Deframer {
    /// Feed one wire byte, accumulating into `acc`.
    pub(crate) fn push(&mut self, acc: &mut [u8], byte: u8) -> Push {
        if byte == 0x00 {
            let len = self.len;
            let overflow = self.overflow;
            self.len = 0;
            self.overflow = false;
            if overflow || len == 0 {
                return Push::Skipped;
            }
            return Push::Frame { len };
        }
        if self.overflow {
            return Push::Consumed;
        }
        if self.len == acc.len() {
            // Frame exceeds the configured MTU: poison it and discard bytes
            // until the next delimiter resynchronizes the stream.
            self.overflow = true;
            return Push::Consumed;
        }
        acc[self.len] = byte;
        self.len += 1;
        Push::Consumed
    }
}

/// Why a completed frame was dropped instead of delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FrameError {
    Cobs,
    TooShort,
    Crc,
    ForeignChan { chan: u8 },
}

/// Decode a completed frame body in place and validate it for `chan`.
///
/// On success returns the payload length; the payload occupies
/// `acc[1..1 + len]`.
pub(crate) fn decode_frame(acc: &mut [u8], chan: u8) -> Result<usize, FrameError> {
    let inner = cobs::decode_in_place(acc).map_err(|_| FrameError::Cobs)?;
    if inner < FRAME_OVERHEAD {
        return Err(FrameError::TooShort);
    }
    let payload_len = inner - FRAME_OVERHEAD;
    let mut expected = [0u8; 4];
    expected.copy_from_slice(&acc[1 + payload_len..inner]);
    if crc(acc[0], &acc[1..1 + payload_len]) != u32::from_le_bytes(expected) {
        return Err(FrameError::Crc);
    }
    if acc[0] != chan {
        return Err(FrameError::ForeignChan { chan: acc[0] });
    }
    Ok(payload_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MTU: usize = 64;
    const CHAN: u8 = 3;

    fn encode(chan: u8, payload: &[u8], out: &mut [u8]) -> usize {
        let mut inner = [0u8; inner_len(MTU)];
        encode_frame(chan, payload, &mut inner, out)
    }

    /// Feed `wire` byte-by-byte; collect payloads of every valid frame.
    fn feed(deframer: &mut Deframer, acc: &mut [u8], wire: &[u8], out: &mut [u8]) -> Option<usize> {
        for &byte in wire {
            if let Push::Frame { len } = deframer.push(acc, byte)
                && let Ok(n) = decode_frame(&mut acc[..len], CHAN)
            {
                out[..n].copy_from_slice(&acc[1..1 + n]);
                return Some(n);
            }
        }
        None
    }

    #[test]
    fn roundtrip_byte_at_a_time() {
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode(CHAN, b"hello uart", &mut wire);
        assert!(n <= max_frame_len(MTU));
        assert_eq!(wire[n - 1], 0x00);
        assert!(
            !wire[..n - 1].contains(&0x00),
            "COBS body must be zero-free"
        );

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let mut out = [0u8; MTU];
        let got = feed(&mut deframer, &mut acc, &wire[..n], &mut out);
        assert_eq!(got, Some(10));
        assert_eq!(&out[..10], b"hello uart");
    }

    #[test]
    fn empty_payload_roundtrips() {
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode(CHAN, b"", &mut wire);

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let mut out = [0u8; MTU];
        assert_eq!(feed(&mut deframer, &mut acc, &wire[..n], &mut out), Some(0));
    }

    #[test]
    fn zero_heavy_payload_roundtrips() {
        let payload = [0u8; MTU];
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode(CHAN, &payload, &mut wire);
        assert!(n <= max_frame_len(MTU));

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let mut out = [0u8; MTU];
        assert_eq!(
            feed(&mut deframer, &mut acc, &wire[..n], &mut out),
            Some(MTU)
        );
        assert_eq!(out, payload);
    }

    #[test]
    fn resynchronizes_after_leading_garbage() {
        let mut wire = [0u8; max_frame_len(MTU) + 8];
        // Line noise with no delimiter, then a mid-noise delimiter, then a frame.
        wire[..7].copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF, 0x17, 0x2A, 0x00]);
        let n = encode(CHAN, b"clean", &mut wire[7..]);

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let mut out = [0u8; MTU];
        let got = feed(&mut deframer, &mut acc, &wire[..7 + n], &mut out);
        assert_eq!(got, Some(5));
        assert_eq!(&out[..5], b"clean");
    }

    #[test]
    fn corrupt_crc_is_dropped() {
        let mut inner = [0u8; inner_len(MTU)];
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode_frame(CHAN, b"corrupt me", &mut inner, &mut wire);
        // Flip a payload byte on the wire without introducing a delimiter.
        wire[3] ^= 0x01;

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let mut out = [0u8; MTU];
        assert_eq!(feed(&mut deframer, &mut acc, &wire[..n], &mut out), None);
    }

    #[test]
    fn foreign_channel_is_rejected() {
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode(CHAN + 1, b"not for us", &mut wire);

        let mut deframer = Deframer::default();
        let mut acc = [0u8; max_frame_len(MTU) - 1];
        let Push::Frame { len } = wire[..n]
            .iter()
            .map(|&b| deframer.push(&mut acc, b))
            .find(|p| *p != Push::Consumed)
            .expect("delimiter closes the frame")
        else {
            panic!("frame should not be skipped");
        };
        assert_eq!(
            decode_frame(&mut acc[..len], CHAN),
            Err(FrameError::ForeignChan { chan: CHAN + 1 })
        );
    }

    #[test]
    fn truncated_frame_is_too_short() {
        // A valid 3-byte COBS message decodes to fewer than FRAME_OVERHEAD bytes.
        let mut acc = [0x03, 0x01, 0x02];
        let mut deframer = Deframer::default();
        let mut store = [0u8; 16];
        for &b in &acc {
            assert_eq!(deframer.push(&mut store, b), Push::Consumed);
        }
        assert_eq!(decode_frame(&mut acc, CHAN), Err(FrameError::TooShort));
    }

    #[test]
    fn oversized_frame_overflows_then_recovers() {
        let mut deframer = Deframer::default();
        let mut acc = [0u8; 8];
        // 20 non-zero bytes overflow the 8-byte accumulator...
        for _ in 0..20 {
            assert_eq!(deframer.push(&mut acc, 0x55), Push::Consumed);
        }
        // ...so the closing delimiter skips the poisoned frame.
        assert_eq!(deframer.push(&mut acc, 0x00), Push::Skipped);
        // A following short frame is accumulated normally again.
        assert_eq!(deframer.push(&mut acc, 0x41), Push::Consumed);
        assert_eq!(deframer.push(&mut acc, 0x00), Push::Frame { len: 1 });
    }

    #[test]
    fn consecutive_delimiters_are_skipped() {
        let mut deframer = Deframer::default();
        let mut acc = [0u8; 8];
        assert_eq!(deframer.push(&mut acc, 0x00), Push::Skipped);
        assert_eq!(deframer.push(&mut acc, 0x00), Push::Skipped);
    }

    #[test]
    fn max_frame_len_bounds_worst_case_encoding() {
        // Worst COBS expansion comes from zero-free payloads.
        let payload = [0xFFu8; MTU];
        let mut wire = [0u8; max_frame_len(MTU)];
        let n = encode(CHAN, &payload, &mut wire);
        assert!(n <= max_frame_len(MTU));
    }
}
