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
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, Side};

use crate::{
    abstraction::Ipcc,
    chan::{IPCC_CHAN_COUNT, IPCC_CHAN_OCCUPIED},
    error::IpccError,
};

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
        if IPCC_CHAN_OCCUPIED.contains(&id) {
            Err(IpccError::ChanReserved)
        } else if id >= IPCC_CHAN_COUNT {
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
        std::sync::Arc::clone(&self.notify)
    }
}

impl<S: Side, const N: usize> Doorbell for IpccDoorbell<S, N> {
    type Error = IpccError;
    type WaitFut<'a>
        = IpccWaitFut<'a, S, N>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), IpccError> {
        let id = Self::channel_of(ch)?;
        if self.ipcc.outgoing_occupied(id) {
            return Err(IpccError::ChannelOccupied);
        }

        compiler_fence(Ordering::Release);

        // SAFETY: `id` is validated `< 16`; SCR is this side's bank.
        unsafe { self.ipcc.set_outgoing_occupied(id) };
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        IpccWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        match Self::channel_of(ch) {
            Ok(id) => self.ipcc.incoming_occupied(id),
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
        Self {
            doorbell,
            ch,
            armed: false,
            #[cfg(feature = "std")]
            notified: None,
        }
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

        if !this.armed {
            // SAFETY: `id` is validated `< 16`; armed once from this task.
            unsafe { this.doorbell.ipcc.enable_rx_occupied_interrupt(id) };
            this.armed = true;

            #[cfg(feature = "std")]
            {
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

            #[cfg(not(feature = "std"))]
            IPCC_WAKER.register(cx.waker());
        }

        if this.doorbell.ipcc.incoming_occupied(id) {
            compiler_fence(Ordering::Acquire);

            // SAFETY: `id` is validated `< 16`; the incoming bit is set.
            unsafe { this.doorbell.ipcc.clear_incoming_occupied(id) };
            // SAFETY: `id` is validated `< 16`; re-mask after completion.
            unsafe { this.doorbell.ipcc.mask_rx_occupied_interrupt(id) };

            return Poll::Ready(Ok(this.ch));
        }

        #[cfg(feature = "std")]
        if let Some(notified) = this.notified.as_mut() {
            // SAFETY: `notified` lives inside the pinned future.
            let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
        }

        #[cfg(not(feature = "std"))]
        IPCC_WAKER.register(cx.waker());

        Poll::Pending
    }
}

// ── no_std: shared waker and IRQ entry point ─────────────────────────────────

/// Waker shared by all IPCC `wait()` futures on the bare-metal path.
#[cfg(not(feature = "std"))]
static IPCC_WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Wake tasks waiting on an IPCC RX-occupied interrupt. Call from the verified
/// STM32MP2 IPCC RX-occupied interrupt vector.
///
/// The exact NVIC line is intentionally not encoded here because it differs by
/// family and interrupt-routing configuration.
#[cfg(not(feature = "std"))]
pub fn notify_rx_occupied_interrupt() {
    IPCC_WAKER.wake();
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
    fn validation_rejects_reserved_and_out_of_range_channels() {
        let mut regs = [0u32; 8];
        let db = doorbell(&mut regs);

        assert_eq!(db.ring(ch(0)), Err(IpccError::ChanReserved));
        assert_eq!(db.ring(ch(16)), Err(IpccError::ChanOutOfRange));
    }
}
