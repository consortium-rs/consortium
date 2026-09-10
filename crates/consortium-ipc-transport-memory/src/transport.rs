use core::{future::Future, marker::PhantomData};

use consortium_ipc::{
    AckSendTransport, Chan, Connect, Doorbell, MaybeSend, Primary, RecvTransport, Secondary,
    SendTransport, Side, Transport, TransportError,
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

/// Outcome of one pass looking for a writable TX slot.
enum Acquire {
    /// A free slot was found, already checked against the channel's capacity.
    Ready { slot: *mut u8, channel_id: u8 },
    /// Every TX channel still holds a message the peer has not consumed.
    ///
    /// `all_acked` reports whether *every* occupying message set
    /// [`FLAG_ACK_REQUESTED`](slot::FLAG_ACK_REQUESTED). Only then is a slot guaranteed to
    /// announce itself when it frees, so only then may a caller park rather than fail: an
    /// unacknowledged message frees its slot silently, and waiting for an edge that will
    /// never arrive would hang.
    Full { all_acked: bool },
}

/// One pass over the channel table looking for a TX slot this side may write.
///
/// Always begins with a region invalidate, so the pass observes the peer's consumption
/// rather than a stale cache line.
///
/// # Safety
/// `base` must be valid for reads/writes covering the full descriptor layout.
unsafe fn acquire_tx_slot<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable>(
    region: &R,
    base: *mut u8,
    data: &[u8],
) -> Result<Acquire, Error<D>> {
    let mut channel_id = 0xFF;
    let mut all_acked = true;

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
        if !channel.direction.is_tx::<S>() {
            continue;
        }
        if consumed {
            channel_id = chan;
            break;
        }
        // SAFETY: `slot_addr` points to a valid slot in the shared memory region.
        all_acked &= unsafe { slot::ack_requested(slot_addr) };
    }

    if channel_id == 0xFF {
        consortium_log::trace!(
            "send: no consumed tx channel available among {} channels (all acked: {})",
            num_channels,
            all_acked
        );
        return Ok(Acquire::Full { all_acked });
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

    Ok(Acquire::Ready {
        slot: slot_base,
        channel_id,
    })
}

/// Fire-and-forget send: claim a slot, write it, ring, return.
///
/// # Safety
/// `base` must be valid for reads/writes covering the full descriptor layout.
unsafe fn send_bytes<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable>(
    ch: Chan,
    doorbell: &D,
    region: &R,
    base: *mut u8,
    data: &[u8],
) -> Result<(), Error<D>> {
    // SAFETY: forwarded to the caller's `base` guarantee.
    let slot_base = match unsafe { acquire_tx_slot::<S, D, R>(region, base, data) }? {
        Acquire::Ready { slot, channel_id } => {
            consortium_log::trace!(
                "send: writing {} bytes to channel {}, slot_base = {:#x}",
                data.len(),
                channel_id,
                slot as usize
            );
            slot
        }
        // Depth is one frame per direction and this path never waits, so a peer that has
        // not consumed the previous message is reported rather than queued behind.
        Acquire::Full { .. } => return Err(Error::NoChannel),
    };

    // SAFETY: `slot_base` points to a valid, capacity-checked slot.
    unsafe { slot::write(slot_base, data, false) };

    region.flush();
    doorbell.ring(ch).map_err(Error::Doorbell)
}

/// Outcome of one pass over the channel table.
enum Scan<D: Doorbell> {
    /// A readable slot was drained into the caller's buffer.
    Delivered(usize),
    /// Acknowledging a drained slot failed; the message is already delivered or
    /// discarded, but the sender can no longer be woken, so the error is surfaced.
    Failed(Error<D>),
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
fn scan_once<T: RecvAccess>(transport: &T, buf: &mut [u8]) -> Scan<T::D> {
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
    let outcome = unsafe { slot::try_read(slot_base, buf, region) };

    // Acknowledge before reporting.  `try_read` has already flushed the consumed (or
    // integrity-error) flag, so the peer can observe the state change the moment the line
    // fires.  The corrupt path rings too: the sender is parked on this edge and a message
    // that failed its CRC still frees the slot, so staying silent would strand it.
    if let slot::ReadOutcome::Delivered {
        ack_requested: true,
        ..
    }
    | slot::ReadOutcome::Corrupt {
        ack_requested: true,
    } = outcome
        && let Some(ch_ack) = transport.ch_ack_tx()
    {
        consortium_log::trace!(
            "recv: acknowledging channel_id = {} on {:?}",
            channel_id,
            ch_ack
        );
        if let Err(error) = transport.doorbell().ring(ch_ack) {
            return Scan::Failed(Error::Doorbell(error));
        }
    }

    if let slot::ReadOutcome::Delivered { len, .. } = outcome {
        consortium_log::trace!(
            "recv: read {} bytes from channel_id = {}, slot_base = {:#x}",
            len,
            channel_id,
            slot_base as usize
        );
        return Scan::Delivered(len);
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
            Scan::Failed(error) => return Err(error),
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
        // resolving a stale receive.  It also runs the doorbell's `Drop`, which
        // for the three MMIO doorbells disarms the line without acknowledging
        // its status.
        match scan_once(&*transport, &mut *buf) {
            Scan::Delivered(n) => return Ok(n),
            Scan::Failed(error) => return Err(error),
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

/// Park on `ch` until `ready` reports the condition holds.
///
/// The ordering is the one [`recv_bytes`] documents at length and is load-bearing:
/// `ready` runs, then the wait future is created and polled **once** through `poll_fn` to
/// register the waker, then `ready` runs again, and only then does the caller park. A
/// doorbell whose fan-out stores no permit (`notify_waiters`, and every hardware doorbell)
/// drops an edge delivered between a check and the registration, so re-checking after
/// arming is what turns that race into a guaranteed delivery.
///
/// Spurious wakes are harmless: readiness is decided from shared memory, never from the
/// edge itself.
///
/// `ready` is handed the transport rather than closing over its region and base pointer,
/// for the same reason [`recv_bytes`] re-derives them on each pass: a raw pointer is not
/// `Send`, so holding one across an `await` would make this future `!Send` and break the
/// `MaybeSend` bound the trait promises.
async fn park_until<T: SendAccess>(
    transport: &mut T,
    ch: Chan,
    mut ready: impl FnMut(&T) -> Result<bool, Error<T::D>>,
) -> Result<(), Error<T::D>> {
    loop {
        if ready(&*transport)? {
            return Ok(());
        }

        let mut wait = core::pin::pin!(transport.doorbell().wait(ch));
        let armed =
            core::future::poll_fn(|cx| core::task::Poll::Ready(wait.as_mut().poll(cx))).await;

        if ready(&*transport)? {
            return Ok(());
        }

        match armed {
            // The peer rang before we subscribed and the re-check still found nothing.
            // The future is spent, so start over — after a yield, so a doorbell that keeps
            // firing without changing memory cannot starve the executor.
            core::task::Poll::Ready(fired) => {
                fired.map_err(Error::Doorbell)?;
                yield_now().await;
            }
            core::task::Poll::Pending => {
                wait.as_mut().await.map_err(Error::Doorbell)?;
            }
        }
    }
}

/// Resolve a TX channel index to its slot pointer.
///
/// # Safety
/// `base` must cover the descriptor and a channel table containing `channel_id`.
unsafe fn slot_of(base: *mut u8, channel_id: u8) -> *mut u8 {
    // SAFETY: forwarded to the caller's channel-table guarantee.
    let channel = unsafe { read_channel(base, channel_id as usize) };
    // SAFETY: `channel` was read from that table, so its slot lies within the region.
    unsafe { channel.slot_addr(base) }
}

/// Send `data` and resolve only once the peer has drained it.
///
/// Takes `&mut T` rather than `&T` purely so the returned future is `Send` whenever `T`
/// is, exactly as [`recv_bytes`] does. For the same reason nothing derived from the
/// transport — the region, the base pointer, the chosen slot — is held across an `await`;
/// only the channel *index* survives between phases, and the pointer is re-derived from it.
async fn send_acked_bytes<T: SendAccess>(
    transport: &mut T,
    data: &[u8],
) -> Result<(), Error<T::D>> {
    // Fail fast rather than park on a line nobody rings.  Both halves of the check matter:
    // this side may have no ack lines wired at all, and the handshake may have agreed that
    // the peer does not acknowledge on these channels.
    let (Some(ch_ack_rx), true) = (transport.ch_ack_rx(), transport.tx_ack_capable()) else {
        consortium_log::trace!("send_acked: acknowledgement not wired for this channel");
        return Err(Error::AckNotEnabled);
    };

    // Claim a slot.  Parking here is only sound while every message occupying this side's
    // TX channels asked to be acknowledged; an ordinary `send` frees its slot silently, so
    // with one of those in flight there is no edge to wait for and this reports
    // `NoChannel` just as `send` would, leaving the retry to the caller.
    let mut channel_id = 0u8;
    park_until(transport, ch_ack_rx, |t| {
        // SAFETY: `SendAccess` guarantees `base` covers the descriptor and channel table.
        match unsafe { acquire_tx_slot::<T::S, T::D, T::R>(t.region(), t.base(), data) }? {
            Acquire::Ready {
                channel_id: found, ..
            } => {
                channel_id = found;
                Ok(true)
            }
            Acquire::Full { all_acked: true } => Ok(false),
            Acquire::Full { all_acked: false } => Err(Error::NoChannel),
        }
    })
    .await?;

    // SAFETY: `channel_id` came from `acquire_tx_slot`, which read it from this same
    // descriptor's channel table.
    let slot_base = unsafe { slot_of(transport.base(), channel_id) };
    // SAFETY: `acquire_tx_slot` capacity-checked that slot against `data`.
    unsafe { slot::write(slot_base, data, true) };
    transport.region().flush();
    transport
        .doorbell()
        .ring(transport.ch())
        .map_err(Error::Doorbell)?;

    consortium_log::trace!(
        "send_acked: published {} bytes on channel {}, awaiting acknowledgement on {:?}",
        data.len(),
        channel_id,
        ch_ack_rx
    );

    // Wait for that slot to change state.  `is_consumed` would also accept an integrity
    // error, so the two are distinguished here: a corrupt delivery frees the slot but is
    // not a successful send.
    park_until(transport, ch_ack_rx, |t| {
        t.region().invalidate();
        // SAFETY: same descriptor and channel index as the write above.
        let slot_base = unsafe { slot_of(t.base(), channel_id) };
        // SAFETY: `slot_base` addresses a valid slot in this region.
        let flags = unsafe { slot::flags(slot_base) };
        if flags & slot::FLAG_INTEGRITY_ERROR != 0 {
            return Err(Error::PeerIntegrityError);
        }
        Ok(flags & slot::FLAG_CONSUMED != 0)
    })
    .await
}

pub struct SharedMemoryTransport<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch_tx: Chan,
    ch_rx: Chan,
    /// Line this side rings to acknowledge a message the peer asked to have confirmed.
    ch_ack_tx: Option<Chan>,
    /// Line this side waits on for the peer's acknowledgement of its own messages.
    ch_ack_rx: Option<Chan>,
    doorbell: D,
    region: R,
    /// Base pointer to offset 0x00 of the shared memory region (descriptor header).
    base: *mut u8,
    tx_mtu: u32,
    rx_mtu: u32,
    /// Whether the peer published the acknowledgement capability for this side's TX
    /// channels; recorded by the handshake, `false` until then.
    tx_ack_capable: bool,
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
            ch_ack_tx: None,
            ch_ack_rx: None,
            doorbell,
            region,
            base,
            tx_mtu: 0,
            rx_mtu: 0,
            tx_ack_capable: false,
            #[cfg(feature = "std")]
            _keep_alive: alloc::vec::Vec::new(),
            _side: PhantomData,
        }
    }

    /// Construct a transport with acknowledgement doorbell lines wired.
    ///
    /// `ch_ack_tx` is rung by this side after it drains a message that asked to be
    /// confirmed; `ch_ack_rx` is what [`send_acked`] parks on. They must be distinct from
    /// each other and from `ch_tx`/`ch_rx`, because each line carries exactly one waiter —
    /// this is the whole reason acknowledgement gets its own pair rather than reusing the
    /// data lines.
    ///
    /// Both ends must be built this way. The capability is published in the descriptor and
    /// [`Connect::connect`] rejects a disagreement, so a half-configured pair fails during
    /// bring-up rather than hanging on the first acknowledged send.
    ///
    /// # Safety
    ///
    /// Same invariants as [`new`](SharedMemoryTransport::new).
    ///
    /// [`send_acked`]: consortium_ipc::AckSendTransport::send_acked
    pub unsafe fn new_acked(
        ch_tx: Chan,
        ch_rx: Chan,
        ch_ack_tx: Chan,
        ch_ack_rx: Chan,
        doorbell: D,
        region: R,
        base: *mut u8,
    ) -> Self {
        // SAFETY: forwarded to the caller.
        let mut this = unsafe { Self::new(ch_tx, ch_rx, doorbell, region, base) };
        this.ch_ack_tx = Some(ch_ack_tx);
        this.ch_ack_rx = Some(ch_ack_rx);
        this
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
        // A channel this side cannot acknowledge on disables the capability for the whole
        // direction: `send_acked` picks whichever TX slot is free, so it must be able to
        // count on an acknowledgement from any of them.
        let mut tx_ack = None;
        for chan in 0..num_channels {
            // SAFETY: `chan` is within the published channel table range.
            let channel = unsafe { read_channel(self.base, chan as usize) };
            let capacity = channel.size.saturating_sub(slot::SLOT_HEADER as u32);
            if channel.direction.is_tx::<S>() {
                tx = Some(tx.map_or(capacity, |c: u32| c.min(capacity)));
                tx_ack = Some(tx_ack.unwrap_or(true) && channel.ack_capable());
            } else if channel.direction.is_rx::<S>() {
                rx = Some(rx.map_or(capacity, |c: u32| c.min(capacity)));
            }
        }

        self.tx_mtu = tx.unwrap_or(0);
        self.rx_mtu = rx.unwrap_or(0);
        self.tx_ack_capable = tx_ack.unwrap_or(false) && self.ch_ack_rx.is_some();
        consortium_log::trace!(
            "connect: slot capacities tx {} rx {}, tx ack {}",
            self.tx_mtu,
            self.rx_mtu,
            self.tx_ack_capable
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
                ch_ack_rx: self.ch_ack_rx,
                doorbell: self.doorbell,
                region: self.region,
                base: self.base,
                tx_mtu: self.tx_mtu,
                tx_ack_capable: self.tx_ack_capable,
                #[cfg(feature = "std")]
                _keep_alive: self._keep_alive,
                _side: PhantomData,
            },
            SharedMemoryTransportRx {
                ch: self.ch_rx,
                ch_ack_tx: self.ch_ack_tx,
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
    ch_ack_rx: Option<Chan>,
    doorbell: D,
    region: R,
    base: *mut u8,
    tx_mtu: u32,
    tx_ack_capable: bool,
    #[cfg(feature = "std")]
    _keep_alive: alloc::vec::Vec<alloc::sync::Arc<dyn Send + Sync>>,
    _side: PhantomData<S>,
}

/// Receive-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportRx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    ch: Chan,
    ch_ack_tx: Option<Chan>,
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
    /// Line to ring when a drained message asked to be acknowledged, if one is wired.
    fn ch_ack_tx(&self) -> Option<Chan>;
}

/// Accessor so the acknowledged send path can be written once for both
/// [`SharedMemoryTransport`] and [`SharedMemoryTransportTx`].
///
/// # Safety
///
/// Same contract as [`RecvAccess`]: [`base`](SendAccess::base) must stay valid for the
/// descriptor header and the channel table it declares, and [`region`](SendAccess::region)
/// must be the region that maps it.
pub unsafe trait SendAccess {
    type S: Side;
    type D: Doorbell;
    type R: RegionSharedMemoryTransportable;
    fn doorbell(&self) -> &Self::D;
    fn region(&self) -> &Self::R;
    fn base(&self) -> *mut u8;
    fn ch(&self) -> Chan;
    /// Line the peer rings to acknowledge this side's messages, if one is wired.
    fn ch_ack_rx(&self) -> Option<Chan>;
    /// Whether the handshake agreed acknowledgements for this side's TX channels.
    fn tx_ack_capable(&self) -> bool;
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
    fn ch_ack_tx(&self) -> Option<Chan> {
        self.ch_ack_tx
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
    fn ch_ack_tx(&self) -> Option<Chan> {
        self.ch_ack_tx
    }
}

// SAFETY: `SharedMemoryTransport::new` requires the caller to establish exactly this
// invariant for `base`, and the transport owns the region that maps it.
unsafe impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> SendAccess
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
        self.ch_tx
    }
    fn ch_ack_rx(&self) -> Option<Chan> {
        self.ch_ack_rx
    }
    fn tx_ack_capable(&self) -> bool {
        self.tx_ack_capable
    }
}

// SAFETY: `split` propagates the `base` and region of a transport that already upheld
// this invariant.
unsafe impl<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> SendAccess
    for SharedMemoryTransportTx<S, D, R>
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
    fn ch_ack_rx(&self) -> Option<Chan> {
        self.ch_ack_rx
    }
    fn tx_ack_capable(&self) -> bool {
        self.tx_ack_capable
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

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> AckSendTransport
    for SharedMemoryTransport<S, D, R>
{
    fn send_acked(
        &mut self,
        data: &[u8],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        send_acked_bytes(self, data)
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> AckSendTransport
    for SharedMemoryTransportTx<S, D, R>
{
    fn send_acked(
        &mut self,
        data: &[u8],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        send_acked_bytes(self, data)
    }
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
        unsafe { publish_descriptor(self.base, params) };
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
        let descriptor = unsafe { accept_descriptor(self.base, params) }?;
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
