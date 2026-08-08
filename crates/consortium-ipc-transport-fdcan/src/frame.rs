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

//! Segmentation and reassembly of IPC messages over fixed-size CAN frames.
//!
//! A CAN controller already delimits frames and protects each one with a CRC,
//! so — unlike the UART transport — there is nothing here to delimit or
//! checksum. What CAN does not do is carry a message longer than one frame's
//! payload, and 8 bytes (classic) or 64 (FD) is below the size of most encoded
//! IPC messages. This module is that missing piece.
//!
//! Every frame spends one payload byte on a header:
//!
//! ```text
//! byte 0: [ kind:2 | low:6 ]
//!   00 Single   : low = payload length; the whole message follows
//!   01 First    : low = 0; bytes 1..3 = total length u16 LE, then the head
//!   10 Continue : low = sequence number, 1,2,3,… mod 64; then the body
//!   11 reserved
//! ```
//!
//! The 6-bit field does double duty, and both readings fit exactly. On a
//! `Single` it is the payload length, which is at most
//! [`single_max`]`(64) == 63` even for FD — so the length never has to be
//! inferred from the DLC. That matters because FD's DLC is quantized above 8
//! bytes (…, 32, 48, 64), so a 30-byte frame arrives padded to 32 and an
//! implied length would be wrong. On a `Continue` the same field is a wrapping
//! sequence number, which catches a dropped frame; the message's real end comes
//! from the `First` frame's total, so padding on the final frame is ignored.
//!
//! The sequence counter wraps at 64, so losing exactly a multiple of 64
//! consecutive frames is undetectable here. At classic-CAN sizes that is a
//! 448-byte hole in a single message; the codec's own decode is the backstop.

use consortium_log::warn;

/// Bytes of frame payload spent on the header.
pub const HEADER_LEN: usize = 1;
/// Additional bytes a `First` frame spends on the total-length field.
pub const FIRST_EXTRA: usize = 2;
/// Data bytes a classic CAN frame carries.
pub const CLASSIC_PAYLOAD: usize = 8;
/// Data bytes a CAN FD frame carries.
pub const FD_PAYLOAD: usize = 64;
/// Largest MTU the total-length field can describe.
pub const MAX_MTU: usize = u16::MAX as usize;

/// Smallest frame payload that can carry a segmented message: a header, the
/// length field, and at least one data byte.
pub const MIN_FRAME_PAYLOAD: usize = HEADER_LEN + FIRST_EXTRA + 1;

/// Sequence numbers wrap at this value, the range of the 6-bit header field.
const SEQ_WRAP: u8 = 64;
const KIND_SHIFT: u32 = 6;
const LOW_MASK: u8 = 0x3F;

/// Largest message a single unsegmented frame carries.
pub const fn single_max(frame_payload: usize) -> usize {
    assert!(frame_payload >= MIN_FRAME_PAYLOAD);
    frame_payload - HEADER_LEN
}

/// Message bytes a `First` frame carries alongside the total-length field.
pub const fn first_max(frame_payload: usize) -> usize {
    assert!(frame_payload >= MIN_FRAME_PAYLOAD);
    frame_payload - HEADER_LEN - FIRST_EXTRA
}

/// Message bytes a `Continue` frame carries.
pub const fn continue_max(frame_payload: usize) -> usize {
    assert!(frame_payload >= MIN_FRAME_PAYLOAD);
    frame_payload - HEADER_LEN
}

/// CAN frames a message of exactly `mtu` bytes occupies.
///
/// Useful for budgeting bus time: at 500 kbit/s a classic frame is roughly
/// 130 µs on the wire, so this is what one full-MTU message costs.
pub const fn frames_for(mtu: usize, frame_payload: usize) -> usize {
    if mtu <= single_max(frame_payload) {
        return 1;
    }
    let rest = mtu - first_max(frame_payload);
    let cont = continue_max(frame_payload);
    1 + rest.div_ceil(cont)
}

/// Round `len` up to a length a CAN frame can actually describe.
///
/// Classic CAN encodes every length `0..=8` exactly, so this is the identity
/// there. CAN FD's DLC instead jumps 8, 12, 16, 20, 24, 32, 48, 64 — a
/// controller handed 9 bytes has no way to say "nine". Segments are therefore
/// padded up to the next representable size before being handed to
/// [`CanFrame::build`](crate::CanFrame::build).
///
/// The padding is free of consequence on the wire: a `Single` carries its
/// length in the header rather than inferring it from the DLC, and a
/// `Continue` is bounded by the `First` frame's total, so the receiver stops
/// at the real end either way.
pub const fn padded_len(len: usize) -> usize {
    assert!(len <= FD_PAYLOAD);
    match len {
        0..=8 => len,
        9..=12 => 12,
        13..=16 => 16,
        17..=20 => 20,
        21..=24 => 24,
        25..=32 => 32,
        33..=48 => 48,
        _ => 64,
    }
}

/// Reassembly buffer size for an MTU: one full message.
///
/// There is no `tx_buf_len` counterpart. A CAN frame is fixed-size, so the
/// transmit path stages each frame in a small stack array and slices the
/// caller's payload directly — it needs no `&'static mut [u8]` at all.
pub const fn rx_buf_len(mtu: usize) -> usize {
    mtu
}

/// What a frame's header says the frame is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    Single,
    First,
    Continue,
}

impl Kind {
    const fn code(self) -> u8 {
        match self {
            Kind::Single => 0b00,
            Kind::First => 0b01,
            Kind::Continue => 0b10,
        }
    }

    const fn from_code(code: u8) -> Option<Self> {
        match code {
            0b00 => Some(Kind::Single),
            0b01 => Some(Kind::First),
            0b10 => Some(Kind::Continue),
            _ => None,
        }
    }
}

/// Pack a header byte. `low` is a length for `Single`, a sequence number for
/// `Continue`, and zero for `First`.
pub(crate) const fn encode_header(kind: Kind, low: u8) -> u8 {
    debug_assert!(low <= LOW_MASK);
    (kind.code() << KIND_SHIFT) | (low & LOW_MASK)
}

/// Unpack a header byte, or `None` if it names the reserved kind.
pub(crate) const fn decode_header(byte: u8) -> Option<(Kind, u8)> {
    match Kind::from_code(byte >> KIND_SHIFT) {
        Some(kind) => Some((kind, byte & LOW_MASK)),
        None => None,
    }
}

/// Why a received frame was dropped.
///
/// None of these reach the caller: the receiver logs them, discards whatever it
/// had collected, and resynchronizes on the next `Single` or `First`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum FrameError {
    /// The frame carried an identifier this endpoint does not listen to.
    ForeignId,
    /// A remote-transmission-request frame, which carries no payload.
    Remote,
    /// The header named the reserved kind.
    BadKind,
    /// The frame was too short for the fields its header promised.
    Short,
    /// A `Continue` arrived with no `First` to continue.
    Orphan,
    /// A `Continue` skipped a sequence number, so at least one frame was lost.
    SeqGap { expected: u8, got: u8 },
    /// The declared message length exceeds this endpoint's MTU.
    TooLong { total: usize, mtu: usize },
}

/// Outcome of feeding one frame to the [`Reassembler`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Push {
    /// Absorbed; the message is still incomplete.
    Consumed,
    /// A complete message now occupies `acc[..len]`.
    Message { len: usize },
}

/// Reassembly state machine.
///
/// Like the UART deframer, the accumulator itself lives in the caller's buffer
/// and is passed in on every call, so this type stays trivially movable and
/// carries no lifetime.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Reassembler {
    state: State,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Idle,
    Collecting {
        total: usize,
        got: usize,
        next_seq: u8,
    },
}

impl Reassembler {
    /// Feed one frame's payload bytes.
    ///
    /// `acc` is the reassembly buffer, at least [`rx_buf_len`]`(mtu)` bytes.
    /// On `Err` the partial message has already been discarded and the state
    /// machine is idle, waiting for the next `Single` or `First`.
    pub(crate) fn push(
        &mut self,
        acc: &mut [u8],
        mtu: usize,
        data: &[u8],
    ) -> Result<Push, FrameError> {
        // Only the header itself is required up front: a zero-length message is
        // a legitimate one-byte `Single`. Each kind checks its own fields below.
        if data.is_empty() {
            self.state = State::Idle;
            return Err(FrameError::Short);
        }
        let Some((kind, low)) = decode_header(data[0]) else {
            self.state = State::Idle;
            return Err(FrameError::BadKind);
        };

        match kind {
            Kind::Single => {
                self.note_abandoned();
                let len = usize::from(low);
                if HEADER_LEN + len > data.len() {
                    self.state = State::Idle;
                    return Err(FrameError::Short);
                }
                if len > mtu {
                    self.state = State::Idle;
                    return Err(FrameError::TooLong { total: len, mtu });
                }
                acc[..len].copy_from_slice(&data[HEADER_LEN..HEADER_LEN + len]);
                self.state = State::Idle;
                Ok(Push::Message { len })
            }
            Kind::First => {
                self.note_abandoned();
                if data.len() < HEADER_LEN + FIRST_EXTRA {
                    self.state = State::Idle;
                    return Err(FrameError::Short);
                }
                let total = usize::from(u16::from_le_bytes([data[1], data[2]]));
                if total > mtu {
                    self.state = State::Idle;
                    return Err(FrameError::TooLong { total, mtu });
                }
                let head = &data[HEADER_LEN + FIRST_EXTRA..];
                let n = head.len().min(total);
                acc[..n].copy_from_slice(&head[..n]);
                if n == total {
                    self.state = State::Idle;
                    return Ok(Push::Message { len: total });
                }
                self.state = State::Collecting {
                    total,
                    got: n,
                    next_seq: 1,
                };
                Ok(Push::Consumed)
            }
            Kind::Continue => {
                // A continuation carrying no body cannot advance the message,
                // so accepting it would stall reassembly forever.
                if data.len() <= HEADER_LEN {
                    self.state = State::Idle;
                    return Err(FrameError::Short);
                }
                let State::Collecting {
                    total,
                    got,
                    next_seq,
                } = self.state
                else {
                    return Err(FrameError::Orphan);
                };
                if low != next_seq {
                    self.state = State::Idle;
                    return Err(FrameError::SeqGap {
                        expected: next_seq,
                        got: low,
                    });
                }
                // The body is bounded by what is still outstanding, so trailing
                // padding on the last frame — which CAN FD's quantized DLC can
                // introduce — is ignored rather than appended.
                let body = &data[HEADER_LEN..];
                let n = body.len().min(total - got);
                acc[got..got + n].copy_from_slice(&body[..n]);
                let got = got + n;
                if got == total {
                    self.state = State::Idle;
                    return Ok(Push::Message { len: total });
                }
                self.state = State::Collecting {
                    total,
                    got,
                    next_seq: (next_seq + 1) % SEQ_WRAP,
                };
                Ok(Push::Consumed)
            }
        }
    }

    /// Warn when a new message displaces one still being collected — the sender
    /// restarted, or every remaining frame of the old message was lost.
    fn note_abandoned(&self) {
        if let State::Collecting { total, got, .. } = self.state {
            warn!(
                "fdcan recv: abandoning partial message, {} of {} bytes collected",
                got, total
            );
        }
    }
}

/// Splits a payload into frame-sized segments.
///
/// Holds no buffer of its own: each call writes one frame's bytes into caller
/// scratch, which the transport keeps on the stack.
pub(crate) struct Segmenter<'a> {
    payload: &'a [u8],
    frame_payload: usize,
    sent: usize,
    seq: u8,
    started: bool,
}

impl<'a> Segmenter<'a> {
    pub(crate) fn new(payload: &'a [u8], frame_payload: usize) -> Self {
        Self {
            payload,
            frame_payload,
            sent: 0,
            seq: 1,
            started: false,
        }
    }

    /// Write the next frame into `out`, returning its length, or `None` once the
    /// whole payload has been emitted.
    ///
    /// `out` must be at least `frame_payload` bytes.
    pub(crate) fn next(&mut self, out: &mut [u8]) -> Option<usize> {
        if self.started && self.sent == self.payload.len() {
            return None;
        }

        if !self.started {
            self.started = true;
            if self.payload.len() <= single_max(self.frame_payload) {
                let len = self.payload.len();
                out[0] = encode_header(Kind::Single, len as u8);
                out[HEADER_LEN..HEADER_LEN + len].copy_from_slice(self.payload);
                self.sent = len;
                return Some(HEADER_LEN + len);
            }
            let n = first_max(self.frame_payload);
            out[0] = encode_header(Kind::First, 0);
            out[1..1 + FIRST_EXTRA].copy_from_slice(&(self.payload.len() as u16).to_le_bytes());
            out[HEADER_LEN + FIRST_EXTRA..HEADER_LEN + FIRST_EXTRA + n]
                .copy_from_slice(&self.payload[..n]);
            self.sent = n;
            return Some(self.frame_payload);
        }

        let n = continue_max(self.frame_payload).min(self.payload.len() - self.sent);
        out[0] = encode_header(Kind::Continue, self.seq);
        out[HEADER_LEN..HEADER_LEN + n].copy_from_slice(&self.payload[self.sent..self.sent + n]);
        self.sent += n;
        self.seq = (self.seq + 1) % SEQ_WRAP;
        Some(HEADER_LEN + n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MTU: usize = 1024;
    const P: usize = CLASSIC_PAYLOAD;
    /// Enough for a full-MTU message at the smallest classic segment size.
    const MAX_FRAMES: usize = 160;

    /// Frames emitted for one message, stored inline so these tests need no
    /// allocator and run under `--no-default-features`.
    struct Frames {
        data: [[u8; FD_PAYLOAD]; MAX_FRAMES],
        lens: [usize; MAX_FRAMES],
        count: usize,
    }

    impl Frames {
        fn get(&self, i: usize) -> &[u8] {
            &self.data[i][..self.lens[i]]
        }

        /// Drop frame `i`, as a bus error or a FIFO overrun would.
        fn drop_frame(&mut self, i: usize) {
            for j in i..self.count - 1 {
                self.data[j] = self.data[j + 1];
                self.lens[j] = self.lens[j + 1];
            }
            self.count -= 1;
        }
    }

    /// Run a payload through the segmenter, collecting the frames it emits.
    fn segment(payload: &[u8], frame_payload: usize) -> Frames {
        let mut frames = Frames {
            data: [[0u8; FD_PAYLOAD]; MAX_FRAMES],
            lens: [0usize; MAX_FRAMES],
            count: 0,
        };
        let mut seg = Segmenter::new(payload, frame_payload);
        let mut buf = [0u8; FD_PAYLOAD];
        while let Some(len) = seg.next(&mut buf[..frame_payload]) {
            frames.data[frames.count][..len].copy_from_slice(&buf[..len]);
            frames.lens[frames.count] = len;
            frames.count += 1;
        }
        frames
    }

    /// Feed every frame to a fresh reassembler; the message must complete on
    /// the last one. Returns its length within `acc`.
    fn reassemble(frames: &Frames, acc: &mut [u8]) -> Result<usize, FrameError> {
        let mut r = Reassembler::default();
        for i in 0..frames.count {
            match r.push(acc, MTU, frames.get(i))? {
                Push::Message { len } => {
                    assert_eq!(i, frames.count - 1, "message completed early");
                    return Ok(len);
                }
                Push::Consumed => {}
            }
        }
        panic!("frames did not complete a message");
    }

    /// A payload whose every byte is a function of its index, so a misordered
    /// or truncated reassembly is visible.
    fn ramp<const N: usize>(start: u8) -> [u8; N] {
        let mut out = [0u8; N];
        let mut i = 0;
        while i < N {
            out[i] = start.wrapping_add(i as u8);
            i += 1;
        }
        out
    }

    /// Assert that `payload` survives a segment/reassemble round trip.
    fn assert_round_trips(payload: &[u8], frame_payload: usize) {
        let frames = segment(payload, frame_payload);
        assert_eq!(frames.count, frames_for(payload.len(), frame_payload));
        let mut acc = [0u8; MTU];
        let len = reassemble(&frames, &mut acc).expect("reassembly failed");
        assert_eq!(len, payload.len());
        assert_eq!(&acc[..len], payload);
    }

    #[test]
    fn header_round_trips_every_kind_and_low_value() {
        for kind in [Kind::Single, Kind::First, Kind::Continue] {
            for low in 0..=LOW_MASK {
                let byte = encode_header(kind, low);
                assert_eq!(decode_header(byte), Some((kind, low)));
            }
        }
    }

    #[test]
    fn reserved_kind_decodes_to_none() {
        assert_eq!(decode_header(0b11 << KIND_SHIFT), None);
    }

    #[test]
    fn single_length_always_fits_the_six_bit_field() {
        // The whole reason `Single` carries an explicit length: it must fit the
        // header's low field even at FD's 64-byte payload.
        assert!(single_max(FD_PAYLOAD) <= usize::from(LOW_MASK));
        assert_eq!(single_max(CLASSIC_PAYLOAD), 7);
        assert_eq!(single_max(FD_PAYLOAD), 63);
    }

    #[test]
    fn sizing_constants_match_the_layout() {
        assert_eq!(first_max(CLASSIC_PAYLOAD), 5);
        assert_eq!(continue_max(CLASSIC_PAYLOAD), 7);
        assert_eq!(rx_buf_len(MTU), MTU);
    }

    #[test]
    fn frames_for_counts_the_wire_cost() {
        assert_eq!(frames_for(0, P), 1);
        assert_eq!(frames_for(7, P), 1);
        // 8 bytes: 5 in the first frame, 3 in one continuation.
        assert_eq!(frames_for(8, P), 2);
        // 12 bytes: 5 + 7 exactly.
        assert_eq!(frames_for(12, P), 2);
        assert_eq!(frames_for(13, P), 3);
        // FD carries the same message in far fewer frames.
        assert_eq!(frames_for(128, FD_PAYLOAD), 3);
    }

    #[test]
    fn padded_len_is_the_identity_for_classic_lengths() {
        // Classic CAN encodes 0..=8 exactly, so segmentation over an 8-byte
        // frame type must never pad.
        for len in 0..=CLASSIC_PAYLOAD {
            assert_eq!(padded_len(len), len);
        }
    }

    #[test]
    fn padded_len_rounds_up_to_representable_fd_lengths() {
        // The DLC jumps 8, 12, 16, 20, 24, 32, 48, 64 above eight bytes.
        const VALID: [usize; 7] = [12, 16, 20, 24, 32, 48, 64];
        assert_eq!(padded_len(9), 12);
        assert_eq!(padded_len(10), 12);
        assert_eq!(padded_len(13), 16);
        assert_eq!(padded_len(33), 48);
        assert_eq!(padded_len(64), 64);
        // Never shrinks, and always lands on a length the medium can describe.
        for len in 9..=FD_PAYLOAD {
            let p = padded_len(len);
            assert!(p >= len, "padded_len({len}) shrank to {p}");
            assert!(VALID.contains(&p), "padded_len({len}) = {p} is not a DLC");
        }
    }

    #[test]
    fn single_frame_message_round_trips() {
        let payload = [1u8, 2, 3, 4, 5];
        let frames = segment(&payload, P);
        assert_eq!(frames.count, 1);
        assert_eq!(decode_header(frames.get(0)[0]), Some((Kind::Single, 5)));
        assert_round_trips(&payload, P);
    }

    #[test]
    fn empty_message_round_trips() {
        assert_round_trips(&[], P);
        assert_round_trips(&[], FD_PAYLOAD);
    }

    #[test]
    fn multi_frame_message_round_trips() {
        let payload = ramp::<100>(0);
        let frames = segment(&payload, P);
        assert_eq!(decode_header(frames.get(0)[0]), Some((Kind::First, 0)));
        assert_eq!(decode_header(frames.get(1)[0]), Some((Kind::Continue, 1)));
        assert_round_trips(&payload, P);
    }

    #[test]
    fn message_lengths_around_every_frame_boundary_round_trip() {
        // 7 is the single/segmented boundary, 12 fills two frames exactly.
        let payload = ramp::<40>(0);
        for len in 0..payload.len() {
            assert_round_trips(&payload[..len], P);
        }
    }

    #[test]
    fn message_exactly_filling_its_last_frame_round_trips() {
        // 12 bytes = first_max(5) + continue_max(7), no partial tail.
        let payload = ramp::<12>(0);
        let frames = segment(&payload, P);
        assert_eq!(frames.count, 2);
        assert_eq!(frames.lens[1], P);
        assert_round_trips(&payload, P);
    }

    #[test]
    fn sequence_numbers_wrap_at_sixty_four() {
        // Long enough to push the counter past one wrap.
        let payload = ramp::<500>(0);
        let frames = segment(&payload, P);
        assert!(frames.count > usize::from(SEQ_WRAP));
        assert_eq!(decode_header(frames.get(64)[0]), Some((Kind::Continue, 0)));
        assert_round_trips(&payload, P);
    }

    #[test]
    fn fd_sized_frames_round_trip() {
        let payload = ramp::<200>(0);
        let frames = segment(&payload, FD_PAYLOAD);
        // The same message costs far fewer frames on FD than on classic.
        assert!(frames.count < segment(&payload, P).count / 4);
        assert_round_trips(&payload, FD_PAYLOAD);
    }

    #[test]
    fn trailing_padding_on_the_last_frame_is_ignored() {
        let payload = ramp::<20>(0);
        let mut frames = segment(&payload, P);
        // Emulate a controller that padded the final short frame out to the
        // full DLC with garbage, as CAN FD's quantized lengths can.
        let last = frames.count - 1;
        assert!(frames.lens[last] < P, "test needs a short final frame");
        for b in &mut frames.data[last][frames.lens[last]..P] {
            *b = 0xAA;
        }
        frames.lens[last] = P;

        let mut acc = [0u8; MTU];
        let len = reassemble(&frames, &mut acc).unwrap();
        assert_eq!(&acc[..len], &payload);
    }

    #[test]
    fn dropped_continuation_is_reported_as_a_gap() {
        let payload = ramp::<40>(0);
        let mut frames = segment(&payload, P);
        frames.drop_frame(2);
        let mut acc = [0u8; MTU];
        assert_eq!(
            reassemble(&frames, &mut acc).unwrap_err(),
            FrameError::SeqGap {
                expected: 2,
                got: 3
            }
        );
    }

    #[test]
    fn orphan_continuation_is_rejected() {
        let mut acc = [0u8; MTU];
        let mut r = Reassembler::default();
        let frame = [encode_header(Kind::Continue, 1), 0xDE, 0xAD];
        assert_eq!(
            r.push(&mut acc, MTU, &frame).unwrap_err(),
            FrameError::Orphan
        );
    }

    #[test]
    fn oversized_declared_length_is_rejected() {
        let mut acc = [0u8; 16];
        let mut r = Reassembler::default();
        let mut frame = [0u8; P];
        frame[0] = encode_header(Kind::First, 0);
        frame[1..3].copy_from_slice(&999u16.to_le_bytes());
        assert_eq!(
            r.push(&mut acc, 16, &frame).unwrap_err(),
            FrameError::TooLong {
                total: 999,
                mtu: 16
            }
        );
    }

    #[test]
    fn oversized_single_is_rejected() {
        let mut acc = [0u8; 4];
        let mut r = Reassembler::default();
        let mut frame = [0u8; FD_PAYLOAD];
        frame[0] = encode_header(Kind::Single, 40);
        assert_eq!(
            r.push(&mut acc, 4, &frame[..41]).unwrap_err(),
            FrameError::TooLong { total: 40, mtu: 4 }
        );
    }

    #[test]
    fn truncated_frame_is_rejected() {
        let mut acc = [0u8; MTU];
        let mut r = Reassembler::default();
        // Header claims five payload bytes but only two follow.
        let frame = [encode_header(Kind::Single, 5), 1, 2];
        assert_eq!(
            r.push(&mut acc, MTU, &frame).unwrap_err(),
            FrameError::Short
        );
        // A header with no body at all cannot progress either.
        let frame = [encode_header(Kind::First, 0)];
        assert_eq!(
            r.push(&mut acc, MTU, &frame).unwrap_err(),
            FrameError::Short
        );
    }

    #[test]
    fn reserved_kind_is_rejected() {
        let mut acc = [0u8; MTU];
        let mut r = Reassembler::default();
        let frame = [0b11 << KIND_SHIFT, 1, 2];
        assert_eq!(
            r.push(&mut acc, MTU, &frame).unwrap_err(),
            FrameError::BadKind
        );
    }

    #[test]
    fn receiver_resynchronizes_after_a_dropped_message() {
        let mut acc = [0u8; MTU];
        let mut r = Reassembler::default();

        // Start a long message, then lose the rest of it.
        let long = ramp::<40>(0);
        let long_frames = segment(&long, P);
        assert_eq!(
            r.push(&mut acc, MTU, long_frames.get(0)).unwrap(),
            Push::Consumed
        );

        // A fresh message displaces the abandoned one and completes normally.
        let next = [9u8, 8, 7];
        let next_frames = segment(&next, P);
        assert_eq!(
            r.push(&mut acc, MTU, next_frames.get(0)).unwrap(),
            Push::Message { len: 3 }
        );
        assert_eq!(&acc[..3], &next);
    }

    #[test]
    fn a_new_multi_frame_message_displaces_a_partial_one() {
        let mut acc = [0u8; MTU];
        let mut r = Reassembler::default();

        let a = ramp::<40>(0);
        let b = ramp::<20>(100);
        let a_frames = segment(&a, P);
        let b_frames = segment(&b, P);

        // Two frames in, the first message is still incomplete.
        r.push(&mut acc, MTU, a_frames.get(0)).unwrap();
        r.push(&mut acc, MTU, a_frames.get(1)).unwrap();

        let mut done = None;
        for i in 0..b_frames.count {
            if let Push::Message { len } = r.push(&mut acc, MTU, b_frames.get(i)).unwrap() {
                done = Some(len);
            }
        }
        assert_eq!(done, Some(b.len()));
        assert_eq!(&acc[..b.len()], &b);
    }
}
