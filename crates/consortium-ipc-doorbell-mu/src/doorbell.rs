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
#[cfg(all(not(feature = "std")))]
use crate::registers::MU_BASES;

// ── Struct ────────────────────────────────────────────────────────────────────

/// MU-backed doorbell.
///
/// Signaling uses MU TX/RX register pairs:
/// - [`ring()`] writes a sentinel value to the local TRn register, which sets
///   RSR bit n on the remote side and (if enabled) fires their RFP interrupt.
/// - [`wait()`] enables the RCR interrupt for the channel, then polls RSR until
///   the remote core writes to their TRn. RRn is read to clear RSR (reading
///   is the clearing mechanism — there is no separate ICR register).
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
/// | Aspect              | HSEM                              | MU                                     |
/// |---------------------|-----------------------------------|----------------------------------------|
/// | Signal mechanism    | 1-step read-lock on semaphore reg | Write 1 to TX register (TRn)          |
/// | `ring()` check      | LOCKID ownership (MASTERID)       | TSR bit set (TRn empty/ready)          |
/// | Interrupt clear     | Dedicated ICR (write-1-to-clear)  | Read RRn (destructive read clears RSR) |
/// | Interrupt enable    | IER bit per semaphore             | RCR bit per RX register               |
/// | IRQ handlers        | `HSEM_IRQHandler`                 | `MU1_IRQHandler` … `MUN_IRQHandler`   |
///
/// [`ring()`]: Doorbell::ring
/// [`wait()`]: Doorbell::wait
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

    /// Address of TRn (Transmit Register n).
    #[inline]
    fn tr_addr(base: u32, reg_idx: u8) -> *mut u32 {
        (base + side::TR0 + reg_idx as u32 * 4) as *mut u32
    }

    /// Address of RRn (Receive Register n).
    #[inline]
    fn rr_addr(base: u32, reg_idx: u8) -> *mut u32 {
        (base + side::RR0 + reg_idx as u32 * 4) as *mut u32
    }

    /// Address of TSR (TX Status Register).
    #[inline]
    fn tsr_addr(base: u32) -> *const u32 {
        (base + side::TSR) as *const u32
    }

    /// Address of RSR (RX Status Register).
    #[inline]
    fn rsr_addr(base: u32) -> *const u32 {
        (base + side::RSR) as *const u32
    }

    /// Address of RCR (RX Control Register — per-channel RX interrupt enable).
    #[inline]
    fn rcr_addr(base: u32) -> *mut u32 {
        (base + side::RCR) as *mut u32
    }

    /// Returns `true` if TRn is empty (TSR bit `reg_idx` set — ready to send).
    ///
    /// TSR reset value = `0x0000_000F` → all 4 TX slots initially empty.
    #[inline]
    pub fn tx_empty(base: u32, reg_idx: u8) -> bool {
        // SAFETY: base is a valid mapped MU peripheral address; volatile read.
        unsafe { Self::tsr_addr(base).read_volatile() & (1 << reg_idx) != 0 }
    }

    /// Returns `true` if RRn has data (RSR bit `reg_idx` set).
    #[inline]
    pub fn rx_full(base: u32, reg_idx: u8) -> bool {
        // SAFETY: base is a valid mapped MU peripheral address; volatile read.
        unsafe { Self::rsr_addr(base).read_volatile() & (1 << reg_idx) != 0 }
    }

    /// Enable RX interrupt for `reg_idx` (set RCR bit).
    ///
    /// # Safety
    /// Caller must ensure `base` is valid and not racing other RCR writes.
    #[inline]
    pub unsafe fn enable_rx_interrupt(base: u32, reg_idx: u8) {
        let rcr = Self::rcr_addr(base);
        // SAFETY: caller guarantees.
        unsafe { rcr.write_volatile(rcr.read_volatile() | (1 << reg_idx)) };
    }

    /// Disable RX interrupt for `reg_idx` (clear RCR bit).
    ///
    /// # Safety
    /// Caller must ensure `base` is valid and not racing other RCR writes.
    #[inline]
    pub unsafe fn disable_rx_interrupt(base: u32, reg_idx: u8) {
        let rcr = Self::rcr_addr(base);
        // SAFETY: caller guarantees.
        unsafe { rcr.write_volatile(rcr.read_volatile() & !(1 << reg_idx)) };
    }

    /// Read RRn, which clears the RSR bit (the only clearing mechanism for MU).
    ///
    /// Unlike HSEM (dedicated ICR), the MU clears RX status only by reading RRn.
    /// Call only when the corresponding RSR bit is set.
    ///
    /// # Safety
    /// Caller must ensure `base` is valid and `reg_idx` < 4.
    #[inline]
    pub unsafe fn consume_rr(base: u32, reg_idx: u8) -> u32 {
        // SAFETY: caller guarantees.
        unsafe { Self::rr_addr(base, reg_idx).read_volatile() }
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

    /// Signal the remote core by writing a sentinel value to TRn.
    ///
    /// Checks TSR to ensure TRn is empty (remote consumed the last signal)
    /// before writing. Returns [`MuError::AlreadyLocked`] if the TX register
    /// is still full — the remote hasn't consumed the previous signal yet.
    fn ring(&self, ch: Chan) -> Result<(), MuError> {
        if ch.id() >= MU_CHAN_COUNT {
            return Err(MuError::ChanOutOfRange);
        }

        let base = Self::base_for(&self.base_addrs, ch);
        let reg_idx = ch.tx_rx_id();

        // Ensure preceding slot writes are visible to the remote before
        // the TX write that signals it.
        compiler_fence(Ordering::Release);

        if !Self::tx_empty(base, reg_idx) {
            return Err(MuError::AlreadyLocked);
        }

        // Write sentinel. This sets RSR on the remote side and fires their
        // RFP interrupt if RCR is enabled.
        // SAFETY: base is a valid MU peripheral address; reg_idx < 4.
        unsafe { Self::tr_addr(base, reg_idx).write_volatile(1) };

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
        Self::rx_full(base, ch.tx_rx_id())
    }
}

// ─── MuWaitFut (no_std) ───────────────────────────────────────────────────────

/// Future returned by [`MuDoorbell::wait()`] on the M-core (no_std) path.
///
/// State machine:
///
///   Init → enable RCR bit → register waker → poll RSR
///        ↓ RSR bit set (RRn has data)
///   read RRn (clears RSR) → disable RCR → compiler_fence(Acquire) → return Ok(ch)
///
/// RSR is checked eagerly; if already set on first poll the future completes
/// without waiting for an interrupt.
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
        let reg_idx = self.ch.tx_rx_id();
        let mu_id = self.ch.mu_id();

        if !self.interrupt_armed {
            // SAFETY: base is valid; reg_idx < 4; called from single executor task.
            unsafe { MuDoorbell::enable_rx_interrupt(base, reg_idx) };
            self.interrupt_armed = true;

            waker_for(mu_id).register(cx.waker());
        }

        if MuDoorbell::rx_full(base, reg_idx) {
            // Acquire fence: RSR set → remote's TRn write → our slot reads
            // must see all preceding writes the remote made before ring().
            compiler_fence(Ordering::Acquire);

            // SAFETY: RSR bit confirmed set; base and reg_idx are valid.
            let _ = unsafe { MuDoorbell::consume_rr(base, reg_idx) };
            unsafe { MuDoorbell::disable_rx_interrupt(base, reg_idx) };

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
// Each handler wakes the futures waiting on that instance's RX interrupt.
// Register these in your interrupt vector table.

/// # Safety
/// Must be called only from the MU1 receive IRQ handler.
#[cfg(all(not(feature = "std"), any(feature = "imx93", feature = "imx95")))]
#[unsafe(no_mangle)]
pub extern "C" fn MU1_IRQHandler() {
    MU_WAKERS[0].wake();
}

/// # Safety
/// Must be called only from the MU2 receive IRQ handler.
#[cfg(all(not(feature = "std"), any(feature = "imx93", feature = "imx95")))]
#[unsafe(no_mangle)]
pub extern "C" fn MU2_IRQHandler() {
    MU_WAKERS[1].wake();
}

/// # Safety
/// Must be called only from the MU3 receive IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU3_IRQHandler() {
    MU_WAKERS[2].wake();
}

/// # Safety
/// Must be called only from the MU4 receive IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU4_IRQHandler() {
    MU_WAKERS[3].wake();
}

/// # Safety
/// Must be called only from the MU5 receive IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU5_IRQHandler() {
    MU_WAKERS[4].wake();
}

/// # Safety
/// Must be called only from the MU6 receive IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU6_IRQHandler() {
    MU_WAKERS[5].wake();
}

/// # Safety
/// Must be called only from the MU7 receive IRQ handler.
#[cfg(all(not(feature = "std"), feature = "imx95"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU7_IRQHandler() {
    MU_WAKERS[6].wake();
}

/// # Safety
/// Must be called only from the MU8 receive IRQ handler.
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
    /// by calling `notify.notify_waiters()` when a MU receive interrupt fires.
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
        let reg_idx = ch.tx_rx_id();

        compiler_fence(Ordering::Release);

        if !Self::tx_empty(base, reg_idx) {
            return Err(MuError::AlreadyLocked);
        }

        // SAFETY: base is a valid mmap'd MU peripheral address; reg_idx < 4.
        unsafe { Self::tr_addr(base, reg_idx).write_volatile(1) };

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
        Self::rx_full(base, ch.tx_rx_id())
    }
}

// ─── MuWaitFut (std) ──────────────────────────────────────────────────────────

/// Future returned by [`MuDoorbell::wait()`] on the A-core (std/Linux) path.
///
/// Arms the RCR interrupt on first poll, then waits for the `Notify` held in
/// [`MuDoorbell`] to fire (driven by the UIO IRQ loop or other external notifier).
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
        let reg_idx = this.ch.tx_rx_id();

        if !this.interrupt_armed {
            // SAFETY: base is valid (mmap'd); reg_idx < 4; single tokio task.
            unsafe { MuDoorbell::enable_rx_interrupt(base, reg_idx) };
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

        if MuDoorbell::rx_full(base, reg_idx) {
            compiler_fence(Ordering::Acquire);

            // SAFETY: RSR bit confirmed set; base and reg_idx are valid.
            let _ = unsafe { MuDoorbell::consume_rr(base, reg_idx) };
            unsafe { MuDoorbell::disable_rx_interrupt(base, reg_idx) };

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
