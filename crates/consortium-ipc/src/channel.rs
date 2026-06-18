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
use consortium_codec::{Codec, CodecFor};
use core::marker::PhantomData;
use core::ops::Deref;

use crate::chan::Chan;
use crate::transport::{RecvTransport, SendTransport, TransportError};

/// Marker: this channel can only send.
pub struct Tx;

/// Marker: this channel can only receive.
pub struct Rx;

/// A typed, directed communication endpoint over a transport half.
///
/// # Type parameters
///
/// - `D`  - direction: [`Tx`] or [`Rx`]
/// - `T`  - message type
/// - `Tr` - send half ([`SendTransport`]) for `Tx`, receive half ([`RecvTransport`]) for `Rx`
/// - `C`  - codec, defaults to [`PostcardCodec`]
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
/// The buffer must be at least `transport.max_send_size()` / `max_recv_size()` bytes.
pub struct Channel<D, T, Tr, C: CodecFor<T>> {
    chan: Chan,
    transport: Tr,
    buf: &'static mut [u8],
    _dir: PhantomData<D>,
    _msg: PhantomData<T>,
    _codec: PhantomData<C>,
}

impl<D, T, Tr, C: CodecFor<T>> Channel<D, T, Tr, C> {
    /// Construct a [`Channel`].
    ///
    /// `chan`      - validated channel identifier
    /// `transport` - initialised transport half
    /// `buf`       - scratch buffer, sized for the transport MTU
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
    Tr: SendTransport,
    C: CodecFor<T>,
{
    /// Encode `msg` with the codec and hand the bytes to the transport.
    ///
    /// The transport rings the doorbell internally.  
    /// Returns when bytes are handed off - not when the remote has
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
    Tr: RecvTransport,
    C: CodecFor<T>,
{
    /// Await the next message from the transport and decode it.
    ///
    /// Blocks until the transport's doorbell fires and bytes arrive.  
    /// Returns a [`ReceivedMessage`] wrapping the decoded value.
    ///
    /// When `C = PostcardCodec` the message is an owned `T`.  
    /// When `C` is a future zero-copy codec the returned handle will
    /// borrow the internal buffer - the `'_` lifetime on
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
/// For [`PostcardCodec`] this owns the decoded `T` - the `'a`
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
pub enum ChannelError<C: Codec, Tr: TransportError> {
    Codec(C::Error),
    Transport(Tr::Error),
}

impl<C: Codec, Tr: TransportError> core::fmt::Debug for ChannelError<C, Tr>
where
    C::Error: core::fmt::Debug,
    Tr::Error: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Codec(err) => f.debug_tuple("Codec").field(err).finish(),
            Self::Transport(err) => f.debug_tuple("Transport").field(err).finish(),
        }
    }
}

impl<C: Codec, Tr: TransportError> core::fmt::Display for ChannelError<C, Tr>
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

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::fmt;
    use futures::executor::block_on;
    use futures::future::{Ready, ready};
    use std::boxed::Box;
    use std::vec::Vec;

    #[derive(Debug, PartialEq)]
    struct Message {
        sequence: u8,
    }

    #[derive(Debug, PartialEq)]
    enum CodecError {
        Encode,
        Decode,
    }

    impl fmt::Display for CodecError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }

    struct MockCodec;

    impl Codec for MockCodec {
        type Error = CodecError;
    }

    impl CodecFor<Message> for MockCodec {
        type Decoded<'buf>
            = Message
        where
            Message: 'buf;

        fn encode(msg: &Message, buf: &mut [u8]) -> Result<usize, Self::Error> {
            if msg.sequence == 0xFF || buf.is_empty() {
                return Err(CodecError::Encode);
            }
            buf[0] = msg.sequence;
            Ok(1)
        }

        fn decode<'buf>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf>, Self::Error>
        where
            Message: 'buf,
        {
            match buf {
                [0xEE, ..] | [] => Err(CodecError::Decode),
                [sequence, ..] => Ok(Message {
                    sequence: *sequence,
                }),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum TransportError {
        Send,
        Recv,
        BufferTooSmall,
    }

    impl fmt::Display for TransportError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[derive(Default)]
    struct MockTransport {
        sent: Vec<u8>,
        recv_payload: Vec<u8>,
        fail_send: bool,
        fail_recv: bool,
    }

    impl crate::transport::TransportError for MockTransport {
        type Error = TransportError;
    }

    impl crate::transport::SendTransport for MockTransport {
        type SendFut<'a>
            = Ready<Result<(), Self::Error>>
        where
            Self: 'a;

        fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a> {
            if self.fail_send {
                return ready(Err(TransportError::Send));
            }
            self.sent.clear();
            self.sent.extend_from_slice(data);
            ready(Ok(()))
        }

        fn max_send_size(&self) -> usize {
            64
        }
    }

    impl crate::transport::RecvTransport for MockTransport {
        type RecvFut<'a>
            = Ready<Result<usize, Self::Error>>
        where
            Self: 'a;

        fn recv<'a>(&mut self, buf: &mut [u8]) -> Self::RecvFut<'a> {
            if self.fail_recv {
                return ready(Err(TransportError::Recv));
            }
            if self.recv_payload.len() > buf.len() {
                return ready(Err(TransportError::BufferTooSmall));
            }
            let n = self.recv_payload.len();
            buf[..n].copy_from_slice(&self.recv_payload);
            ready(Ok(n))
        }

        fn max_recv_size(&self) -> usize {
            64
        }
    }

    impl crate::transport::Transport for MockTransport {}

    fn scratch<const N: usize>() -> &'static mut [u8] {
        Box::leak(Box::new([0; N])).as_mut_slice()
    }

    #[test]
    fn tx_channel_encodes_message_and_sends_bytes() {
        let ch = unsafe { Chan::new_unchecked(3) };
        let transport = MockTransport::default();
        let mut channel = Channel::<Tx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        block_on(channel.send(&Message { sequence: 42 })).expect("send succeeds");

        assert_eq!(channel.chan(), ch);
        assert_eq!(channel.transport.sent, [42]);
    }

    #[test]
    fn tx_channel_reports_codec_error_before_transport_send() {
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = MockTransport::default();
        let mut channel = Channel::<Tx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        let err = block_on(channel.send(&Message { sequence: 0xFF }))
            .expect_err("codec encode should fail");

        assert!(matches!(err, ChannelError::Codec(CodecError::Encode)));
        assert!(channel.transport.sent.is_empty());
    }

    #[test]
    fn tx_channel_reports_transport_send_error() {
        let ch = unsafe { Chan::new_unchecked(0) };
        let transport = MockTransport {
            fail_send: true,
            ..MockTransport::default()
        };
        let mut channel = Channel::<Tx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        let err = block_on(channel.send(&Message { sequence: 7 }))
            .expect_err("transport send should fail");

        assert!(matches!(err, ChannelError::Transport(TransportError::Send)));
    }

    #[test]
    fn rx_channel_receives_bytes_and_decodes_message() {
        let ch = unsafe { Chan::new_unchecked(1) };
        let transport = MockTransport {
            recv_payload: std::vec![9],
            ..MockTransport::default()
        };
        let mut channel = Channel::<Rx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        let decoded = block_on(channel.recv())
            .expect("recv succeeds")
            .into_inner();

        assert_eq!(decoded, Message { sequence: 9 });
    }

    #[test]
    fn rx_channel_reports_transport_recv_error() {
        let ch = unsafe { Chan::new_unchecked(1) };
        let transport = MockTransport {
            fail_recv: true,
            ..MockTransport::default()
        };
        let mut channel = Channel::<Rx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        let err = block_on(channel.recv()).expect_err("transport recv should fail");

        assert!(matches!(err, ChannelError::Transport(TransportError::Recv)));
    }

    #[test]
    fn rx_channel_reports_codec_decode_error() {
        let ch = unsafe { Chan::new_unchecked(1) };
        let transport = MockTransport {
            recv_payload: std::vec![0xEE],
            ..MockTransport::default()
        };
        let mut channel = Channel::<Rx, Message, _, MockCodec>::new(ch, transport, scratch::<8>());

        let err = block_on(channel.recv()).expect_err("codec decode should fail");

        assert!(matches!(err, ChannelError::Codec(CodecError::Decode)));
    }
}
