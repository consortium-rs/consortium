#![cfg_attr(not(feature = "std"), no_std)]
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

//! GPIO-based doorbell for Consortium IPC.
//!
//! This doorbell uses a general-purpose GPIO output pin to signal events between
//! processors. Unlike hardware semaphores (HSEM) or message units (MU), GPIO
//! provides maximum flexibility at the cost of requiring manual wiring.
//!
//! # How it signals and waits
//!
//! `ring()` pulses the pin (high then low) to raise a rising-edge interrupt on the
//! remote side. Receiving is a [`GpioSignal`]: a software pending flag plus one
//! waker. Whatever observes the incoming edge calls [`GpioSignal::wake`] — the GPIO
//! IRQ handler on firmware, a `gpiod` event loop or UIO IRQ service on Linux — and
//! `wait()` completes once the flag is set, clearing it.
//!
//! The flag is what makes this safe to use on both sides: a wake that lands before
//! the waiter first polls is kept, not dropped, so a handshake that rings before
//! its peer is listening still completes. `std` and `no_std` run the same code; only
//! the cell around the pin differs (`Mutex` on `std`, `RefCell` otherwise).
//!
//! # Single-Channel Design
//!
//! Unlike HSEM (16 channels) or MU (8 channels), GPIO is single-channel per instance.
//! The `ch` argument to `ring()` and `wait()` is ignored; all signals use the same pin.
//! For multi-channel GPIO designs, instantiate multiple `GpioDoorbell<P>` with
//! different pins.
//!
//! # When to reach for this
//!
//! GPIO is the fallback, not the default. Prefer a real IPC peripheral —
//! IPCC (`consortium_ipc_transport_memory::bell::ipcc`) on STM32MP2, MU (`bell::mu`) on
//! i.MX 9 — whenever the two cores share one. GPIO earns its place when they do
//! not: two separate chips on one board, a core whose IPC peripheral is assigned
//! to another security domain, or a bring-up spike where a spare pin is faster to
//! wire than a RIF assignment.
//!
//! # Considerations and limitations
//!
//! - **No pin ownership beyond `embedded-hal`.** The doorbell is generic over any
//!   [`OutputPin`], so pinmux, drive strength, pull configuration, and — on the
//!   receiving side — edge-triggered interrupt setup are all the integrator's. This
//!   crate only pulses and only waits.
//! - **Rising-edge signaling, so a pulse can be missed.** `ring`
//!   sets the pin high then immediately low. If the peer's edge interrupt is masked
//!   across both transitions, the signal is gone — there is no status flag to
//!   re-read, unlike IPCC's occupancy bit or MU's `GSR`. Pair it with a transport
//!   whose payload state is self-describing; the shared-memory transport's
//!   `FLAG_VALID` makes a missed doorbell a latency problem rather than a lost
//!   message.
//! - **`ring` takes `&self` but `OutputPin` needs `&mut self`,** hence the
//!   `RefCell` (`no_std`) / `Mutex` (`std`) around the pin. On `no_std` that means
//!   `ring` must not be called re-entrantly from an interrupt while a `ring` is in
//!   progress, or the `RefCell` panics.
//! - **One waiter per signal.** A [`GpioSignal`] holds a single waker, like every
//!   Type-A bell line. Give each doorbell its own `static` signal; two doorbells
//!   sharing one would share their wakes.
//! - **Nothing to mask.** GPIO wakeups are edge-driven, so the Linux service is
//!   just the wake: `uio.bind_service(Arc::new(|| SIGNAL.wake()))`.
//!
//! [`OutputPin`]: embedded_hal::digital::OutputPin
//!
//! # Interrupts are the application's
//!
//! Like every Consortium doorbell, this crate installs no interrupt handler. The
//! application calls [`GpioSignal::wake`] from whatever observes the incoming edge:
//! its GPIO IRQ handler on `no_std`, or on `std` whatever watches the line (a
//! `gpiod` event loop, a UIO IRQ, a sysfs edge poll).
//!
//! ```rust,ignore
//! static SIGNAL: GpioSignal = GpioSignal::new();
//!
//! let doorbell = GpioDoorbell::new(pin, &SIGNAL);
//!
//! #[interrupt]
//! fn GPIO2() {
//!     // acknowledge the edge in the GPIO block, then:
//!     SIGNAL.wake();
//! }
//! ```
//!
//! # Features
//!
//! | Feature | Effect                                                                 |
//! | ------- | ---------------------------------------------------------------------- |
//! | `std`   | Default. `Mutex` around the pin, plus `linux-embedded-hal` for pins.    |
//! | `defmt` | Implement `defmt::Format` for [`GpioError`].                           |
//!
//! Build with `default-features = false` for bare-metal targets.

use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering, fence},
    task::{Context, Poll},
};

use atomic_waker::AtomicWaker;
use consortium_ipc::{Chan, Doorbell, MaybeSend};
use embedded_hal::digital::OutputPin;

mod error;
pub use error::GpioError;

/// The receive side of a GPIO doorbell: a pending flag and the waiter's waker.
///
/// Declare one `static` per doorbell and call [`wake`](Self::wake) from whatever
/// observes the incoming edge. It is `const`-constructible and needs no allocator.
pub struct GpioSignal {
    pending: AtomicBool,
    waker: AtomicWaker,
}

impl GpioSignal {
    /// A signal with nothing pending.
    pub const fn new() -> Self {
        Self {
            pending: AtomicBool::new(false),
            waker: AtomicWaker::new(),
        }
    }

    /// Record an incoming edge and wake the waiter, if one is registered.
    ///
    /// Safe to call from an interrupt handler. A wake with no waiter yet stays
    /// pending until the next `wait()` consumes it.
    pub fn wake(&self) {
        self.pending.store(true, Ordering::Release);
        self.waker.wake();
    }
}

impl Default for GpioSignal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "std"))]
type PinCell<P> = core::cell::RefCell<P>;
#[cfg(feature = "std")]
type PinCell<P> = std::sync::Mutex<P>;

/// A single-line doorbell that pulses an output pin and waits on a [`GpioSignal`].
pub struct GpioDoorbell<P> {
    pin: PinCell<P>,
    signal: &'static GpioSignal,
}

impl<P: OutputPin> GpioDoorbell<P> {
    /// A doorbell that rings by pulsing `pin` and waits on `signal`.
    pub fn new(pin: P, signal: &'static GpioSignal) -> Self {
        Self {
            pin: PinCell::new(pin),
            signal,
        }
    }

    /// The signal this doorbell waits on.
    pub fn signal(&self) -> &'static GpioSignal {
        self.signal
    }

    fn pulse(&self) -> Result<(), GpioError> {
        #[cfg(not(feature = "std"))]
        let mut pin = self.pin.borrow_mut();
        // A poisoned lock only means another ring panicked mid-pulse; the pin is
        // still a pin.
        #[cfg(feature = "std")]
        let mut pin = self
            .pin
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        pin.set_high()?;
        pin.set_low()?;
        Ok(())
    }
}

impl<P: OutputPin + MaybeSend> Doorbell for GpioDoorbell<P> {
    type Error = GpioError;

    fn ring(&self, _ch: Chan) -> Result<(), Self::Error> {
        // Order the caller's payload writes before the edge the peer observes.
        fence(Ordering::Release);
        self.pulse()
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        GpioWait {
            signal: self.signal,
            ch,
        }
    }
}

/// Future returned by [`GpioDoorbell::wait`].
pub struct GpioWait {
    signal: &'static GpioSignal,
    ch: Chan,
}

impl Future for GpioWait {
    type Output = Result<Chan, GpioError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Register before checking, so a wake landing between the two is seen by
        // the check or wakes this task — never neither.
        self.signal.waker.register(cx.waker());
        // Load then store rather than `swap`: ARMv6-M (the STM32MP25 CM0+) has no
        // atomic read-modify-write. A wake landing between the two folds into this
        // completion, which is harmless — the transport re-reads shared memory, so
        // one completion per burst of edges is all it needs.
        if self.signal.pending.load(Ordering::Acquire) {
            self.signal.pending.store(false, Ordering::Relaxed);
            // Pairs with the peer's Release before its ring.
            fence(Ordering::Acquire);
            return Poll::Ready(Ok(self.ch));
        }
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::task::{Wake, Waker};

    struct Pin(Vec<bool>);

    impl embedded_hal::digital::ErrorType for Pin {
        type Error = core::convert::Infallible;
    }

    impl OutputPin for Pin {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            self.0.push(false);
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), Self::Error> {
            self.0.push(true);
            Ok(())
        }
    }

    struct Count(AtomicUsize);

    impl Wake for Count {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn ch() -> Chan {
        // SAFETY: a GPIO doorbell has one line and ignores the id.
        unsafe { Chan::new_unchecked(0) }
    }

    fn leak() -> &'static GpioSignal {
        Box::leak(Box::new(GpioSignal::new()))
    }

    #[test]
    fn ring_pulses_the_pin() {
        let db = GpioDoorbell::new(Pin(Vec::new()), leak());
        db.ring(ch()).unwrap();
        assert_eq!(db.pin.lock().unwrap().0, [true, false]);
    }

    /// The case the old `std` path lost: a wake that lands before the waiter first
    /// polls must still complete the wait.
    #[test]
    fn a_wake_before_the_wait_is_kept() {
        let signal = leak();
        let db = GpioDoorbell::new(Pin(Vec::new()), signal);
        signal.wake();
        assert!(signal.pending.load(Ordering::Acquire));

        let mut cx = Context::from_waker(Waker::noop());
        let mut wait = std::pin::pin!(db.wait(ch()));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch())));
        assert!(!signal.pending.load(Ordering::Acquire));
    }

    #[test]
    fn a_wake_after_registration_wakes_the_waiter_once() {
        let signal = leak();
        let db = GpioDoorbell::new(Pin(Vec::new()), signal);
        let count = Arc::new(Count(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&count));
        let mut cx = Context::from_waker(&waker);
        let mut wait = std::pin::pin!(db.wait(ch()));

        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Pending);
        signal.wake();
        assert_eq!(count.0.load(Ordering::SeqCst), 1);
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Ok(ch())));

        // Consumed: the next wait parks again.
        let mut again = std::pin::pin!(db.wait(ch()));
        assert_eq!(again.as_mut().poll(&mut cx), Poll::Pending);
    }
}
