use core::marker::PhantomData;
use core::ops::Deref;

use crate::chan::Chan;
use crate::transport::Transport;

/// Serializes and deserializes messages into/from byte slices.
///
/// Implemented today by [`PostcardCodec`].
/// Can be implemented by a zero-copy backend (e.g. rkyv) later
/// without changing [`Channel`] or application code.
///
/// # Contract
///
/// - `encode` writes into a caller-provided buffer and returns
///   the number of bytes written. Buffer is sized from
///   [`Transport::max_message_size`].
///
/// - `decode` borrows from the input slice and returns a handle
///   whose lifetime is tied to that slice. No allocation required.
///
/// Both operations must be usable in `no_std` without a global
/// allocator.
pub trait Codec {
    /// The decoded view of a message.
    ///
    /// `PostcardCodec`  → owned `T`  (copied off the wire)
    /// Future RkyvCodec → `&'buf T::Archived` (zero-copy borrow)
    ///
    /// The `'buf` lifetime allows zero-copy codecs to tie the
    /// decoded value's lifetime to the receive buffer, while
    /// postcard just ignores it and returns an owned value.
    type Decoded<'buf, T: 'buf>
    where
        T: 'buf;

    type Error: core::fmt::Debug;

    /// Serialize `msg` into `buf`.
    /// Returns the number of bytes written.
    fn encode<T>(msg: &T, buf: &mut [u8]) -> Result<usize, Self::Error>
    where
        T: serde::Serialize;

    /// Deserialize a `T` from `buf`.
    fn decode<'buf, T>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf, T>, Self::Error>
    where
        T: serde::de::DeserializeOwned + 'buf;
}

/// [`Codec`] implementation backed by `postcard`.
///
/// - `no_std`, no alloc required
/// - compact wire format
/// - any `Serialize + DeserializeOwned` type works out of the box
///
/// Switching to a zero-copy codec later only requires defining a
/// new `Codec` implementor and changing the `C` type parameter on
/// [`Channel`]. Transport, Doorbell, and application logic are
/// untouched.
pub struct PostcardCodec;

impl Codec for PostcardCodec {
    type Decoded<'buf, T: 'buf>
        = T
    where
        T: 'buf;

    type Error = postcard::Error;

    fn encode<T>(msg: &T, buf: &mut [u8]) -> Result<usize, Self::Error>
    where
        T: serde::Serialize,
    {
        let written = postcard::to_slice(msg, buf)?;
        Ok(written.len())
    }

    fn decode<'buf, T>(buf: &'buf [u8]) -> Result<Self::Decoded<'buf, T>, Self::Error>
    where
        T: serde::de::DeserializeOwned + 'buf,
    {
        postcard::from_bytes(buf)
    }
}

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
/// ```rust,no_run
/// static mut BUF: [u8; 256] = [0u8; 256];
/// let ch = Channel::<Tx, MyMsg, _>::new(chan, transport, unsafe { &mut BUF });
/// ```
///
/// The buffer must be at least `transport.max_message_size()` bytes.
pub struct Channel<D, T, Tr: Transport, C: Codec = PostcardCodec> {
    chan: Chan,
    transport: Tr,
    buf: &'static mut [u8],
    _dir: PhantomData<D>,
    _msg: PhantomData<T>,
    _codec: PhantomData<C>,
}

impl<D, T, Tr: Transport, C: Codec> Channel<D, T, Tr, C> {
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
    T: serde::Serialize,
    Tr: Transport,
    C: Codec,
{
    /// Encode `msg` with the codec and hand the bytes to the transport.
    ///
    /// The transport rings the doorbell internally.  
    /// Returns when bytes are handed off — not when the remote has
    /// processed them.
    pub async fn send(&mut self, msg: &T) -> Result<(), ChannelError<C, Tr>> {
        let n = C::encode(msg, self.buf).map_err(ChannelError::Codec)?;

        self.transport
            .send(self.chan, &self.buf[..n])
            .await
            .map_err(ChannelError::Transport)
    }
}

impl<T, Tr, C> Channel<Rx, T, Tr, C>
where
    T: serde::de::DeserializeOwned,
    Tr: Transport,
    C: Codec,
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
            .recv(self.chan, self.buf)
            .await
            .map_err(ChannelError::Transport)?;

        let decoded = C::decode::<T>(&self.buf[..n]).map_err(ChannelError::Codec)?;

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
pub struct ReceivedMessage<'a, T: 'a, C: Codec> {
    value: C::Decoded<'a, T>,
    /// Carries the `'a` lifetime for zero-copy codecs.
    /// Unused by PostcardCodec but harmless.
    _marker: PhantomData<&'a ()>,
}

impl<'a, T, C: Codec> ReceivedMessage<'a, T, C> {
    /// Consume the handle and return the inner decoded value.
    pub fn into_inner(self) -> C::Decoded<'a, T> {
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
    C: Codec<Decoded<'a, T> = T>,
    T: 'a,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T, C> core::fmt::Debug for ReceivedMessage<'a, T, C>
where
    C: Codec,
    C::Decoded<'a, T>: core::fmt::Debug,
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
