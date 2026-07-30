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

//! [`PostcardCodec`] — the workspace default: serde over the postcard wire format.
//!
//! Postcard is the pragmatic choice for cross-core traffic. It is `no_std` and
//! allocation-free when serializing into a slice, its varint encoding keeps small
//! messages small, and it reaches any type that already derives serde. The cost is
//! that decoding is a real deserialization pass producing an owned value, so
//! `Decoded<'buf> = T` and the receive buffer is free again immediately.
//!
//! # Considerations
//!
//! - **Not self-describing.** Both ends must agree on the type. In this workspace
//!   that is arranged by sharing one message crate (see
//!   `examples/melt-pot/shared`); a field added on one side only will misparse
//!   rather than error cleanly.
//! - **Encode fails, never truncates.** `postcard::to_slice` errors when the
//!   destination is too small, so a channel whose scratch buffer is under-sized
//!   surfaces a codec error rather than emitting a short frame.
//! - **Decode wants an exact frame.** The transport reports the true byte count and
//!   the channel passes `buf[..n]`; handing over the whole buffer would let
//!   trailing bytes be read as further fields.
//!
//! Failure paths log through `consortium-log` at `trace` level, which resolves to
//! `defmt` on firmware and `tracing` on the host.

use crate::{Codec, CodecFor};
use core::result::Result;

/// Serde-based codec using the [postcard](https://docs.rs/postcard) wire format.
///
/// Implements [`CodecFor<T>`] for any `T: Serialize + DeserializeOwned`, with
/// `Decoded<'buf> = T`.
pub struct PostcardCodec;

impl Codec for PostcardCodec {
    type Error = postcard::Error;
}

impl<T> CodecFor<T> for PostcardCodec
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    type Decoded<'buf>
        = T
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> Result<usize, postcard::Error> {
        let cap = buf.len();
        let written = postcard::to_slice(msg, buf).inspect_err(|e| {
            consortium_log::trace!("postcard encode failed into {}-byte buffer: {}", cap, e);
        })?;
        Ok(written.len())
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, postcard::Error>
    where
        T: 'buf,
    {
        postcard::from_bytes(buf).inspect_err(|e| {
            consortium_log::trace!("postcard decode failed on {} bytes: {}", buf.len(), e);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Message {
        sequence: u32,
        ready: bool,
    }

    #[test]
    fn round_trips_owned_message() {
        let msg = Message {
            sequence: 42,
            ready: true,
        };
        let mut buf = [0u8; 16];

        let len = PostcardCodec::encode(&msg, &mut buf).expect("encode should fit");
        let decoded = <PostcardCodec as CodecFor<Message>>::decode(&buf[..len])
            .expect("decode should succeed");

        assert_eq!(decoded, msg);
    }

    #[test]
    fn rejects_buffer_that_is_too_small() {
        let msg = Message {
            sequence: u32::MAX,
            ready: true,
        };
        let mut buf = [0u8; 1];

        assert!(PostcardCodec::encode(&msg, &mut buf).is_err());
    }

    #[test]
    fn rejects_truncated_message() {
        let buf = [42u8];

        assert!(<PostcardCodec as CodecFor<Message>>::decode(&buf).is_err());
    }
}
