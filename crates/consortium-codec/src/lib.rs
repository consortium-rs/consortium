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
#![cfg_attr(not(feature = "std"), no_std)]

//! Serialization families for Consortium IPC and TEE parameter conversion.
//!
//! A Consortium link carries typed messages, but the wire carries bytes. This
//! crate is the seam: [`Codec`] names a serialization *family*, [`CodecFor<T>`]
//! says that family can handle a particular message type, and everything above —
//! `consortium_ipc::Channel`, `consortium_tee::TeeParam` — is generic over it.
//!
//! # Architecture
//!
//! The trait pair is split so that a family's error type is declared once:
//!
//! ```text
//!   Codec            one impl per family; declares Error
//!     └── CodecFor<T>   one impl per (family, message type); encode/decode
//! ```
//!
//! The load-bearing detail is that [`CodecFor::Decoded`] is an associated *type
//! constructor*, not `T`:
//!
//! | Codec           | `Decoded<'buf>`      | Cost of a `recv`                  |
//! | --------------- | -------------------- | --------------------------------- |
//! | `PostcardCodec` | `T`                | deserializes into an owned value   |
//! | `ProstCodec`    | `T`                | deserializes into an owned value   |
//! | `RkyvCodec`     | `&'buf T::Archived` | validates and borrows the buffer   |
//!
//! Callers must therefore not assume `decode` produces an owned `T`.
//! `consortium_ipc::ReceivedMessage` wraps `Decoded<'buf>` and `Deref`s to `T`
//! where the codec allows it, so call sites written against it survive a codec
//! swap unchanged.
//!
//! Encoding is always into a caller-supplied `&mut [u8]` and returns the byte
//! count. There is no allocation in the trait signature, which is what lets the
//! same codec serve a `no_std` firmware channel and a Linux one.
//!
//! # Features
//!
//! `no_std` by default, and **there is no default codec** — a consumer names the
//! family it wants, so firmware does not pay for one it never calls.
//!
//! | Feature    | Exposes                                          |
//! | ---------- | ------------------------------------------------ |
//! | `postcard` | `PostcardCodec` (serde, compact, owned decode)   |
//! | `prost`    | `ProstCodec` (protobuf, owned decode)            |
//! | `rkyv`     | `RkyvCodec`, `TrustedArchive` (zero-copy)      |
//! | `alloc`    | heap-backed paths, including validated rkyv        |
//! | `std`      | `alloc` plus `tracing`-routed diagnostics          |
//! | `defmt`    | routes the same diagnostics to `defmt`             |
//!
//! # Example
//!
//! ```rust,ignore
//! use consortium_codec::{CodecFor, PostcardCodec};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Telemetry { sequence: u32, ready: bool }
//!
//! let mut buf = [0u8; 32];
//! let n = PostcardCodec::encode(&Telemetry { sequence: 7, ready: true }, &mut buf)?;
//! let back: Telemetry = <PostcardCodec as CodecFor<Telemetry>>::decode(&buf[..n])?;
//! ```
//!
//! # Implementing a codec
//!
//! Add a unit struct, `impl Codec` for it to declare `Error`, then
//! `impl<T> CodecFor<T> for MyCodec where T: <your serialization bound>`. Set
//! `Decoded<'buf>` to `T` for an owning codec or to a borrow of `'buf` for a
//! zero-copy one. Encode must report the exact byte count written and must fail
//! rather than truncate when `buf` is too small — a transport sends `buf[..n]`
//! verbatim.

#[cfg(feature = "postcard")]
pub(crate) mod postcard;

#[cfg(feature = "prost")]
pub(crate) mod prost;

#[cfg(feature = "rkyv")]
pub(crate) mod rkyv;

#[cfg(feature = "postcard")]
pub use postcard::PostcardCodec;

#[cfg(feature = "prost")]
pub use prost::ProstCodec;

#[cfg(feature = "rkyv")]
pub use rkyv::RkyvCodec;

/// A serialization family.
///
/// Implemented once per family (not per message type) so that the family's error
/// type is declared in one place and [`CodecFor<T>`] can stay a blanket impl over
/// whatever bound the underlying library needs. Codecs are zero-sized markers;
/// they are named as type parameters and never instantiated.
pub trait Codec: core::marker::Sized {
    /// Error returned by both [`CodecFor::encode`] and [`CodecFor::decode`].
    ///
    /// Typically the backing library's own error, widened where the codec adds
    /// failure modes of its own (a too-small destination buffer, say).
    type Error: core::fmt::Debug;
}

/// A [`Codec`] that can round-trip the message type `T`.
///
/// Both directions work against a caller-supplied slice, so the trait is usable
/// from `no_std` firmware with a `static` scratch buffer.
pub trait CodecFor<T>: Codec {
    /// What [`decode`](CodecFor::decode) yields for a buffer living as long as
    /// `'buf`.
    ///
    /// This is a type *constructor*, not `T`, which is what lets one trait cover
    /// both owning and zero-copy codecs: `PostcardCodec` sets it to `T`, while a
    /// zero-copy codec sets it to a borrow such as `&'buf T::Archived`. Callers
    /// must not assume decoding produces an owned value —
    /// `consortium_ipc::ReceivedMessage` exists to abstract over exactly this.
    type Decoded<'buf>: 'buf
    where
        T: 'buf;

    /// Serialize `msg` into `buf`, returning the number of bytes written.
    ///
    /// Implementations must never truncate: a transport transmits `buf[..n]`
    /// verbatim, so a destination that cannot hold the whole message is an error,
    /// not a partial write.
    fn encode(msg: &T, buf: &mut [u8]) -> core::result::Result<usize, Self::Error>;

    /// Deserialize a `T` from exactly the bytes in `buf`.
    ///
    /// `buf` is the received frame trimmed to its true length; trailing slack is
    /// not tolerated by every family. Zero-copy codecs borrow from `buf`, which is
    /// why the returned value is tied to `'buf`.
    fn decode<'buf>(buf: &'buf [u8]) -> core::result::Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf;
}

#[cfg(feature = "rkyv")]
pub use consortium_codec_macros::TrustedArchive;

#[cfg(feature = "rkyv")]
pub use crate::rkyv::TrustedArchive;
