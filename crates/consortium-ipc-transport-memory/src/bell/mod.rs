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

//! Type-A doorbells: memory-mapped signaling blocks that carry no payload and only
//! tell the peer "transport state changed".
//!
//! IPCC, MU, and HSEM differ in *which* registers they poke, not in *how* a wait has
//! to be driven. That protocol is subtle — a lost wakeup or an acknowledged-on-arm
//! notification deadlocks a link that only a reboot clears — so it lives here exactly
//! once, and each block supplies only its register model through [`BellHw`]:
//!
//! ```text
//! BellA<H>      Doorbell impl: fence + ring, raw-status pending, LineWait
//! LineWait<H>   the one wait future, with its Drop-time disarm
//! service       ISR core: read asserted lines, mask them, then wake
//! H: BellHw     per block: ring, raw status, ack, arm/disarm, waker table
//! ```
//!
//! # The wait protocol
//!
//! [`LineWait`] polls in this order, and the order is load-bearing:
//!
//! 1. Register the task's waker on **every** poll, before anything else, in the
//!    block's **per-line** table ([`BellHw::waker`]). One `AtomicWaker` per instance
//!    would let a second waiter on another line overwrite the first registration.
//!    `std` and `no_std` share this table: the UIO IRQ task wakes it exactly as a
//!    firmware vector does, so there is one wake mechanism, not two.
//! 2. `compiler_fence(SeqCst)`, then arm the line once, gated on `armed`.
//! 3. Every register read-modify-write — arm, completion, `Drop`, ISR service, and
//!    `ring` — runs inside `serialized`, because task context and the vector both
//!    write it: a critical section on `no_std`, a process-wide lock on `std`, where
//!    the UIO IRQ task can run on another thread.
//! 4. `compiler_fence(SeqCst)`, then decide readiness from the **raw** status
//!    ([`BellHw::raw_pending`]) — never the masked view, never the wake itself.
//!    Registering in step 1 *before* this read is what makes a ring that lands in
//!    between wake the task rather than vanish.
//! 5. Arming never acknowledges. A status bit set before the wait began is an
//!    unconsumed notification, and the remote cannot signal again while it waits.
//! 6. On readiness: `fence(Acquire)`, acknowledge, then disarm.
//!
//! A cancelled wait disarms its line in `Drop` and does **not** acknowledge, for the
//! reason in step 5. Nothing else would clear the level-sensitive status, so leaving
//! the line armed would make the UIO loop re-fire forever.
//!
//! # Interrupt service
//!
//! The blocks are level-sensitive, so the ISR masks every asserted line *before* it
//! wakes anything — an ISR that only woke would return with the line still asserted,
//! re-enter, and starve the executor. Firmware forwards a vector into
//! [`service_interrupt::<H>()`](service_interrupt); `consortium-cfg` generates that
//! vector from the manifest. On Linux, bind [`BellA::service_interrupt`] to the UIO
//! IRQ fan-out with `bind_service`.

#[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
pub mod hsem;
#[cfg(any(feature = "stm32mp21x", feature = "stm32mp23x", feature = "stm32mp25x"))]
pub mod ipcc;
#[cfg(any(feature = "mimx93", feature = "mimx95"))]
pub mod mu;

use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{Ordering, compiler_fence, fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, ChanValidate, Doorbell, MaybeSend};
use consortium_log::{debug, trace};

/// Error reported by a Type-A doorbell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BellError {
    /// The channel id names no line on this block.
    #[error("doorbell channel id out of range")]
    ChanOutOfRange,
    /// The previous signal on this line has not been consumed by the peer.
    #[error("doorbell line is still busy with the previous signal")]
    Busy,
}

/// A set of lines, bit `n` for channel `n`.
pub type LineSet = u32;

/// One Type-A doorbell instance's register model.
///
/// Implementations are thin MMIO wrappers. They do not fence, mask interrupts, or
/// touch wakers: [`BellA`], [`LineWait`], and [`service_interrupt`] do that, once,
/// for every block.
///
/// # Safety
///
/// - [`PHYS_BASE`](Self::PHYS_BASE) is the instance's vetted physical register
///   window, so [`service_interrupt`] can address it without a caller-supplied base.
/// - [`raw_pending`](Self::raw_pending) reads the raw status, which survives masking.
/// - [`arm`](Self::arm), [`disarm`](Self::disarm), [`ack`](Self::ack), and
///   [`disarm_all`](Self::disarm_all) touch only the lines they are given.
/// - [`CHANNELS`](Self::CHANNELS) is at most [`LineSet::BITS`], and
///   [`asserted`](Self::asserted) and [`disarm_all`](Self::disarm_all) use bit `n`
///   for channel `n`.
pub unsafe trait BellHw: Clone + Sync + 'static {
    /// A line, resolved from an in-range channel id: the id itself, or a (kind,
    /// index) pair on MU.
    type Line: Copy;

    /// Physical base of this instance's register window.
    const PHYS_BASE: usize;

    /// Number of channels; ids `0..CHANNELS` are lines, the rest are rejected.
    const CHANNELS: u8;

    /// A handle over the register window at `base`.
    ///
    /// # Safety
    ///
    /// `base` must address this instance's window: its physical base on firmware, or
    /// its `mmap`'d virtual base on Linux.
    unsafe fn at(base: usize) -> Self;

    /// Resolve channel `id` (`< CHANNELS`) to its line.
    fn line(id: u8) -> Self::Line;

    /// Signal the peer on `line`, reporting [`BellError::Busy`] where the block can
    /// tell that the previous signal is unconsumed.
    ///
    /// [`BellA`] serializes calls against every other register read-modify-write,
    /// so an implementation may read-modify-write a shared register here.
    fn ring(&self, line: Self::Line) -> Result<(), BellError>;

    /// Whether the peer has signalled `line`, from the raw (unmasked) status.
    fn raw_pending(&self, line: Self::Line) -> bool;

    /// Acknowledge a signalled `line`, clearing its status.
    ///
    /// # Safety
    ///
    /// `line` came from [`line`](Self::line) and its status is set.
    unsafe fn ack(&self, line: Self::Line);

    /// Enable and unmask `line`'s interrupt. Must not acknowledge.
    ///
    /// # Safety
    ///
    /// `line` came from [`line`](Self::line), and the caller serializes this
    /// read-modify-write against the ISR.
    unsafe fn arm(&self, line: Self::Line);

    /// Mask `line`'s interrupt. Must not acknowledge.
    ///
    /// # Safety
    ///
    /// As for [`arm`](Self::arm).
    unsafe fn disarm(&self, line: Self::Line);

    /// Every line whose raw status is asserted.
    fn asserted(&self) -> LineSet;

    /// Mask every line in `lines` in one read-modify-write per register.
    ///
    /// # Safety
    ///
    /// `lines` came from [`asserted`](Self::asserted), and the caller serializes this
    /// read-modify-write against task context.
    unsafe fn disarm_all(&self, lines: LineSet);

    /// The waker slot for channel `id` (`< CHANNELS`) on this instance.
    ///
    /// Every handle over one register window must return the same slot — a static
    /// table indexed by instance is the natural shape — because the handle that
    /// services the interrupt (a fresh [`at`](Self::at) on firmware, a clone on
    /// Linux) is not the one the waiter registered through.
    fn waker(&self, id: u8) -> &atomic_waker::AtomicWaker;
}

/// Resolve `ch` to `H`'s line: the one bounds check every operation shares.
#[inline]
fn line_of<H: BellHw>(ch: Chan) -> Result<H::Line, BellError> {
    const {
        assert!(
            H::CHANNELS as u32 <= LineSet::BITS,
            "a Type-A bell has more channels than a LineSet can hold"
        );
    }
    if ch.id() < H::CHANNELS {
        Ok(H::line(ch.id()))
    } else {
        Err(BellError::ChanOutOfRange)
    }
}

/// Validates user channel ids for this bell: in range.
impl<H: BellHw> ChanValidate for BellA<H> {
    type Error = BellError;

    fn validate(id: u8) -> Result<Chan, BellError> {
        if id >= H::CHANNELS {
            Err(BellError::ChanOutOfRange)
        } else {
            // SAFETY: `id` is below `CHANNELS`, so it names a line of this bell.
            Ok(unsafe { Chan::new_unchecked(id) })
        }
    }
}

#[cfg(not(feature = "std"))]
#[inline]
fn serialized<R>(f: impl FnOnce() -> R) -> R {
    critical_section::with(|_| f())
}

// On `std` the "ISR" is the UIO IRQ task, which Tokio may run on another worker
// thread than the waiter, so masking nothing would let the two read-modify-writes
// interleave and lose one (a line left masked parks its receiver forever). One
// process-wide lock covers every instance: a register window can be mapped more
// than once (one `BellA` per channel on a shared block), so a per-instance lock
// would not serialize them, and the critical sections are a handful of MMIO
// accesses.
#[cfg(feature = "std")]
#[inline]
fn serialized<R>(f: impl FnOnce() -> R) -> R {
    static RMW: std::sync::Mutex<()> = std::sync::Mutex::new(());
    // A panic inside `f` leaves no invariant behind the lock to poison.
    let _guard = RMW
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    f()
}

/// A Type-A doorbell over register model `H`.
///
/// Every handle over one window shares its waker table ([`BellHw::waker`]), so
/// `SharedMemoryTransport::split` can hand a clone to each half and an IRQ-side wake
/// reaches whichever half is waiting.
#[derive(Clone)]
pub struct BellA<H: BellHw> {
    hw: H,
}

impl<H: BellHw> BellA<H> {
    /// Create a doorbell over the register window at `base`.
    ///
    /// This is the Linux/UIO constructor: a mapped window has no peripheral
    /// singleton, so the caller carries the obligation instead. Firmware uses the
    /// block's singleton constructor (such as [`ipcc::IpccDoorbell::new`]).
    ///
    /// # Safety
    ///
    /// `base` must be the `mmap`'d virtual base of this instance's window. Passing a
    /// raw physical address on Linux is undefined behaviour.
    pub unsafe fn new_mapped(base: usize) -> Self {
        trace!("BellA::new_mapped(base = {:#x})", base);
        Self {
            // SAFETY: forwarded obligation.
            hw: unsafe { H::at(base) },
        }
    }

    /// The register model, for diagnostics and tests.
    pub fn hw(&self) -> &H {
        &self.hw
    }

    /// Service this instance's interrupt through this handle: mask every asserted
    /// line, then wake that line's waiter.
    ///
    /// This is the Linux entry point. Bind it with `UioDevice::bind_service` so a
    /// level-sensitive line does not immediately re-fire the UIO IRQ when the loop
    /// re-arms it. Firmware, which has no handle in its vector, uses the free
    /// `bell::service_interrupt::<H>()` instead; both run the same
    /// `service` core.
    pub fn service_interrupt(&self) {
        service(&self.hw);
    }
}

impl<H: BellHw> Doorbell for BellA<H> {
    type Error = BellError;

    fn ring(&self, ch: Chan) -> Result<(), BellError> {
        trace!("BellA::ring(ch = {:?})", ch);
        let line = line_of::<H>(ch)?;
        // Order the caller's slot writes before the register write the peer
        // observes.
        fence(Ordering::Release);
        // Some blocks ring with a read-modify-write (MU sets a `GCR` bit), and the
        // two transport halves can ring the same window from different tasks.
        serialized(|| self.hw.ring(line))
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        trace!("BellA::wait(ch = {:?})", ch);
        LineWait::new(self, ch)
    }
}

/// Future returned by [`BellA::wait`]; see the [module docs](self) for its protocol.
pub struct LineWait<'a, H: BellHw> {
    bell: &'a BellA<H>,
    ch: Chan,
    armed: bool,
}

impl<'a, H: BellHw> LineWait<'a, H> {
    fn new(bell: &'a BellA<H>, ch: Chan) -> Self {
        Self {
            bell,
            ch,
            armed: false,
        }
    }
}

impl<H: BellHw> Drop for LineWait<'_, H> {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let Ok(line) = line_of::<H>(self.ch) else {
            return;
        };
        // Disarm only. Acknowledging here would drop a notification the caller has
        // not consumed.
        serialized(|| {
            // SAFETY: `line` was validated and this future owns the armed wait.
            unsafe { self.bell.hw.disarm(line) };
        });
    }
}

impl<H: BellHw> Future for LineWait<'_, H> {
    type Output = Result<Chan, BellError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Every field is `Unpin`, so the future never needs pinning in place.
        let this = self.get_mut();
        let line = match line_of::<H>(this.ch) {
            Ok(line) => line,
            Err(err) => return Poll::Ready(Err(err)),
        };

        this.bell.hw.waker(this.ch.id()).register(cx.waker());

        compiler_fence(Ordering::SeqCst);

        if !this.armed {
            trace!("wait({:?}) -> arm", this.ch);
            serialized(|| {
                // SAFETY: `line` was validated; armed once from this task, and the
                // read-modify-write is serialized against the ISR.
                unsafe { this.bell.hw.arm(line) };
            });
            this.armed = true;
        }

        compiler_fence(Ordering::SeqCst);

        if this.bell.hw.raw_pending(line) {
            // Pairs with the remote's Release before its ring, so the caller's
            // subsequent slot reads observe the remote's writes.
            fence(Ordering::Acquire);

            // SAFETY: `line` was validated and its status is set.
            unsafe { this.bell.hw.ack(line) };
            serialized(|| {
                // SAFETY: `line` was validated; re-mask after completion.
                unsafe { this.bell.hw.disarm(line) };
            });
            // Completed: the line is already masked, so `Drop` has nothing to do.
            this.armed = false;
            debug!("wait({:?}) -> acknowledged", this.ch);
            return Poll::Ready(Ok(this.ch));
        }

        Poll::Pending
    }
}

/// Read the asserted lines, mask them all *before* waking, then wake each one's
/// waiter.
///
/// Masking first keeps a level-sensitive line from re-asserting immediately — on
/// `no_std` re-entering the ISR, on `std` re-firing the UIO IRQ. The woken wait then
/// acknowledges its line, and the next wait unmasks it again. The status is read raw,
/// so a line that is already masked is still serviced.
fn service<H: BellHw>(hw: &H) {
    let asserted = hw.asserted();
    serialized(|| {
        // SAFETY: the set came straight from `asserted`.
        unsafe { hw.disarm_all(asserted) };
    });
    for id in 0..H::CHANNELS {
        if asserted & (1 << id) != 0 {
            hw.waker(id).wake();
        }
    }
}

/// Service instance `H`'s interrupt from its vector, then wake the futures waiting
/// on the asserted lines.
///
/// This crate installs no vector-table entries. Generated bring-up emits the
/// `#[interrupt]` and forwards to this with the instance's register model, such as
/// `service_interrupt::<ipcc::Ipcc<Secondary, 1>>()`. It takes no base: the window is
/// [`BellHw::PHYS_BASE`], the same vetted address the waiting doorbell was built
/// from, so a vector cannot be pointed at the wrong instance.
#[cfg(not(feature = "std"))]
pub fn service_interrupt<H: BellHw>() {
    // SAFETY: `PHYS_BASE` is the instance's vetted window (a `BellHw` obligation).
    // This deliberately aliases the window the waiting doorbell holds — servicing
    // the line it raised is the point — and every enable-register write underneath
    // runs in a critical section, so it cannot tear against task context.
    let hw = unsafe { H::at(H::PHYS_BASE) };
    service(&hw);
}

#[cfg(all(test, feature = "std"))]
mod tests;
