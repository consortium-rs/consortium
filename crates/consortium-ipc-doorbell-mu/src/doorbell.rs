use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::{
    chan::{MU_CHAN_COUNT, MU_INSTANCE_COUNT, MuChanInfo},
    error::MuError,
    registers::side,
};

// Default (no_std / bare-metal) on i.MX93 uses compile-time base addresses.
#[cfg(not(feature = "std"))]
use crate::registers::MU_BASES;

// ── Struct ────────────────────────────────────────────────────────────────────

/// MU-backed doorbell using the general-purpose interrupt (GI) mechanism.
///
/// Signaling uses the GCR/GSR/GIER register group:
/// - [`ring()`] sets `GCR[GIRn]` on the local side; the hardware propagates
///   this to `GSR[GIPn]` on the remote side and fires their GI interrupt if
///   `GIER[GIEn]` is enabled.
/// - [`wait()`] enables `GIER[GIEn]`, then polls `GSR[GIPn]` until the remote
///   core has set it. Clears the pending bit by writing 1 to `GSR[GIPn]`
///   (write-1-to-clear), which also clears `GCR[GIRn]` on the sender.
/// - [`pending()`] reads `GSR[GIPn]` non-destructively.
///
/// **Base addresses** must match the side being accessed: MUA for the A-core
/// (Linux), MUB for the M-core (bare-metal).  On Linux the addresses must be
/// virtual (mmap'd); on bare-metal they may be the physical constants from
/// [`crate::registers`].  Consult your SoC's Reference Manual for values.
///
/// The number of MU instances is selected at compile time via a chip feature:
///
/// | Feature | Instances | `base_addrs` size |
/// |---------|-----------|-------------------|
/// | `imx93` |     2     | `[u32; 2]`        |
/// | `imx95` |     8     | `[u32; 8]`        |
///
/// # Difference from HSEM
///
/// | Aspect              | HSEM                              | MU (GI)                                  |
/// |---------------------|-----------------------------------|------------------------------------------|
/// | Signal mechanism    | 1-step read-lock on semaphore reg | Set `GCR[GIRn]` (idempotent flag)        |
/// | `ring()` guard      | LOCKID ownership (MASTERID)       | None — GCR set is always valid           |
/// | Interrupt clear     | Dedicated ICR (write-1-to-clear)  | Write 1 to `GSR[GIPn]` (W1C)            |
/// | Interrupt enable    | IER bit per semaphore             | `GIER[GIEn]` bit per channel             |
/// | Interrupt status    | MISR (masked)                     | `GSR[GIPn]` (non-destructive read)       |
/// | IRQ handlers        | `HSEM_IRQHandler`                 | `MU1_IRQHandler` … `MUN_IRQHandler`     |
///
/// [`ring()`]: Doorbell::ring
/// [`wait()`]: Doorbell::wait
/// [`pending()`]: Doorbell::pending
pub struct MuDoorbell {
    /// Base addresses for each MU instance, ordered by instance index
    /// (index 0 = MU1, index 1 = MU2, …).  Size is determined by the
    /// active chip feature (`imx93` = 2, `imx95` = 8).
    base_addrs: [u32; MU_INSTANCE_COUNT],
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// ── Register helpers ──────────────────────────────────────────────────────────

impl MuDoorbell {
    /// Look up the base address for the MU instance that owns channel `ch`.
    #[inline]
    fn base_for(base_addrs: &[u32], ch: Chan) -> u32 {
        base_addrs[ch.mu_id() as usize]
    }

    /// Address of GCR (General-purpose Control Register).
    #[inline]
    fn gcr_addr(base: u32) -> *mut u32 {
        (base + side::GCR) as *mut u32
    }

    /// Address of GSR (General-purpose Status Register).
    #[inline]
    fn gsr_addr(base: u32) -> *mut u32 {
        (base + side::GSR) as *mut u32
    }

    /// Address of GIER (General-purpose Interrupt Enable Register).
    #[inline]
    fn gier_addr(base: u32) -> *mut u32 {
        (base + side::GIER) as *mut u32
    }

    /// Set `GCR[GIRn]` (bit `gi_idx`) to signal the remote core.
    ///
    /// This sets `GSR[GIPn]` on the remote side and fires their GI interrupt
    /// if `GIER[GIEn]` is enabled. Setting the bit again while already pending
    /// is idempotent — the remote will see one pending signal.
    ///
    /// # Safety
    /// Caller must ensure `base` is a valid mapped MU peripheral address and
    /// `gi_idx` < 4.
    #[inline]
    pub unsafe fn set_gcr_gir(base: u32, gi_idx: u8) {
        let gcr = Self::gcr_addr(base);
        // SAFETY: caller guarantees.
        unsafe { gcr.write_volatile(gcr.read_volatile() | (1 << gi_idx)) };
    }

    /// Returns `true` if `GSR[GIPn]` (bit `gi_idx`) is set — the remote core
    /// has signaled this channel and the interrupt is pending.
    ///
    /// Non-destructive: reading GSR does not clear the pending bit.
    #[inline]
    pub fn gsr_pending(base: u32, gi_idx: u8) -> bool {
        // SAFETY: base is a valid mapped MU peripheral address; volatile read.
        unsafe { Self::gsr_addr(base).read_volatile() & (1 << gi_idx) != 0 }
    }

    /// Clear `GSR[GIPn]` (bit `gi_idx`) by writing 1 (write-1-to-clear).
    ///
    /// Also clears `GCR[GIRn]` on the sender side. Only the target bit is
    /// written — other pending GI channels are unaffected.
    ///
    /// # Safety
    /// Caller must ensure `base` is valid, `gi_idx` < 4, and `GSR[GIPn]` is
    /// actually set (i.e. call only after [`gsr_pending`] returns `true`).
    ///
    /// [`gsr_pending`]: Self::gsr_pending
    #[inline]
    pub unsafe fn clear_gsr_gip(base: u32, gi_idx: u8) {
        // W1C: write only the target bit; do NOT read-modify-write.
        // SAFETY: caller guarantees.
        unsafe { Self::gsr_addr(base).write_volatile(1 << gi_idx) };
    }

    /// Enable `GIER[GIEn]` (bit `gi_idx`) — arm the GI interrupt for this channel.
    ///
    /// # Safety
    /// Caller must ensure `base` is valid and not racing other GIER writes.
    #[inline]
    pub unsafe fn enable_gi_interrupt(base: u32, gi_idx: u8) {
        let gier = Self::gier_addr(base);
        // SAFETY: caller guarantees.
        unsafe { gier.write_volatile(gier.read_volatile() | (1 << gi_idx)) };
    }

    /// Disable `GIER[GIEn]` (bit `gi_idx`).
    ///
    /// # Safety
    /// Caller must ensure `base` is valid and not racing other GIER writes.
    #[inline]
    pub unsafe fn disable_gi_interrupt(base: u32, gi_idx: u8) {
        let gier = Self::gier_addr(base);
        // SAFETY: caller guarantees.
        unsafe { gier.write_volatile(gier.read_volatile() & !(1 << gi_idx)) };
    }
}

// ── no_std (M-core / bare-metal) ─────────────────────────────────────────────

/// `Default` is only available for `imx93` (no_std) where the physical base
/// addresses are known at compile time.  For other chips, construct explicitly
/// with `MuDoorbell::new(bases)`.
#[cfg(not(feature = "std"))]
impl Default for MuDoorbell {
    fn default() -> Self {
        Self::new(MU_BASES)
    }
}

#[cfg(not(feature = "std"))]
impl MuDoorbell {
    /// Create a doorbell from an array of MUB (Side B) base addresses, one per
    /// MU instance.  The array length is determined by the chip feature:
    /// `[u32; 2]` for `imx93`, `[u32; 8]` for `imx95`.
    pub const fn new(bases: [u32; MU_INSTANCE_COUNT]) -> Self {
        Self { base_addrs: bases }
    }
}

#[cfg(not(feature = "std"))]
impl Doorbell for MuDoorbell {
    type Error = MuError;
    type WaitFut<'a>
        = MuWaitFut<'a>
    where
        Self: 'a;

    /// Signal the remote core by setting `GCR[GIRn]`.
    ///
    /// The hardware propagates this to `GSR[GIPn]` on the remote side and
    /// fires their GI interrupt if `GIER[GIEn]` is enabled.  Setting the bit
    /// while the previous signal is still pending is idempotent — the remote
    /// will see one pending signal regardless.
    fn ring(&self, ch: Chan) -> Result<(), MuError> {
        if ch.id() >= MU_CHAN_COUNT {
            return Err(MuError::ChanOutOfRange);
        }

        let base = Self::base_for(&self.base_addrs, ch);
        let gi_idx = ch.gi_id();

        // Ensure preceding slot writes are visible to the remote before
        // the GCR write that signals it.
        compiler_fence(Ordering::Release);

        // Set GCR[GIRn]. This sets GSR[GIPn] on the remote side and fires
        // their GI interrupt if GIER[GIEn] is enabled.
        // SAFETY: base is a valid MU peripheral address; gi_idx < 4.
        unsafe { Self::set_gcr_gir(base, gi_idx) };

        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        MuWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        if ch.id() >= MU_CHAN_COUNT {
            return false;
        }
        let base = Self::base_for(&self.base_addrs, ch);
        Self::gsr_pending(base, ch.gi_id())
    }
}

// ─── MuWaitFut (no_std) ───────────────────────────────────────────────────────

/// Future returned by [`MuDoorbell::wait()`] on the M-core (no_std) path.
///
/// State machine:
///
///   Init → enable `GIER[GIEn]` → register waker → poll `GSR[GIPn]`
///        ↓ `GSR[GIPn]` set
///   write 1 to `GSR[GIPn]` (W1C) → disable `GIER[GIEn]` → compiler_fence(Acquire) → return Ok(ch)
///
/// `GSR[GIPn]` is checked eagerly; if already set on the first poll the future
/// completes without waiting for an interrupt.
#[cfg(not(feature = "std"))]
pub struct MuWaitFut<'a> {
    doorbell: &'a MuDoorbell,
    ch: Chan,
    interrupt_armed: bool,
}

#[cfg(not(feature = "std"))]
impl<'a> MuWaitFut<'a> {
    fn new(doorbell: &'a MuDoorbell, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            interrupt_armed: false,
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a> core::future::Future for MuWaitFut<'a> {
    type Output = Result<Chan, MuError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ch.id() >= MU_CHAN_COUNT {
            return Poll::Ready(Err(MuError::ChanOutOfRange));
        }

        let base = MuDoorbell::base_for(&self.doorbell.base_addrs, self.ch);
        let gi_idx = self.ch.gi_id();
        let mu_id = self.ch.mu_id();

        if !self.interrupt_armed {
            // SAFETY: base is valid; gi_idx < 4; called from single executor task.
            unsafe { MuDoorbell::enable_gi_interrupt(base, gi_idx) };
            self.interrupt_armed = true;

            waker_for(mu_id).register(cx.waker());
        }

        if MuDoorbell::gsr_pending(base, gi_idx) {
            // Acquire fence: GSR[GIPn] set → remote's GCR write → our slot reads
            // must see all preceding writes the remote made before ring().
            compiler_fence(Ordering::Acquire);

            // SAFETY: GSR[GIPn] confirmed set; base and gi_idx are valid.
            unsafe { MuDoorbell::clear_gsr_gip(base, gi_idx) };
            // SAFETY: base is valid; gi_idx < 4; called from single executor task.
            unsafe { MuDoorbell::disable_gi_interrupt(base, gi_idx) };

            Poll::Ready(Ok(self.ch))
        } else {
            waker_for(mu_id).register(cx.waker());
            Poll::Pending
        }
    }
}

// ── no_std: per-instance AtomicWakers and IRQ handlers ───────────────────────

/// Look up the `AtomicWaker` for the given zero-based MU instance index.
#[cfg(not(feature = "std"))]
#[inline]
fn waker_for(mu_id: u8) -> &'static atomic_waker::AtomicWaker {
    &MU_WAKERS[mu_id as usize]
}

// `const { }` blocks make the repeat expression valid for non-Copy types.
#[cfg(not(feature = "std"))]
static MU_WAKERS: [atomic_waker::AtomicWaker; MU_INSTANCE_COUNT] =
    [const { atomic_waker::AtomicWaker::new() }; MU_INSTANCE_COUNT];

// IRQ handlers — one per MU instance, chip-gated.
//
// Each handler wakes the futures waiting on that instance's GI interrupt.
// Register these in your interrupt vector table.

/// # Safety
/// Must be called only from the MU1 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), any(feature = "imx93", feature = "imx95")))]
#[unsafe(no_mangle)]
pub extern "C" fn MU1_IRQHandler() {
    MU_WAKERS[0].wake();
}

/// # Safety
/// Must be called only from the MU2 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), any(feature = "imx93", feature = "imx95")))]
#[unsafe(no_mangle)]
pub extern "C" fn MU2_IRQHandler() {
    MU_WAKERS[1].wake();
}

/// # Safety
/// Must be called only from the MU3 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU3_IRQHandler() {
    MU_WAKERS[2].wake();
}

/// # Safety
/// Must be called only from the MU4 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU4_IRQHandler() {
    MU_WAKERS[3].wake();
}

/// # Safety
/// Must be called only from the MU5 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU5_IRQHandler() {
    MU_WAKERS[4].wake();
}

/// # Safety
/// Must be called only from the MU6 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU6_IRQHandler() {
    MU_WAKERS[5].wake();
}

/// # Safety
/// Must be called only from the MU7 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU7_IRQHandler() {
    MU_WAKERS[6].wake();
}

/// # Safety
/// Must be called only from the MU8 general-purpose IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU8_IRQHandler() {
    MU_WAKERS[7].wake();
}

// ── std (A-core / Linux) ──────────────────────────────────────────────────────

#[cfg(feature = "std")]
impl MuDoorbell {
    /// Create a doorbell from an array of MUA (Side A) **virtual** base
    /// addresses, one per MU instance, as returned by `mmap`.
    ///
    /// The array length is determined by the chip feature:
    /// `[u32; 2]` for `imx93`, `[u32; 8]` for `imx95`.
    ///
    /// # Safety (caller obligation)
    /// Each element must be a virtual address obtained by mapping the
    /// corresponding MU peripheral (e.g. via `/dev/mem` or a UIO resource).
    /// Passing raw physical addresses is **undefined behaviour** on Linux.
    pub fn new(bases: [u32; MU_INSTANCE_COUNT]) -> Self {
        Self {
            base_addrs: bases,
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`.
    ///
    /// Pass this to the UIO IRQ loop so it can wake waiting [`wait()`] futures
    /// by calling `notify.notify_waiters()` when a MU GI interrupt fires.
    ///
    /// [`wait()`]: Doorbell::wait
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

#[cfg(feature = "std")]
impl Doorbell for MuDoorbell {
    type Error = MuError;
    type WaitFut<'a>
        = MuWaitFut<'a>
    where
        Self: 'a;

    fn ring(&self, ch: Chan) -> Result<(), MuError> {
        if ch.id() >= MU_CHAN_COUNT {
            return Err(MuError::ChanOutOfRange);
        }

        let base = Self::base_for(&self.base_addrs, ch);
        let gi_idx = ch.gi_id();

        compiler_fence(Ordering::Release);

        // SAFETY: base is a valid mmap'd MU peripheral address; gi_idx < 4.
        unsafe { Self::set_gcr_gir(base, gi_idx) };

        Ok(())
    }

    fn wait<'a>(&'a self, ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        MuWaitFut::new(self, ch)
    }

    fn pending(&self, ch: Chan) -> bool {
        if ch.id() >= MU_CHAN_COUNT {
            return false;
        }
        let base = Self::base_for(&self.base_addrs, ch);
        Self::gsr_pending(base, ch.gi_id())
    }
}

// ─── MuWaitFut (std) ──────────────────────────────────────────────────────────

/// Future returned by [`MuDoorbell::wait()`] on the A-core (std/Linux) path.
///
/// Arms `GIER[GIEn]` on first poll, then waits for the `Notify` held in
/// [`MuDoorbell`] to fire (driven by the UIO IRQ loop or other external
/// notifier).  Clears `GSR[GIPn]` (W1C) once the signal is detected.
#[cfg(feature = "std")]
pub struct MuWaitFut<'a> {
    doorbell: &'a MuDoorbell,
    ch: Chan,
    interrupt_armed: bool,
    /// Lifetime-erased `Notified` future.
    ///
    /// # Safety
    /// Safe because `doorbell` (which owns the `Arc<Notify>`) is borrowed for
    /// `'a`, so the `Notify` outlives this future. The transmute only erases
    /// the borrow lifetime; the underlying `Arc` keeps the allocation alive.
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

#[cfg(feature = "std")]
impl<'a> MuWaitFut<'a> {
    fn new(doorbell: &'a MuDoorbell, ch: Chan) -> Self {
        Self {
            doorbell,
            ch,
            interrupt_armed: false,
            notified: None,
        }
    }
}

#[cfg(feature = "std")]
impl<'a> core::future::Future for MuWaitFut<'a> {
    type Output = Result<Chan, MuError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: MuWaitFut has no self-referential fields before `notified`
        // is stored; after that we never move the future.
        let this = unsafe { self.get_unchecked_mut() };

        if this.ch.id() >= MU_CHAN_COUNT {
            return Poll::Ready(Err(MuError::ChanOutOfRange));
        }

        let base = MuDoorbell::base_for(&this.doorbell.base_addrs, this.ch);
        let gi_idx = this.ch.gi_id();

        if !this.interrupt_armed {
            // SAFETY: base is valid (mmap'd); gi_idx < 4; single tokio task.
            unsafe { MuDoorbell::enable_gi_interrupt(base, gi_idx) };
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

        if MuDoorbell::gsr_pending(base, gi_idx) {
            compiler_fence(Ordering::Acquire);

            // SAFETY: GSR[GIPn] confirmed set; base and gi_idx are valid.
            unsafe { MuDoorbell::clear_gsr_gip(base, gi_idx) };
            unsafe { MuDoorbell::disable_gi_interrupt(base, gi_idx) };

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
