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

//! HSEM hardware-semaphore doorbell.
//!
//! [`HsemDoorbell`] wraps an [`Hsem`] HAL handle and implements [`Doorbell`]
//! over the semaphores:
//!
//! - [`ring`](Doorbell::ring) performs a 1-step read-lock on the semaphore and
//!   immediately unlocks it; the unlock raises the remote core's HSEM interrupt.
//! - [`wait`](Doorbell::wait) arms this core's `CnIER`, then resolves once the
//!   semaphore interrupt is observed in `CnMISR`, clearing it.
//! - [`pending`](Doorbell::pending) reads `CnMISR` non-destructively.

use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence, fence},
    task::{Context, Poll},
};

use consortium_hal_stm32mp2::{Peri, PeripheralType};
use consortium_ipc::{Chan, Doorbell, MaybeSend, Side};

use crate::{abstraction::Hsem, chan::HSEM_CHAN_COUNT, error::HsemError, registers::base};

/// Run `f` with interrupts masked, so a read-modify-write of `CnIER` cannot be
/// torn by the HSEM vector preempting task context.
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

/// A chip-HAL peripheral singleton naming the HSEM peripheral.
///
/// The window's address still comes from [`base::BASE`] in this crate rather
/// than from the singleton, so the security alias the doorbell has always used
/// does not change underfoot. The processor side stays a type parameter of
/// [`HsemDoorbell`]: HSEM has one register block with a bank per core, so the
/// singleton names the block, not the view.
pub trait Instance: PeripheralType {
    /// 1-based HSEM instance number. HSEM is single-instance, so this is 1.
    const INSTANCE: usize;
}

#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
impl Instance for consortium_hal_stm32mp2::peripherals::HSEM {
    const INSTANCE: usize = 1;
}

/// HSEM-backed doorbell for the HSEM peripheral (instance `N`, always `1`),
/// processor side `S`.
pub struct HsemDoorbell<S: Side, const N: usize> {
    hsem: Hsem<S, N>,
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// Manual `Clone` (bound independent of `S`): the transport's `split` clones the
// doorbell for the Rx half. Both halves share the same `Notify`, so an IRQ-side
// wake reaches whichever half is waiting.
impl<S: Side, const N: usize> Clone for HsemDoorbell<S, N> {
    fn clone(&self) -> Self {
        Self {
            hsem: self.hsem.clone(),
            #[cfg(feature = "std")]
            notify: self.notify.clone(),
        }
    }
}

impl<S: Side, const N: usize> HsemDoorbell<S, N> {
    #[inline]
    fn sem_of(ch: Chan) -> Result<u8, HsemError> {
        let sem = ch.id();
        if sem >= HSEM_CHAN_COUNT {
            Err(HsemError::ChanOutOfRange)
        } else {
            Ok(sem)
        }
    }
}

impl<S: Side, const N: usize> HsemDoorbell<S, N> {
    /// Create a doorbell over the HSEM peripheral from the chip HAL's
    /// peripheral singleton, for processor side `S`.
    ///
    /// This is the firmware-side constructor: the singleton proves nothing else
    /// drives HSEM, and the window address comes from this crate's
    /// [`base::BASE`] table. `N` must match the singleton, which the const
    /// assertion below checks at compile time.
    ///
    /// On Linux the window is an `mmap` of a UIO device rather than a physical
    /// address, and there is no singleton for it — use
    /// [`new_mapped`](Self::new_mapped) there.
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        const {
            assert!(
                T::INSTANCE == N,
                "HsemDoorbell instance number does not match the peripheral singleton",
            );
        }
        // The singleton is proof of exclusivity, not state: `init()` issues it
        // once and the `'static` bound means nothing ever hands it back, so
        // dropping it here keeps the instance out of reach of safe code. It is
        // deliberately not stored: `split()` clones the doorbell, and other
        // doorbells may drive disjoint lines of the same block.
        let _ = peri;
        // SAFETY: `base::BASE[N - 1]` is the HSEM window from this crate's
        // vetted address table. The consumed singleton keeps every other
        // driver off the window; another doorbell on it can only come from a
        // `steal()` whose caller guarantees disjoint semaphores, and the shared
        // enable-register writes run in a critical section.
        unsafe { Self::new_mapped(base::BASE[N - 1]) }
    }

    /// Create a doorbell over the HSEM peripheral from an `mmap`'d virtual base
    /// address, for processor side `S`.
    ///
    /// This is the Linux/UIO constructor: mapped windows have no peripheral
    /// singleton, so the caller carries the obligation instead.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of the HSEM window. Passing a raw
    /// physical address is undefined behaviour on Linux.
    pub unsafe fn new_mapped(base: usize) -> Self {
        consortium_log::trace!("HsemDoorbell::new_mapped(base = {:#x})", base);
        Self {
            // SAFETY: forwarded obligation; `base` is the mapped HSEM window.
            hsem: unsafe { Hsem::<S, N>::new(base) },
            #[cfg(feature = "std")]
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`. Pass it to the UIO IRQ
    /// loop so it can wake waiting [`wait`](Doorbell::wait) futures via
    /// `notify_waiters()` when the HSEM interrupt fires.
    ///
    /// Userspace owns `CnIER`/`CnICR` through the mapped HSEM window on this
    /// target: [`wait`](Doorbell::wait) enables a semaphore's interrupt when it
    /// subscribes and clears+disables it on completion, so the level-sensitive
    /// line de-asserts before the UIO IRQ is re-armed.
    #[cfg(feature = "std")]
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

impl<S: Side, const N: usize> Doorbell for HsemDoorbell<S, N> {
    type Error = HsemError;

    fn ring(&self, ch: Chan) -> Result<(), HsemError> {
        consortium_log::trace!("HsemDoorbell::ring(ch = {:?})", ch);
        let sem = Self::sem_of(ch)?;

        // Ensure preceding slot writes are ordered before the semaphore lock
        // that signals the other core.
        fence(Ordering::Release);

        // 1-step read-lock: atomically lock the semaphore and read it back.
        // SAFETY: `sem` is validated `< 16`.
        let locked = unsafe { self.hsem.read_lock(sem) };
        consortium_log::trace!("HsemDoorbell::ring: read_lock({}) = {:?}", sem, locked);
        if locked.lock() && locked.lockid() as u8 != Hsem::<S, N>::LOCKID {
            consortium_log::error!(
                "HsemDoorbell::ring: semaphore {} already locked by core {}",
                sem,
                locked.procid()
            );
            return Err(HsemError::AlreadyLocked);
        }
        consortium_log::info!(
            "HsemDoorbell::ring: semaphore {} locked by core {}",
            sem,
            locked.procid()
        );

        // Unlock by writing back with LOCK = 0, raising the remote interrupt.
        let mut unlock = locked;
        unlock.set_lock(false);
        // SAFETY: `sem` is validated `< 16`.
        consortium_log::trace!("HsemDoorbell::ring: writing unlock to semaphore {}", sem);
        unsafe { self.hsem.write_rx(sem, unlock) };
        consortium_log::info!(
            "HsemDoorbell::ring: semaphore {} unlocked by core {}",
            sem,
            unlock.procid()
        );

        fence(Ordering::Release);
        consortium_log::info!(
            "HsemDoorbell::ring: signaled remote core on semaphore {}",
            sem
        );
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        HsemWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        match Self::sem_of(ch) {
            Ok(sem) => {
                consortium_log::trace!("HsemDoorbell::pending(ch = {:?})", ch);
                // Raw `CnISR`, matching the readiness check in `HsemWaitFut`: a
                // release the caller has not consumed is pending whether or not
                // the line happens to be masked right now.
                let pending = self.hsem.raw_interrupt_pending(sem);
                if pending {
                    consortium_log::trace!(
                        "HsemDoorbell::pending: semaphore {} interrupt pending",
                        sem
                    );
                    fence(Ordering::Acquire);
                }
                pending
            }
            Err(_) => false,
        }
    }
}

// ── Wait future ───────────────────────────────────────────────────────────────

/// Future returned by [`HsemDoorbell::wait`].
///
/// Arms this core's `CnIER` on the first poll, then completes once the
/// semaphore interrupt is observed, disarming `CnIER`/`CnICR` on completion. On
/// `no_std` the shared `AtomicWaker` is woken from the application's HSEM
/// `#[interrupt]` handler (via `notify`) and the future re-checks
/// `CnISR`; on `std` the doorbell's `Notify` is driven by the UIO IRQ loop.
/// Both paths own the interrupt registers through the mapped HSEM window and
/// clear+disable the line when the wait resolves.
pub struct HsemWaitFut<'a, S: Side, const N: usize> {
    doorbell: &'a HsemDoorbell<S, N>,
    ch: Chan,
    armed: bool,
    #[cfg(feature = "std")]
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

impl<'a, S: Side, const N: usize> HsemWaitFut<'a, S, N> {
    fn new(doorbell: &'a HsemDoorbell<S, N>, ch: Chan) -> Self {
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
    ///   register: a release landing between the read and the registration would
    ///   be lost, and the remote cannot signal again while `CnISR` stays set —
    ///   the transport would deadlock rather than merely stall.
    /// - It fans out to every waiter on the shared `Notify`, not just this
    ///   semaphore's. A wake caused by another semaphore completes this future's
    ///   `Notified` even though this semaphore is still idle, and a completed
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
            consortium_log::trace!("wait({:?}) -> Notify consumed, re-subscribing", self.ch);
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

impl<S: Side, const N: usize> Drop for HsemWaitFut<'_, S, N> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }

        let Ok(sem) = HsemDoorbell::<S, N>::sem_of(self.ch) else {
            return;
        };

        // Disarm only; do not touch `CnICR`. The status bit *is* the
        // notification, so acknowledging it here would discard a release that
        // the next `wait()` should observe. Masking is the part that matters on
        // cancel: `CnIER` stays enabled otherwise, and with no waiter left to
        // clear the level-sensitive line, `irq_loop` re-arming the UIO IRQ
        // re-fires it immediately — the spurious-interrupt storm the readiness
        // path already guards against.
        with_interrupts_masked(|| {
            // SAFETY: `sem` was validated `< 16`, and the critical section makes
            // this `CnIER` read-modify-write atomic against interrupt context.
            unsafe { self.doorbell.hsem.disable_interrupt(sem) };
        });
    }
}

impl<'a, S: Side, const N: usize> core::future::Future for HsemWaitFut<'a, S, N> {
    type Output = Result<Chan, HsemError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: no pinned field is moved; `notified` is only set in place.
        let this = unsafe { self.get_unchecked_mut() };
        let sem = match HsemDoorbell::<S, N>::sem_of(this.ch) {
            Ok(sem) => sem,
            Err(err) => return Poll::Ready(Err(err)),
        };

        consortium_log::trace!(
            "HsemWaitFut::poll(ch = {:?}) armed = {}",
            this.ch,
            this.armed
        );

        // Register on every poll, before the status read: the waker this task
        // was polled with may differ from the last one.
        #[cfg(not(feature = "std"))]
        sem_waker(sem).register(cx.waker());

        compiler_fence(Ordering::SeqCst);

        if !this.armed {
            consortium_log::trace!("HsemWaitFut::poll: arming semaphore {} interrupt", sem);
            // Do *not* clear `CnICR` here. A release that landed before this
            // wait started is a notification the caller has not consumed yet;
            // acknowledging it on arm would drop it and park until the remote
            // signals again, which it cannot do while it waits for a reply.
            with_interrupts_masked(|| {
                // SAFETY: `sem` is validated `< 16`, and the critical section
                // makes this `CnIER` read-modify-write atomic against the ISR.
                unsafe { this.doorbell.hsem.enable_interrupt(sem) };
            });
            this.armed = true;
        }

        // Subscribe *before* reading the status register below, on every poll.
        // See `subscribe`: a release observed after the read must still be able
        // to wake this future, and a `Notified` consumed by another semaphore's
        // wake has to be replaced.
        #[cfg(feature = "std")]
        {
            consortium_log::trace!(
                "HsemWaitFut::poll: subscribing Notify for semaphore {}",
                sem
            );
            this.subscribe(cx);
        }

        compiler_fence(Ordering::SeqCst);

        // Read `CnISR`, not `CnMISR`. The status bit is the notification and it
        // survives masking, so a line the ISR (or a cancelled wait) masked is
        // still delivered here. Gating on the `Notify` instead would be wrong on
        // both counts: it fans out across every semaphore, so another line's
        // wake would resolve this wait, and it says nothing about whether *this*
        // semaphore was released.
        if this.doorbell.hsem.raw_interrupt_pending(sem) {
            consortium_log::trace!("HsemWaitFut::poll: semaphore {} interrupt pending", sem);
            // Acquire fence: pairs with the remote's Release before `ring`, so
            // subsequent slot reads observe the remote's writes.
            fence(Ordering::Acquire);

            // SAFETY: `sem` is validated `< 16`; the status bit is set.
            unsafe { this.doorbell.hsem.clear_interrupt(sem) };
            with_interrupts_masked(|| {
                // SAFETY: `sem` is validated `< 16`; re-mask after completion.
                unsafe { this.doorbell.hsem.disable_interrupt(sem) };
            });
            consortium_log::trace!(
                "HsemWaitFut::poll: semaphore {} acknowledged, line masked",
                sem
            );

            return Poll::Ready(Ok(this.ch));
        }

        Poll::Pending
    }
}

// ── no_std: per-semaphore wakers and IRQ handler ─────────────────────────────

/// Wakers separated by semaphore on the bare-metal path.
///
/// One shared `AtomicWaker` would be wrong: it holds a single waker, so a second
/// task waiting on another semaphore overwrites the first registration and that
/// task is never woken again. A duplex endpoint waits on two semaphores, so this
/// is the ordinary case, not a corner. Same shape as IPCC's `IPCC_WAKERS`.
#[cfg(not(feature = "std"))]
static HSEM_WAKERS: [atomic_waker::AtomicWaker; HSEM_CHAN_COUNT as usize] =
    [const { atomic_waker::AtomicWaker::new() }; HSEM_CHAN_COUNT as usize];

#[cfg(not(feature = "std"))]
#[inline]
fn sem_waker(sem: u8) -> &'static atomic_waker::AtomicWaker {
    &HSEM_WAKERS[sem as usize]
}

/// Wake tasks waiting on an HSEM semaphore interrupt.
///
/// This crate installs no vector-table entry; the application owns the HSEM
/// `#[interrupt]` handler and forwards it here (e.g. `#[interrupt] fn HSEM`).
///
/// Unlike IPCC's and MU's service entry points this takes no side/instance and
/// does not mask: HSEM's vector carries no way to name the window, so it cannot
/// read `CnISR` to find which semaphores fired. It therefore wakes every
/// per-semaphore waiter and lets each one re-check its own `CnISR` bit — a
/// waiter whose semaphore did not fire simply parks again, which is sound
/// because readiness is decided by the status register and never by the wake.
#[cfg(not(feature = "std"))]
#[inline]
pub fn notify() {
    for waker in &HSEM_WAKERS {
        waker.wake();
    }
}

#[cfg(all(test, not(feature = "std")))]
mod no_std_tests {
    extern crate std;

    use super::*;
    use consortium_ipc::Primary;
    use core::sync::atomic::AtomicUsize;
    use std::{
        sync::Arc,
        task::{RawWaker, RawWakerVTable, Waker},
    };

    fn ch(id: u8) -> Chan {
        // SAFETY: the test uses valid HSEM channel ids.
        unsafe { Chan::new_unchecked(id) }
    }

    fn counting_waker() -> (Waker, Arc<AtomicUsize>) {
        unsafe fn clone(p: *const ()) -> RawWaker {
            // SAFETY: `p` came from `Arc::into_raw` and remains live.
            unsafe { Arc::increment_strong_count(p as *const AtomicUsize) };
            RawWaker::new(p, &VTABLE)
        }
        unsafe fn wake(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            let count = unsafe { Arc::from_raw(p as *const AtomicUsize) };
            count.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn wake_by_ref(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw` and remains live.
            unsafe { &*(p as *const AtomicUsize) }.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn drop_fn(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            unsafe { drop(Arc::from_raw(p as *const AtomicUsize)) };
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_fn);

        let count = Arc::new(AtomicUsize::new(0));
        let raw = RawWaker::new(Arc::into_raw(Arc::clone(&count)) as *const (), &VTABLE);
        // SAFETY: `VTABLE` implements the `RawWaker` contract for the Arc payload.
        (unsafe { Waker::from_raw(raw) }, count)
    }

    #[test]
    fn distinct_semaphores_keep_distinct_wakers() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs the fake HSEM MMIO window for this test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let mut first = core::pin::pin!(db.wait(ch(2)));
        let mut second = core::pin::pin!(db.wait(ch(3)));
        let (first_waker, first_wakes) = counting_waker();
        let (second_waker, second_wakes) = counting_waker();

        assert_eq!(
            first.as_mut().poll(&mut Context::from_waker(&first_waker)),
            Poll::Pending
        );
        assert_eq!(
            second
                .as_mut()
                .poll(&mut Context::from_waker(&second_waker)),
            Poll::Pending
        );

        notify();

        assert_eq!(first_wakes.load(Ordering::SeqCst), 1);
        assert_eq!(second_wakes.load(Ordering::SeqCst), 1);
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::registers::{cr::InterruptRegister, offsets};
    use consortium_ipc::Primary;
    use core::sync::atomic::AtomicUsize;

    fn ch(id: u8) -> Chan {
        // SAFETY: tests pick ids deliberately.
        unsafe { Chan::new_unchecked(id) }
    }

    // Primary is core C1; CnIER/CnICR/CnISR/CnMISR are the C1 block at 0x100.
    fn reg_index(offset: u32) -> usize {
        offset as usize / 4
    }

    fn ier(regs: &[u32]) -> u32 {
        InterruptRegister::from_bits(regs[reg_index(offsets::C1IER)]).sems()
    }

    /// A waker that counts how many times the task was woken, so a test can
    /// assert the future is actually *registered* rather than relying on being
    /// re-polled by hand.
    fn counting_waker() -> (std::task::Waker, std::sync::Arc<AtomicUsize>) {
        use std::{
            sync::Arc,
            task::{RawWaker, RawWakerVTable, Waker},
        };

        unsafe fn clone(p: *const ()) -> RawWaker {
            // SAFETY: `p` came from `Arc::into_raw` below and is still alive.
            unsafe { Arc::increment_strong_count(p as *const AtomicUsize) };
            RawWaker::new(p, &VTABLE)
        }
        unsafe fn wake(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            let count = unsafe { Arc::from_raw(p as *const AtomicUsize) };
            count.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn wake_by_ref(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw` and is still alive.
            unsafe { &*(p as *const AtomicUsize) }.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn drop_fn(p: *const ()) {
            // SAFETY: `p` came from `Arc::into_raw`; this consumes one count.
            unsafe { Arc::from_raw(p as *const AtomicUsize) };
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_fn);

        let count = Arc::new(AtomicUsize::new(0));
        let raw = RawWaker::new(Arc::into_raw(Arc::clone(&count)) as *const (), &VTABLE);
        // SAFETY: `VTABLE`'s functions uphold the `RawWaker` contract for an
        // `Arc<AtomicUsize>` payload.
        (unsafe { Waker::from_raw(raw) }, count)
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert_ne!(ier(&regs) & (1 << 3), 0, "arming must enable CnIER");

        regs[reg_index(offsets::C1ISR)] = 1 << 3;
        db.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));
        assert_eq!(
            regs[reg_index(offsets::C1ICR)],
            1 << 3,
            "resolving the wait must acknowledge the status bit"
        );
    }

    /// On completion the wait path must acknowledge (`CnICR`) and disarm
    /// (`CnIER`). HSEM is level-sensitive: `CnISR` stays set until cleared, so
    /// returning `Ready` without both leaves the line asserted and `irq_loop`
    /// re-arming the UIO IRQ re-fires it in a spurious-interrupt storm.
    #[test]
    fn wait_disarms_line_on_completion() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert_ne!(
            ier(&regs) & (1 << 3),
            0,
            "semaphore interrupt must be enabled while waiting"
        );

        regs[reg_index(offsets::C1ISR)] = 1 << 3;
        db.notifier().notify_waiters();
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));

        assert_eq!(
            ier(&regs) & (1 << 3),
            0,
            "semaphore interrupt must be masked once the wait resolves"
        );
        assert_eq!(
            regs[reg_index(offsets::C1ICR)],
            1 << 3,
            "completion must write-1-clear the status bit"
        );
    }

    /// Readiness comes from `CnISR`, not from the `Notify`. The `Notify` is
    /// shared by every semaphore on the block, so a wake caused by a *different*
    /// semaphore must not resolve this wait — it would hand the caller a
    /// doorbell that never rang and consume a message slot that has no data.
    #[test]
    fn wake_for_another_semaphore_does_not_resolve_the_waiter() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let (waker, wakes) = counting_waker();
        let mut cx = Context::from_waker(&waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);

        // Semaphore 5 fires; the shared `Notify` wakes the semaphore-3 waiter
        // too. Its own status bit is clear, so it must park again.
        regs[reg_index(offsets::C1ISR)] = 1 << 5;
        db.notifier().notify_waiters();
        assert_eq!(wakes.load(Ordering::SeqCst), 1);
        assert_eq!(
            wait.as_mut().poll(&mut cx),
            Poll::Pending,
            "another semaphore's wake must not resolve this wait"
        );

        // Semaphore 3 now fires for real. The wake only reaches the task if the
        // poll above replaced the `Notified` consumed by the semaphore-5 wake.
        regs[reg_index(offsets::C1ISR)] = (1 << 5) | (1 << 3);
        db.notifier().notify_waiters();
        assert_eq!(
            wakes.load(Ordering::SeqCst),
            2,
            "waiter was not re-subscribed after a foreign semaphore consumed its notification"
        );

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));
        assert_eq!(regs[reg_index(offsets::C1ICR)], 1 << 3);
    }

    /// A release that landed before the wait started is a notification nobody
    /// has consumed. Arming must not acknowledge it: the remote cannot signal
    /// again while it waits for a reply, so dropping it deadlocks the transport.
    #[test]
    fn wait_delivers_a_release_latched_before_it_started() {
        let mut regs = [0u32; 0x140 / 4];
        regs[reg_index(offsets::C1ISR)] = 1 << 3;
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(
            wait.as_mut().poll(&mut cx),
            Poll::Ready(Ok(ch(3))),
            "a status bit set before the wait began must still be delivered"
        );
    }

    /// Cancelling a pending wait (a `select!` timeout, a dropped `recv`) must
    /// disarm the line too, not just the readiness path. Otherwise `CnIER` stays
    /// enabled with nobody left to clear `CnISR`, and re-arming the UIO IRQ
    /// re-fires the level-sensitive line forever.
    #[test]
    fn dropping_pending_wait_disarms_its_semaphore() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        {
            let mut wait = std::pin::pin!(db.wait(ch(3)));
            assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
            assert_ne!(
                ier(&regs) & (1 << 3),
                0,
                "semaphore interrupt must be enabled while waiting"
            );
        }

        assert_eq!(
            ier(&regs) & (1 << 3),
            0,
            "cancelled wait must mask the semaphore interrupt"
        );
    }

    /// The status bit is the notification, so a cancelled wait must leave it
    /// latched for the next `wait()` rather than acknowledging it.
    #[test]
    fn dropping_pending_wait_does_not_acknowledge_the_status() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        {
            let mut wait = std::pin::pin!(db.wait(ch(3)));
            assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
            // The remote releases after the poll parked but before the cancel.
            regs[reg_index(offsets::C1ISR)] = 1 << 3;
        }

        assert_eq!(
            regs[reg_index(offsets::C1ICR)],
            0,
            "cancelled wait must not acknowledge a pending release"
        );
        assert_eq!(
            regs[reg_index(offsets::C1ISR)],
            1 << 3,
            "the status bit must survive for the next wait"
        );
    }

    /// A future that was never polled never armed anything, so its `Drop` must
    /// not write the interrupt registers at all.
    #[test]
    fn dropping_unpolled_wait_touches_nothing() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(regs.as_mut_ptr() as usize) };
        regs[reg_index(offsets::C1IER)] = u32::MAX;

        drop(db.wait(ch(3)));

        assert_eq!(
            regs[reg_index(offsets::C1IER)],
            u32::MAX,
            "an unarmed wait must not disturb CnIER"
        );
    }

    /// Every wait in this test is bounded. An unbounded `join`/spin turns a
    /// readiness regression into a hung test run rather than a failure, which
    /// costs far more to diagnose than an assertion — reading the masked
    /// `CnMISR` instead of `CnISR`, for instance, never resolves against a fake
    /// register window that does not synthesize `ISR & IER`.
    #[test]
    fn wait_thread_wakes_after_notify() {
        const LIMIT: std::time::Duration = std::time::Duration::from_secs(10);

        let regs = Box::leak(Box::new([0u32; 0x140 / 4]));
        let base = regs.as_mut_ptr() as usize;
        // SAFETY: `regs` backs a fake MMIO window leaked for the spawned thread.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new_mapped(base) };
        let notifier = db.notifier();
        let (armed_tx, armed_rx) = std::sync::mpsc::channel();
        let (done_tx, done_rx) = std::sync::mpsc::channel();

        let waiter = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create waiter runtime");
            let resolved = rt.block_on(async move {
                armed_tx.send(()).expect("signal waiter is starting");
                db.wait(ch(3)).await.expect("wait for HSEM doorbell")
            });
            let _ = done_tx.send(resolved);
        });

        armed_rx
            .recv_timeout(LIMIT)
            .expect("waiter thread should start");

        let deadline = std::time::Instant::now() + LIMIT;
        while InterruptRegister::from_bits(unsafe {
            (base as *const u32)
                .add(reg_index(offsets::C1IER))
                .read_volatile()
        })
        .sems()
            & (1 << 3)
            == 0
        {
            assert!(
                std::time::Instant::now() < deadline,
                "waiter never armed CnIER"
            );
            std::thread::yield_now();
        }

        // Publish the status the waiter checks, then wake it.
        // SAFETY: the leaked window outlives both threads.
        unsafe {
            (base as *mut u32)
                .add(reg_index(offsets::C1ISR))
                .write_volatile(1 << 3)
        };
        notifier.notify_waiters();

        let resolved = done_rx
            .recv_timeout(LIMIT)
            .expect("waiter must resolve once CnISR is set and the Notify fires");
        assert_eq!(resolved, ch(3));
        waiter.join().expect("waiter thread completes");
    }
}
