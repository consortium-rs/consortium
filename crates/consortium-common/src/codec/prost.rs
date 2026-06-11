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

use super::{Codec, CodecFor};

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
            return Err(ProstError::BufferTooSmall);
        }
        let mut slice = &mut buf[..len];
        msg.encode(&mut slice).map_err(ProstError::Encode)?;
        Ok(len)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, ProstError>
    where
        T: 'buf,
    {
        T::decode(buf).map_err(ProstError::Decode)
    }
}
