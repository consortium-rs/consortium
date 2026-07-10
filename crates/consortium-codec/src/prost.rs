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

use crate::{Codec, CodecFor};

pub struct ProstCodec;

#[derive(Debug)]
pub enum ProstError {
    Encode(prost::EncodeError),
    Decode(prost::DecodeError),
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
