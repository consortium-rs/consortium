use core::{future::Future, marker::PhantomData};

use consortium_ipc::{
    Chan, Connect, Doorbell, MaybeSend, Primary, RecvTransport, Secondary, SendTransport, Side,
    Transport, TransportError,
};
use consortium_shared_memory::{CachedRegion, Region, UncachedRegion};

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

impl RegionSharedMemoryTransportable for CachedRegion {}
impl RegionSharedMemoryTransportable for UncachedRegion {}

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

/// Outcome of one pass over the channel table.
enum Scan {
    /// A readable slot was drained into the caller's buffer.
    Delivered(usize),
    /// A slot advertised data but the read did not complete.  The caller must
    /// yield and scan again rather than park, so this path never waits on an
    /// unrelated doorbell edge.
    Retry,
    /// Nothing readable on this side's channels.
    Empty,
}

/// One pass over the channel table, delivering the first readable RX slot.
///
/// Always begins with a region invalidate, so each pass observes what the peer
/// published rather than a stale cache line.
fn scan_once<T: RecvAccess>(transport: &T, buf: &mut [u8]) -> Scan {
    let region = transport.region();
    let base = transport.base();

    region.invalidate();

    // Fast path: the channel named by the updated-id field.
    // SAFETY: `RecvAccess` guarantees `base` covers the descriptor header.
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
        "recv: channel_id = {}, channel_num = {}, channel = {:?}",
        channel_id,
        channel_num,
        channel
    );

    let Some(channel) = channel else {
        return Scan::Empty;
    };

    // SAFETY: slot pointer is within the region.
    let slot_base = unsafe { channel.slot_addr(base) };

    consortium_log::trace!(
        "recv: delivering channel_id = {}, slot_base = {:#x}",
        channel_id,
        slot_base as usize
    );

    // SAFETY: `slot_base` is a valid slot; `buf` is exclusively owned by the caller.
    if let Some(n) = unsafe { slot::try_read(slot_base, buf, region) } {
        consortium_log::trace!(
            "recv: read {} bytes from channel_id = {}, slot_base = {:#x}",
            n,
            channel_id,
            slot_base as usize
        );
        return Scan::Delivered(n);
    }

    consortium_log::trace!(
        "recv: slot_base = {:#x} was readable but read failed; invalidating region and retrying",
        slot_base as usize
    );

    // Flushes for integrity-error flags happen inside `try_read`; invalidate so
    // the retry pass does not re-read the same stale line.
    region.invalidate();
    Scan::Retry
}

/// Hand control back to the executor once, then resume.
///
/// Used where the recv loop must not park — it has work to redo immediately but
/// must not starve other tasks by spinning.
fn yield_now() -> impl Future<Output = ()> {
    let mut yielded = false;
    core::future::poll_fn(move |cx| {
        if yielded {
            core::task::Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            core::task::Poll::Pending
        }
    })
}

/// Receive one message, parking on the doorbell until a slot is readable.
///
/// Takes `&mut T` rather than `&T` purely so the returned future is `Send`
/// whenever `T` is; nothing here mutates the transport.
async fn recv_bytes<T: RecvAccess>(
    transport: &mut T,
    buf: &mut [u8],
) -> Result<usize, Error<T::D>> {
    let ch = transport.ch();
    consortium_log::trace!("recv: waiting for doorbell on channel {:?}", ch);

    loop {
        match scan_once(&*transport, &mut *buf) {
            Scan::Delivered(n) => return Ok(n),
            Scan::Retry => {
                yield_now().await;
                continue;
            }
            Scan::Empty => {}
        }

        // Arm the doorbell *before* the second scan.  The fan-out
        // (`Notify::notify_waiters`) wakes only already-registered waiters and
        // stores no permit, so an edge delivered between a scan and the waker
        // registration would be lost.  Registering first and re-scanning turns
        // that race into a guaranteed delivery, independent of the doorbell's
        // notification semantics.
        //
        // This is why the wait future is created here and awaited below rather
        // than simply `.await`ed: awaiting it directly would register the waker
        // and park in one step, leaving no point at which to re-scan.
        consortium_log::trace!("recv: no data available, arming doorbell wait on {:?}", ch);
        let mut wait = core::pin::pin!(transport.doorbell().wait(ch));
        let armed =
            core::future::poll_fn(|cx| core::task::Poll::Ready(wait.as_mut().poll(cx))).await;

        // The wait future lives to the end of this iteration and no longer.
        // Dropping it on delivery is what keeps a later stray doorbell from
        // resolving a stale receive, and runs whatever disarm the doorbell put
        // in its future's `Drop`.
        match scan_once(&*transport, &mut *buf) {
            Scan::Delivered(n) => return Ok(n),
            Scan::Retry => {
                yield_now().await;
                continue;
            }
            Scan::Empty => {}
        }

        match armed {
            // The remote rang before we subscribed and the re-scan still found
            // nothing.  The future is spent, so start the iteration over — after
            // a yield, so a doorbell that keeps firing without publishing data
            // cannot starve the executor.
            core::task::Poll::Ready(fired) => {
                consortium_log::trace!("recv: doorbell had already fired on channel {:?}", ch);
                fired.map_err(Error::Doorbell)?;
                yield_now().await;
            }
            // Waker is registered; park until the remote rings.
            core::task::Poll::Pending => {
                consortium_log::trace!("recv: no data after subscribe; parking on {:?}", ch);
                wait.as_mut().await.map_err(Error::Doorbell)?;
                consortium_log::trace!("recv: doorbell fired on channel {:?}", ch);
            }
        }
    }
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
    #[cfg(feature = "std")]
    _keep_alive: alloc::vec::Vec<alloc::sync::Arc<dyn Send + Sync>>,
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
            #[cfg(feature = "std")]
            _keep_alive: alloc::vec::Vec::new(),
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

    /// Retain an owner of memory or device resources used by this transport.
    ///
    /// Generated Linux bring-up uses this to keep each UIO mapping alive even
    /// after the aggregate platform resource bundle is partially moved or
    /// dropped.
    #[cfg(feature = "std")]
    pub fn keep_alive(&mut self, owner: alloc::sync::Arc<dyn Send + Sync>) {
        self._keep_alive.push(owner);
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
        #[cfg(feature = "std")]
        let rx_keep_alive = self._keep_alive.clone();
        (
            SharedMemoryTransportTx {
                ch: self.ch_tx,
                doorbell: self.doorbell,
                region: self.region,
                base: self.base,
                tx_mtu: self.tx_mtu,
                #[cfg(feature = "std")]
                _keep_alive: self._keep_alive,
                _side: PhantomData,
            },
            SharedMemoryTransportRx {
                ch: self.ch_rx,
                doorbell: rx_doorbell,
                region: rx_region,
                base: self.base,
                rx_mtu: self.rx_mtu,
                #[cfg(feature = "std")]
                _keep_alive: rx_keep_alive,
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
    #[cfg(feature = "std")]
    _keep_alive: alloc::vec::Vec<alloc::sync::Arc<dyn Send + Sync>>,
    _side: PhantomData<S>,
}

/// Receive-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportRx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch: Chan,
    doorbell: D,
    region: R,
    base: *mut u8,
    rx_mtu: u32,
    #[cfg(feature = "std")]
    _keep_alive: alloc::vec::Vec<alloc::sync::Arc<dyn Send + Sync>>,
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

/// Accessor so the receive path can be written once for both
/// [`SharedMemoryTransport`] and [`SharedMemoryTransportRx`].
///
/// # Safety
///
/// [`base`](RecvAccess::base) must return a pointer valid for reads covering
/// the descriptor header and a channel table of the length the header declares,
/// for as long as `Self` lives, and [`region`](RecvAccess::region) must be the
/// region that maps it.  The receive path reads both without further checks.
pub unsafe trait RecvAccess {
    type S: Side;
    type D: Doorbell;
    type R: RegionSharedMemoryTransportable;
    fn doorbell(&self) -> &Self::D;
    fn region(&self) -> &Self::R;
    fn base(&self) -> *mut u8;
    fn ch(&self) -> Chan;
}

// SAFETY: `SharedMemoryTransport::new` requires the caller to establish exactly
// this invariant for `base`, and the transport owns the region that maps it.
unsafe impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> RecvAccess
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

// SAFETY: `split` propagates the `base` and region of a transport that already
// upheld this invariant.
unsafe impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> RecvAccess
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

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransport<S, D, R>
{
    fn recv(
        &mut self,
        data: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        recv_bytes(self, data)
    }

    fn max_recv_size(&self) -> usize {
        self.rx_mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransportRx<S, D, R>
{
    fn recv(
        &mut self,
        data: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        recv_bytes(self, data)
    }

    fn max_recv_size(&self) -> usize {
        self.rx_mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> Transport
    for SharedMemoryTransport<S, D, R>
{
}

/// The primary end of the descriptor handshake: publish the layout, ring, and
/// wait for the secondary's ack.
///
/// Unlike the raw [`descriptor_init`] form, this owns the region, so it can
/// perform the cache maintenance the handshake needs on a non-coherent
/// [`CachedRegion`] and record the negotiated MTU afterwards.
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
