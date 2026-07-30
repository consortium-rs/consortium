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

//! [`RkyvCodec`] — zero-copy access to an archived message.
//!
//! `rkyv` writes a memory layout rather than a byte stream, so decoding is a
//! pointer cast instead of a deserialization pass. `Decoded<'buf> =
//! &'buf T::Archived`, and the returned reference borrows the receive buffer for
//! as long as it is held — the reason `consortium_ipc::ReceivedMessage` carries a
//! lifetime at all. Choose this codec when messages are large enough that copying
//! them out dominates, and accept that the buffer stays pinned while a message is
//! in use.
//!
//! # Two paths, one type
//!
//! [`RkyvCodec`] is generic over a `SCRATCH` byte budget because its two feature
//! paths need different machinery:
//!
//! | Build              | Serializer              | Decode                          |
//! | ------------------ | ----------------------- | ------------------------------- |
//! | `alloc` enabled    | `AlignedVec` arena      | `rkyv::access` — **validated**   |
//! | `alloc` disabled   | `SubAllocator<SCRATCH>` | `access_unchecked` — **trusted** |
//!
//! With `alloc` the serializer grows freely and `bytecheck` validates the archive
//! before handing out the reference, so any `T: Archive + Serialize` works and a
//! corrupt frame is an error. Without `alloc` there is nowhere to grow: encoding
//! runs in two `SCRATCH`-sized `MaybeUninit` buffers on the stack, so `SCRATCH`
//! must be chosen to fit the largest message *plus* rkyv's scratch overhead, and
//! an under-sized parameter surfaces as an encode error at runtime, not a compile
//! error.
//!
//! # Why [`TrustedArchive`] exists
//!
//! `bytecheck`'s validator needs `alloc`. On the no-`alloc` path decode therefore
//! calls `rkyv::access_unchecked`, which is unsound on bytes that are not a
//! correctly aligned archive of `T`. [`TrustedArchive`] is the marker that carries
//! that obligation: the no-`alloc` `CodecFor` impl is bounded on it, so only types
//! whose layout has been vetted can be decoded without validation.
//!
//! Blanket impls cover the fixed-width primitives, arrays, and tuples up to eight
//! elements. For user types use `#[derive(TrustedArchive)]`, which checks that the
//! type declares a stable `repr` and that every field is itself `TrustedArchive`.
//!
//! # Limitations
//!
//! - The no-`alloc` path decodes **without validating**, so it must only be pointed
//!   at bytes a peer produced with the same codec and type. A hostile or
//!   desynchronized sender is out of its threat model; use the `alloc` path where
//!   the peer is not trusted.
//! - `rkyv` archives are not a stable wire format across `rkyv` versions. Both ends
//!   of a link must be built against the same one.
//! - Archived types are alignment-sensitive; the transport must hand over a buffer
//!   whose start is suitably aligned.

use crate::{Codec, CodecFor};
use core::fmt;

/// Zero-copy codec over [rkyv](https://docs.rs/rkyv).
///
/// `SCRATCH` is the stack byte budget used for serialization on `no_std` builds
/// (`alloc` disabled); it must cover the largest message plus rkyv's own scratch
/// space. With `alloc` enabled the parameter is unused by encoding, which
/// serializes through a growable `AlignedVec`.
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

macro_rules! trusted_archive_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            // SAFETY: These fixed-size primitives archive to plain value data
            // with no address-space-local state. Values decoded by this codec
            // are bytes produced by rkyv serialization for the same type.
            unsafe impl TrustedArchive for $ty {}
        )*
    };
}

trusted_archive_primitives!(
    (),
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
);

// SAFETY: arrays archive to [T::Archived; N]; when T: TrustedArchive, every element's
// archived form is safe to access without validation, so the array is too.
unsafe impl<T: TrustedArchive, const N: usize> TrustedArchive for [T; N] {}

macro_rules! impl_trusted_archive_tuple {
    ($($T:ident),+) => {
        // SAFETY: tuples archive to a tuple of the elements' archived forms; when every
        // element is TrustedArchive, the archived tuple is safe to access without validation.
        unsafe impl<$($T: TrustedArchive),+> TrustedArchive for ($($T,)+) {}
    };
}

impl_trusted_archive_tuple!(T0);
impl_trusted_archive_tuple!(T0, T1);
impl_trusted_archive_tuple!(T0, T1, T2);
impl_trusted_archive_tuple!(T0, T1, T2, T3);
impl_trusted_archive_tuple!(T0, T1, T2, T3, T4);
impl_trusted_archive_tuple!(T0, T1, T2, T3, T4, T5);
impl_trusted_archive_tuple!(T0, T1, T2, T3, T4, T5, T6);
impl_trusted_archive_tuple!(T0, T1, T2, T3, T4, T5, T6, T7);

impl<const SCRATCH: usize> Codec for RkyvCodec<SCRATCH> {
    type Error = rkyv::rancor::Error;
}

#[derive(Debug)]
struct BufferTooSmall;

impl fmt::Display for BufferTooSmall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("destination buffer is too small")
    }
}

impl core::error::Error for BufferTooSmall {}

fn buffer_too_small() -> rkyv::rancor::Error {
    <rkyv::rancor::Error as rkyv::rancor::Source>::new(BufferTooSmall)
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
        if len > buf.len() {
            consortium_log::trace!(
                "rkyv encode needs {} bytes but buffer is {}",
                len,
                buf.len()
            );
            return Err(buffer_too_small());
        }
        buf[..len].copy_from_slice(&bytes);
        Ok(len)
    }

    fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf,
    {
        rkyv::access::<T::Archived, RkyvError>(buf).inspect_err(|_| {
            consortium_log::trace!("rkyv access/validation failed on {} bytes", buf.len());
        })
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
        if len > buf.len() {
            consortium_log::trace!(
                "rkyv encode needs {} bytes but buffer is {}",
                len,
                buf.len()
            );
            return Err(buffer_too_small());
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_trusted_archive<T: TrustedArchive>() {}

    #[test]
    fn fixed_size_primitives_are_trusted_archives() {
        assert_trusted_archive::<()>();
        assert_trusted_archive::<bool>();
        assert_trusted_archive::<char>();
        assert_trusted_archive::<u8>();
        assert_trusted_archive::<u16>();
        assert_trusted_archive::<u32>();
        assert_trusted_archive::<u64>();
        assert_trusted_archive::<u128>();
        assert_trusted_archive::<i8>();
        assert_trusted_archive::<i16>();
        assert_trusted_archive::<i32>();
        assert_trusted_archive::<i64>();
        assert_trusted_archive::<i128>();
        assert_trusted_archive::<f32>();
        assert_trusted_archive::<f64>();
    }

    #[test]
    fn round_trips_archived_value() {
        let msg = 0x1234_5678u32;
        let mut buf = [0u8; 16];

        let len = RkyvCodec::<64>::encode(&msg, &mut buf).expect("encode should fit");
        let decoded =
            <RkyvCodec<64> as CodecFor<u32>>::decode(&buf[..len]).expect("decode should succeed");

        assert_eq!(*decoded, msg);
    }

    #[test]
    fn rejects_buffer_that_is_too_small() {
        let msg = 0x1234_5678u32;
        let mut buf = [0u8; 1];

        assert!(RkyvCodec::<64>::encode(&msg, &mut buf).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn rejects_invalid_archived_bytes() {
        let buf = [0u8; 1];

        assert!(<RkyvCodec<64> as CodecFor<u32>>::decode(&buf).is_err());
    }
}
