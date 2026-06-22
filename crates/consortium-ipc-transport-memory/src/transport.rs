use core::{
    future::Future,
    marker::{PhantomData, PhantomPinned},
};

use consortium_ipc::{
    Chan, DdrRegion, Doorbell, RecvTransport, Region, SendTransport, SramRegion, Transport,
    TransportError,
};

use crate::{
    descriptor::{ChannelLayout, NUM_CHANNELS_OFFSET, RX_UPDATED_ID_OFFSET, read_channel},
    error::Error,
    slot_internal as slot,
    utils::rv_u8,
};

// ---------------------------------------------------------------------------
// Shared send helper — avoids duplicating slot-write logic
// ---------------------------------------------------------------------------

/// # Safety
/// `base` must be valid for reads/writes covering the full descriptor layout.
unsafe fn send_bytes<D: Doorbell, R: Region>(
    ch: Chan,
    doorbell: &D,
    region: &R,
    base: *mut u8,
    data: &[u8],
) -> Result<(), Error<D>> {
    let mut channel_id = 0xFF;

    // SAFETY: caller guarantees `base` covers the descriptor header.
    let num_channels = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

    for chan in 0..num_channels {
        // SAFETY: `chan` is within the validated channel table range.
        let channel = unsafe { read_channel(base, chan as usize) };
        // SAFETY: `channel` was read from the descriptor; slot pointer is within the region.
        let slot_addr = unsafe { channel.slot_addr(base) };
        // SAFETY: `slot_addr` points to a valid slot in the shared memory region.
        let consumed = unsafe { slot::is_consumed(slot_addr) };
        if consumed && channel.direction.is_tx() {
            channel_id = chan;
            break;
        }
    }

    if channel_id == 0xFF {
        return Err(Error::NoChannel);
    }

    // SAFETY: `channel_id` was validated against `num_channels` above.
    let channel = unsafe { read_channel(base, channel_id as usize) };
    let capacity = (channel.size as usize)
        .checked_sub(slot::SLOT_HEADER)
        .ok_or(Error::PayloadTooLarge {
            payload_len: data.len(),
            capacity: 0,
        })?;

    if data.len() > capacity {
        return Err(Error::PayloadTooLarge {
            payload_len: data.len(),
            capacity,
        });
    }

    // SAFETY: `channel` was read from the descriptor; slot pointer is within the region.
    let slot_base = unsafe { channel.slot_addr(base) };
    // SAFETY: `slot_base` points to a valid, capacity-checked slot.
    unsafe { slot::write(slot_base, data) };

    region.flush();
    doorbell.ring(ch).map_err(Error::Doorbell)
}

// ---------------------------------------------------------------------------
// Shared recv-poll helper — slot scanning + persistent doorbell subscription
// ---------------------------------------------------------------------------

/// Poll the recv path once.
///
/// `wait_fut` is persistent storage owned by the caller's future struct.  On
/// the first `Pending` it is populated with a doorbell wait future so the waker
/// registration survives across executor yields.  On re-poll after a wakeup the
/// stored future is polled first; only if it fires do we re-scan the slot.
///
/// # Safety
/// `base` must cover the descriptor header.  The doorbell must outlive `*wait_fut`.
unsafe fn poll_recv<D: Doorbell + Send + 'static, R: SharedMemoryTransportable>(
    doorbell: &D,
    region: &R,
    base: *mut u8,
    ch: Chan,
    buf: &mut [u8],
    wait_fut: &mut Option<D::WaitFut<'static>>,
    cx: &mut core::task::Context<'_>,
) -> core::task::Poll<Result<usize, Error<D>>> {
    // If a wait future is already armed, poll it.  If it is still pending our
    // waker is already registered — park until the doorbell fires.  If it fires,
    // clear it and fall through to re-scan the slot.
    if let Some(ref mut wf) = *wait_fut {
        // SAFETY: `wf` lives in the caller's pinned struct and is never moved.
        match unsafe { core::pin::Pin::new_unchecked(wf) }.poll(cx) {
            core::task::Poll::Pending => return core::task::Poll::Pending,
            core::task::Poll::Ready(_) => {
                *wait_fut = None;
            }
        }
    }

    // Fast path: check the channel indicated by the updated-id field.
    // SAFETY: caller guarantees `base` covers the descriptor header.
    let mut channel_id = unsafe { rv_u8(base, RX_UPDATED_ID_OFFSET) };
    // SAFETY: same `base` validity guarantee.
    let channel_num = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

    let mut channel: Option<ChannelLayout> = if channel_id < channel_num {
        // SAFETY: `channel_id` is within the validated channel table range.
        let layout = unsafe { read_channel(base, channel_id as usize) };
        // SAFETY: slot pointer is within the region.
        let slot_addr = unsafe { layout.slot_addr(base) };
        // SAFETY: `slot_addr` points to a valid slot.
        let consumed = unsafe { slot::is_consumed(slot_addr) };
        if layout.direction.is_rx() && !consumed {
            Some(layout)
        } else {
            None
        }
    } else {
        None
    };

    // Fallback: scan all channels.
    if channel.is_none() {
        for chan in 0..channel_num {
            // SAFETY: `chan` is within the channel table range.
            let layout = unsafe { read_channel(base, chan as usize) };
            // SAFETY: slot pointer is within the region.
            let slot_addr = unsafe { layout.slot_addr(base) };
            // SAFETY: `slot_addr` points to a valid slot.
            let consumed = unsafe { slot::is_consumed(slot_addr) };
            if layout.direction.is_rx() && !consumed {
                channel_id = chan;
                channel = Some(layout);
                break;
            }
        }
    }

    if channel.is_none() {
        // No data yet — arm the doorbell so the waker stays registered until
        // the remote calls ring().
        return unsafe { arm_wait(doorbell, ch, wait_fut, cx) };
    }

    // SAFETY: `channel_id` was verified against `channel_num` during scan.
    let channel = unsafe { read_channel(base, channel_id as usize) };
    // SAFETY: slot pointer is within the region.
    let slot_base = unsafe { channel.slot_addr(base) };

    // SAFETY: `slot_base` is a valid slot; `buf` is exclusively owned by the caller.
    if let Some(n) = unsafe { slot::try_read(slot_base, buf) } {
        return core::task::Poll::Ready(Ok(n));
    }

    // Slot was visible but integrity check failed — invalidate caches and wait.
    region.invalidate();
    unsafe { arm_wait(doorbell, ch, wait_fut, cx) }
}

/// Create and store a doorbell wait future, poll it once to register the waker,
/// then return `Pending`.
///
/// If a stored notification is immediately available (the remote rang before we
/// subscribed), the future fires on the first poll: we clear it and schedule an
/// immediate re-poll so the slot is re-checked on the very next executor cycle.
///
/// # Safety
/// The doorbell must outlive `*wait_fut`.  `wait_fut` must point to a field in a
/// pinned struct (i.e. the caller's future is `!Unpin` and pinned by the executor)
/// so the stored future is never moved after it has been polled.
unsafe fn arm_wait<D: Doorbell + Send + 'static>(
    doorbell: &D,
    ch: Chan,
    wait_fut: &mut Option<D::WaitFut<'static>>,
    cx: &mut core::task::Context<'_>,
) -> core::task::Poll<Result<usize, Error<D>>> {
    let wf = doorbell.wait(ch);
    // SAFETY: the doorbell is inside the transport that exclusively owns the recv
    // future, and the transport is guaranteed to outlive the future.  Erasing the
    // lifetime to 'static is sound because `wait_fut` is dropped before the
    // transport (enforced by the PhantomData<&'a mut Transport> on the future struct).
    let wf = unsafe { core::mem::transmute::<D::WaitFut<'_>, D::WaitFut<'static>>(wf) };
    *wait_fut = Some(wf);
    // SAFETY: `wait_fut` is now in a pinned location; we never move it after this poll.
    match unsafe { core::pin::Pin::new_unchecked(wait_fut.as_mut().unwrap()) }.poll(cx) {
        core::task::Poll::Ready(_) => {
            // A notification was already stored (remote rang before we subscribed).
            // Clear the future and ask the executor to re-poll immediately.
            *wait_fut = None;
            cx.waker().wake_by_ref();
        }
        core::task::Poll::Pending => {}
    }
    core::task::Poll::Pending
}

// ---------------------------------------------------------------------------
// Combined transport
// ---------------------------------------------------------------------------

pub struct SharedMemoryTransport<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    /// Base pointer to offset 0x00 of the shared memory region (descriptor header).
    base: *mut u8,
    tx_mtu: u32,
    rx_mtu: u32,
}

impl<D: Doorbell, R: SharedMemoryTransportable> SharedMemoryTransport<D, R> {
    /// Construct a `SharedMemoryTransport` for a single channel.
    ///
    /// # Safety
    ///
    /// - `base` must be valid for reads and writes for at least the descriptor
    ///   header (0x20 bytes) plus the channel table (32 × 0x10 bytes).
    /// - No other Rust reference to the shared memory region may exist while
    ///   this transport is in use.
    /// - The memory must be accessible to both this core and the remote core.
    pub unsafe fn new(ch: Chan, doorbell: D, region: R, base: *mut u8) -> Self {
        Self {
            ch,
            doorbell,
            region,
            base,
            tx_mtu: 0,
            rx_mtu: 0,
        }
    }

    /// Consume this transport and return a `(send, recv)` half-pair.
    ///
    /// A single `SharedMemoryTransport` cannot be held by two [`Channel`]s at once
    /// because both would need `&mut` access.  `split` solves this by producing two
    /// independent owned halves:
    ///
    /// - [`SharedMemoryTransportTx`] implements [`SendTransport`] — pass it to
    ///   `Channel::<Tx, …>`.
    /// - [`SharedMemoryTransportRx`] implements [`RecvTransport`] — pass it to
    ///   `Channel::<Rx, …>`.
    ///
    /// Both halves alias the same shared memory region; the same aliasing invariants
    /// as [`new`](Self::new) apply.  The doorbell and region are cloned for the Rx
    /// half; the originals move into the Tx half.
    ///
    /// [`Channel`]: consortium_ipc::Channel
    /// [`SendTransport`]: consortium_ipc::SendTransport
    /// [`RecvTransport`]: consortium_ipc::RecvTransport
    pub fn split(self) -> (SharedMemoryTransportTx<D, R>, SharedMemoryTransportRx<D, R>)
    where
        D: Clone,
        R: Clone,
    {
        let rx_doorbell = self.doorbell.clone();
        let rx_region = self.region.clone();
        (
            SharedMemoryTransportTx {
                ch: self.ch,
                doorbell: self.doorbell,
                region: self.region,
                base: self.base,
                tx_mtu: self.tx_mtu,
            },
            SharedMemoryTransportRx {
                ch: self.ch,
                doorbell: rx_doorbell,
                region: rx_region,
                base: self.base,
                rx_mtu: self.rx_mtu,
            },
        )
    }
}

// ---------------------------------------------------------------------------
// Send / receive halves
// ---------------------------------------------------------------------------

/// Send-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportTx<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    base: *mut u8,
    tx_mtu: u32,
}

/// Receive-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportRx<D: Doorbell, R: SharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    base: *mut u8,
    rx_mtu: u32,
}

// SAFETY: `base` is an exclusive pointer to a shared memory region managed by this
// transport.  No shared reference to the pointed-to memory is ever handed out through
// the safe API; the only aliasing is the intentional inter-processor sharing modelled
// by the doorbell/region abstraction.  Sending the transport to another thread is
// sound when D and R are both Send.
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send
    for SharedMemoryTransport<D, R>
{
}
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send
    for SharedMemoryTransportTx<D, R>
{
}
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send
    for SharedMemoryTransportRx<D, R>
{
}

// ---------------------------------------------------------------------------
// RecvAccess — common field accessor used by RecvFut
// ---------------------------------------------------------------------------

/// Sealed accessor so `RecvFut` can be generic over both `SharedMemoryTransport`
/// and `SharedMemoryTransportRx` without duplicating the future type.
pub trait RecvAccess {
    type D: Doorbell + 'static;
    type R: SharedMemoryTransportable;
    fn doorbell(&self) -> &Self::D;
    fn region(&self) -> &Self::R;
    fn base(&self) -> *mut u8;
    fn ch(&self) -> Chan;
}

impl<D: Doorbell + 'static, R: SharedMemoryTransportable> RecvAccess
    for SharedMemoryTransport<D, R>
{
    type D = D;
    type R = R;
    fn doorbell(&self) -> &D {
        &self.doorbell
    }
    fn region(&self) -> &R {
        &self.region
    }
    fn base(&self) -> *mut u8 {
        self.base
    }
    fn ch(&self) -> Chan {
        self.ch
    }
}

impl<D: Doorbell + 'static, R: SharedMemoryTransportable> RecvAccess
    for SharedMemoryTransportRx<D, R>
{
    type D = D;
    type R = R;
    fn doorbell(&self) -> &D {
        &self.doorbell
    }
    fn region(&self) -> &R {
        &self.region
    }
    fn base(&self) -> *mut u8 {
        self.base
    }
    fn ch(&self) -> Chan {
        self.ch
    }
}

// ---------------------------------------------------------------------------
// Futures
// ---------------------------------------------------------------------------

/// Future returned by [`SendTransport::send`] on shared-memory transports.
///
/// The send is completed eagerly inside `send()`; this future simply wraps the
/// pre-computed result and is always `Ready` on the first poll.  The `'a`
/// lifetime ties the future to the `&mut self` borrow of the transport so the
/// borrow checker enforces correct ownership at the call site.
///
/// Both [`SharedMemoryTransport`] and [`SharedMemoryTransportTx`] use this same
/// type; neither `D` nor `R` is referenced at runtime (the result is already
/// computed), but both must appear as type parameters so the GAT `where Self: 'a`
/// bound — which implies `D: 'a` and `R: 'a` — can be satisfied.
pub struct SendFut<'a, D: Doorbell, R: SharedMemoryTransportable> {
    result: Result<(), Error<D>>,
    _ph: PhantomData<(&'a D, &'a R)>,
}

/// Future returned by [`RecvTransport::recv`] on shared-memory transports.
///
/// Generic over any transport type `T` that implements [`RecvAccess`], so a
/// single concrete type covers both [`SharedMemoryTransport`] and
/// [`SharedMemoryTransportRx`].
pub struct RecvFut<'a, T: RecvAccess> {
    transport: *mut T,
    buf_ptr: *mut u8,
    buf_len: usize,
    /// Persisted doorbell wait future. Kept alive between polls so the waker
    /// registration is not lost when the executor parks this task.
    wait_fut: Option<<T::D as Doorbell>::WaitFut<'static>>,
    _ph: PhantomData<&'a mut T>,
    _pin: PhantomPinned,
}

// SAFETY: the pre-computed result contains no non-Send types; D::Error is Send when D: Send.
unsafe impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> Send for SendFut<'_, D, R> {}
// SAFETY: the raw pointer is exclusively owned by this future for its lifetime;
// all concrete T, T::D, and T::R types used here are Send.
unsafe impl<T: RecvAccess + Send> Send for RecvFut<'_, T>
where
    T::D: Send,
    T::R: Send,
{
}

// ---------------------------------------------------------------------------
// Future impls — eager send
// ---------------------------------------------------------------------------

impl<D: Doorbell, R: SharedMemoryTransportable> Future for SendFut<'_, D, R> {
    type Output = Result<(), Error<D>>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // SAFETY: `SendFut` is never moved after being polled; result is stored inline.
        let this = unsafe { self.get_unchecked_mut() };
        core::task::Poll::Ready(match &this.result {
            Ok(()) => Ok(()),
            Err(_) => core::mem::replace(&mut this.result, Ok(())),
        })
    }
}

// ---------------------------------------------------------------------------
// Future impl — async recv (delegates to poll_recv helper)
// ---------------------------------------------------------------------------

impl<T: RecvAccess> Future for RecvFut<'_, T>
where
    T::D: Send + 'static,
    T::R: Send,
{
    type Output = Result<usize, Error<T::D>>;

    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // SAFETY: we never move fields out of `this`; `_pin: PhantomPinned` makes this !Unpin.
        let this = unsafe { self.get_unchecked_mut() };
        // SAFETY: `transport` is exclusively owned by this future.
        let transport = unsafe { &mut *this.transport };
        // SAFETY: `buf_ptr` and `buf_len` are exclusively owned by this future.
        let buf = unsafe { core::slice::from_raw_parts_mut(this.buf_ptr, this.buf_len) };
        // SAFETY: `base` covers the descriptor header; doorbell outlives `this.wait_fut`.
        unsafe {
            poll_recv(
                transport.doorbell(),
                transport.region(),
                transport.base(),
                transport.ch(),
                buf,
                &mut this.wait_fut,
                cx,
            )
        }
    }
}

// ---------------------------------------------------------------------------
// TransportError impls
// ---------------------------------------------------------------------------

impl<D: Doorbell, R: SharedMemoryTransportable> TransportError for SharedMemoryTransport<D, R> {
    type Error = Error<D>;
}

impl<D: Doorbell, R: SharedMemoryTransportable> TransportError for SharedMemoryTransportTx<D, R> {
    type Error = Error<D>;
}

impl<D: Doorbell, R: SharedMemoryTransportable> TransportError for SharedMemoryTransportRx<D, R> {
    type Error = Error<D>;
}

// ---------------------------------------------------------------------------
// SendTransport impls
// ---------------------------------------------------------------------------

impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransport<D, R>
{
    type SendFut<'a>
        = SendFut<'a, D, R>
    where
        Self: 'a;

    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a> {
        // SAFETY: `self.base` satisfies the descriptor layout invariant established in `new`.
        let result = unsafe { send_bytes(self.ch, &self.doorbell, &self.region, self.base, data) };
        SendFut {
            result,
            _ph: PhantomData,
        }
    }

    fn max_send_size(&self) -> usize {
        self.tx_mtu as usize
    }
}

impl<D: Doorbell + Send, R: SharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransportTx<D, R>
{
    type SendFut<'a>
        = SendFut<'a, D, R>
    where
        Self: 'a;

    fn send<'a>(&mut self, data: &[u8]) -> Self::SendFut<'a> {
        // SAFETY: `self.base` satisfies the descriptor layout invariant established by `split`.
        let result = unsafe { send_bytes(self.ch, &self.doorbell, &self.region, self.base, data) };
        SendFut {
            result,
            _ph: PhantomData,
        }
    }

    fn max_send_size(&self) -> usize {
        self.tx_mtu as usize
    }
}

// ---------------------------------------------------------------------------
// RecvTransport impls
// ---------------------------------------------------------------------------

impl<D: Doorbell + Send + 'static, R: SharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransport<D, R>
{
    type RecvFut<'a>
        = RecvFut<'a, SharedMemoryTransport<D, R>>
    where
        Self: 'a;

    fn recv<'a>(&mut self, data: &mut [u8]) -> Self::RecvFut<'a> {
        RecvFut {
            transport: self as *mut _,
            buf_ptr: data.as_mut_ptr(),
            buf_len: data.len(),
            wait_fut: None,
            _ph: PhantomData,
            _pin: PhantomPinned,
        }
    }

    fn max_recv_size(&self) -> usize {
        self.rx_mtu as usize
    }
}

impl<D: Doorbell + Send + 'static, R: SharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransportRx<D, R>
{
    type RecvFut<'a>
        = RecvFut<'a, SharedMemoryTransportRx<D, R>>
    where
        Self: 'a;

    fn recv<'a>(&mut self, data: &mut [u8]) -> Self::RecvFut<'a> {
        RecvFut {
            transport: self as *mut _,
            buf_ptr: data.as_mut_ptr(),
            buf_len: data.len(),
            wait_fut: None,
            _ph: PhantomData,
            _pin: PhantomPinned,
        }
    }

    fn max_recv_size(&self) -> usize {
        self.rx_mtu as usize
    }
}

// ---------------------------------------------------------------------------
// Combined Transport impl
// ---------------------------------------------------------------------------

impl<D: Doorbell + Send + 'static, R: SharedMemoryTransportable + Send> Transport
    for SharedMemoryTransport<D, R>
{
}

// ---------------------------------------------------------------------------
// SharedMemoryTransportable marker
// ---------------------------------------------------------------------------

pub trait SharedMemoryTransportable: Region {}

impl SharedMemoryTransportable for DdrRegion {}
impl SharedMemoryTransportable for SramRegion {}
