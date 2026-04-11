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

pub trait Codec: core::marker::Sized {
    type Error: core::fmt::Debug;
}

pub trait CodecFor<T>: Codec {
    type Decoded<'buf>: 'buf
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> core::result::Result<usize, Self::Error>;

    fn decode<'buf>(buf: &'buf [u8]) -> core::result::Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf;
}
