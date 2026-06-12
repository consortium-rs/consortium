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

pub struct RkyvCodec<const SCRATCH: usize>;

/// A marker trait to indicate that the type can be safely archived and deserialized without validation.
///
/// Because in `no_alloc`, the `rkyv` crate doesn't provide safe access to archived data, we need
/// to ensure that the types we archive are safe to access without validation.
///
/// # Safety
///
/// When the data is strictly aligned via `#[repr(align(16))]`, it is safe to access the archived
/// data without validation. This is because the alignment guarantees that the data will be
/// correctly aligned in memory, preventing undefined behavior when accessing it.
///
/// In `no_alloc` mode, we only serialize and deserialize the types implementing this trait,
/// ensuring that all archived data is safe to access without validation.
pub unsafe trait TrustedArchive: rkyv::Archive {}

impl<const SCRATCH: usize> Codec for RkyvCodec<SCRATCH> {
    type Error = rkyv::rancor::Error;
}

#[cfg(feature = "alloc")]
use rkyv::{
    Archive, Serialize,
    api::high::{HighSerializer, HighValidator},
    bytecheck::CheckBytes,
    rancor::Error as RkyvError,
    ser::allocator::ArenaHandle,
    util::AlignedVec,
};

#[cfg(feature = "alloc")]
impl<T, const SCRATCH: usize> CodecFor<T> for RkyvCodec<SCRATCH>
where
    T: Archive + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RkyvError>>,
    T::Archived: for<'buf> CheckBytes<HighValidator<'buf, RkyvError>>,
{
    type Decoded<'buf>
        = &'buf T::Archived
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let bytes = rkyv::to_bytes::<RkyvError>(msg)?;
        let len = bytes.len();
        buf[..len].copy_from_slice(&bytes);
        Ok(len)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf,
    {
        rkyv::access::<T::Archived, RkyvError>(buf)
    }
}

#[cfg(not(feature = "alloc"))]
use rkyv::{
    Serialize,
    api::low::{LowSerializer, to_bytes_in_with_alloc},
    rancor,
    ser::{allocator::SubAllocator, writer::Buffer},
    util::Align,
};

#[cfg(not(feature = "alloc"))]
impl<T, const SCRATCH: usize> CodecFor<T> for RkyvCodec<SCRATCH>
where
    T: TrustedArchive
        + for<'a> Serialize<LowSerializer<Buffer<'a>, SubAllocator<'a>, rancor::Error>>,
{
    type Decoded<'buf>
        = &'buf T::Archived
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> Result<usize, Self::Error> {
        use core::mem::MaybeUninit;
        let mut output = Align([MaybeUninit::<u8>::uninit(); SCRATCH]);
        let mut alloc = [MaybeUninit::<u8>::uninit(); SCRATCH];
        let bytes = to_bytes_in_with_alloc::<_, _, rancor::Error>(
            msg,
            Buffer::from(&mut *output),
            SubAllocator::new(&mut alloc),
        )?;
        let len = bytes.len();
        buf[..len].copy_from_slice(&bytes);
        Ok(len)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf,
    {
        // SAFETY: T: TrustedArchive guarantees buf was produced by encode
        // and is correctly aligned
        Ok(unsafe { rkyv::access_unchecked::<T::Archived>(buf) })
    }
}
