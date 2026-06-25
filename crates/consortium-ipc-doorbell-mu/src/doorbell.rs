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

//! MU general-purpose-interrupt doorbell.
//!
//! [`MuDoorbell`] wraps a single-instance [`Mu`] HAL handle and implements
//! [`Doorbell`] over its GI channels:
//!
//! - [`ring`](Doorbell::ring) sets `GCR[GIRn]`, which the hardware reflects to
//!   `GSR[GIPn]` on the remote side and raises its GI interrupt when armed.
//! - [`wait`](Doorbell::wait) arms `GIER[GIEn]`, then resolves once `GSR[GIPn]`
//!   is observed, acknowledging it with a write-1-to-clear.
//! - [`pending`](Doorbell::pending) reads `GSR[GIPn]` non-destructively.

use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, Side};

use crate::{abstraction::Mu, chan::MU_CHAN_COUNT, error::MuError};

/// MU-backed doorbell for instance `N` (1-based), processor side `S`.
pub struct MuDoorbell<S: Side, const N: usize> {
    mu: Mu<S, N>,
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

impl<S: Side, const N: usize> MuDoorbell<S, N> {
    #[inline]
    fn gi_of(ch: Chan) -> Result<u8, MuError> {
        let gi = ch.id();
        if gi >= MU_CHAN_COUNT {
            Err(MuError::ChanOutOfRange)
        } else {
            Ok(gi)
        }
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> MuDoorbell<consortium_ipc::Primary, N> {
    /// Create a doorbell over the side-A (MUA) view of MU instance `N` using
    /// the compile-time base from [`crate::registers::base`].
    ///
    /// # Safety
    /// Grants raw MU register access; the caller must ensure no conflicting
    /// handle to the same instance/side exists.
    pub const unsafe fn steal() -> Self {
        Self {
            // SAFETY: forwarded obligation from this constructor.
            mu: unsafe { Mu::<consortium_ipc::Primary, N>::steal() },
        }
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> MuDoorbell<consortium_ipc::Secondary, N> {
    /// Create a doorbell over the side-B (MUB) view of MU instance `N` using
    /// the compile-time base from [`crate::registers::base`].
    ///
    /// # Safety
    /// Grants raw MU register access; the caller must ensure no conflicting
    /// handle to the same instance/side exists.
    pub const unsafe fn steal() -> Self {
        Self {
            // SAFETY: forwarded obligation from this constructor.
            mu: unsafe { Mu::<consortium_ipc::Secondary, N>::steal() },
        }
    }
}

#[cfg(feature = "std")]
impl<S: Side, const N: usize> MuDoorbell<S, N> {
    /// Create a doorbell over MU instance `N`, side `S`, from an `mmap`'d
    /// virtual base address.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of MU instance `N` for the side
    /// matching `S`. Passing a raw physical address is undefined behaviour on
    /// Linux.
    pub unsafe fn new(base: usize) -> Self {
        Self {
            // SAFETY: forwarded obligation; `base` is the mapped MU window.
            mu: unsafe { Mu::<S, N>::new(base) },
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`. Pass it to the UIO IRQ
    /// loop so it can wake waiting [`wait`](Doorbell::wait) futures via
    /// `notify_waiters()` when this instance's GI interrupt fires.
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

impl<S: Side, const N: usize> Doorbell for MuDoorbell<S, N> {
    type Error = MuError;
    type WaitFut<'a>
        = MuWaitFut<'a, S, N>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), MuError> {
        let gi = Self::gi_of(ch)?;

        // Ensure preceding slot writes are visible to the remote before the
        // GCR write that signals it.
        compiler_fence(Ordering::Release);

        // SAFETY: `gi` is validated `< 4`; ring is the sole writer of GCR here.
        unsafe { self.mu.request_gi(gi) };
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        MuWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        match Self::gi_of(ch) {
            Ok(gi) => self.mu.gi_pending(gi),
            Err(_) => false,
        }
    }
}

// ── Wait future ───────────────────────────────────────────────────────────────

/// Future returned by [`MuDoorbell::wait`].
///
/// Arms `GIER[GIEn]` on the first poll, then completes once `GSR[GIPn]` is
/// observed, acknowledging it (W1C) and disarming the interrupt. On `no_std`
/// the per-instance `AtomicWaker` is woken from `MUx_IRQHandler`; on `std` the
/// doorbell's `Notify` is driven by the UIO IRQ loop.
pub struct MuWaitFut<'a, S: Side, const N: usize> {
    doorbell: &'a MuDoorbell<S, N>,
    ch: Chan,
    armed: bool,
    #[cfg(feature = "std")]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

impl<'a, S: Side, const N: usize> MuWaitFut<'a, S, N> {
    fn new(doorbell: &'a MuDoorbell<S, N>, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            armed: false,
            #[cfg(feature = "std")]
            notified: None,
        }
    }
}

impl<'a, S: Side, const N: usize> core::future::Future for MuWaitFut<'a, S, N> {
    type Output = Result<Chan, MuError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: no pinned field is moved; `notified` is only set in place.
        let this = unsafe { self.get_unchecked_mut() };
        let gi = match MuDoorbell::<S, N>::gi_of(this.ch) {
            Ok(gi) => gi,
            Err(err) => return Poll::Ready(Err(err)),
        };

        if !this.armed {
            // SAFETY: `gi` is validated `< 4`; armed once from this task.
            unsafe { this.doorbell.mu.enable_gi(gi) };
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
            waker_for::<N>().register(cx.waker());
        }

        if this.doorbell.mu.gi_pending(gi) {
            // Acquire fence: pairs with the remote's Release before ring(), so
            // our subsequent slot reads observe the remote's writes.
            compiler_fence(Ordering::Acquire);

            // SAFETY: GSR[GIPn] confirmed set; `gi` is valid.
            unsafe { this.doorbell.mu.clear_gi(gi) };
            // SAFETY: `gi` is valid; disarm after completion.
            unsafe { this.doorbell.mu.disable_gi(gi) };

            return Poll::Ready(Ok(this.ch));
        }

        #[cfg(feature = "std")]
        if let Some(notified) = this.notified.as_mut() {
            // SAFETY: `notified` lives inside the pinned future.
            let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
        }

        #[cfg(not(feature = "std"))]
        waker_for::<N>().register(cx.waker());

        Poll::Pending
    }
}

// ── no_std: per-instance wakers and IRQ handlers ─────────────────────────────

#[cfg(not(feature = "std"))]
#[inline]
fn waker_for<const N: usize>() -> &'static atomic_waker::AtomicWaker {
    &MU_WAKERS[N - 1]
}

#[cfg(not(feature = "std"))]
static MU_WAKERS: [atomic_waker::AtomicWaker; crate::chan::MU_INSTANCE_COUNT] =
    [const { atomic_waker::AtomicWaker::new() }; crate::chan::MU_INSTANCE_COUNT];

// One IRQ handler per MU instance, chip-gated. Each wakes the futures waiting
// on that instance's GI interrupt; register them in the interrupt vector table.

/// # Safety
/// Call only from the MU1 general-purpose IRQ handler.
#[cfg(all(
    not(feature = "std"),
    any(feature = "imx93", feature = "imx95", feature = "imx8mp")
))]
#[unsafe(no_mangle)]
pub extern "C" fn MU1_IRQHandler() {
    MU_WAKERS[0].wake();
}

/// # Safety
/// Call only from the MU2 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), any(feature = "imx93", feature = "imx95")))]
#[unsafe(no_mangle)]
pub extern "C" fn MU2_IRQHandler() {
    MU_WAKERS[1].wake();
}

/// # Safety
/// Call only from the MU3 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU3_IRQHandler() {
    MU_WAKERS[2].wake();
}

/// # Safety
/// Call only from the MU4 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU4_IRQHandler() {
    MU_WAKERS[3].wake();
}

/// # Safety
/// Call only from the MU5 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU5_IRQHandler() {
    MU_WAKERS[4].wake();
}

/// # Safety
/// Call only from the MU6 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU6_IRQHandler() {
    MU_WAKERS[5].wake();
}

/// # Safety
/// Call only from the MU7 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU7_IRQHandler() {
    MU_WAKERS[6].wake();
}

/// # Safety
/// Call only from the MU8 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU8_IRQHandler() {
    MU_WAKERS[7].wake();
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::registers::{cr::GeneralInterruptRegister, offsets};

    fn ch(id: u8) -> Chan {
        // SAFETY: tests pick ids deliberately.
        unsafe { Chan::new_unchecked(id) }
    }

    // MU7 on i.MX95 is the CA55 <-> CM7 default route.
    const N: usize = if cfg!(feature = "imx95") { 7 } else { 1 };

    fn gier(regs: &[u32]) -> GeneralInterruptRegister {
        GeneralInterruptRegister::from_bits(regs[offsets::GIER as usize / 4])
    }

    #[test]
    fn ring_sets_gcr_bit() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db =
            unsafe { MuDoorbell::<consortium_ipc::Primary, N>::new(regs.as_mut_ptr() as usize) };

        db.ring(ch(2)).unwrap();

        let gcr = GeneralInterruptRegister::from_bits(regs[offsets::GCR as usize / 4]);
        assert!(gcr.gi() & (1 << 2) != 0);
    }

    #[test]
    fn pending_reads_gsr() {
        let mut regs = [0u32; 0x300 / 4];
        regs[offsets::GSR as usize / 4] = 1 << 3;
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db =
            unsafe { MuDoorbell::<consortium_ipc::Primary, N>::new(regs.as_mut_ptr() as usize) };

        assert!(db.pending(ch(3)));
        assert!(!db.pending(ch(2)));
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db =
            unsafe { MuDoorbell::<consortium_ipc::Primary, N>::new(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(gier(&regs).gi() & (1 << 2) != 0);

        regs[offsets::GSR as usize / 4] = 1 << 2;
        db.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        // GIER bit cleared on completion.
        assert!(gier(&regs).gi() & (1 << 2) == 0);
        // GSR acknowledged (W1C).
        assert_eq!(regs[offsets::GSR as usize / 4], 1 << 2);
    }

    #[test]
    fn wait_thread_wakes_after_remote_status_and_notify() {
        let regs = Box::leak(Box::new([0u32; 0x300 / 4]));
        let base = regs.as_mut_ptr() as usize;
        // SAFETY: `regs` backs a fake MMIO window leaked for the spawned thread.
        let db = unsafe { MuDoorbell::<consortium_ipc::Primary, N>::new(base) };
        let notifier = db.notifier();
        let (armed_tx, armed_rx) = std::sync::mpsc::channel();

        let waiter = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create waiter runtime");
            rt.block_on(async move {
                armed_tx.send(()).expect("signal waiter is starting");
                db.wait(ch(2)).await.expect("wait for MU doorbell")
            })
        });

        armed_rx.recv().expect("waiter thread started");
        while GeneralInterruptRegister::from_bits(unsafe {
            (base as *const u32)
                .add(offsets::GIER as usize / 4)
                .read_volatile()
        })
        .gi()
            & (1 << 2)
            == 0
        {
            std::thread::yield_now();
        }

        unsafe {
            (base as *mut u32)
                .add(offsets::GSR as usize / 4)
                .write_volatile(1 << 2);
        }
        notifier.notify_waiters();

        assert_eq!(waiter.join().expect("waiter thread completes"), ch(2));
        assert_eq!(regs[offsets::GSR as usize / 4], 1 << 2);
    }
}
