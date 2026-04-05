use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::{
    error::HsemError,
    registers::{
        interrupt_nonsecure,
        semaphore_r::{self, fields as r_fields},
        semaphore_rlr,
    },
};

// ── Struct definition ────────────────────────────────────────────────────────

/// HSEM-backed doorbell.
///
/// `PROC`: which processor's interrupt line to use for `wait()`.
///   - 1 = C1 (e.g. Cortex-M, bare-metal)
///   - 2 = C2 (e.g. Cortex-A, Linux)
///   - 3 = C3
///
/// Without the `std` feature (M-core/bare-metal): uses `AtomicWaker` and the
/// `HSEM_IRQHandler` interrupt vector.
///
/// With the `std` feature (A-core/Linux): holds a `tokio::sync::Notify` that
/// external code (e.g. the UIO IRQ loop) must call `notify_waiters()` on when
/// the HSEM interrupt fires.  Obtain the `Arc<Notify>` via [`notifier()`].
///
/// [`notifier()`]: HsemDoorbell::notifier
pub struct HsemDoorbell<const PROC: u8> {
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// ── Shared low-level register accessors ─────────────────────────────────────

impl<const PROC: u8> HsemDoorbell<PROC> {
    #[inline]
    fn rx_addr(sem: u8) -> *mut u32 {
        (semaphore_r::HSEM_R0 + sem as u32 * 4) as *mut u32
    }

    #[inline]
    fn rlr_addr(sem: u8) -> *const u32 {
        (semaphore_rlr::HSEM_RLR0 + sem as u32 * 4) as *const u32
    }

    /// Read the raw value of HSEM_Rx.
    #[inline]
    unsafe fn read_rx(sem: u8) -> u32 {
        // SAFETY: caller must ensure sem is valid and access is hardware-synchronized.
        unsafe { Self::rx_addr(sem).read_volatile() }
    }

    /// Attempt a 1-step read lock via HSEM_RLRx. Returns the register value.
    /// If LOCK=1 and LOCKID matches us → we got the lock.
    #[inline]
    unsafe fn read_lock(sem: u8) -> u32 {
        // SAFETY: caller must ensure sem is valid and access is hardware-synchronized.
        unsafe { Self::rlr_addr(sem).read_volatile() }
    }

    /// Returns true if the semaphore is currently locked (LOCK bit set).
    #[inline]
    pub fn is_locked(sem: u8) -> bool {
        // SAFETY: sem is caller-validated; volatile read is hardware-synchronized.
        unsafe { Self::read_rx(sem) & r_fields::LOCK_BIT != 0 }
    }

    // ── Interrupt register helpers ───────────────────────────────────────────

    #[inline]
    pub(crate) fn ier_addr() -> *mut u32 {
        match PROC {
            1 => interrupt_nonsecure::HSEM_C1IER as *mut u32,
            2 => interrupt_nonsecure::HSEM_C2IER as *mut u32,
            3 => interrupt_nonsecure::HSEM_C3IER as *mut u32,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn icr_addr() -> *mut u32 {
        match PROC {
            1 => interrupt_nonsecure::HSEM_C1ICR as *mut u32,
            2 => interrupt_nonsecure::HSEM_C2ICR as *mut u32,
            3 => interrupt_nonsecure::HSEM_C3ICR as *mut u32,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn misr_addr() -> *const u32 {
        match PROC {
            1 => interrupt_nonsecure::HSEM_C1MISR as *const u32,
            2 => interrupt_nonsecure::HSEM_C2MISR as *const u32,
            3 => interrupt_nonsecure::HSEM_C3MISR as *const u32,
            _ => unreachable!(),
        }
    }

    /// Enable the interrupt for semaphore `sem` on our processor's IER.
    ///
    /// # Safety
    /// Caller must ensure `sem` is a valid semaphore index (0–15) and that the
    /// read-modify-write is not racing with other cores' IER accesses.
    #[inline]
    pub unsafe fn enable_interrupt(sem: u8) {
        let ier = Self::ier_addr();
        // SAFETY: caller must ensure sem is valid and access is hardware-synchronized.
        unsafe { ier.write_volatile(ier.read_volatile() | (1 << sem)) };
    }

    /// Clear the interrupt status for semaphore `sem` (write 1 to ICR bit).
    ///
    /// # Safety
    /// Caller must ensure `sem` is a valid semaphore index (0–15) and that the
    /// write is issued from the correct interrupt context.
    #[inline]
    pub unsafe fn clear_interrupt(sem: u8) {
        // SAFETY: caller must ensure sem is valid and access is hardware-synchronized.
        unsafe { Self::icr_addr().write_volatile(1 << sem) };
    }

    /// Check the masked interrupt status — has semaphore `sem` fired an
    /// interrupt that is both pending AND enabled?
    #[inline]
    pub fn interrupt_pending(sem: u8) -> bool {
        // SAFETY: volatile read is hardware-synchronized.
        unsafe { Self::misr_addr().read_volatile() & (1 << sem) != 0 }
    }
}

// ── no_std (M-core / bare-metal) ─────────────────────────────────────────────

#[cfg(not(feature = "std"))]
impl<const PROC: u8> Default for HsemDoorbell<PROC> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "std"))]
impl<const PROC: u8> HsemDoorbell<PROC> {
    pub const fn new() -> Self {
        assert!(PROC >= 1 && PROC <= 3, "PROC must be 1, 2, or 3");
        Self {}
    }
}

#[cfg(not(feature = "std"))]
impl<const PROC: u8> Doorbell for HsemDoorbell<PROC> {
    type Error = HsemError;
    type WaitFut<'a>
        = HsemWaitFut<'a, PROC>
    where
        Self: 'a;

    /// Signal the remote core by locking semaphore `ch`.
    ///
    /// Uses 1-step read lock (HSEM_RLRx). Sync and IRQ-safe.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error> {
        let sem = ch.id();
        if sem >= 16 {
            return Err(HsemError::ChanOutOfRange);
        }

        // Compiler fence: ensure preceding slot writes are ordered before
        // the semaphore lock that signals the other core.
        compiler_fence(Ordering::Release);

        let result = unsafe { Self::read_lock(sem) };

        let lock_bit = result & r_fields::LOCK_BIT;
        let lockid = ((result & r_fields::LOCKID_MASK) >> r_fields::LOCKID_SHIFT) as u8;

        if lock_bit != 0 && lockid == crate::consts::M_CORE_LOCKID {
            Ok(())
        } else {
            Err(HsemError::AlreadyLocked)
        }
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
        Self::interrupt_pending(sem)
    }
}

// ─── HsemWaitFut (no_std) ────────────────────────────────────────────────────

/// Future returned by `HsemDoorbell::wait()` on the M-core (no_std) path.
///
/// State machine:
///
///   Init → clear ICR → enable IER → register waker → poll MISR
///        ↓ MISR set
///   clear IER + ICR → compiler_fence(Acquire) → return Ok(ch)
#[cfg(not(feature = "std"))]
pub struct HsemWaitFut<'a, const PROC: u8> {
    _doorbell: &'a HsemDoorbell<PROC>,
    ch: Chan,
    interrupt_armed: bool,
}

#[cfg(not(feature = "std"))]
impl<'a, const PROC: u8> HsemWaitFut<'a, PROC> {
    fn new(doorbell: &'a HsemDoorbell<PROC>, ch: Chan) -> Self {
        Self {
            _doorbell: doorbell,
            ch,
            interrupt_armed: false,
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a, const PROC: u8> core::future::Future for HsemWaitFut<'a, PROC> {
    type Output = Result<Chan, HsemError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let sem = self.ch.id();

        if sem >= 16 {
            return Poll::Ready(Err(HsemError::ChanOutOfRange));
        }

        if !self.interrupt_armed {
            unsafe {
                HsemDoorbell::<PROC>::clear_interrupt(sem);
                HsemDoorbell::<PROC>::enable_interrupt(sem);
            }
            self.interrupt_armed = true;
            WAKER.register(cx.waker());
        }

        if HsemDoorbell::<PROC>::interrupt_pending(sem) {
            compiler_fence(Ordering::Acquire);

            unsafe {
                // Clear IER bit
                let ier = HsemDoorbell::<PROC>::ier_addr();
                ier.write_volatile(ier.read_volatile() & !(1 << sem));
                // Clear ICR
                HsemDoorbell::<PROC>::clear_interrupt(sem);
            }

            Poll::Ready(Ok(self.ch))
        } else {
            WAKER.register(cx.waker());
            Poll::Pending
        }
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

// ── std (A-core / Linux) ──────────────────────────────────────────────────────

#[cfg(feature = "std")]
impl<const PROC: u8> Default for HsemDoorbell<PROC> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl<const PROC: u8> HsemDoorbell<PROC> {
    pub fn new() -> Self {
        assert!(PROC >= 1 && PROC <= 3, "PROC must be 1, 2, or 3");
        Self {
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`.
    ///
    /// Pass this to the UIO IRQ loop (or any other interrupt source) so it can
    /// wake waiting [`wait()`] futures by calling `notify.notify_waiters()`.
    ///
    /// [`wait()`]: Doorbell::wait
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

#[cfg(feature = "std")]
impl<const PROC: u8> Doorbell for HsemDoorbell<PROC> {
    type Error = HsemError;
    type WaitFut<'a>
        = HsemWaitFut<'a, PROC>
    where
        Self: 'a;

    /// Signal the remote core by locking semaphore `ch`.
    ///
    /// Uses 1-step read lock (HSEM_RLRx). Sync-safe.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error> {
        let sem = ch.id();
        if sem >= 16 {
            return Err(HsemError::ChanOutOfRange);
        }

        // Compiler fence: ensure preceding slot writes are ordered before the
        // semaphore lock that signals the other core.
        compiler_fence(Ordering::Release);

        let result = unsafe { Self::read_lock(sem) };

        let lock_bit = result & r_fields::LOCK_BIT;
        let lockid = ((result & r_fields::LOCKID_MASK) >> r_fields::LOCKID_SHIFT) as u8;

        if lock_bit != 0 && lockid == crate::consts::A_CORE_LOCKID {
            Ok(())
        } else {
            Err(HsemError::AlreadyLocked)
        }
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
        Self::interrupt_pending(sem)
    }
}

// ─── HsemWaitFut (std) ───────────────────────────────────────────────────────

/// Future returned by `HsemDoorbell::wait()` on the A-core (std/Linux) path.
///
/// Arms the HSEM interrupt registers on first poll, then waits for the
/// `Notify` held in [`HsemDoorbell`] to fire (driven by the UIO IRQ loop or
/// another external notifier).
#[cfg(feature = "std")]
pub struct HsemWaitFut<'a, const PROC: u8> {
    doorbell: &'a HsemDoorbell<PROC>,
    ch: Chan,
    interrupt_armed: bool,
    /// Lifetime-erased `Notified` future.
    ///
    /// # Safety
    /// Safe because `doorbell` (which owns the `Arc<Notify>`) is borrowed for
    /// `'a`, so the `Notify` outlives this future.  The `transmute` only erases
    /// the borrow lifetime; the underlying `Arc` keeps the allocation alive.
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

#[cfg(feature = "std")]
impl<'a, const PROC: u8> HsemWaitFut<'a, PROC> {
    fn new(doorbell: &'a HsemDoorbell<PROC>, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            interrupt_armed: false,
            notified: None,
        }
    }
}

#[cfg(feature = "std")]
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

        // First poll: clear any stale interrupt status, arm the interrupt, and
        // create the Notified handle so we get woken when the notify fires.
        if !this.interrupt_armed {
            unsafe {
                HsemDoorbell::<PROC>::clear_interrupt(sem);
                HsemDoorbell::<PROC>::enable_interrupt(sem);
            }
            this.interrupt_armed = true;

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

        if HsemDoorbell::<PROC>::interrupt_pending(sem) {
            compiler_fence(Ordering::Acquire);

            unsafe {
                let ier = HsemDoorbell::<PROC>::ier_addr();
                ier.write_volatile(ier.read_volatile() & !(1 << sem));
                HsemDoorbell::<PROC>::clear_interrupt(sem);
            }

            return Poll::Ready(Ok(this.ch));
        }

        // Poll the Notified future to register our waker with the Notify.
        if let Some(notified) = this.notified.as_mut() {
            // SAFETY: `notified` lives inside `this` which is pinned.
            let _ = unsafe { Pin::new_unchecked(notified) }.poll(cx);
        }

        Poll::Pending
    }
}
