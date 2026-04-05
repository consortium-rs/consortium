use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::hsem::{
    consts::M_CORE_LOCKID,
    error::HsemError,
    registers::{
        interrupt_nonsecure,
        semaphore_r::{self, fields as r_fields},
        semaphore_rlr,
    },
};

/// HSEM-backed doorbell.
///
/// `PROC`: which processor's interrupt line to use for `wait()`.
///   - 1 = C1 (e.g. Cortex-M, bare-metal)
///   - 2 = C2 (e.g. Cortex-A, Linux)
///   - 3 = C3
pub struct HsemDoorbell<const PROC: u8>;

impl<const PROC: u8> HsemDoorbell<PROC> {
    pub const fn new() -> Self {
        // Validate PROC at compile time
        assert!(PROC >= 1 && PROC <= 3, "PROC must be 1, 2, or 3");
        Self
    }

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
        // SAFETY: It's in `unsafe` fn, caller must ensure sem is valid and that this read is properly synchronized with the hardware.
        unsafe { Self::rx_addr(sem).read_volatile() }
    }

    /// Attempt a 1-step read lock. Returns the register value after the attempt.
    /// If LOCK=1 and LOCKID matches us → we got it.
    #[inline]
    unsafe fn read_lock(sem: u8) -> u32 {
        // SAFETY: It's in `unsafe` fn, caller must ensure sem is valid and that this read is properly synchronized with the hardware.
        unsafe { Self::rlr_addr(sem).read_volatile() }
    }

    /// Write unlock: LOCK=0, our LOCKID, PROCID=0.
    #[inline]
    unsafe fn _write_unlock(sem: u8) {
        let val = ((M_CORE_LOCKID as u32) << r_fields::LOCKID_SHIFT) & r_fields::LOCKID_MASK;
        // LOCK bit = 0, so this is an unlock attempt
        // SAFETY: It's in `unsafe` fn, caller must ensure sem is valid and that this write is properly synchronized with the hardware.
        unsafe { Self::rx_addr(sem).write_volatile(val) };
    }

    /// Returns true if the semaphore is currently locked (LOCK bit set).
    #[inline]
    pub fn is_locked(sem: u8) -> bool {
        // SAFETY: We guarantee sem is valid and that this read is properly synchronized with the hardware.
        unsafe { Self::read_rx(sem) & r_fields::LOCK_BIT != 0 }
    }

    // ── Interrupt register helpers ───────────────────────────────────────────

    #[inline]
    fn ier_addr() -> *mut u32 {
        match PROC {
            1 => interrupt_nonsecure::HSEM_C1IER as *mut u32,
            2 => interrupt_nonsecure::HSEM_C2IER as *mut u32,
            3 => interrupt_nonsecure::HSEM_C3IER as *mut u32,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn icr_addr() -> *mut u32 {
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
    #[inline]
    pub unsafe fn enable_interrupt(sem: u8) {
        let ier = Self::ier_addr();
        // SAFETY: Caller must ensure sem is valid and that this read/write is properly synchronized with the hardware.
        unsafe { ier.write_volatile(ier.read_volatile() | (1 << sem)) };
    }

    /// Clear the interrupt status for semaphore `sem` (write 1 to ICR bit).
    #[inline]
    pub unsafe fn clear_interrupt(sem: u8) {
        // SAFETY: Caller must ensure sem is valid and that this write is properly synchronized with the hardware.
        unsafe { Self::icr_addr().write_volatile(1 << sem) };
    }

    /// Check the masked interrupt status — has semaphore `sem` fired an
    /// interrupt that is both pending AND enabled?
    #[inline]
    pub fn interrupt_pending(sem: u8) -> bool {
        // SAFETY: We guarantee sem is valid and that this read is properly synchronized with the hardware.
        unsafe { Self::misr_addr().read_volatile() & (1 << sem) != 0 }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Doorbell trait implementation
// ─────────────────────────────────────────────────────────────────────────────

impl<const PROC: u8> Doorbell for HsemDoorbell<PROC> {
    type Error = HsemError;
    type WaitFut<'a>
        = HsemWaitFut<'a, PROC>
    where
        Self: 'a;

    /// Signal the remote core by locking semaphore `ch`.
    ///
    /// Uses 1-step read lock (HSEM_RLRx). Sync and IRQ-safe.
    /// Per the spec: after ringing, the sender should later call
    /// clear() once the remote acknowledges — but that's the
    /// transport's responsibility, not the doorbell's.
    fn ring(&self, ch: Chan) -> Result<(), Self::Error> {
        let sem = ch.id();
        if sem >= 16 {
            return Err(HsemError::ChanOutOfRange);
        }

        // Compiler fence: ensure preceding slot writes are ordered before
        // the semaphore lock that signals the other core.
        compiler_fence(Ordering::Release);

        let result = unsafe { Self::read_lock(sem) };

        // Check LOCK bit and LOCKID to confirm we won the lock.
        let lock_bit = result & r_fields::LOCK_BIT;
        let lockid = ((result & r_fields::LOCKID_MASK) >> r_fields::LOCKID_SHIFT) as u8;

        if lock_bit != 0 && lockid == M_CORE_LOCKID {
            Ok(())
        } else {
            Err(HsemError::AlreadyLocked)
        }
    }

    /// Await a signal from the remote core on channel `ch`.
    ///
    /// Returns the Chan that fired. The future polls `pending()` on each
    /// wakeup and yields until the MISR bit is set, then clears it.
    ///
    /// On a bare-metal executor, pair this with the HSEM IRQ handler
    /// calling `wake()` on the stored waker. On Linux/UIO the executor
    /// handles wakeup via the UIO fd — same poll contract, different waker.
    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        HsemWaitFut::new(self, ch)
    }

    /// Non-blocking check: has the semaphore been unlocked and the
    /// interrupt fired (MISR bit set)?
    fn pending(&self, ch: Chan) -> bool {
        let sem = ch.id();
        if sem >= 16 {
            return false;
        }
        Self::interrupt_pending(sem)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// HsemWaitFut
// ─────────────────────────────────────────────────────────────────────────────

/// Future returned by `HsemDoorbell::wait()`.
///
/// State machine:
///
///   Init → enable ISR for sem → poll MISR
///        ↓ MISR set
///   clear ICR → compiler_fence(Acquire) → return Ok(ch)
pub struct HsemWaitFut<'a, const PROC: u8> {
    _doorbell: &'a HsemDoorbell<PROC>,
    ch: Chan,
    /// Whether we have already enabled the interrupt for this sem.
    interrupt_armed: bool,
}

impl<'a, const PROC: u8> HsemWaitFut<'a, PROC> {
    fn new(doorbell: &'a HsemDoorbell<PROC>, ch: Chan) -> Self {
        Self {
            _doorbell: doorbell,
            ch,
            interrupt_armed: false,
        }
    }
}

impl<'a, const PROC: u8> Future for HsemWaitFut<'a, PROC> {
    type Output = Result<Chan, HsemError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let sem = self.ch.id();

        if sem >= 16 {
            return Poll::Ready(Err(HsemError::ChanOutOfRange));
        }

        // First poll: clear any stale interrupt status, then arm the interrupt.
        // This mirrors the procedure in Section 22.3.7 of the HSEM spec:
        //   1. Try to lock → fails → clear ISR → retry → fails → enable IER.
        if !self.interrupt_armed {
            unsafe {
                HsemDoorbell::<PROC>::clear_interrupt(sem);
                HsemDoorbell::<PROC>::enable_interrupt(sem);
            }
            self.interrupt_armed = true;

            // Store waker so the IRQ handler can wake this task.
            // (IRQ handler calls WAKER.wake() after seeing MISR set.)
            WAKER.register(cx.waker());
        }

        // Check if MISR bit is set — interrupt has fired.
        if HsemDoorbell::<PROC>::interrupt_pending(sem) {
            // Acquire fence: ensure reads of the slot happen after
            // we observe the semaphore unlock signal.
            compiler_fence(Ordering::Acquire);

            // Disable and clear the interrupt for this semaphore.
            unsafe {
                // Clear IER bit
                let ier = HsemDoorbell::<PROC>::ier_addr();
                ier.write_volatile(ier.read_volatile() & !(1 << sem));
                // Clear ICR
                HsemDoorbell::<PROC>::clear_interrupt(sem);
            }

            Poll::Ready(Ok(self.ch))
        } else {
            // Not yet fired — re-register waker and yield.
            WAKER.register(cx.waker());
            Poll::Pending
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// IRQ handler (bare-metal side)
// ─────────────────────────────────────────────────────────────────────────────

/// Global waker storage. In a real system use an AtomicWaker per channel,
/// or an array indexed by semaphore ID.
static WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Call this from your HSEM IRQ vector.
/// The executor will re-poll any futures waiting on this doorbell.
/// SAFETY: It's more of name conflict for `unsafe`; we just need to ensure this is called from the correct IRQ handler and that the handler is properly registered in the vector table.
#[unsafe(no_mangle)]
pub extern "C" fn HSEM_IRQHandler() {
    WAKER.wake();
}
