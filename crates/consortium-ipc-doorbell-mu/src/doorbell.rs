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

//! MU doorbell over general-purpose-interrupt (GI) and receive-full (RF) lines.
//!
//! [`MuDoorbell`] wraps a single-instance [`Mu`] HAL handle and implements
//! [`Doorbell`] over a channel space that packs two line kinds: channels
//! `0..4` are GI lines and channels `4..8` are RF lines (see
//! [`crate::chan`]).
//!
//! On a GI channel:
//!
//! - [`ring`](Doorbell::ring) sets `GCR[GIRn]`, which the hardware reflects to
//!   `GSR[GIPn]` on the remote side and raises its GI interrupt when armed.
//! - [`wait`](Doorbell::wait) arms `GIER[GIEn]`, then resolves once `GSR[GIPn]`
//!   is observed, acknowledging it with a write-1-to-clear.
//! - [`pending`](Doorbell::pending) reads `GSR[GIPn]` non-destructively.
//!
//! On an RF channel:
//!
//! - [`ring`](Doorbell::ring) writes transmit register `TRn`, which sets
//!   `RSR[RFn]` on the remote side and raises its receive interrupt when armed.
//! - [`wait`](Doorbell::wait) arms `RCR[RIEn]`, then resolves once `RSR[RFn]`
//!   is observed, acknowledging it by reading `RRn`.
//! - [`pending`](Doorbell::pending) reads `RSR[RFn]` non-destructively.

use core::{
    pin::Pin,
    sync::atomic::{Ordering, fence},
    task::{Context, Poll},
};

use consortium_hal_imx9::{Peri, PeripheralType};
use consortium_ipc::{Chan, Doorbell, Side};

use crate::{
    abstraction::Mu,
    chan::{MU_CHAN_COUNT, MU_GI_CHAN_COUNT},
    error::MuError,
    registers::base,
};

/// A chip-HAL peripheral singleton naming one MU instance window.
///
/// The dump names encode both facts this needs: `MU7_MUB` is instance 7 seen
/// from side B. The window's address still comes from [`base::BASE`] in this
/// crate rather than from the singleton, so the security alias the doorbell has
/// always used does not change underfoot.
pub trait Instance: PeripheralType {
    /// Processor side this window belongs to: [`Primary`](consortium_ipc::Primary)
    /// is MUA, [`Secondary`](consortium_ipc::Secondary) is MUB.
    type Side: Side;
    /// 1-based MU instance number.
    const INSTANCE: usize;
}

/// Bind a chip-HAL peripheral singleton to this doorbell.
macro_rules! impl_mu {
    ($instance:ident, $side:ty, $number:expr) => {
        impl crate::doorbell::Instance for ::consortium_hal_imx9::peripherals::$instance {
            type Side = $side;
            const INSTANCE: usize = $number;
        }
    };
}

// The i.MX95 CM7 reaches MU5 side A and MU7/MU8 side B; the i.MX93 CM33 reaches
// MU1/MU2 side A. Both lists come from the same per-core dumps that generate the
// HAL's singletons, so a window this core cannot see has no singleton to pass.
#[cfg(feature = "mimx95")]
mod bindings {
    use consortium_ipc::{Primary, Secondary};

    impl_mu!(MU5_MUA, Primary, 5);
    impl_mu!(MU7_MUB, Secondary, 7);
    impl_mu!(MU8_MUB, Secondary, 8);
}

#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
mod bindings {
    use consortium_ipc::Primary;

    impl_mu!(MU1_MUA, Primary, 1);
    impl_mu!(MU2_MUA, Primary, 2);
}

/// MU-backed doorbell for instance `N` (1-based), processor side `S`.
pub struct MuDoorbell<S: Side, const N: usize> {
    mu: Mu<S, N>,
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// Manual `Clone` (bound independent of `S`): the transport's `split` clones the
// doorbell for the Rx half. Both halves share the same `Notify`, so an IRQ-side
// wake reaches whichever half is waiting.
impl<S: Side, const N: usize> Clone for MuDoorbell<S, N> {
    fn clone(&self) -> Self {
        Self {
            mu: self.mu.clone(),
            #[cfg(feature = "std")]
            notify: self.notify.clone(),
        }
    }
}

/// The hardware line a doorbell [`Chan`] maps to within an MU instance.
#[derive(Clone, Copy)]
enum MuLine {
    /// General-purpose interrupt line, index `< 4` (`GCR`/`GSR`/`GIER`).
    Gi(u8),
    /// Receive-full line, index `< 4` (transmit/receive registers, `RSR`/`RCR`).
    Rf(u8),
}

impl<S: Side, const N: usize> MuDoorbell<S, N> {
    /// Classify a channel id into its GI or RF hardware line. Channels
    /// `0..MU_GI_CHAN_COUNT` are GI lines; the following `MU_RF_CHAN_COUNT`
    /// channels are RF lines.
    #[inline]
    fn line_of(ch: Chan) -> Result<MuLine, MuError> {
        let id = ch.id();
        if id < MU_GI_CHAN_COUNT {
            Ok(MuLine::Gi(id))
        } else if id < MU_CHAN_COUNT {
            Ok(MuLine::Rf(id - MU_GI_CHAN_COUNT))
        } else {
            Err(MuError::ChanOutOfRange)
        }
    }
}

impl<S: Side, const N: usize> MuDoorbell<S, N> {
    /// Create a doorbell over MU instance `N`, side `S`, from the chip HAL's
    /// peripheral singleton.
    ///
    /// This is the firmware-side constructor: the singleton proves nothing else
    /// drives the instance, and the window address comes from this crate's
    /// [`base::BASE`] table. `S` and `N` must match the singleton, which the
    /// const assertions below check at compile time.
    ///
    /// On Linux the window is an `mmap` of a UIO device rather than a physical
    /// address, and there is no singleton for it — use
    /// [`new_mapped`](Self::new_mapped) there.
    pub fn new<T: Instance<Side = S>>(peri: Peri<'static, T>) -> Self {
        const {
            assert!(
                T::INSTANCE == N,
                "MuDoorbell instance number does not match the peripheral singleton",
            );
        }
        // Owning the singleton for the doorbell's lifetime is what makes the
        // MMIO access exclusive; it is consumed here and never reissued.
        let _ = peri;
        // Side B sits one window above side A of the same instance.
        let window = usize::from(S::INDEX) * base::LENGTH;
        // SAFETY: `base::BASE[N - 1] + window` is instance `N`'s side-`S`
        // window from this crate's vetted address table, and the consumed
        // singleton guarantees nothing else drives it.
        unsafe { Self::new_mapped(base::BASE[N - 1] + window) }
    }

    /// Create a doorbell over MU instance `N`, side `S`, from an `mmap`'d
    /// virtual base address.
    ///
    /// This is the Linux/UIO constructor: mapped windows have no peripheral
    /// singleton, so the caller carries the obligation instead.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of MU instance `N` for the side
    /// matching `S`. Passing a raw physical address is undefined behaviour on
    /// Linux.
    pub unsafe fn new_mapped(base: usize) -> Self {
        Self {
            // SAFETY: forwarded obligation; `base` is the mapped MU window.
            mu: unsafe { Mu::<S, N>::new(base) },
            #[cfg(feature = "std")]
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`. Pass it to the UIO IRQ
    /// loop so it can wake waiting [`wait`](Doorbell::wait) futures via
    /// `notify_waiters()` when this instance's GI interrupt fires.
    #[cfg(feature = "std")]
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }

    /// Service this MU instance's interrupt on the `std`/UIO side.
    ///
    /// Prefer this over driving [`notifier`](Self::notifier) directly: it
    /// *disables* every asserted GI/RF line (masking the level-sensitive MU
    /// interrupt) before waking, so the line does not immediately re-fire the
    /// UIO IRQ while the async waiter has yet to acknowledge it. Disabling only
    /// masks the interrupt — it leaves the `GSR`/`RSR` status set — so the woken
    /// [`wait`](Doorbell::wait) future still observes the flag, acknowledges it,
    /// and re-arms the line on the next wait.
    ///
    /// The doorbell is [`Clone`], so an integrator can clone it before moving the
    /// original into a transport and bind this method to the UIO IRQ fanout.
    #[cfg(feature = "std")]
    pub fn notify_gi_interrupt(&self) {
        let mut asserted = false;
        for gi in 0..MU_GI_CHAN_COUNT {
            if self.mu.gi_pending(gi) {
                // SAFETY: `gi` is `< MU_GI_CHAN_COUNT`; masks this line's IRQ.
                unsafe { self.mu.disable_gi(gi) };
                asserted = true;
            }
        }
        for rf in 0..crate::chan::MU_RF_CHAN_COUNT {
            if self.mu.rf_pending(rf) {
                // SAFETY: `rf` is `< MU_RF_CHAN_COUNT`; masks this line's IRQ.
                unsafe { self.mu.disable_rf(rf) };
                asserted = true;
            }
        }
        if asserted {
            self.notify.notify_waiters();
        }
    }
}

impl<S: Side, const N: usize> Doorbell for MuDoorbell<S, N> {
    type Error = MuError;
    type WaitFut<'a>
        = MuWaitFut<'a, S, N>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), MuError> {
        let line = Self::line_of(ch)?;

        // Ensure preceding slot writes are visible to the remote before the
        // register write that signals it.
        fence(Ordering::Release);

        match line {
            // SAFETY: `gi` is validated `< 4`; ring is the sole writer of GCR.
            MuLine::Gi(gi) => unsafe { self.mu.request_gi(gi) },
            // SAFETY: `rf` is validated `< 4`; ring is the sole writer of TRn.
            MuLine::Rf(rf) => unsafe { self.mu.request_rf(rf) },
        }
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        MuWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        match Self::line_of(ch) {
            Ok(MuLine::Gi(gi)) => {
                let pending = self.mu.gi_pending(gi);
                if pending {
                    fence(Ordering::Acquire);
                }
                pending
            }
            Ok(MuLine::Rf(rf)) => {
                let pending = self.mu.rf_pending(rf);
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

/// Future returned by [`MuDoorbell::wait`].
///
/// On the first poll it arms the interrupt for the channel's line — `GIER[GIEn]`
/// for a GI channel or `RCR[RIEn]` for an RF channel — then completes once the
/// matching status flag (`GSR[GIPn]` or `RSR[RFn]`) is observed, acknowledging
/// it (W1C of `GSR` for GI, or a read of `RRn` for RF) and disarming the
/// interrupt. On `no_std` the per-instance `AtomicWaker` is woken from the
/// application's MU `#[interrupt]` handler (via [`notify`]); on `std` the
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

    /// Register with the doorbell's `Notify`, replacing a subscription that has
    /// already completed.
    ///
    /// Two properties of `Notify::notify_waiters` make this necessary:
    ///
    /// - It stores no permit and wakes only futures that have *already*
    ///   registered. So the caller must subscribe **before** reading the status
    ///   flags: a ring landing between the read and the registration would be
    ///   lost, and the remote cannot ring again while the line stays asserted —
    ///   the transport deadlocks rather than merely stalling.
    /// - It fans out to every waiter on the instance's shared `Notify`, not just
    ///   this line's. A wake caused by another line completes this future's
    ///   `Notified` even though this line is still idle, and a completed
    ///   `Notified` never fires again — so it must be replaced, or this future
    ///   parks forever.
    #[cfg(feature = "std")]
    fn subscribe(&mut self, cx: &mut Context<'_>) {
        if let Some(notified) = self.notified.as_mut() {
            // SAFETY: the `Notified` lives inside this pinned future and is
            // only ever replaced after being dropped, never moved while
            // registered.
            if unsafe { Pin::new_unchecked(notified) }
                .poll(cx)
                .is_pending()
            {
                return;
            }
            self.notified = None;
        }

        let notified = self.doorbell.notify.notified();
        // SAFETY: `doorbell` is borrowed for `'a`; its Arc-backed `Notify`
        // outlives this future, so erasing the borrow is sound.
        let notified = unsafe {
            core::mem::transmute::<
                tokio::sync::futures::Notified<'_>,
                tokio::sync::futures::Notified<'static>,
            >(notified)
        };
        let notified = self.notified.insert(notified);

        // A freshly created `Notified` registers on its first poll and always
        // returns `Pending` (`notify_waiters` leaves no permit behind), so this
        // single poll is enough — no loop, no spin.
        // SAFETY: the `Notified` lives inside this pinned future and is not
        // moved for as long as it stays registered.
        let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
    }
}

impl<'a, S: Side, const N: usize> core::future::Future for MuWaitFut<'a, S, N> {
    type Output = Result<Chan, MuError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: no pinned field is moved; `notified` is only set in place.
        let this = unsafe { self.get_unchecked_mut() };
        let line = match MuDoorbell::<S, N>::line_of(this.ch) {
            Ok(line) => line,
            Err(err) => return Poll::Ready(Err(err)),
        };

        if !this.armed {
            match line {
                // SAFETY: `gi` is validated `< 4`; armed once from this task.
                MuLine::Gi(gi) => unsafe { this.doorbell.mu.enable_gi(gi) },
                // SAFETY: `rf` is validated `< 4`; armed once from this task.
                MuLine::Rf(rf) => unsafe { this.doorbell.mu.enable_rf(rf) },
            }
            this.armed = true;
        }

        // Subscribe *before* reading the status flags below, on every poll. See
        // `subscribe`: a ring observed after the read must still be able to wake
        // this future, and a `Notified` consumed by another line's wake has to
        // be replaced.
        #[cfg(feature = "std")]
        this.subscribe(cx);

        #[cfg(not(feature = "std"))]
        waker_for::<N>().register(cx.waker());

        let ready = match line {
            MuLine::Gi(gi) => this.doorbell.mu.gi_pending(gi),
            MuLine::Rf(rf) => this.doorbell.mu.rf_pending(rf),
        };
        if ready {
            // Acquire fence: pairs with the remote's Release before ring(), so
            // our subsequent slot reads observe the remote's writes.
            fence(Ordering::Acquire);

            match line {
                MuLine::Gi(gi) => {
                    // SAFETY: GSR[GIPn] confirmed set; `gi` is valid.
                    unsafe { this.doorbell.mu.clear_gi(gi) };
                    // SAFETY: `gi` is valid; disarm after completion.
                    unsafe { this.doorbell.mu.disable_gi(gi) };
                }
                MuLine::Rf(rf) => {
                    // SAFETY: RSR[RFn] confirmed set; `rf` is valid.
                    unsafe { this.doorbell.mu.ack_rf(rf) };
                    // SAFETY: `rf` is valid; disarm after completion.
                    unsafe { this.doorbell.mu.disable_rf(rf) };
                }
            }

            return Poll::Ready(Ok(this.ch));
        }

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

/// Wake the futures waiting on MU instance `N`'s general-purpose interrupt.
///
/// This crate installs no vector-table entries; the application owns the MU
/// `#[interrupt]` handlers and forwards each one here with the matching instance
/// number. i.MX93 routes over the side-B lines of MU1/MU2 (`notify::<1>` /
/// `notify::<2>`); the i.MX95 CM7 uses MU5 side A plus MU7/MU8 side B
/// (`notify::<5>` / `notify::<7>` / `notify::<8>`).
#[cfg(not(feature = "std"))]
#[inline]
pub fn notify<const N: usize>() {
    MU_WAKERS[N - 1].wake();
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::registers::{
        cr::{FlagRegister, GeneralInterruptRegister},
        offsets, reset,
    };

    fn ch(id: u8) -> Chan {
        // SAFETY: tests pick ids deliberately.
        unsafe { Chan::new_unchecked(id) }
    }

    // MU7 on i.MX95 is the CA55 <-> CM7 default route.
    const N: usize = if cfg!(feature = "mimx95") { 7 } else { 1 };

    fn gier(regs: &[u32]) -> GeneralInterruptRegister {
        GeneralInterruptRegister::from_bits(regs[offsets::GIER as usize / 4])
    }

    fn rcr(regs: &[u32]) -> FlagRegister {
        FlagRegister::from_bits(regs[offsets::RCR as usize / 4])
    }

    #[test]
    fn ring_sets_gcr_bit() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };

        db.ring(ch(2)).unwrap();

        let gcr = GeneralInterruptRegister::from_bits(regs[offsets::GCR as usize / 4]);
        assert!(gcr.gi() & (1 << 2) != 0);
    }

    #[test]
    fn pending_reads_gsr() {
        let mut regs = [0u32; 0x300 / 4];
        regs[offsets::GSR as usize / 4] = 1 << 3;
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };

        assert!(db.pending(ch(3)));
        assert!(!db.pending(ch(2)));
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };
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
    fn notify_gi_interrupt_masks_line_before_waking_waiters() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };
        let mut wait = std::pin::pin!(db.wait(ch(2)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        // Arm the wait: enables GIER for line 2, then parks pending.
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(gier(&regs).gi() & (1 << 2) != 0);

        // The remote asserts line 2 and the IRQ entry point runs. Unlike a bare
        // `notify_waiters`, it disables (masks) the asserted line before waking,
        // so a level-sensitive MU line cannot immediately re-fire the UIO IRQ.
        regs[offsets::GSR as usize / 4] = 1 << 2;
        db.notify_gi_interrupt();
        assert!(
            gier(&regs).gi() & (1 << 2) == 0,
            "line 2 must be masked by the IRQ entry point, before the waiter re-checks it"
        );

        // Disabling only masks the interrupt; the status flag survives, so the
        // woken future still completes and acknowledges it (W1C of GSR).
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(2))));
        assert_eq!(regs[offsets::GSR as usize / 4], 1 << 2);
    }

    #[test]
    fn wait_thread_wakes_after_remote_status_and_notify() {
        let regs = Box::leak(Box::new([0u32; 0x300 / 4]));
        let base = regs.as_mut_ptr() as usize;
        // SAFETY: `regs` backs a fake MMIO window leaked for the spawned thread.
        let db = unsafe { MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(base) };
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

    // ── RF (receive-full) channels: 4..8 map to RF indices 0..4 ────────────────

    #[test]
    fn ring_rf_writes_transmit_register() {
        let mut regs = [0u32; 0x300 / 4];
        // TSR reset: all transmit registers empty, so ring is allowed.
        regs[offsets::TSR as usize / 4] = reset::TSR;
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };

        // ch(5) -> RF index 1 -> TR1.
        db.ring(ch(5)).unwrap();

        assert_ne!(regs[offsets::TR1 as usize / 4], 0);
        assert_eq!(regs[offsets::TR0 as usize / 4], 0);
    }

    #[test]
    fn ring_rf_skips_when_not_empty() {
        let mut regs = [0u32; 0x300 / 4];
        // TSR with TE1 clear: a prior transmit is still pending, so skip.
        regs[offsets::TSR as usize / 4] = reset::TSR & !(1 << 1);
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };

        db.ring(ch(5)).unwrap();

        assert_eq!(regs[offsets::TR1 as usize / 4], 0);
    }

    #[test]
    fn pending_rf_reads_rsr() {
        let mut regs = [0u32; 0x300 / 4];
        regs[offsets::RSR as usize / 4] = 1 << 2;
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };

        // ch(6) -> RF index 2.
        assert!(db.pending(ch(6)));
        // ch(5) -> RF index 1, not full.
        assert!(!db.pending(ch(5)));
    }

    #[test]
    fn wait_rf_registers_rcr_then_ready_after_notify() {
        let mut regs = [0u32; 0x300 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe {
            MuDoorbell::<consortium_ipc::Primary, N>::new_mapped(regs.as_mut_ptr() as usize)
        };
        // ch(5) -> RF index 1.
        let mut wait = std::pin::pin!(db.wait(ch(5)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        // RCR[RIE1] armed.
        assert!(rcr(&regs).flags() & (1 << 1) != 0);

        regs[offsets::RSR as usize / 4] = 1 << 1;
        db.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(5))));
        // RCR[RIE1] disarmed on completion.
        assert!(rcr(&regs).flags() & (1 << 1) == 0);
    }
}
