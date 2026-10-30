use core::{future::Future, marker::PhantomData, task::Poll};

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

/// What a transport, or either half of one, holds of the shared window: the
/// doorbell, the region, and the region's base pointer.
///
/// Keeping the window mapped is the region's job, not the transport's: a Linux
/// region type such as `consortium_runtime_app::uio::UioRegion` owns the UIO
/// devices it and the doorbell are mapped from, and clones of it keep them alive.
///
/// A `Link` is only ever built by [`SharedMemoryTransport::new`], whose caller
/// vouches that `region` maps this channel's window, valid for reads and writes
/// for its whole length while any `Link` over it lives. Every function below
/// relies on that invariant to dereference `base` through the descriptor and slot
/// accessors.
struct Link<D, R> {
    doorbell: D,
    region: R,
    /// Offset 0x00 of the region (the descriptor header), cached from
    /// [`Region::as_ptr`].
    base: *mut u8,
}

// SAFETY: `base` points into the inter-processor window `region` maps; no
// reference to that memory is ever handed out through the safe API, and the only
// aliasing is the intentional sharing with the peer that the slot protocol and
// doorbell order. Moving a `Link` to another thread is sound when its doorbell and
// region handles may move.
unsafe impl<D: Send, R: Send> Send for Link<D, R> {}

impl<D: Clone, R: Clone> Clone for Link<D, R> {
    fn clone(&self) -> Self {
        Self {
            doorbell: self.doorbell.clone(),
            region: self.region.clone(),
            base: self.base,
        }
    }
}

/// The transmit direction's lines and handshake results.
struct TxState {
    /// Line rung after publishing a message.
    ch: Chan,
    /// Line the peer rings to acknowledge this side's messages, if one is wired.
    ch_ack_rx: Option<Chan>,
    /// Smallest usable TX slot payload; `0` until the handshake records it.
    mtu: u32,
    /// Whether the peer published the acknowledgement capability for this side's TX
    /// channels; recorded by the handshake, `false` until then.
    ack_capable: bool,
}

/// The receive direction's lines and handshake results.
struct RxState {
    /// Line the peer rings after publishing a message.
    ch: Chan,
    /// Line this side rings to acknowledge a message the peer asked to have confirmed.
    ch_ack_tx: Option<Chan>,
    /// Smallest usable RX slot payload; `0` until the handshake records it.
    mtu: u32,
}

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
fn acquire_tx_slot<S: Side, D: Doorbell, R: Region>(
    link: &Link<D, R>,
    data: &[u8],
) -> Result<Acquire, Error<D>> {
    let base = link.base;
    link.region.invalidate();

    // SAFETY: the `Link` invariant covers the descriptor header.
    let num_channels = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

    let mut free = None;
    let mut all_acked = true;
    for chan in 0..num_channels {
        // SAFETY: `chan` is within the channel table the header declares.
        let channel = unsafe { read_channel(base, chan as usize) };
        if !channel.direction.is_tx::<S>() {
            continue;
        }
        // SAFETY: `channel` was read from the descriptor; its slot lies within the region.
        let slot_addr = unsafe { channel.slot_addr(base) };
        // SAFETY: `slot_addr` points to a valid slot in the shared memory region.
        if unsafe { slot::is_consumed(slot_addr) } {
            free = Some((chan, channel, slot_addr));
            break;
        }
        // SAFETY: as above.
        all_acked &= unsafe { slot::ack_requested(slot_addr) };
    }

    let Some((channel_id, channel, slot_addr)) = free else {
        consortium_log::trace!(
            "send: no consumed tx channel available among {} channels (all acked: {})",
            num_channels,
            all_acked
        );
        return Ok(Acquire::Full { all_acked });
    };

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

    Ok(Acquire::Ready {
        slot: slot_addr,
        channel_id,
    })
}

/// Fire-and-forget send: claim a slot, write it, ring, return.
fn send_bytes<S: Side, D: Doorbell, R: Region>(
    link: &Link<D, R>,
    tx: &TxState,
    data: &[u8],
) -> Result<(), Error<D>> {
    let slot_base = match acquire_tx_slot::<S, D, R>(link, data)? {
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

    link.region.flush();
    link.doorbell.ring(tx.ch).map_err(Error::Doorbell)
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
fn scan_once<S: Side, D: Doorbell, R: Region>(
    link: &Link<D, R>,
    rx: &RxState,
    buf: &mut [u8],
) -> Scan<D> {
    let region = &link.region;
    let base = link.base;

    region.invalidate();

    // Fast path: the channel named by the updated-id field.
    // SAFETY: the `Link` invariant covers the descriptor header.
    let mut channel_id = unsafe { rv_u8(base, RX_UPDATED_ID_OFFSET) };
    // SAFETY: same `base` validity guarantee.
    let channel_num = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

    let readable = |chan: u8| -> Option<ChannelLayout> {
        // SAFETY: callers pass `chan < channel_num`, within the channel table.
        let layout = unsafe { read_channel(base, chan as usize) };
        // SAFETY: slot pointer is within the region.
        let slot_addr = unsafe { layout.slot_addr(base) };
        // SAFETY: `slot_addr` points to a valid slot.
        let consumed = unsafe { slot::is_consumed(slot_addr) };
        (layout.direction.is_rx::<S>() && !consumed).then_some(layout)
    };

    let mut channel = if channel_id < channel_num {
        readable(channel_id)
    } else {
        None
    };

    // Fallback: scan all channels.
    if channel.is_none()
        && let Some((chan, layout)) =
            (0..channel_num).find_map(|chan| readable(chan).map(|layout| (chan, layout)))
    {
        channel_id = chan;
        channel = Some(layout);
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
        && let Some(ch_ack) = rx.ch_ack_tx
    {
        consortium_log::trace!(
            "recv: acknowledging channel_id = {} on {:?}",
            channel_id,
            ch_ack
        );
        if let Err(error) = link.doorbell.ring(ch_ack) {
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
/// Used where a wait loop must not park — it has work to redo immediately but
/// must not starve other tasks by spinning.
fn yield_now() -> impl Future<Output = ()> {
    let mut yielded = false;
    core::future::poll_fn(move |cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
}

/// What one readiness check in [`park_until`] found.
enum Step<T> {
    /// The condition holds; resolve with this value.
    Done(T),
    /// Nothing yet; park on the doorbell until the peer rings.
    Park,
    /// Shared memory is mid-update; yield once and check again without parking, so
    /// this never waits on an unrelated doorbell edge.
    Yield,
}

/// Wait on `ch` until `check` reports [`Step::Done`].
///
/// This is the one implementation of the wait protocol every path shares (receive,
/// and both phases of an acknowledged send), and its ordering is load-bearing:
/// `check` runs, then the wait future is created and polled **once** through
/// `poll_fn` to register the waker, then `check` runs again, and only then does the
/// caller park. A doorbell whose fan-out stores no permit (`notify_waiters`, and
/// every hardware doorbell) drops an edge delivered between a check and the
/// registration, so re-checking after arming is what turns that race into a
/// guaranteed delivery. Awaiting the wait future directly would register and park
/// in one step, leaving no point at which to re-check.
///
/// Spurious wakes are harmless: readiness is decided from shared memory, never from
/// the edge itself.
///
/// Takes `&mut Link` rather than `&Link` purely so the returned future is `Send`
/// whenever the link is: a `Link` holds a raw pointer and is not `Sync`. For the
/// same reason `check` is handed the link on each pass instead of closing over its
/// region or base pointer, and nothing derived from it is held across an `await`.
async fn park_until<D: Doorbell, R: Region, T>(
    link: &mut Link<D, R>,
    ch: Chan,
    mut check: impl FnMut(&Link<D, R>) -> Result<Step<T>, Error<D>>,
) -> Result<T, Error<D>> {
    loop {
        let step = check(&*link)?;
        match step {
            Step::Done(value) => return Ok(value),
            Step::Yield => {
                yield_now().await;
                continue;
            }
            Step::Park => {}
        }

        consortium_log::trace!("park: nothing ready, arming doorbell wait on {:?}", ch);
        let mut wait = core::pin::pin!(link.doorbell.wait(ch));
        let armed = core::future::poll_fn(|cx| Poll::Ready(wait.as_mut().poll(cx))).await;

        // The wait future lives to the end of this iteration and no longer.
        // Dropping it on completion is what keeps a later stray doorbell from
        // resolving a stale wait.  It also runs the doorbell's `Drop`, which for
        // the MMIO doorbells disarms the line without acknowledging its status.
        let step = check(&*link)?;
        match step {
            Step::Done(value) => return Ok(value),
            Step::Yield => {
                yield_now().await;
                continue;
            }
            Step::Park => {}
        }

        match armed {
            // The peer rang before we subscribed and the re-check still found nothing.
            // The future is spent, so start over — after a yield, so a doorbell that
            // keeps firing without changing memory cannot starve the executor.
            Poll::Ready(fired) => {
                consortium_log::trace!("park: doorbell had already fired on {:?}", ch);
                fired.map_err(Error::Doorbell)?;
                yield_now().await;
            }
            // Waker is registered; park until the peer rings.
            Poll::Pending => {
                consortium_log::trace!("park: parking on {:?}", ch);
                wait.as_mut().await.map_err(Error::Doorbell)?;
                consortium_log::trace!("park: doorbell fired on {:?}", ch);
            }
        }
    }
}

/// Receive one message, parking on the doorbell until a slot is readable.
async fn recv_bytes<S: Side, D: Doorbell, R: Region>(
    link: &mut Link<D, R>,
    rx: &RxState,
    buf: &mut [u8],
) -> Result<usize, Error<D>> {
    consortium_log::trace!("recv: waiting for doorbell on channel {:?}", rx.ch);
    park_until(link, rx.ch, |link| {
        Ok(match scan_once::<S, D, R>(link, rx, &mut *buf) {
            Scan::Delivered(n) => Step::Done(n),
            Scan::Failed(error) => return Err(error),
            Scan::Retry => Step::Yield,
            Scan::Empty => Step::Park,
        })
    })
    .await
}

/// Resolve a TX channel index to its slot pointer.
///
/// # Safety
/// `channel_id` must index the channel table of the descriptor at `base`.
unsafe fn slot_of(base: *mut u8, channel_id: u8) -> *mut u8 {
    // SAFETY: forwarded to the caller's channel-table guarantee.
    let channel = unsafe { read_channel(base, channel_id as usize) };
    // SAFETY: `channel` was read from that table, so its slot lies within the region.
    unsafe { channel.slot_addr(base) }
}

/// Send `data` and resolve only once the peer has drained it.
///
/// Only the channel *index* survives between phases and the slot pointer is
/// re-derived from it, for the `Send` reason [`park_until`] gives.
async fn send_acked_bytes<S: Side, D: Doorbell, R: Region>(
    link: &mut Link<D, R>,
    tx: &TxState,
    data: &[u8],
) -> Result<(), Error<D>> {
    // Fail fast rather than park on a line nobody rings.  Both halves of the check matter:
    // this side may have no ack lines wired at all, and the handshake may have agreed that
    // the peer does not acknowledge on these channels.
    let (Some(ch_ack_rx), true) = (tx.ch_ack_rx, tx.ack_capable) else {
        consortium_log::trace!("send_acked: acknowledgement not wired for this channel");
        return Err(Error::AckNotEnabled);
    };

    // Claim a slot.  Parking here is only sound while every message occupying this side's
    // TX channels asked to be acknowledged; an ordinary `send` frees its slot silently, so
    // with one of those in flight there is no edge to wait for and this reports
    // `NoChannel` just as `send` would, leaving the retry to the caller.
    let channel_id = park_until(link, ch_ack_rx, |link| {
        match acquire_tx_slot::<S, D, R>(link, data)? {
            Acquire::Ready { channel_id, .. } => Ok(Step::Done(channel_id)),
            Acquire::Full { all_acked: true } => Ok(Step::Park),
            Acquire::Full { all_acked: false } => Err(Error::NoChannel),
        }
    })
    .await?;

    // SAFETY: `channel_id` came from `acquire_tx_slot`, which read it from this same
    // descriptor's channel table.
    let slot_base = unsafe { slot_of(link.base, channel_id) };
    // SAFETY: `acquire_tx_slot` capacity-checked that slot against `data`.
    unsafe { slot::write(slot_base, data, true) };
    link.region.flush();
    link.doorbell.ring(tx.ch).map_err(Error::Doorbell)?;

    consortium_log::trace!(
        "send_acked: published {} bytes on channel {}, awaiting acknowledgement on {:?}",
        data.len(),
        channel_id,
        ch_ack_rx
    );

    // Wait for that slot to change state.  `is_consumed` would also accept an integrity
    // error, so the two are distinguished here: a corrupt delivery frees the slot but is
    // not a successful send.
    park_until(link, ch_ack_rx, |link| {
        link.region.invalidate();
        // SAFETY: same descriptor and channel index as the write above.
        let slot_base = unsafe { slot_of(link.base, channel_id) };
        // SAFETY: `slot_base` addresses a valid slot in this region.
        let flags = unsafe { slot::flags(slot_base) };
        if flags & slot::FLAG_INTEGRITY_ERROR != 0 {
            return Err(Error::PeerIntegrityError);
        }
        Ok(if flags & slot::FLAG_CONSUMED != 0 {
            Step::Done(())
        } else {
            Step::Park
        })
    })
    .await
}

/// A full-duplex shared-memory transport: both directions over one window.
///
/// It holds exactly what its two [`split`](Self::split) halves hold — the
/// doorbell, region and base pointer, plus each direction's state — so sending
/// and receiving run the same code whether or not the transport has been split.
pub struct SharedMemoryTransport<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    link: Link<D, R>,
    tx: TxState,
    rx: RxState,
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
    /// The descriptor lives at offset 0 of `region`, so the region's own pointer is
    /// the transport's base: there is no second address to keep in step with it.
    ///
    /// # Safety
    ///
    /// `region` must describe this channel's shared window: valid for reads and
    /// writes for its whole length for as long as the transport (or either half)
    /// lives, and touched by nothing but this transport and its peer through the
    /// descriptor/slot protocol.
    pub unsafe fn new(ch_tx: Chan, ch_rx: Chan, doorbell: D, region: R) -> Self {
        // SAFETY: the caller vouches for the region; the pointer is only used through
        // the descriptor/slot accessors, within its length.
        let base = unsafe { region.as_ptr() };
        Self {
            link: Link {
                doorbell,
                region,
                base,
            },
            tx: TxState {
                ch: ch_tx,
                ch_ack_rx: None,
                mtu: 0,
                ack_capable: false,
            },
            rx: RxState {
                ch: ch_rx,
                ch_ack_tx: None,
                mtu: 0,
            },
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
    ) -> Self {
        // SAFETY: forwarded to the caller.
        let mut this = unsafe { Self::new(ch_tx, ch_rx, doorbell, region) };
        this.tx.ch_ack_rx = Some(ch_ack_rx);
        this.rx.ch_ack_tx = Some(ch_ack_tx);
        this
    }

    /// Descriptor base pointer (offset 0x00 of the shared region).
    pub fn base(&self) -> *mut u8 {
        self.link.base
    }

    /// The doorbell this transport rings and waits on.
    pub fn doorbell(&self) -> &D {
        &self.link.doorbell
    }

    /// This side's transmit and receive channel ids, in `(tx, rx)` order.
    pub fn channels(&self) -> (Chan, Chan) {
        (self.tx.ch, self.rx.ch)
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
        self.link.region.invalidate();
        let base = self.link.base;

        // SAFETY: the `Link` invariant covers the descriptor header, and the
        // handshake has completed, so the channel table is published.
        let num_channels = unsafe { rv_u8(base, NUM_CHANNELS_OFFSET) };

        let (mut tx, mut rx): (Option<u32>, Option<u32>) = (None, None);
        // A channel this side cannot acknowledge on disables the capability for the whole
        // direction: `send_acked` picks whichever TX slot is free, so it must be able to
        // count on an acknowledgement from any of them.
        let mut tx_ack = None;
        for chan in 0..num_channels {
            // SAFETY: `chan` is within the published channel table range.
            let channel = unsafe { read_channel(base, chan as usize) };
            let capacity = channel.size.saturating_sub(slot::SLOT_HEADER as u32);
            if channel.direction.is_tx::<S>() {
                tx = Some(tx.map_or(capacity, |c: u32| c.min(capacity)));
                tx_ack = Some(tx_ack.unwrap_or(true) && channel.ack_capable());
            } else if channel.direction.is_rx::<S>() {
                rx = Some(rx.map_or(capacity, |c: u32| c.min(capacity)));
            }
        }

        self.tx.mtu = tx.unwrap_or(0);
        self.rx.mtu = rx.unwrap_or(0);
        self.tx.ack_capable = tx_ack.unwrap_or(false) && self.tx.ch_ack_rx.is_some();
        consortium_log::trace!(
            "connect: slot capacities tx {} rx {}, tx ack {}",
            self.tx.mtu,
            self.rx.mtu,
            self.tx.ack_capable
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
        let rx_link = self.link.clone();
        (
            SharedMemoryTransportTx {
                link: self.link,
                tx: self.tx,
                _side: PhantomData,
            },
            SharedMemoryTransportRx {
                link: rx_link,
                rx: self.rx,
                _side: PhantomData,
            },
        )
    }
}

/// Send-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportTx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    link: Link<D, R>,
    tx: TxState,
    _side: PhantomData<S>,
}

/// Receive-only half produced by [`SharedMemoryTransport::split`].
pub struct SharedMemoryTransportRx<S: Side, D: Doorbell, R: RegionSharedMemoryTransportable> {
    link: Link<D, R>,
    rx: RxState,
    _side: PhantomData<S>,
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

// The send completes eagerly inside `send()`, so the returned future is simply
// `Ready` with the result; it still captures the `&mut self` borrow.
impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransport<S, D, R>
{
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        core::future::ready(send_bytes::<S, D, R>(&self.link, &self.tx, data))
    }

    fn max_send_size(&self) -> usize {
        self.tx.mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> SendTransport
    for SharedMemoryTransportTx<S, D, R>
{
    fn send(&mut self, data: &[u8]) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        core::future::ready(send_bytes::<S, D, R>(&self.link, &self.tx, data))
    }

    fn max_send_size(&self) -> usize {
        self.tx.mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransport<S, D, R>
{
    fn recv(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        recv_bytes::<S, D, R>(&mut self.link, &self.rx, buf)
    }

    fn max_recv_size(&self) -> usize {
        self.rx.mtu as usize
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> RecvTransport
    for SharedMemoryTransportRx<S, D, R>
{
    fn recv(
        &mut self,
        buf: &mut [u8],
    ) -> impl Future<Output = Result<usize, Self::Error>> + MaybeSend {
        recv_bytes::<S, D, R>(&mut self.link, &self.rx, buf)
    }

    fn max_recv_size(&self) -> usize {
        self.rx.mtu as usize
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
        send_acked_bytes::<S, D, R>(&mut self.link, &self.tx, data)
    }
}

impl<S: Side, D: Doorbell + Send, R: RegionSharedMemoryTransportable + Send> AckSendTransport
    for SharedMemoryTransportTx<S, D, R>
{
    fn send_acked(
        &mut self,
        data: &[u8],
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        send_acked_bytes::<S, D, R>(&mut self.link, &self.tx, data)
    }
}

/// The primary end of the descriptor handshake: publish the layout, ring, and
/// wait for the secondary's ack.
///
/// The transport owns the region, so it performs the cache maintenance the
/// handshake needs on a non-coherent [`CachedRegion`] and records the negotiated
/// MTU afterwards.
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
        // SAFETY: the `Link` invariant covers the descriptor header and a channel
        // table of `params.num_channels` entries, and the region is DMA-coherent or
        // maintained through `R`.
        unsafe { publish_descriptor(self.link.base, params) };
        // Publish the descriptor to memory before the doorbell announces it.
        self.link.region.flush();

        self.link
            .doorbell
            .ring(self.tx.ch)
            .map_err(Error::Doorbell)?;
        self.link
            .doorbell
            .wait(self.rx.ch)
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
        self.link.region.invalidate();
        // SAFETY: the `Link` invariant covers the descriptor header.
        if !unsafe { descriptor_published(self.link.base) } {
            consortium_log::trace!(
                "connect(secondary): descriptor not published, awaiting primary doorbell"
            );
            self.link
                .doorbell
                .wait(self.rx.ch)
                .await
                .map_err(Error::Doorbell)?;
            self.link.region.invalidate();
        }

        // SAFETY: the `Link` invariant covers the descriptor header and channel
        // table; the gate above guarantees the primary finished writing it.
        let descriptor = unsafe { accept_descriptor(self.link.base, params) }?;
        // Push the validated state and agreed transfer size out before acking.
        self.link.region.flush();

        consortium_log::trace!(
            "connect(secondary): {} channels validated, acking primary",
            descriptor.num_channels
        );
        self.link
            .doorbell
            .ring(self.tx.ch)
            .map_err(Error::Doorbell)?;

        self.record_slot_capacities();
        Ok(())
    }
}
