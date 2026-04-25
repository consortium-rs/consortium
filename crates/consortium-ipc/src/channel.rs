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

//! Typed, directed communication channels over a [`Transport`].
//!
//! `Channel` is the main API for application code. It provides typed `send()` and `recv()` methods that encode and decode messages via a [`Codec`] and hand bytes to the underlying transport.
use consortium_common::codec::{Codec, CodecFor};
use core::marker::PhantomData;
use core::ops::Deref;

use crate::chan::Chan;
use crate::transport::Transport;

/// Marker: this channel can only send.
pub struct Tx;

/// Marker: this channel can only receive.
pub struct Rx;

/// A typed, directed communication endpoint over a [`Transport`].
///
/// # Type parameters
///
/// - `D`  — direction: [`Tx`] or [`Rx`]
/// - `T`  — message type
/// - `Tr` — underlying transport (HSEM, MU, serial, host socket…)
/// - `C`  — codec, defaults to [`PostcardCodec`]
///
/// # Buffer ownership
///
/// `Channel` does not allocate. The caller provides a `&'static mut [u8]`
/// scratch buffer at construction. In `no_std` environments this is
/// typically a `static mut` array:
///
/// ```compile_fail
/// static mut BUF: [u8; 256] = [0u8; 256];
/// let ch = Channel::<Tx, MyMsg, _>::new(chan, transport, unsafe { &mut BUF });
/// ```
///
/// The buffer must be at least `transport.max_message_size()` bytes.
pub struct Channel<D, T, Tr: Transport, C: CodecFor<T>> {
    chan: Chan,
    transport: Tr,
    buf: &'static mut [u8],
    _dir: PhantomData<D>,
    _msg: PhantomData<T>,
    _codec: PhantomData<C>,
}

impl<D, T, Tr: Transport, C: CodecFor<T>> Channel<D, T, Tr, C> {
    /// Construct a [`Channel`].
    ///
    /// `chan`      — validated channel identifier  
    /// `transport` — initialised transport  
    /// `buf`       — scratch buffer, at least `transport.max_message_size()` bytes
    pub fn new(chan: Chan, transport: Tr, buf: &'static mut [u8]) -> Self {
        Self {
            chan,
            transport,
            buf,
            _dir: PhantomData,
            _msg: PhantomData,
            _codec: PhantomData,
        }
    }

    /// The [`Chan`] this endpoint is bound to.
    pub fn chan(&self) -> Chan {
        self.chan
    }
}

impl<T, Tr, C> Channel<Tx, T, Tr, C>
where
    Tr: Transport,
    C: CodecFor<T>,
{
    /// Encode `msg` with the codec and hand the bytes to the transport.
    ///
    /// The transport rings the doorbell internally.  
    /// Returns when bytes are handed off — not when the remote has
    /// processed them.
    pub async fn send(&mut self, msg: &T) -> Result<(), ChannelError<C, Tr>> {
        let n = C::encode(msg, self.buf).map_err(ChannelError::Codec)?;

        self.transport
            .send(self.buf[..n].as_ref())
            .await
            .map_err(ChannelError::Transport)
    }
}

impl<T, Tr, C> Channel<Rx, T, Tr, C>
where
    Tr: Transport,
    C: CodecFor<T>,
{
    /// Await the next message from the transport and decode it.
    ///
    /// Blocks until the transport's doorbell fires and bytes arrive.  
    /// Returns a [`ReceivedMessage`] wrapping the decoded value.
    ///
    /// When `C = PostcardCodec` the message is an owned `T`.  
    /// When `C` is a future zero-copy codec the returned handle will
    /// borrow the internal buffer — the `'_` lifetime on
    /// [`ReceivedMessage`] already accommodates this without any
    /// change to this signature.
    pub async fn recv(&mut self) -> Result<ReceivedMessage<'_, T, C>, ChannelError<C, Tr>> {
        let n = self
            .transport
            .recv(self.buf)
            .await
            .map_err(ChannelError::Transport)?;

        let decoded = C::decode(&self.buf[..n]).map_err(ChannelError::Codec)?;

        Ok(ReceivedMessage {
            value: decoded,
            _marker: PhantomData,
        })
    }
}

/// A handle to a decoded received message.
///
/// For [`PostcardCodec`] this owns the decoded `T` — the `'a`
/// lifetime is unused and the value can be moved out freely via
/// [`into_inner`](ReceivedMessage::into_inner).
///
/// For a future zero-copy codec `'a` will borrow the channel's
/// internal receive buffer, preventing a second `recv()` call
/// while this handle is live. Application code using [`Deref`]
/// does not need to change.
pub struct ReceivedMessage<'a, T: 'a, C: CodecFor<T>> {
    value: C::Decoded<'a>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T, C: CodecFor<T>> ReceivedMessage<'a, T, C> {
    /// Consume the handle and return the inner decoded value.
    pub fn into_inner(self) -> C::Decoded<'a> {
        self.value
    }
}

// Deref is only implementable when C::Decoded<'a, T> derefs to T.
// PostcardCodec: Decoded = T, which derefs trivially via blanket impl.
// RkyvCodec:     Decoded = &T::Archived, which derefs to T::Archived.
//
// We provide the postcard case explicitly. The rkyv case will require
// a separate impl block or a newtype once that codec is written.
impl<'a, T, C> Deref for ReceivedMessage<'a, T, C>
where
    C: CodecFor<T>,
    C::Decoded<'a>: core::ops::Deref<Target = T>,
    T: 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T, C> core::fmt::Debug for ReceivedMessage<'a, T, C>
where
    C: CodecFor<T>,
    C::Decoded<'a>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

/// Unified error for [`Channel`] operations.
///
/// Separates codec failures from transport failures so callers
/// can handle them independently.
#[derive(Debug)]
pub enum ChannelError<C: Codec, Tr: Transport> {
    Codec(C::Error),
    Transport(Tr::Error),
}

impl<C: Codec, Tr: Transport> core::fmt::Display for ChannelError<C, Tr>
where
    C::Error: core::fmt::Display,
    Tr::Error: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Codec(e) => write!(f, "codec error: {e}"),
            Self::Transport(e) => write!(f, "transport error: {e}"),
        }
    }
}
