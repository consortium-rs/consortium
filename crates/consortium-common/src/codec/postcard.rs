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
use core::result::Result;

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
        let written = postcard::to_slice(msg, buf)?;
        Ok(written.len())
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, postcard::Error>
    where
        T: 'buf,
    {
        postcard::from_bytes(buf)
    }
}
