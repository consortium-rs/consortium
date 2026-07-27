use core::{
    future::Future,
    marker::{PhantomData, PhantomPinned},
};

use consortium_ipc::{
    Chan, Connect, Doorbell, MaybeSend, Primary, RecvTransport, Secondary, SendTransport, Side,
    Transport, TransportError,
};
use consortium_shared_memory::{DdrRegion, Region, SramRegion};

use crate::{
    descriptor::{
        ChannelLayout, DescriptorLayout, NUM_CHANNELS_OFFSET, RX_UPDATED_ID_OFFSET,
        accept_descriptor, descriptor_published, publish_descriptor, read_channel,
    },
    error::Error,
    slot_internal as slot,
    utils::rv_u8,
};

pub trait RegionSharedMemoryTransportable: consortium_shared_memory::region::Region {}

impl RegionSharedMemoryTransportable for DdrRegion {}
impl RegionSharedMemoryTransportable for SramRegion {}

/// # Safety
/// `base` must be valid for reads/writes covering the full descriptor layout.
unsafe fn send_bytes<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable>(
    ch: Chan,
    doorbell: &D,
    region: &R,
    base: *mut u8,
    data: &[u8],
) -> Result<(), Error<D>> {
    let mut channel_id = 0xFF;

    region.invalidate();

    // SAFETY: caller guarantees `base` covers the descriptor header.
    let num_channels = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

    for chan in 0..num_channels {
        // SAFETY: `chan` is within the validated channel table range.
        let channel = unsafe { read_channel(base, chan as usize) };
        // SAFETY: `channel` was read from the descriptor; slot pointer is within the region.
        let slot_addr = unsafe { channel.slot_addr(base) };
        // SAFETY: `slot_addr` points to a valid slot in the shared memory region.
        let consumed = unsafe { slot::is_consumed(slot_addr) };
        if consumed && channel.direction.is_tx::<S>() {
            channel_id = chan;
            break;
        }
    }

    if channel_id == 0xFF {
        consortium_log::trace!(
            "send: no consumed tx channel available among {} channels",
            num_channels
        );
        return Err(Error::NoChannel);
    }

    // SAFETY: `channel_id` was validated against `num_channels` above.
    let channel = unsafe { read_channel(base, channel_id as usize) };
    let capacity = (channel.size as usize)
        .checked_sub(slot::SLOT_HEADER)
        .ok_or_else(|| {
            consortium_log::trace!(
                "send: channel {} slot size {} smaller than slot header",
                channel_id,
                channel.size
            );
            Error::PayloadTooLarge {
                payload_len: data.len(),
                capacity: 0,
            }
        })?;

    if data.len() > capacity {
        consortium_log::trace!(
            "send: payload {} exceeds channel {} capacity {}",
            data.len(),
            channel_id,
            capacity
        );
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

/// Poll the recv path once.
///
/// `wait_fut` is persistent storage owned by the caller's future struct.  On
/// the first `Pending` it is populated with a doorbell wait future so the waker
/// registration survives across executor yields.  On re-poll after a wakeup the
/// stored future is polled first; only if it fires do we re-scan the slot.
///
/// # Safety
/// `transport.base()` must cover the descriptor header.  The doorbell must
/// outlive `*wait_fut`.
unsafe fn poll_recv<T: RecvAccess>(
    transport: &T,
    buf: &mut [u8],
    wait_fut: &mut Option<<T::D as Doorbell>::WaitFut<'static>>,
    cx: &mut core::task::Context<'_>,
) -> core::task::Poll<Result<usize, Error<T::D>>>
where
    T::D: Send + 'static,
{
    let doorbell = transport.doorbell();
    let region = transport.region();
    let base = transport.base();
    let ch = transport.ch();
    consortium_log::trace!("poll_recv: waiting for doorbell on channel {:?}", ch);

    // If a wait future is already armed, poll it.  If it is still pending our
    // waker is already registered — park until the doorbell fires.  If it fires,
    // clear it and fall through to re-scan the slot.
    if let Some(ref mut wf) = *wait_fut {
        // SAFETY: `wf` lives in the caller's pinned struct and is never moved.
        match unsafe { core::pin::Pin::new_unchecked(wf) }.poll(cx) {
            core::task::Poll::Pending => return core::task::Poll::Pending,
            core::task::Poll::Ready(_) => {
                consortium_log::trace!("poll_recv: doorbell fired on channel {:?}", ch);
                *wait_fut = None;
            }
        }
    }

    // Scan for a readable RX slot.  If none is found, subscribe for the next
    // doorbell and then scan the slots **once more** before parking.  That
    // second scan is load-bearing: an interrupt delivered in the window between
    // the first scan and the subscribe would otherwise be lost, because the
    // doorbell fan-out (`Notify::notify_waiters`) wakes only already-registered
    // waiters and stores no permit.  Re-checking the slots after the waker is
    // registered turns that race into a guaranteed delivery, independent of the
    // doorbell's notification semantics.  `subscribed` bounds the loop to the
    // scan-then-recheck pair (at most two scans per poll).
    let mut subscribed = false;
    loop {
        region.invalidate();

        // Fast path: the channel named by the updated-id field.
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
            if layout.direction.is_rx::<T::S>() && !consumed {
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
                if layout.direction.is_rx::<T::S>() && !consumed {
                    channel_id = chan;
                    channel = Some(layout);
                    break;
                }
            }
        }

        consortium_log::trace!(
            "poll_recv: channel_id = {}, channel_num = {}, channel = {:?}, subscribed = {}",
            channel_id,
            channel_num,
            channel,
            subscribed
        );

        let Some(channel) = channel else {
            if subscribed {
                // Waker already registered on the first pass; a later doorbell
                // edge will re-poll us.  Nothing to deliver yet.
                consortium_log::trace!("poll_recv: no data after subscribe; parking");
                return core::task::Poll::Pending;
            }
            // No data yet — subscribe so the waker stays registered until the
            // remote rings, then loop once more to re-scan and close the
            // scan→subscribe race.
            consortium_log::trace!("poll_recv: no data available, arming doorbell wait future");
            // SAFETY: `wait_fut` is the stored wait future for this receive
            // operation.  `arm_wait` registers the waker (and self-wakes if a
            // notification was already latched); we discard its `Pending` return
            // deliberately because the re-scan below decides whether to park.
            let _ = unsafe { arm_wait(doorbell, ch, wait_fut, cx) };
            subscribed = true;
            continue;
        };

        // SAFETY: slot pointer is within the region.
        let slot_base = unsafe { channel.slot_addr(base) };

        consortium_log::trace!(
            "poll_recv: delivering channel_id = {}, slot_base = {:#x}",
            channel_id,
            slot_base as usize
        );

        // SAFETY: `slot_base` is a valid slot; `buf` is exclusively owned by the caller.
        if let Some(n) = unsafe { slot::try_read(slot_base, buf, region) } {
            // Data delivered; drop any wait future armed on the first pass so a
            // later stray doorbell cannot resolve a stale receive.
            *wait_fut = None;
            consortium_log::trace!(
                "poll_recv: read {} bytes from channel_id = {}, slot_base = {:#x}",
                n,
                channel_id,
                slot_base as usize
            );
            return core::task::Poll::Ready(Ok(n));
        }

        consortium_log::trace!(
            "poll_recv: slot_base = {:#x} was readable but read failed; invalidating region and re-polling",
            slot_base as usize
        );

        // The scan saw a readable slot but the read could not complete.  Flushes
        // for integrity-error flags happen inside `try_read`; invalidate and
        // force an immediate re-poll so this path does not depend on an
        // unrelated doorbell.
        region.invalidate();
        cx.waker().wake_by_ref();
        return core::task::Poll::Pending;
    }
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
    consortium_log::trace!("poll_recv: arming doorbell wait future on channel {:?}", ch);
    let wf = doorbell.wait(ch);
    // SAFETY: the doorbell is inside the transport that exclusively owns the recv
    // future, and the transport is guaranteed to outlive the future.  Erasing the
    // lifetime to 'static is sound because `wait_fut` is dropped before the
    // transport (enforced by the PhantomData<&'a mut Transport> on the future struct).
    let wf = unsafe { core::mem::transmute::<D::WaitFut<'_>, D::WaitFut<'static>>(wf) };
    consortium_log::trace!(
        "poll_recv: storing doorbell wait future on channel {:?}",
        ch
    );
    *wait_fut = Some(wf);
    // SAFETY: `wait_fut` is now in a pinned location; we never move it after this poll.
    match unsafe { core::pin::Pin::new_unchecked(wait_fut.as_mut().unwrap()) }.poll(cx) {
        core::task::Poll::Ready(_) => {
            consortium_log::trace!("poll_recv: doorbell fired immediately on channel {:?}", ch);
            // A notification was already stored (remote rang before we subscribed).
            // Clear the future and ask the executor to re-poll immediately.
            *wait_fut = None;
            cx.waker().wake_by_ref();
        }
        core::task::Poll::Pending => {}
    }
    core::task::Poll::Pending
}

pub struct SharedMemoryTransport<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch_tx: Chan,
    ch_rx: Chan,
    doorbell: D,
    region: R,
    /// Base pointer to offset 0x00 of the shared memory region (descriptor header).
    base: *mut u8,
    tx_mtu: u32,
    rx_mtu: u32,
    /// Which core's perspective resolves the absolute channel directions.
    _side: PhantomData<S>,
}

impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> SharedMemoryTransport<S, D, R> {
    /// Construct a `SharedMemoryTransport` for an explicit [`Side`] `S`.
    ///
    /// Channel directions in the descriptor are absolute
    /// (`PrimaryToSecondary` / `SecondaryToPrimary`); the side `S` selects which
    /// core's view interprets them as TX vs RX.
    ///
    /// # Safety
    ///
    /// Same invariants as [`new`](SharedMemoryTransport::new).
    pub unsafe fn new(ch_tx: Chan, ch_rx: Chan, doorbell: D, region: R, base: *mut u8) -> Self {
        Self {
            ch_tx,
            ch_rx,
            doorbell,
            region,
            base,
            tx_mtu: 0,
            rx_mtu: 0,
            _side: PhantomData,
        }
    }

    /// Descriptor base pointer (offset 0x00 of the shared region).
    ///
    /// Exposed so the descriptor handshake ([`descriptor_init`] /
    /// [`descriptor_ack`]) can run on this transport before it is
    /// [`split`](Self::split) into channel halves.
    ///
    /// [`descriptor_init`]: SharedMemoryTransport::descriptor_init
    /// [`descriptor_ack`]: SharedMemoryTransport::descriptor_ack
    pub fn base(&self) -> *mut u8 {
        self.base
    }

    /// The doorbell this transport rings and waits on, borrowed for the
    /// descriptor handshake before [`split`](Self::split).
    pub fn doorbell(&self) -> &D {
        &self.doorbell
    }

    /// This side's transmit and receive channel ids, in `(tx, rx)` order.
    pub fn channels(&self) -> (Chan, Chan) {
        (self.ch_tx, self.ch_rx)
    }

    /// Record this side's per-direction slot capacities as the transport MTU.
    ///
    /// Called at the end of the [`Connect`] handshake, which is the first moment
    /// the channel table is known to be published and validated; before that
    /// [`max_send_size`](SendTransport::max_send_size) reports `0`.
    ///
    /// The reported size is the smallest usable payload across this side's
    /// channels in that direction (`size - SLOT_HEADER`) — exactly the bound
    /// `send` enforces, so a caller that respects `max_send_size` cannot be
    /// rejected for length. Note this is deliberately *not* the descriptor's
    /// negotiated transfer-size field: the secondary may raise that field above
    /// what the published slots can actually hold.
    fn record_slot_capacities(&mut self) {
        // The peer wrote to the descriptor before signalling; drop stale lines
        // first (a no-op on coherent regions).
        self.region.invalidate();

        // SAFETY: `new` established that `base` covers the descriptor header, and
        // the handshake has completed, so the channel table is published.
        let num_channels = unsafe { rv_u8(self.base, NUM_CHANNELS_OFFSET) };

        let (mut tx, mut rx): (Option<u32>, Option<u32>) = (None, None);
        for chan in 0..num_channels {
            // SAFETY: `chan` is within the published channel table range.
            let channel = unsafe { read_channel(self.base, chan as usize) };
            let capacity = channel.size.saturating_sub(slot::SLOT_HEADER as u32);
            if channel.direction.is_tx::<S>() {
                tx = Some(tx.map_or(capacity, |c: u32| c.min(capacity)));
            } else if channel.direction.is_rx::<S>() {
                rx = Some(rx.map_or(capacity, |c: u32| c.min(capacity)));
            }
        }

        self.tx_mtu = tx.unwrap_or(0);
        self.rx_mtu = rx.unwrap_or(0);
        consortium_log::trace!(
            "connect: slot capacities tx {} rx {}",
            self.tx_mtu,
            self.rx_mtu
        );
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
    pub fn split(
        self,
    ) -> (
        SharedMemoryTransportTx<S, D, R>,
        SharedMemoryTransportRx<S, D, R>,
    )
    where
        D: Clone,
        R: Clone,
    {
        let rx_doorbell = self.doorbell.clone();
        let rx_region = self.region.clone();
        (
            SharedMemoryTransportTx {
                ch: self.ch_tx,
                doorbell: self.doorbell,
                region: self.region,
                base: self.base,
                tx_mtu: self.tx_mtu,
                _side: PhantomData,
            },
            SharedMemoryTransportRx {
                ch: self.ch_rx,
                doorbell: rx_doorbell,
                region: rx_region,
                base: self.base,
                rx_mtu: self.rx_mtu,
                _side: PhantomData,
            },
        )
    }
}

/// Send-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportTx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    base: *mut u8,
    tx_mtu: u32,
    _side: PhantomData<S>,
}

/// Receive-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportRx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    base: *mut u8,
    rx_mtu: u32,
    _side: PhantomData<S>,
}

// SAFETY: `base` is an exclusive pointer to a shared memory region managed by this
// transport.  No shared reference to the pointed-to memory is ever handed out through
// the safe API; the only aliasing is the intentional inter-processor sharing modelled
// by the doorbell/region abstraction.  Sending the transport to another thread is
// sound when D and R are both Send.
unsafe impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Send
    for SharedMemoryTransport<S, D, R>
{
}
// SAFETY: the transmit half owns only the raw base pointer and never exposes shared
// references to the pointed-to memory through the safe API. Moving it is sound
// when its doorbell and region handles are Send.
unsafe impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Send
    for SharedMemoryTransportTx<S, D, R>
{
}
// SAFETY: the receive half owns only the raw base pointer and never exposes shared
// references to the pointed-to memory through the safe API. Moving it is sound
// when its doorbell and region handles are Send.
unsafe impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Send
    for SharedMemoryTransportRx<S, D, R>
{
}

/// Sealed accessor so `RecvFut` can be generic over both `SharedMemoryTransport`
/// and `SharedMemoryTransportRx` without duplicating the future type.
pub trait RecvAccess {
    type S: Side;
    type D: Doorbell + 'static;
    type R: RegionSharedMemoryTransportable;
    fn doorbell(&self) -> &Self::D;
    fn region(&self) -> &Self::R;
    fn base(&self) -> *mut u8;
    fn ch(&self) -> Chan;
}

impl<S: Side, D: Doorbell + 'static, R: RegionSharedMemoryTransportable> RecvAccess
    for SharedMemoryTransport<S, D, R>
{
    type S = S;
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
        self.ch_rx
    }
}

impl<S: Side, D: Doorbell + 'static, R: RegionSharedMemoryTransportable> RecvAccess
    for SharedMemoryTransportRx<S, D, R>
{
    type S = S;
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

/// Future returned by [`SendTransport::send`] on shared-memory transports.
///
/// The send is completed eagerly inside `send()`; this future simply wraps the
/// pre-computed result and is always `Ready` on the first poll.  The opaque
/// `impl Future` returned by the trait method captures the `&mut self` borrow,
/// so the borrow checker still enforces correct ownership at the call site.
///
/// Both [`SharedMemoryTransport`] and [`SharedMemoryTransportTx`] use this same
/// type.
pub struct SendFut<D: Doorbell> {
    result: Result<(), Error<D>>,
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

// SAFETY: the raw pointer is exclusively owned by this future for its lifetime;
// all concrete T, T::D, and T::R types used here are Send.
unsafe impl<T: RecvAccess + Send> Send for RecvFut<'_, T>
where
    T::D: Send,
    T::R: Send,
{
}

impl<D: Doorbell> Future for SendFut<D> {
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
        unsafe { poll_recv(transport, buf, &mut this.wait_fut, cx) }
    }
}

impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> TransportError
    for SharedMemoryTransport<S, D, R>
{
    type Error = Error<D>;
}

impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> TransportError
    for SharedMemoryTransportTx<S, D, R>
{
    type Error = Error<D>;
}

impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> TransportError
    for SharedMemoryTransportRx<S, D, R>
{
    type Error = Error<D>;
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransport<S, D, R>
{
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        // SAFETY: `self.base` satisfies the descriptor layout invariant established in `new`.
        let result = unsafe {
            send_bytes::<S, D, R>(self.ch_tx, &self.doorbell, &self.region, self.base, data)
        };
        SendFut { result }
    }

    fn max_send_size(&self) -> usize {
        self.tx_mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransportTx<S, D, R>
{
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        // SAFETY: `self.base` satisfies the descriptor layout invariant established by `split`.
        let result = unsafe {
            send_bytes::<S, D, R>(self.ch, &self.doorbell, &self.region, self.base, data)
        };
        SendFut { result }
    }

    fn max_send_size(&self) -> usize {
        self.tx_mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send + 'static, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransport<S, D, R>
{
    fn recv<'s>(
        &'s mut self,
        data: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        RecvFut::<'s, SharedMemoryTransport<S, D, R>> {
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

impl<S: Side, D: Doorbell + Send + 'static, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransportRx<S, D, R>
{
    fn recv<'s>(
        &'s mut self,
        data: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        RecvFut::<'s, SharedMemoryTransportRx<S, D, R>> {
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

impl<S: Side, D: Doorbell + Send + 'static, R: RegionSharedMemoryTransportable + Send> Transport
    for SharedMemoryTransport<S, D, R>
{
}

/// The primary end of the descriptor handshake: publish the layout, ring, and
/// wait for the secondary's ack.
///
/// Unlike the raw [`descriptor_init`] form, this owns the region, so it can
/// perform the cache maintenance the handshake needs on a non-coherent
/// [`DdrRegion`] and record the negotiated MTU afterwards.
///
/// [`descriptor_init`]: SharedMemoryTransport::descriptor_init
impl<D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Connect
    for SharedMemoryTransport<Primary, D, R>
{
    type Params = DescriptorLayout;

    async fn connect(&mut self, params: DescriptorLayout) -> Result<(), Self::Error> {
        consortium_log::trace!(
            "connect(primary): publishing {} channels, transfer_size {}",
            params.num_channels,
            params.mtu
        );
        // SAFETY: `new` established that `base` covers the descriptor header and a
        // channel table of `params.num_channels` entries, and that the region is
        // DMA-coherent or maintained through `R`.
        unsafe { publish_descriptor(self.base, params.num_channels, params.mtu) };
        // Publish the descriptor to memory before the doorbell announces it.
        self.region.flush();

        self.doorbell.ring(self.ch_tx).map_err(Error::Doorbell)?;
        self.doorbell
            .wait(self.ch_rx)
            .await
            .map_err(Error::Doorbell)?;

        self.record_slot_capacities();
        consortium_log::trace!("connect(primary): handshake complete");
        Ok(())
    }
}

/// The secondary end of the descriptor handshake: wait for the descriptor to be
/// published, validate it, and ack.
///
/// The counterpart to the [`Primary`] impl above — same trait, same call site,
/// opposite protocol role, selected by the transport's [`Side`] rather than by a
/// runtime branch in the caller.
impl<D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Connect
    for SharedMemoryTransport<Secondary, D, R>
{
    type Params = DescriptorLayout;

    async fn connect(&mut self, params: DescriptorLayout) -> Result<(), Self::Error> {
        consortium_log::trace!(
            "connect(secondary): expecting {} channels, transfer_size {}",
            params.num_channels,
            params.mtu
        );

        // The header magic is the readiness gate: the primary publishes it last,
        // so a match implies the whole descriptor is written.
        self.region.invalidate();
        // SAFETY: `new` established that `base` covers the descriptor header.
        if !unsafe { descriptor_published(self.base) } {
            consortium_log::trace!(
                "connect(secondary): descriptor not published, awaiting primary doorbell"
            );
            self.doorbell
                .wait(self.ch_rx)
                .await
                .map_err(Error::Doorbell)?;
            self.region.invalidate();
        }

        // SAFETY: `new` established that `base` covers the descriptor header and
        // channel table; the gate above guarantees the primary finished writing it.
        let descriptor =
            unsafe { accept_descriptor(self.base, params.num_channels as usize, params.mtu) }?;
        // Push the validated state and agreed transfer size out before acking.
        self.region.flush();

        consortium_log::trace!(
            "connect(secondary): {} channels validated, acking primary",
            descriptor.num_channels
        );
        self.doorbell.ring(self.ch_tx).map_err(Error::Doorbell)?;

        self.record_slot_capacities();
        Ok(())
    }
}
