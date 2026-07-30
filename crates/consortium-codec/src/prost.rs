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

//! [`ProstCodec`] — protobuf encoding via [prost](https://docs.rs/prost).
//!
//! Use this when the message schema is not owned by this workspace: a `.proto`
//! shared with a cloud service, another language's client, or an existing
//! protobuf-speaking peer. For traffic where both ends are Rust crates in this
//! repository, `PostcardCodec` is smaller and faster.
//!
//! # Considerations
//!
//! - **Schema evolution is the point.** Protobuf's field numbering tolerates
//!   adding and reserving fields, which is what makes it worth the size overhead
//!   when the two ends ship independently.
//! - **Length is measured before writing.** `prost` encodes into a `BufMut`, which
//!   grows rather than failing, so this codec calls `encoded_len()` first and
//!   returns [`ProstError::BufferTooSmall`] itself. Without that check an
//!   over-long message would be a panic or a silent partial frame instead of an
//!   error.
//! - **`Default` is required.** `prost` decodes by filling a default-constructed
//!   message, so the bound is `T: prost::Message + Default`.
//! - **Owned decode.** `Decoded<'buf> = T`; there is no borrow of the receive
//!   buffer.
//!
//! # Limitations
//!
//! `prost` is pulled in with `default-features = false` to stay `no_std`-friendly,
//! but generated protobuf types are typically far heavier than postcard structs —
//! `String` and `Vec` fields need `alloc`. Check the generated code before
//! selecting this codec for a bare-metal core.

use crate::{Codec, CodecFor};

/// Protobuf codec over `prost`.
///
/// Implements [`CodecFor<T>`] for any `T: prost::Message + Default`, with
/// `Decoded<'buf> = T`.
pub struct ProstCodec;

/// Errors produced by [`ProstCodec`].
#[derive(Debug)]
pub enum ProstError {
    /// `prost` rejected the message while serializing.
    Encode(prost::EncodeError),
    /// The received bytes are not a valid encoding of the expected message.
    Decode(prost::DecodeError),
    /// The destination slice is smaller than the message's `encoded_len()`.
    ///
    /// Raised by this codec rather than by `prost`, whose `BufMut` sink would
    /// otherwise grow or panic instead of reporting a bounded-buffer failure.
    BufferTooSmall,
}

impl Codec for ProstCodec {
    type Error = ProstError;
}

impl<T> CodecFor<T> for ProstCodec
where
    T: prost::Message + Default,
{
    type Decoded<'buf>
        = T
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> Result<usize, ProstError> {
        let len = msg.encoded_len();
        if len > buf.len() {
            consortium_log::trace!(
                "prost encode needs {} bytes but buffer is {}",
                len,
                buf.len()
            );
            return Err(ProstError::BufferTooSmall);
        }
        let mut slice = &mut buf[..len];
        msg.encode(&mut slice).map_err(|e| {
            consortium_log::trace!("prost encode failed for {}-byte message", len);
            ProstError::Encode(e)
        })?;
        Ok(len)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, ProstError>
    where
        T: 'buf,
    {
        T::decode(buf).map_err(|e| {
            consortium_log::trace!("prost decode failed on {} bytes", buf.len());
            ProstError::Decode(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    use prost::bytes::{Buf, BufMut};
    use prost::encoding::{self, DecodeContext, WireType};

    #[derive(Clone, Debug, Default, PartialEq)]
    struct Telemetry {
        sequence: u32,
        ready: bool,
    }

    impl Message for Telemetry {
        fn encode_raw(&self, buf: &mut impl BufMut)
        where
            Self: Sized,
        {
            encoding::uint32::encode(1, &self.sequence, buf);
            encoding::bool::encode(2, &self.ready, buf);
        }

        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: WireType,
            buf: &mut impl Buf,
            ctx: DecodeContext,
        ) -> Result<(), prost::DecodeError>
        where
            Self: Sized,
        {
            match tag {
                1 => encoding::uint32::merge(wire_type, &mut self.sequence, buf, ctx),
                2 => encoding::bool::merge(wire_type, &mut self.ready, buf, ctx),
                _ => encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }

        fn encoded_len(&self) -> usize {
            encoding::uint32::encoded_len(1, &self.sequence)
                + encoding::bool::encoded_len(2, &self.ready)
        }

        fn clear(&mut self) {
            *self = Self::default();
        }
    }

    #[test]
    fn round_trips_protobuf_message() {
        let msg = Telemetry {
            sequence: 7,
            ready: true,
        };
        let mut buf = [0u8; 16];

        let len = ProstCodec::encode(&msg, &mut buf).expect("encode should fit");
        let decoded = <ProstCodec as CodecFor<Telemetry>>::decode(&buf[..len])
            .expect("decode should succeed");

        assert_eq!(decoded, msg);
    }

    #[test]
    fn reports_buffer_too_small() {
        let msg = Telemetry {
            sequence: u32::MAX,
            ready: true,
        };
        let mut buf = [0u8; 1];

        assert!(matches!(
            ProstCodec::encode(&msg, &mut buf),
            Err(ProstError::BufferTooSmall)
        ));
    }

    #[test]
    fn rejects_truncated_message() {
        let buf = [0x08];

        assert!(<ProstCodec as CodecFor<Telemetry>>::decode(&buf).is_err());
    }
}
