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
    sync::atomic::{Ordering, fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, Side};

use crate::{abstraction::Hsem, chan::HSEM_CHAN_COUNT, error::HsemError};

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
    /// Create a doorbell over the HSEM peripheral from an `mmap`'d virtual base
    /// address, for processor side `S`.
    ///
    /// # Safety
    /// `base` must be the mapped virtual base of the HSEM window. Passing a raw
    /// physical address is undefined behaviour on Linux.
    pub unsafe fn new(base: usize) -> Self {
        consortium_log::trace!("HsemDoorbell::new(base = {:#x})", base);
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
    type WaitFut<'a>
        = HsemWaitFut<'a, S, N>
    where
        Self: 'a;

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

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        HsemWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        match Self::sem_of(ch) {
            Ok(sem) => {
                consortium_log::trace!("HsemDoorbell::pending(ch = {:?})", ch);
                let pending = self.hsem.interrupt_pending(sem);
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
/// `#[interrupt]` handler (via [`notify`]) and the future re-checks
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

        consortium_log::trace!("HsemWaitFut::poll(ch = {:?})", this.ch);
        if !this.armed {
            consortium_log::trace!("HsemWaitFut::poll: arming semaphore {} interrupt", sem);
            // SAFETY: `sem` is validated `< 16`; armed once from this task.
            unsafe {
                this.doorbell.hsem.clear_interrupt(sem);
                this.doorbell.hsem.enable_interrupt(sem);
            }
            this.armed = true;

            #[cfg(feature = "std")]
            {
                consortium_log::trace!(
                    "HsemWaitFut::poll: registering Notify for semaphore {}",
                    sem
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

            #[cfg(not(feature = "std"))]
            WAKER.register(cx.waker());
        }

        #[cfg(feature = "std")]
        {
            if let Some(notified) = this.notified.as_mut() {
                consortium_log::trace!(
                    "HsemWaitFut::poll: waiting for Notify on semaphore {}",
                    sem
                );
                // SAFETY: `notified` lives inside the pinned future.
                if unsafe { Pin::new_unchecked(notified) }.poll(cx).is_ready() {
                    consortium_log::trace!(
                        "HsemWaitFut::poll: Notify received for semaphore {}",
                        sem
                    );
                    fence(Ordering::Acquire);
                    // Disarm and acknowledge on completion, symmetric with the
                    // no_std path. HSEM is level-sensitive: `CnISR` stays set
                    // until cleared via `CnICR`. If we returned `Ready` without
                    // clearing it, the line would remain asserted and `irq_loop`
                    // re-arming the UIO IRQ (`unmask_irq`) would immediately
                    // re-fire it — a spurious-interrupt storm until the next
                    // `wait()` happened to re-arm. Userspace owns `CnIER`/`CnICR`
                    // through the mapped HSEM window on this target, so clear the
                    // status and mask the line here; the next `wait()` re-arms.
                    // SAFETY: `sem` is validated `< 16`.
                    unsafe {
                        this.doorbell.hsem.disable_interrupt(sem);
                        this.doorbell.hsem.clear_interrupt(sem);
                    }
                    return Poll::Ready(Ok(this.ch));
                }
            }
            Poll::Pending
        }

        #[cfg(not(feature = "std"))]
        {
            if this.doorbell.hsem.interrupt_pending(sem) {
                consortium_log::trace!("HsemWaitFut::poll: semaphore {} interrupt pending", sem);
                fence(Ordering::Acquire);

                // SAFETY: `sem` is validated `< 16`; disarm and acknowledge.
                unsafe {
                    this.doorbell.hsem.disable_interrupt(sem);
                    this.doorbell.hsem.clear_interrupt(sem);
                }

                consortium_log::trace!("HsemWaitFut::poll: semaphore {} interrupt cleared", sem);

                Poll::Ready(Ok(this.ch))
            } else {
                consortium_log::trace!(
                    "HsemWaitFut::poll: semaphore {} interrupt not pending; registering waker",
                    sem
                );
                WAKER.register(cx.waker());
                Poll::Pending
            }
        }
    }
}

// ── no_std: shared waker and IRQ handler ─────────────────────────────────────

/// Waker shared by all HSEM `wait()` futures on the bare-metal path.
#[cfg(not(feature = "std"))]
static WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Wake tasks waiting on an HSEM semaphore interrupt.
///
/// This crate installs no vector-table entry; the application owns the HSEM
/// `#[interrupt]` handler and forwards it here (e.g. `#[interrupt] fn HSEM`).
#[cfg(not(feature = "std"))]
#[inline]
pub fn notify() {
    WAKER.wake();
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use crate::registers::{cr::InterruptRegister, offsets};
    use consortium_ipc::Primary;

    fn ch(id: u8) -> Chan {
        // SAFETY: tests pick ids deliberately.
        unsafe { Chan::new_unchecked(id) }
    }

    // Primary is core C1; CnIER/CnICR/CnMISR are the C1 block at 0x100.
    fn reg_index(offset: u32) -> usize {
        offset as usize / 4
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(
            InterruptRegister::from_bits(regs[reg_index(offsets::C1IER)]).sems() & (1 << 3) != 0
        );
        assert_eq!(regs[reg_index(offsets::C1ICR)], 1 << 3);

        db.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));
    }

    /// On completion the `std` wait path must disarm (`CnIER`) and acknowledge
    /// (`CnICR`) the semaphore, symmetric with no_std. Without this the
    /// level-sensitive line stays asserted and `irq_loop` re-arming the UIO IRQ
    /// re-fires it in a spurious-interrupt storm.
    #[test]
    fn wait_disarms_line_on_completion() {
        let mut regs = [0u32; 0x140 / 4];
        // SAFETY: `regs` backs a fake MMIO window for the test.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new(regs.as_mut_ptr() as usize) };
        let mut wait = std::pin::pin!(db.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        // Enabled on arm.
        assert!(
            InterruptRegister::from_bits(regs[reg_index(offsets::C1IER)]).sems() & (1 << 3) != 0,
            "semaphore interrupt must be enabled while waiting"
        );

        db.notifier().notify_waiters();
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));

        // Disarmed on completion: the line no longer asserts the UIO IRQ.
        assert_eq!(
            InterruptRegister::from_bits(regs[reg_index(offsets::C1IER)]).sems() & (1 << 3),
            0,
            "semaphore interrupt must be masked once the wait resolves"
        );
        // Acknowledged on completion (write-1-to-clear of the status bit).
        assert_eq!(regs[reg_index(offsets::C1ICR)], 1 << 3);
    }

    #[test]
    fn wait_thread_wakes_after_notify() {
        let regs = Box::leak(Box::new([0u32; 0x140 / 4]));
        let base = regs.as_mut_ptr() as usize;
        // SAFETY: `regs` backs a fake MMIO window leaked for the spawned thread.
        let db = unsafe { HsemDoorbell::<Primary, 1>::new(base) };
        let notifier = db.notifier();
        let (armed_tx, armed_rx) = std::sync::mpsc::channel();

        let waiter = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create waiter runtime");
            rt.block_on(async move {
                armed_tx.send(()).expect("signal waiter is starting");
                db.wait(ch(3)).await.expect("wait for HSEM doorbell")
            })
        });

        armed_rx.recv().expect("waiter thread started");
        while InterruptRegister::from_bits(unsafe {
            (base as *const u32)
                .add(reg_index(offsets::C1IER))
                .read_volatile()
        })
        .sems()
            & (1 << 3)
            == 0
        {
            std::thread::yield_now();
        }

        notifier.notify_waiters();

        assert_eq!(waiter.join().expect("waiter thread completes"), ch(3));
    }
}
