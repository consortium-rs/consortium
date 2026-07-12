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

//! STM32 IPCC doorbell.
//!
//! [`IpccDoorbell`] wraps an [`Ipcc`] HAL handle and implements [`Doorbell`]
//! with simplex signaling over the IPCC channels:
//!
//! - [`ring`](Doorbell::ring) sets this side's outgoing channel occupied
//!   (`CxSCR.CHnS`), which the remote observes in its incoming status register.
//! - [`wait`](Doorbell::wait) arms the RX-occupied interrupt, then resolves once
//!   the incoming channel is occupied, acknowledging it (`CxSCR.CHnC`).
//! - [`pending`](Doorbell::pending) reads the incoming status non-destructively.

use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence, fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, Side};
use consortium_log::{debug, error, trace};

use crate::{abstraction::Ipcc, chan::IPCC_CHAN_COUNT, error::IpccError};

#[cfg(not(feature = "std"))]
#[inline]
fn with_interrupts_masked<R>(f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| f())
}

#[cfg(feature = "std")]
#[inline]
fn with_interrupts_masked<R>(f: impl FnOnce() -> R) -> R {
    f()
}

/// IPCC-backed doorbell for instance `N` (1-based), processor side `S`.
pub struct IpccDoorbell<S: Side, const N: usize> {
    ipcc: Ipcc<S, N>,
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// Manual `Clone` (bound independent of `S`): the transport's `split` clones the
// doorbell for the Rx half. Both halves share the same `Notify`, so an IRQ-side
// wake reaches whichever half is waiting.
impl<S: Side, const N: usize> Clone for IpccDoorbell<S, N> {
    fn clone(&self) -> Self {
        Self {
            ipcc: self.ipcc.clone(),
            #[cfg(feature = "std")]
            notify: self.notify.clone(),
        }
    }
}

impl<S: Side, const N: usize> IpccDoorbell<S, N> {
    #[inline]
    fn channel_of(ch: Chan) -> Result<u8, IpccError> {
        let id = ch.id();
        if id >= IPCC_CHAN_COUNT {
            error!("channel_of({:?}) failed: out of range", ch);
            Err(IpccError::ChanOutOfRange)
        } else {
            Ok(id)
        }
    }
}

impl<S: Side, const N: usize> IpccDoorbell<S, N> {
    /// Create a doorbell over IPCC instance `N`, side `S`, from an `mmap`'d
    /// virtual base address.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of IPCC instance `N`. Passing a
    /// raw physical address is undefined behaviour on Linux.
    pub unsafe fn new(base: usize) -> Self {
        trace!("IpccDoorbell::new(base = {:#x})", base);
        Self {
            // SAFETY: forwarded obligation; `base` is the mapped IPCC window.
            ipcc: unsafe { Ipcc::<S, N>::new(base) },
            #[cfg(feature = "std")]
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`. Pass it to the UIO IRQ
    /// loop or platform IRQ adapter so it can wake waiting
    /// [`wait`](Doorbell::wait) futures when IPCC RX-occupied fires.
    #[cfg(feature = "std")]
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        trace!("IpccDoorbell::notifier() -> Arc<Notify>");
        std::sync::Arc::clone(&self.notify)
    }

    /// Service an IPCC RX-occupied interrupt on the `std`/UIO side.
    ///
    /// Prefer this over driving [`notifier`](Self::notifier) directly: it masks
    /// the asserted incoming channels *before* waking, mirroring the `no_std`
    /// [`notify_rx_occupied_interrupt`] so a level-sensitive line does not
    /// immediately re-fire the UIO IRQ. Waking then notifies every waiting
    /// [`wait`](Doorbell::wait) future through the shared `Notify`, each of which
    /// re-checks and acknowledges its own channel.
    ///
    /// The doorbell is [`Clone`], so an integrator can clone it before moving the
    /// original into a transport and bind this method to the UIO IRQ fanout.
    #[cfg(feature = "std")]
    pub fn notify_rx_occupied_interrupt(&self) {
        // `Notify::notify_waiters` is not per-channel, so every asserted channel
        // wakes the same shared `Notify`; the calls after the first find no
        // registered waiters (they re-register only when polled) and are no-ops.
        service_rx_occupied(&self.ipcc, |_id| self.notify.notify_waiters());
    }
}

impl<S: Side, const N: usize> Doorbell for IpccDoorbell<S, N> {
    type Error = IpccError;
    type WaitFut<'a>
        = IpccWaitFut<'a, S, N>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), IpccError> {
        trace!("IpccDoorbell::ring(ch = {:?})", ch);
        let id = Self::channel_of(ch)?;
        if self.ipcc.outgoing_occupied(id) {
            debug!("ring({:?}) failed: outgoing channel occupied", ch);
            return Err(IpccError::ChannelOccupied);
        }

        trace!("ring({:?}) -> set C{}SCR.CH{}S", ch, N, id);

        fence(Ordering::Release);

        // SAFETY: `id` is validated `< 16`; SCR is this side's bank.
        unsafe { self.ipcc.set_outgoing_occupied(id) };
        debug!("ring({:?}) -> C{}SCR.CH{}S set", ch, N, id);

        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        trace!("IpccDoorbell::wait(ch = {:?})", ch);
        IpccWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        trace!("IpccDoorbell::pending(ch = {:?})", ch);
        match Self::channel_of(ch) {
            Ok(id) => {
                let pending = self.ipcc.incoming_occupied(id);
                debug!("pending({:?}) -> result = {}", ch, pending);
                if pending {
                    fence(Ordering::Acquire);
                }
                pending
            }
            Err(_) => false,
        }
    }
}

// ── Wait future ───────────────────────────────────────────────────────────────

/// Future returned by [`IpccDoorbell::wait`].
///
/// Arms the RX-occupied interrupt on the first poll, then completes once the
/// incoming channel is occupied, acknowledging it and re-masking the interrupt.
/// On `no_std` the shared `AtomicWaker` is woken from
/// [`notify_rx_occupied_interrupt`]; on `std` the doorbell's `Notify` is driven
/// by the UIO IRQ loop.
pub struct IpccWaitFut<'a, S: Side, const N: usize> {
    doorbell: &'a IpccDoorbell<S, N>,
    ch: Chan,
    armed: bool,
    #[cfg(feature = "std")]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

impl<'a, S: Side, const N: usize> IpccWaitFut<'a, S, N> {
    fn new(doorbell: &'a IpccDoorbell<S, N>, ch: Chan) -> Self {
        trace!("IpccWaitFut::new(doorbell, ch = {:?})", ch);
        Self {
            doorbell,
            ch,
            armed: false,
            #[cfg(feature = "std")]
            notified: None,
        }
    }
}

impl<S: Side, const N: usize> Drop for IpccWaitFut<'_, S, N> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }

        let Ok(id) = IpccDoorbell::<S, N>::channel_of(self.ch) else {
            return;
        };
        with_interrupts_masked(|| {
            // SAFETY: `id` was validated and this future owns the armed wait.
            unsafe { self.doorbell.ipcc.mask_rx_occupied_interrupt(id) };
        });
    }
}

impl<'a, S: Side, const N: usize> core::future::Future for IpccWaitFut<'a, S, N> {
    type Output = Result<Chan, IpccError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: no pinned field is moved; `notified` is only set in place.
        let this = unsafe { self.get_unchecked_mut() };
        let id = match IpccDoorbell::<S, N>::channel_of(this.ch) {
            Ok(id) => id,
            Err(err) => return Poll::Ready(Err(err)),
        };

        trace!(
            "IpccWaitFut::poll(ch = {:?}) armed = {}",
            this.ch, this.armed
        );

        #[cfg(not(feature = "std"))]
        channel_waker::<N>(id).register(cx.waker());

        compiler_fence(Ordering::SeqCst);

        if !this.armed {
            debug!("wait({:?}) -> arm C{}SCR.CH{} interrupt", this.ch, N, id);
            // Serialize the CxCR/CxMR read-modify-write with the ISR.
            with_interrupts_masked(|| {
                // SAFETY: `id` is validated `< 16`; armed once from this task.
                unsafe { this.doorbell.ipcc.enable_rx_occupied_interrupt(id) };
            });
            this.armed = true;

            #[cfg(feature = "std")]
            {
                debug!(
                    "wait({:?}) -> register Notify for C{}TOC{}SR.CH{} interrupt",
                    this.ch,
                    N,
                    S::INDEX,
                    id
                );
                let notified = this.doorbell.notify.notified();
                // SAFETY: `doorbell` is borrowed for `'a`; its Arc-backed
                // `Notify` outlives this future, so erasing the borrow is sound.
                let notified = unsafe {
                    core::mem::transmute::<
                        tokio::sync::futures::Notified<'_>,
                        tokio::sync::futures::Notified<'static>,
                    >(notified)
                };
                this.notified = Some(notified);
            }
        }

        if this.doorbell.ipcc.incoming_occupied(id) {
            debug!(
                "wait({:?}) -> C{}TOC{}SR.CH{} occupied",
                this.ch,
                N,
                S::INDEX,
                id
            );
            fence(Ordering::Acquire);

            // SAFETY: `id` is validated `< 16`; the incoming bit is set.
            unsafe { this.doorbell.ipcc.clear_incoming_occupied(id) };
            consortium_log::trace!("cleared incoming occupied channel {}", id);
            with_interrupts_masked(|| {
                // SAFETY: `id` is validated `< 16`; re-mask after completion.
                unsafe { this.doorbell.ipcc.mask_rx_occupied_interrupt(id) };
            });
            consortium_log::trace!("interrupt masked at channel {}", id);

            consortium_log::debug!("wait({:?}) -> C{}SCR.CH{} interrupt masked", this.ch, N, id);

            return Poll::Ready(Ok(this.ch));
        }

        #[cfg(feature = "std")]
        if let Some(notified) = this.notified.as_mut() {
            debug!(
                "wait({:?}) -> poll Notify for C{}TOC{}SR.CH{} interrupt",
                this.ch,
                N,
                S::INDEX,
                id
            );
            // SAFETY: `notified` lives inside the pinned future.
            let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
        }

        Poll::Pending
    }
}

// ── shared IRQ servicing ─────────────────────────────────────────────────────

/// Service an IPCC RX-occupied interrupt: read the asserted incoming channels,
/// mask them all in one `CxMR` read-modify-write *before* waking any waiter,
/// then invoke `wake(id)` for each asserted channel.
///
/// Masking before the wake is what keeps a level-sensitive IPCC line from
/// re-asserting immediately (on `no_std`, re-entering the ISR; on `std`,
/// re-firing the UIO IRQ). The wait future subsequently acknowledges the
/// occupied flag, and the next wait unmasks the channel again.
///
/// This is the common core shared by the `no_std` ISR entry point and the `std`
/// notifier; only the wake primitive differs (`AtomicWaker::wake` vs
/// `Notify::notify_waiters`).
fn service_rx_occupied<S: Side, const N: usize>(ipcc: &Ipcc<S, N>, mut wake: impl FnMut(u8)) {
    // Like embassy-stm32, use the remote status directly. Do not filter it
    // through CxMR: an already-masked occupied channel must still be serviced.
    let occupied = ipcc.incoming_occupied_channels();

    // Mask all asserted channels in one CxMR read-modify-write before waking.
    with_interrupts_masked(|| {
        // SAFETY: the bitmap comes directly from the 16-channel status register.
        unsafe { ipcc.mask_rx_occupied_channels(occupied) };
    });

    for id in 0..IPCC_CHAN_COUNT {
        if occupied & (1u16 << id) != 0 {
            wake(id);
        }
    }
}

// ── no_std: shared waker and IRQ entry point ─────────────────────────────────

/// Wakers separated by IPCC instance and channel on the bare-metal path.
#[cfg(not(feature = "std"))]
static IPCC_WAKERS: [[atomic_waker::AtomicWaker; IPCC_CHAN_COUNT as usize]; 2] =
    [const { [const { atomic_waker::AtomicWaker::new() }; IPCC_CHAN_COUNT as usize] }; 2];

#[cfg(not(feature = "std"))]
#[inline]
fn channel_waker<const N: usize>(id: u8) -> &'static atomic_waker::AtomicWaker {
    &IPCC_WAKERS[N - 1][id as usize]
}

/// Service an IPCC RX-occupied interrupt. Call from the corresponding vector.
///
/// Delegates to [`service_rx_occupied`]: asserted channels are masked before
/// their per-channel [`AtomicWaker`](atomic_waker::AtomicWaker)s are woken.
///
/// # Safety
///
/// `base` must be the mapped base of IPCC instance `N`, accessible from side
/// `S`. It has the same requirements as [`IpccDoorbell::new`].
#[cfg(not(feature = "std"))]
pub unsafe fn notify_rx_occupied_interrupt<S: Side, const N: usize>(base: usize) {
    // SAFETY: guaranteed by the caller; the temporary handle accesses the same
    // MMIO window as the doorbell used by the waiting task.
    let ipcc = unsafe { Ipcc::<S, N>::new(base) };
    service_rx_occupied(&ipcc, |id| channel_waker::<N>(id).wake());
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::registers::{
        cr::{ChannelRegister, ControlRegister, StatusSetClearRegister},
        offsets,
    };
    use consortium_ipc::Primary;

    fn ch(id: u8) -> Chan {
        // SAFETY: tests pick ids deliberately to exercise validation paths.
        unsafe { Chan::new_unchecked(id) }
    }

    fn set_flag(ch: u8) -> u32 {
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_set(1 << ch);
        reg.bits()
    }

    fn clear_flag(ch: u8) -> u32 {
        let mut reg = StatusSetClearRegister::from_bits(0);
        reg.set_channel_clear(1 << ch);
        reg.bits()
    }

    fn occupied(ch: u8) -> u32 {
        ChannelRegister::from_bits(1 << ch).bits()
    }

    // Primary is processor 1; it writes C1SCR and reads C2TOC1SR.
    fn doorbell(regs: &mut [u32]) -> IpccDoorbell<Primary, 1> {
        // SAFETY: `regs` backs a fake MMIO window for the test.
        unsafe { IpccDoorbell::<Primary, 1>::new(regs.as_mut_ptr() as usize) }
    }

    #[test]
    fn ring_sets_processor_1_channel_occupied() {
        let mut regs = [0u32; 8];
        let db = doorbell(&mut regs);

        db.ring(ch(2)).unwrap();

        assert_eq!(regs[offsets::C1SCR / 4], set_flag(2));
    }

    #[test]
    fn ring_rejects_occupied_outgoing_channel() {
        let mut regs = [0u32; 8];
        regs[offsets::C1TOC2SR / 4] = occupied(2);
        let db = doorbell(&mut regs);

        assert_eq!(db.ring(ch(2)), Err(IpccError::ChannelOccupied));
    }

    #[test]
    fn pending_reads_incoming_status() {
        let mut regs = [0u32; 8];
        regs[offsets::C2TOC1SR / 4] = occupied(3);
        let db = doorbell(&mut regs);

        assert!(db.pending(ch(3)));
        assert!(!db.pending(ch(4)));
    }

    #[test]
    fn wait_clears_incoming_channel() {
        let mut regs = [0u32; 8];
        regs[offsets::C2TOC1SR / 4] = occupied(2);
        let db = doorbell(&mut regs);
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert!(ControlRegister::from_bits(regs[offsets::C1CR / 4]).rxoie());
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2) != 0);
        assert_eq!(regs[offsets::C1SCR / 4], clear_flag(2));
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify_and_incoming_status() {
        let mut regs = [0u32; 8];
        regs[offsets::C1MR / 4] = u32::MAX;
        let db = doorbell(&mut regs);
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(ControlRegister::from_bits(regs[offsets::C1CR / 4]).rxoie());
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2) == 0);

        regs[offsets::C2TOC1SR / 4] = occupied(2);
        db.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert_eq!(regs[offsets::C1SCR / 4], clear_flag(2));
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2) != 0);
    }

    #[test]
    fn occupied_bitmap_is_masked_in_one_update() {
        let mut regs = [0u32; 8];
        regs[offsets::C2TOC1SR / 4] = occupied(2) | occupied(5);
        regs[offsets::C1MR / 4] = 1 << 7;
        let db = doorbell(&mut regs);

        let occupied = db.ipcc.incoming_occupied_channels();
        // SAFETY: the bitmap was read from the fake 16-channel status register.
        unsafe { db.ipcc.mask_rx_occupied_channels(occupied) };

        assert_eq!(occupied, (1 << 2) | (1 << 5));
        assert_eq!(
            ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels(),
            (1 << 2) | (1 << 5) | (1 << 7)
        );
    }

    #[test]
    fn notify_rx_occupied_masks_channels_before_waking_waiters() {
        let mut regs = [0u32; 8];
        regs[offsets::C1MR / 4] = u32::MAX;
        let db = doorbell(&mut regs);
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        // Arm the wait: unmasks channel 2, then parks pending.
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2) == 0);

        // The remote asserts channel 2 and the IRQ entry point runs. Unlike a bare
        // `notify_waiters`, it masks the occupied channel *before* waking, so a
        // level-sensitive line cannot immediately re-fire.
        regs[offsets::C2TOC1SR / 4] = occupied(2);
        db.notify_rx_occupied_interrupt();
        assert!(
            ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2) != 0,
            "channel 2 must be masked by the IRQ entry point, before any waiter re-checks it"
        );

        // The wake reached the parked future, which completes and acknowledges.
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert_eq!(regs[offsets::C1SCR / 4], clear_flag(2));
    }

    #[test]
    fn dropping_pending_wait_masks_its_channel() {
        let mut regs = [0u32; 8];
        regs[offsets::C1MR / 4] = u32::MAX;
        let db = doorbell(&mut regs);
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        {
            let mut wait = std::pin::pin!(db.wait(ch(2)));
            assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
            assert_eq!(
                ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2),
                0
            );
        }

        assert_ne!(
            ChannelRegister::from_bits(regs[offsets::C1MR / 4]).channels() & (1 << 2),
            0
        );
    }

    #[test]
    fn validation_rejects_out_of_range_channels() {
        let mut regs = [0u32; 8];
        let db = doorbell(&mut regs);

        // Reserved-channel policy is enforced at `Chan::new::<IpccChanValidate>`
        // (covered in `chan.rs`); `ring` itself only bounds-checks the id.
        assert_eq!(db.ring(ch(16)), Err(IpccError::ChanOutOfRange));
    }
}
