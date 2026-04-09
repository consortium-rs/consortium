use core::{
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};

use crate::{
    chan::{MU_CHAN_COUNT, MuChanInfo},
    error::MuError,
    registers::mua,
};
// Physical-address constants are only needed by the no_std Default impl.
#[cfg(not(feature = "std"))]
use crate::registers::{MU1_BASE, MU2_BASE};

// ── Struct ────────────────────────────────────────────────────────────────────

/// MU-backed doorbell for i.MX9x series.
///
/// Signaling uses MU TX/RX register pairs:
/// - [`ring()`] writes a sentinel value to the local TRn register, which sets
///   RSR bit n on the remote side and (if enabled) fires their RFP interrupt.
/// - [`wait()`] enables the RCR interrupt for the channel, then polls RSR until
///   the remote core writes to their TRn. RRn is read to clear RSR (reading
///   is the clearing mechanism — there is no separate ICR register).
///
/// **Base addresses** must match the side being accessed. The A-core (Linux)
/// passes MUA base addresses; the M-core passes MUB base addresses. Use the
/// provided [`MU1_BASE`] / [`MU2_BASE`] constants as defaults for i.MX93
/// or substitute your platform's addresses.
///
/// # Difference from HSEM
///
/// | Aspect              | HSEM                                  | MU                                     |
/// |---------------------|---------------------------------------|----------------------------------------|
/// | Signal mechanism    | 1-step read-lock on semaphore reg     | Write 1 to TX register (TRn)          |
/// | `ring()` check      | LOCKID ownership (MASTERID)           | TSR bit set (TRn empty/ready)          |
/// | `ring()` AlreadyLocked | Another master holds the semaphore | Remote hasn't consumed last signal yet |
/// | Interrupt clear     | Dedicated ICR (write-1-to-clear)      | Read RRn (destructive read clears RSR) |
/// | Interrupt enable    | IER bit per semaphore                 | RCR bit per RX register               |
/// | Interrupt status    | MISR (masked ISR)                     | RSR (RX status, unmasked)             |
/// | Const generic       | `PROC: u8` (interrupt line 1/2/3)     | None (side = base addr + feature flag) |
/// | IRQ handlers        | One: `HSEM_IRQHandler`                | Two: `MU1_IRQHandler`, `MU2_IRQHandler`|
/// | Channel count       | 16 semaphores                         | 8 virtual (2 instances × 4 pairs)     |
///
/// Without `std` (M-core / bare-metal): uses per-MU-instance [`AtomicWaker`]s
/// and `MU1_IRQHandler` / `MU2_IRQHandler` interrupt vectors.
///
/// With `std` (A-core / Linux): holds a `tokio::sync::Notify` that external
/// code (e.g. the UIO IRQ loop) must call `notify_waiters()` on when the MU
/// receive interrupt fires. Obtain the `Arc<Notify>` via [`notifier()`].
///
/// [`ring()`]: Doorbell::ring
/// [`wait()`]: Doorbell::wait
/// [`notifier()`]: MuDoorbell::notifier
pub struct MuDoorbell {
    /// Base addresses for MU instances: index 0 = MU1, index 1 = MU2.
    base_addrs: [u32; 2],
    #[cfg(feature = "std")]
    notify: std::sync::Arc<tokio::sync::Notify>,
}

// ── Register helpers ──────────────────────────────────────────────────────────

impl MuDoorbell {
    #[inline]
    fn base_for(base_addrs: &[u32; 2], ch: Chan) -> u32 {
        base_addrs[ch.mu_id() as usize]
    }

    /// Address of TRn.
    #[inline]
    fn tr_addr(base: u32, reg_idx: u8) -> *mut u32 {
        (base + mua::TR0 + reg_idx as u32 * 4) as *mut u32
    }

    /// Address of RRn.
    #[inline]
    fn rr_addr(base: u32, reg_idx: u8) -> *mut u32 {
        (base + mua::RR0 + reg_idx as u32 * 4) as *mut u32
    }

    /// Address of TSR (TX Status Register).
    #[inline]
    fn tsr_addr(base: u32) -> *const u32 {
        (base + mua::TSR) as *const u32
    }

    /// Address of RSR (RX Status Register).
    #[inline]
    fn rsr_addr(base: u32) -> *const u32 {
        (base + mua::RSR) as *const u32
    }

    /// Address of RCR (RX Control Register — per-channel RX interrupt enable).
    #[inline]
    fn rcr_addr(base: u32) -> *mut u32 {
        (base + mua::RCR) as *mut u32
    }

    /// Returns `true` if TRn is empty (TSR bit `reg_idx` set), i.e. ready to send.
    ///
    /// TSR reset = 0x0000_000F → all 4 TX slots initially empty.
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

    /// Read RRn, which clears the RSR bit (reading is the only clearing mechanism).
    ///
    /// Unlike HSEM (which has a dedicated ICR), the MU clears RX status by reading
    /// the RR register. Only call when RSR bit is set.
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

#[cfg(not(feature = "std"))]
impl Default for MuDoorbell {
    fn default() -> Self {
        Self::new(MU1_BASE, MU2_BASE)
    }
}

#[cfg(not(feature = "std"))]
impl MuDoorbell {
    /// Create a doorbell with the given MU instance base addresses.
    ///
    /// On M-core, pass the **MUB** (Side B) base addresses for each instance.
    pub const fn new(mu1_base: u32, mu2_base: u32) -> Self {
        Self {
            base_addrs: [mu1_base, mu2_base],
        }
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
    ///
    /// This differs from HSEM where `AlreadyLocked` means another master holds
    /// the semaphore; here it means the channel has an unread pending signal.
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
/// Note: unlike HSEM there is no ICR to pre-clear on entry. Reading RRn is the
/// only clearing mechanism. RSR is checked eagerly; if already set on first
/// poll, the future completes without waiting for an interrupt.
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

/// Return the static `AtomicWaker` for the given MU instance (0 = MU1, 1 = MU2).
#[cfg(not(feature = "std"))]
#[inline]
fn waker_for(mu_id: u8) -> &'static atomic_waker::AtomicWaker {
    match mu_id {
        0 => &MU1_WAKER,
        1 => &MU2_WAKER,
        _ => unreachable!(),
    }
}

/// Global waker for MU1 receive interrupts.
#[cfg(not(feature = "std"))]
static MU1_WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Global waker for MU2 receive interrupts.
#[cfg(not(feature = "std"))]
static MU2_WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Call from the MU1 receive IRQ vector. Wakes any future waiting on MU1 channels.
///
/// # Safety
/// Must be called only from the MU1 IRQ handler with a correctly registered
/// interrupt vector table entry.
#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU1_IRQHandler() {
    MU1_WAKER.wake();
}

/// Call from the MU2 receive IRQ vector. Wakes any future waiting on MU2 channels.
///
/// # Safety
/// Must be called only from the MU2 IRQ handler with a correctly registered
/// interrupt vector table entry.
#[cfg(not(feature = "std"))]
#[unsafe(no_mangle)]
pub extern "C" fn MU2_IRQHandler() {
    MU2_WAKER.wake();
}

// ── std (A-core / Linux) ──────────────────────────────────────────────────────

#[cfg(feature = "std")]
impl MuDoorbell {
    /// Create a doorbell with the given MU instance base addresses.
    ///
    /// On A-core (Linux), `mu1_base` and `mu2_base` must be the **virtual**
    /// base addresses returned by `mmap` for each MU instance (e.g. via a UIO
    /// resource or `/dev/mem`).  Passing raw physical addresses here is
    /// **undefined behaviour** on Linux.
    pub fn new(mu1_base: u32, mu2_base: u32) -> Self {
        Self {
            base_addrs: [mu1_base, mu2_base],
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns a cloned `Arc` to the internal `Notify`.
    ///
    /// Pass this to the UIO IRQ loop so it can wake waiting [`wait()`] futures
    /// by calling `notify.notify_waiters()` when the MU receive interrupt fires.
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

    /// Signal the remote core by writing a sentinel value to TRn.
    ///
    /// See the no_std variant for full semantics. Behavior is identical across
    /// both paths — only the waking mechanism differs.
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
            // SAFETY: base is valid; reg_idx < 4; called from single tokio task.
            unsafe { MuDoorbell::enable_rx_interrupt(base, reg_idx) };
            this.interrupt_armed = true;

            // SAFETY: `doorbell` is borrowed for `'a` which outlives this
            // future. The Arc inside keeps the Notify allocation alive.
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
