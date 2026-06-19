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
#![allow(unused)]

use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::error::HsemError;
use crate::registers::{interrupt_nonsecure::fields::SemInterruptRegister, semaphore_r::fields};

use crate::registers::{interrupt_nonsecure, semaphore_r, semaphore_rlr};

// The std path computes all offsets inline from self.base; it does not need
// the physical-address constants from the registers module.

/// HSEM-backed doorbell.
///
/// `PROC`: which processor's interrupt line to use for `wait()`.
///   - 1 = C1 (e.g. Cortex-M, bare-metal)
///   - 2 = C2 (e.g. Cortex-A, Linux)
///   - 3 = C3
///
/// Without the `std` feature (M-core/bare-metal): uses `AtomicWaker` and the
/// `HSEM_IRQHandler` interrupt vector.  Register addresses are resolved at
/// compile time from the constants in [`crate::registers`].
///
/// With the `std` feature (A-core/Linux): the HSEM peripheral registers must
/// be mapped into the process's virtual address space before constructing this
/// type (e.g. via `/dev/mem` or a UIO resource).  Pass the resulting virtual
/// base address to [`new`].  Holds a `tokio::sync::Notify` that external code
/// (e.g. the UIO IRQ loop) must call `notify_waiters()` on when the HSEM
/// interrupt fires; obtain the `Arc<Notify>` via [`notifier()`].
///
/// [`new`]: HsemDoorbell::new
/// [`notifier()`]: HsemDoorbell::notifier
pub struct HsemDoorbell<const PROC: u8> {
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
    /// std/A-core: Virtual base address of the mmap'd HSEM peripheral.
    /// bare-metal/M-core: Physical base address specified in the SoC TRM, used for pointer arithmetic to access registers.
    base: usize,
}

// Register helpers.

#[allow(unused)]
impl<const PROC: u8> HsemDoorbell<PROC> {
    // Offsets from HSEM base per RM0457 §22:
    //   Rx    = 0x000 + 4*x
    //   RLRx  = 0x080 + 4*x
    //   CnIER = 0x100 + 0x10*(n-1)
    //   CnICR = 0x104 + 0x10*(n-1)
    //   CnMISR= 0x10C + 0x10*(n-1)

    #[inline]
    fn rx_addr(&self, sem: u8) -> *mut u32 {
        (self.base + sem as usize * 4) as *mut u32
    }

    #[inline]
    fn rlr_addr(&self, sem: u8) -> *const u32 {
        (self.base + 0x080 + sem as usize * 4) as *const u32
    }

    #[inline]
    unsafe fn read_lock(&self, sem: u8) -> u32 {
        // SAFETY: caller must ensure sem is valid; self.base is a valid mmap'd address.
        unsafe { self.rlr_addr(sem).read_volatile() }
    }

    #[inline]
    unsafe fn write_rx(&self, sem: u8, value: u32) {
        // SAFETY: caller must ensure sem is valid; self.base is a valid mmap'd address.
        unsafe { self.rx_addr(sem).write_volatile(value) }
    }

    /// Returns true if the semaphore is currently locked (LOCK bit set).
    ///
    /// Reads `HSEM_Rx` via the mmap'd virtual base.
    #[inline]
    pub fn is_locked(&self, sem: u8) -> bool {
        // SAFETY: self.base is a valid mmap'd HSEM address; volatile read.
        let reg =
            fields::SemaphoreRegister::from_bits(unsafe { self.rx_addr(sem).read_volatile() });
        reg.lock()
    }

    #[inline]
    pub(crate) fn ier_addr(&self) -> *mut u32 {
        let offset = match PROC {
            1 => interrupt_nonsecure::HSEM_C1IER,
            2 => interrupt_nonsecure::HSEM_C2IER,
            3 => interrupt_nonsecure::HSEM_C3IER,
            _ => unreachable!(),
        };
        (self.base + offset as usize) as *mut u32
    }

    #[inline]
    pub(crate) fn icr_addr(&self) -> *mut u32 {
        let offset = match PROC {
            1 => interrupt_nonsecure::HSEM_C1ICR,
            2 => interrupt_nonsecure::HSEM_C2ICR,
            3 => interrupt_nonsecure::HSEM_C3ICR,
            _ => unreachable!(),
        };
        (self.base + offset as usize) as *mut u32
    }

    #[inline]
    fn misr_addr(&self) -> *const u32 {
        let offset = match PROC {
            1 => interrupt_nonsecure::HSEM_C1MISR as *const u32,
            2 => interrupt_nonsecure::HSEM_C2MISR as *const u32,
            3 => interrupt_nonsecure::HSEM_C3MISR as *const u32,
            _ => unreachable!(),
        };
        (self.base + offset as usize) as *const u32
    }

    /// Enable the interrupt for semaphore `sem` on our processor's IER.
    ///
    /// # Safety
    /// Caller must ensure `sem` is a valid semaphore index (0-15) and that the
    /// read-modify-write is not racing with other accesses to the IER register.
    #[inline]
    pub unsafe fn enable_interrupt(&self, sem: u8) {
        let ier = self.ier_addr();
        // SAFETY: caller must ensure sem is valid; self.base is a valid mmap'd address.
        let reg =
            SemInterruptRegister::from_bits(unsafe { ier.read_volatile() }).with_sem_enabled(sem);
        // SAFETY: caller must ensure sem is valid; self.base is a valid mmap'd address.
        unsafe { ier.write_volatile(reg.bits()) };
    }

    /// Clear the interrupt status for semaphore `sem` (write 1 to ICR bit).
    ///
    /// # Safety
    /// Caller must ensure `sem` is a valid semaphore index (0-15).
    #[inline]
    pub unsafe fn clear_interrupt(&self, sem: u8) {
        let reg = SemInterruptRegister::from_bits(interrupt_nonsecure::fields::sem_bit(sem));
        // SAFETY: caller must ensure sem is valid; self.base is a valid mmap'd address.
        unsafe { self.icr_addr().write_volatile(reg.bits()) };
    }

    /// Check the masked interrupt status - has semaphore `sem` fired an
    /// interrupt that is both pending AND enabled?
    #[inline]
    pub fn interrupt_pending(&self, sem: u8) -> bool {
        // SAFETY: self.base is a valid mmap'd HSEM address; volatile read.
        let reg = SemInterruptRegister::from_bits(unsafe { self.misr_addr().read_volatile() });
        reg.contains_sem(sem)
    }
}

/// Global waker storage for the bare-metal IRQ handler.
/// In a real system use an AtomicWaker per channel, or an array indexed by semaphore ID.
#[cfg(not(feature = "std"))]
static WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Call this from your HSEM IRQ vector.
/// The executor will re-poll any futures waiting on this doorbell.
///
/// # Safety
/// Must be called only from the correct HSEM IRQ handler with a properly
/// registered interrupt vector table entry.
#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
pub extern "C" fn HSEM_IRQHandler() {
    WAKER.wake();
}

#[cfg(feature = "std")]
impl<const PROC: u8> HsemDoorbell<PROC> {
    /// Create a doorbell using the mmap'd virtual base address of the HSEM
    /// peripheral.
    ///
    /// # Arguments
    ///
    /// * `hsem_base` - virtual address returned by `mmap` of the HSEM
    ///   peripheral range (e.g. resource 0 of a UIO device, or a `/dev/mem`
    ///   mapping of the physical base address from the SoC TRM).  Passing a
    ///   raw physical address here is **undefined behaviour** on Linux.
    ///
    /// # Panics
    ///
    /// Panics if `PROC` is not in the range `1..=3`.
    pub fn new(hsem_base: usize) -> Self {
        assert!(PROC >= 1 && PROC <= 3, "PROC must be 1, 2, or 3");
        Self {
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
            base: hsem_base,
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`.
    ///
    /// Pass this to the UIO IRQ loop (or any other interrupt source) so it can
    /// wake waiting [`wait()`] futures by calling `notify.notify_waiters()`.
    ///
    /// Interrupt enable/disable (IER/ICR) on the A-core side is managed by the
    /// kernel driver; userspace should not modify those registers directly.
    ///
    /// [`wait()`]: Doorbell::wait
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

#[cfg(not(feature = "std"))]
impl<const PROC: u8> HsemDoorbell<PROC> {
    /// Constant constructor for the M-core/bare-metal path, where the base address is a compile-time constant from the SoC TRM.
    /// Panics if `PROC` is not in the range `1..=3`.
    pub const fn new() -> Self {
        assert!(PROC >= 1 && PROC <= 3, "PROC must be 1, 2, or 3");
        Self {
            base: crate::registers::HSEM_BASE as usize,
        }
    }
}

#[cfg(feature = "std")]
impl<const PROC: u8> Doorbell for HsemDoorbell<PROC> {
    type Error = HsemError;
    type WaitFut<'a>
        = HsemWaitFut<'a, PROC>
    where
        Self: 'a;

    /// Signal the remote core by locking and immediately unlocking semaphore `ch`.
    ///
    /// Performs a 1-step read lock via `HSEM_RLRx` to atomically lock, then
    /// writes via `HSEM_Rx` with LOCK=0 to unlock and trigger the interrupt.
    /// Sync and IRQ-safe.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error> {
        let sem = ch.id();
        if sem >= 16 {
            return Err(HsemError::ChanOutOfRange);
        }

        // Compiler fence: ensure preceding slot writes are ordered before the
        // semaphore lock that signals the other core.
        compiler_fence(Ordering::Release);

        // 1-step lock: atomically lock the semaphore and read back its state.
        // SAFETY: self.base is a valid mmap'd HSEM virtual address; sem < 16.
        let locked_value = unsafe { self.read_lock(sem) };

        let locked_reg = fields::SemaphoreRegister::from_bits(locked_value);
        let lockid = locked_reg.lockid() as u8;

        if !locked_reg.lock() || lockid != crate::registers::A_CORE_LOCKID {
            return Err(HsemError::AlreadyLocked);
        }

        // Unlock by writing back with LOCK=0, which triggers the interrupt on the
        // remote core. The LOCKID, SEC, PRIV, and PROCID must match what we just locked.
        let mut unlock_reg = locked_reg;
        unlock_reg.set_lock(false);
        let unlock_value = unlock_reg.bits();
        // SAFETY: self.base is a valid mmap'd HSEM virtual address; sem < 16.
        unsafe { self.write_rx(sem, unlock_value) };

        compiler_fence(Ordering::Release);
        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        HsemWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        let sem = ch.id();
        if sem >= 16 {
            return false;
        }
        self.interrupt_pending(sem)
    }
}

/// # HsemWaitFut
///
/// ## A-core/Linux (std) path
///
/// Future returned by `HsemDoorbell::wait()` on the A-core (std/Linux) path.
///
/// On first poll, subscribes to the doorbell's internal `Notify` before
/// returning `Pending`, so that a concurrent `notify_waiters()` call cannot
/// be missed.  Resolves as soon as `notify_waiters()` is called on that
/// `Notify` (typically from a UIO IRQ loop or kernel-triggered callback).
///
/// Interrupt enable/disable (IER/ICR) is deliberately not touched here: on
/// Linux the kernel driver owns the HSEM interrupt state, and userspace must
/// not modify those registers directly.  Wire the doorbell's `Notify` to your
/// interrupt source via [`HsemDoorbell::notifier()`].
///
/// ## M-core/bare-metal (no_std) path
///
/// Future returned by `HsemDoorbell::wait()` on the M-core (no_std) path.
///
/// State machine:
/// ```ignored
///   Init → clear ICR → enable IER → register waker → poll MISR
///        ↓ MISR set
///   clear IER + ICR → compiler_fence(Acquire) → return Ok(ch)
/// ```
#[allow(unused)]
pub struct HsemWaitFut<'a, const PROC: u8> {
    doorbell: &'a HsemDoorbell<PROC>,
    ch: Chan,
    armed: bool,
    #[cfg(feature = "std")]
    /// Lifetime-erased `Notified` future.
    ///
    /// # Safety
    /// Safe because `doorbell` (which owns the `Arc<Notify>`) is borrowed for
    /// `'a`, so the `Notify` outlives this future.  The `transmute` only erases
    /// the borrow lifetime; the underlying `Arc` keeps the allocation alive.
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

#[allow(unused)]
impl<'a, const PROC: u8> HsemWaitFut<'a, PROC> {
    fn new(doorbell: &'a HsemDoorbell<PROC>, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            armed: false,
            #[cfg(feature = "std")]
            notified: None,
        }
    }
}

impl<'a, const PROC: u8> core::future::Future for HsemWaitFut<'a, PROC> {
    type Output = Result<Chan, HsemError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: HsemWaitFut contains no self-referential fields before the
        // Notified is stored; after that we never move the future.
        let this = unsafe { self.get_unchecked_mut() };
        let sem = this.ch.id();

        if sem >= 16 {
            return Poll::Ready(Err(HsemError::ChanOutOfRange));
        }

        // First poll: subscribe to the Notify before doing anything else so
        // that a concurrent notify_waiters() call cannot be missed.
        if !this.armed {
            // SAFETY: must be called from the correct interrupt context; sem is caller-validated.
            unsafe {
                this.doorbell.clear_interrupt(sem);
                this.doorbell.enable_interrupt(sem);
            }
            this.armed = true;
            #[cfg(feature = "std")]
            {
                // SAFETY: `doorbell` is borrowed for `'a` which outlives this
                // future.  The Arc inside keeps the Notify allocation alive.
                let notified = this.doorbell.notify.notified();
                let notified = unsafe {
                    std::mem::transmute::<
                        tokio::sync::futures::Notified<'_>,
                        tokio::sync::futures::Notified<'static>,
                    >(notified)
                };
                this.notified = Some(notified);
            }

            #[cfg(not(feature = "std"))]
            {
                WAKER.register(cx.waker());
            }
        }

        #[cfg(feature = "std")]
        {
            // Poll the Notified future to register our waker and detect completion.
            if let Some(notified) = this.notified.as_mut() {
                // SAFETY: `notified` lives inside `this` which is pinned.
                if unsafe { Pin::new_unchecked(notified) }.poll(cx).is_ready() {
                    compiler_fence(Ordering::Acquire);
                    return Poll::Ready(Ok(this.ch));
                }
            }

            Poll::Pending
        }

        #[cfg(not(feature = "std"))]
        {
            if this.doorbell.interrupt_pending(sem) {
                compiler_fence(Ordering::Acquire);

                // SAFETY: must be called from the correct interrupt context; sem is caller-validated.
                unsafe {
                    // Clear IER bit
                    let ier = this.doorbell.ier_addr();
                    let reg =
                        SemInterruptRegister::from_bits(ier.read_volatile()).with_sem_disabled(sem);
                    ier.write_volatile(reg.bits());
                    // Clear ICR
                    this.doorbell.clear_interrupt(sem);
                }

                Poll::Ready(Ok(this.ch))
            } else {
                WAKER.register(cx.waker());
                Poll::Pending
            }
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    fn ch(id: u8) -> Chan {
        // SAFETY: tests choose ids deliberately.
        unsafe { Chan::new_unchecked(id) }
    }

    #[test]
    fn wait_registers_waker_then_ready_after_notify() {
        let mut regs = [0u32; 0x120 / 4];
        let doorbell = HsemDoorbell::<2>::new(regs.as_mut_ptr() as usize);
        let mut wait = std::pin::pin!(doorbell.wait(ch(3)));
        let waker = std::task::Waker::noop();
        let mut cx = Context::from_waker(waker);

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        assert!(
            SemInterruptRegister::from_bits(regs[interrupt_nonsecure::HSEM_C2IER as usize / 4])
                .contains_sem(3)
        );
        assert_eq!(
            regs[interrupt_nonsecure::HSEM_C2ICR as usize / 4],
            SemInterruptRegister::from_bits(interrupt_nonsecure::fields::sem_bit(3)).bits()
        );

        doorbell.notifier().notify_waiters();

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch(3))));
    }

    #[test]
    fn wait_thread_wakes_after_notify() {
        let regs = Box::leak(Box::new([0u32; 0x120 / 4]));
        let base = regs.as_mut_ptr() as usize;
        let doorbell = HsemDoorbell::<2>::new(base);
        let notifier = doorbell.notifier();
        let (armed_tx, armed_rx) = std::sync::mpsc::channel();

        let waiter = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("create waiter runtime");

            rt.block_on(async move {
                armed_tx.send(()).expect("signal waiter is starting");
                doorbell.wait(ch(3)).await.expect("wait for HSEM doorbell")
            })
        });

        armed_rx.recv().expect("waiter thread started");
        while !SemInterruptRegister::from_bits(unsafe {
            (base as *const u32)
                .add(interrupt_nonsecure::HSEM_C2IER as usize / 4)
                .read_volatile()
        })
        .contains_sem(3)
        {
            std::thread::yield_now();
        }

        notifier.notify_waiters();

        assert_eq!(waiter.join().expect("waiter thread completes"), ch(3));
    }
}
