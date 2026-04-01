#![no_std]

mod slot;

use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use consortium_ipc::{chan::Chan, doorbell::Doorbell, region::Region, transport::Transport};

// ── Error ────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum Error<D: Doorbell> {
    Doorbell(D::Error),
}

// ── MemTransport ─────────────────────────────────────────────────────────────

/// Bare-metal shared-memory transport for the M-core.
///
/// Each instance is bound to a single logical channel — one tx slot and one rx
/// slot inside the shared SRAM or DDR region negotiated at boot.
///
/// The doorbell is injected; `MemTransport` is agnostic to the signaling
/// mechanism (MU, HSEM, GPIO, …).  The region is injected for cache
/// coherency: after every write the region is flushed, before every read it
/// is invalidated.  On SRAM these are noops; on DDR they issue dcache insns.
pub struct MemTransport<D: Doorbell, R: Region> {
    ch: Chan,
    doorbell: D,
    region: R,
    /// Pointer to the transmit slot (M→A direction).
    tx_slot: *mut u8,
    /// Pointer to the receive slot (A→M direction).
    rx_slot: *mut u8,
    mtu: usize,
}

// SAFETY: tx_slot/rx_slot point into shared memory that is externally
// synchronised via the slot protocol (FLAG_VALID/FLAG_CONSUMED) and the
// doorbell.  The caller of `new` takes responsibility for aliasing.
unsafe impl<D: Doorbell + Send, R: Region + Send> Send for MemTransport<D, R> {}

impl<D: Doorbell, R: Region> MemTransport<D, R> {
    /// Construct a `MemTransport` for a single channel.
    ///
    /// # Safety
    ///
    /// - `tx_slot` and `rx_slot` must be valid for reads and writes for
    ///   `slot::SLOT_HEADER + mtu` bytes each, for the lifetime of this
    ///   `MemTransport`.
    /// - No other Rust reference to either slot may exist while this
    ///   transport is in use.
    /// - The memory must be accessible to both this core and the remote core.
    pub unsafe fn new(
        ch: Chan,
        doorbell: D,
        region: R,
        tx_slot: *mut u8,
        rx_slot: *mut u8,
        mtu: usize,
    ) -> Self {
        Self {
            ch,
            doorbell,
            region,
            tx_slot,
            rx_slot,
            mtu,
        }
    }
}

impl<D: Doorbell + Send, R: Region + Send> Transport for MemTransport<D, R>
where
    D::Error: Send,
{
    type Error = Error<D>;
    type SendFut<'a>
        = SendFut<'a, D>
    where
        Self: 'a;
    type RecvFut<'a>
        = RecvFut<'a, D, R>
    where
        Self: 'a;

    /// Write `data` into the tx slot, flush the region, and ring the doorbell.
    ///
    /// Fire-and-forget: returns immediately without waiting for the A-side to
    /// consume the message.
    fn send<'a>(&mut self, _ch: Chan, data: &[u8]) -> SendFut<'a, D> {
        // SAFETY: tx_slot is valid per the constructor contract.
        unsafe { slot::write(self.tx_slot, data) };
        self.region.flush();

        let result = self.doorbell.ring(self.ch).map_err(Error::Doorbell);

        SendFut {
            result: Some(result),
            _ph: PhantomData,
        }
    }

    /// Wait for data in the rx slot and copy it into `buf`.
    ///
    /// Invalidates the region before each read attempt so stale cache lines
    /// do not shadow fresh data written by the A-side.
    fn recv<'a>(&mut self, _ch: Chan, buf: &mut [u8]) -> RecvFut<'a, D, R> {
        RecvFut {
            transport: self as *mut Self,
            buf_ptr: buf.as_mut_ptr(),
            buf_len: buf.len(),
            _ph: PhantomData,
        }
    }

    fn max_message_size(&self) -> usize {
        self.mtu
    }
}

// ── SendFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`MemTransport::send`].
///
/// Always resolves on the first poll — write and doorbell ring are done
/// eagerly inside [`Transport::send`].
pub struct SendFut<'a, D: Doorbell> {
    result: Option<Result<(), Error<D>>>,
    _ph: PhantomData<&'a ()>,
}

// SendFut holds no borrows from the transport; the 'a is only the GAT
// parameter.  Marking Unpin allows get_mut() in poll.
impl<D: Doorbell> Unpin for SendFut<'_, D> {}

impl<D: Doorbell> Future for SendFut<'_, D> {
    type Output = Result<(), Error<D>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(
            self.get_mut()
                .result
                .take()
                .expect("SendFut polled after completion"),
        )
    }
}

// ── RecvFut ──────────────────────────────────────────────────────────────────

/// Future returned by [`MemTransport::recv`].
///
/// Polls the rx slot; if no data is present, polls the doorbell's wait future
/// to register the waker, then returns `Pending`.  The doorbell fires when the
/// A-side writes a message, waking the task for another poll.
///
/// A fresh `WaitFut` is created on each poll rather than stored across polls,
/// avoiding the self-referential lifetime problem that would arise from holding
/// `D::WaitFut<'a>` alongside a borrow of the doorbell inside the transport.
pub struct RecvFut<'a, D: Doorbell, R: Region> {
    /// Raw pointer back to the owning `MemTransport`.
    ///
    /// Valid for `'a` because `PhantomData<&'a mut MemTransport<D, R>>`
    /// logically holds the exclusive borrow, preventing other access while
    /// this future is alive.
    transport: *mut MemTransport<D, R>,
    buf_ptr: *mut u8,
    buf_len: usize,
    _ph: PhantomData<&'a mut MemTransport<D, R>>,
}

// SAFETY: `transport`, `buf_ptr`, and `buf_len` are exclusively owned by this
// future for `'a`.  `MemTransport` is `Send` when `D` and `R` are.
unsafe impl<D: Doorbell + Send, R: Region + Send> Send for RecvFut<'_, D, R> {}

impl<D: Doorbell, R: Region> Future for RecvFut<'_, D, R> {
    type Output = Result<usize, Error<D>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: RecvFut contains no self-referential fields and does not
        // implement Drop, so it is safe to get a mutable reference.
        let this = unsafe { self.get_unchecked_mut() };

        // SAFETY: transport pointer is valid and exclusively owned for 'a.
        let tr = unsafe { &mut *this.transport };

        // ── Step 1: attempt to read the slot ─────────────────────────────
        tr.region.invalidate();

        // SAFETY: rx_slot is valid per MemTransport::new; buf_ptr/buf_len
        // come from Channel's &'static mut [u8].
        let buf = unsafe { core::slice::from_raw_parts_mut(this.buf_ptr, this.buf_len) };

        if let Some(n) = unsafe { slot::try_read(tr.rx_slot, buf) } {
            return Poll::Ready(Ok(n));
        }

        // ── Step 2: no data yet — poll doorbell to register our waker ────
        //
        // A fresh WaitFut is created and immediately polled so the waker in
        // `cx` is registered with the doorbell's interrupt handler.  The
        // future is then dropped; when the doorbell fires the waker wakes
        // this task and we will create a new WaitFut on the next poll.
        let mut wait_fut = tr.doorbell.wait(tr.ch);

        // SAFETY: `wait_fut` is a local; we pin it only for the duration of
        // this poll call.  It will not be moved after pinning.
        match unsafe { Pin::new_unchecked(&mut wait_fut) }.poll(cx) {
            Poll::Ready(_) => {
                // Doorbell already fired before we polled — schedule
                // ourselves for an immediate re-poll to read the slot.
                cx.waker().wake_by_ref();
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}
