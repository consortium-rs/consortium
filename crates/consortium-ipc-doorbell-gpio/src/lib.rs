#![cfg_attr(not(feature = "std"), no_std)]

//! GPIO-based doorbell for Consortium IPC.
//!
//! This doorbell uses a general-purpose GPIO output pin to signal events between
//! processors. Unlike hardware semaphores (HSEM) or message units (MU), GPIO
//! provides maximum flexibility at the cost of requiring manual wiring.
//!
//! # no_std (M-core / Bare-metal)
//!
//! On the M-core side, `GpioDoorbell<P>` holds a GPIO pin wrapped in `RefCell`.
//! The `ring()` method pulses the pin (high then low) to generate a rising-edge
//! interrupt on the remote side. The `wait()` future polls a software `PENDING`
//! flag set by the `GpioDoorbell::wake()` method, which the user must call from
//! their GPIO IRQ handler.
//!
//! # std (A-core / Linux)
//!
//! On the A-core side, `GpioDoorbell<P>` holds a GPIO pin wrapped in `Mutex` and
//! a `tokio::sync::Notify` for async wakeup. The user constructs the doorbell,
//! retrieves the `Notify` via `notifier()`, and wires it up to their interrupt
//! source so that external code calls `notify.notify_waiters()` when the GPIO
//! interrupt fires.
//!
//! # Single-Channel Design
//!
//! Unlike HSEM (16 channels) or MU (8 channels), GPIO is single-channel per instance.
//! The `ch` argument to `ring()` and `wait()` is ignored; all signals use the same pin.
//! For multi-channel GPIO designs, instantiate multiple `GpioDoorbell<P>` with
//! different pins.

use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering, compiler_fence},
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell};
use embedded_hal::digital::OutputPin;

// ── no_std (M-core / bare-metal) ─────────────────────────────────────────────

#[cfg(not(feature = "std"))]
use core::cell::RefCell;

/// Global waker for M-core GPIO IRQ handler.
#[cfg(not(feature = "std"))]
static WAKER: atomic_waker::AtomicWaker = atomic_waker::AtomicWaker::new();

/// Software-tracked pending interrupt for M-core.
/// Set by the ISR via `GpioDoorbell::wake()`, cleared by the future.
#[cfg(not(feature = "std"))]
static PENDING: AtomicBool = AtomicBool::new(false);

/// GPIO doorbell for M-core (bare-metal, no_std).
///
/// Holds the output pin in a `RefCell` for interior mutability
/// (necessary because `ring()` takes `&self` but `OutputPin::set_high()`
/// requires `&mut self`).
#[cfg(not(feature = "std"))]
pub struct GpioDoorbell<P> {
    pin: RefCell<P>,
}

#[cfg(not(feature = "std"))]
impl<P: OutputPin> GpioDoorbell<P> {
    /// Create a new GPIO doorbell with the given output pin.
    pub fn new(pin: P) -> Self {
        Self {
            pin: RefCell::new(pin),
        }
    }

    /// Call this from your GPIO IRQ handler to signal that an interrupt has arrived.
    ///
    /// Sets the `PENDING` flag and wakes any executor awaiting on `wait()`.
    /// The exact IRQ handler name is board-specific; this method is public
    /// for the user to call as needed.
    pub fn wake() {
        PENDING.store(true, Ordering::Release);
        WAKER.wake();
    }
}

#[cfg(not(feature = "std"))]
impl<P: OutputPin> Doorbell for GpioDoorbell<P> {
    type Error = P::Error;
    type WaitFut<'a> = GpioWaitFut<'a, P> where Self: 'a;

    fn ring(&self, _ch: Chan) -> Result<(), Self::Error> {
        // Channel argument ignored — single-channel GPIO.
        // Pulse: set_high then set_low to generate a rising-edge interrupt.
        compiler_fence(Ordering::Release);

        let mut pin = self.pin.borrow_mut();
        pin.set_high()?;
        pin.set_low()?;

        Ok(())
    }

    fn wait<'a>(&'a self, _ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        GpioWaitFut::new(self)
    }

    fn pending(&self, _ch: Chan) -> bool {
        PENDING.load(Ordering::Acquire)
    }
}

/// Future returned by `GpioDoorbell::wait()` on M-core.
///
/// Polls a software `PENDING` flag and arranges for the executor to be
/// woken when an interrupt arrives.
#[cfg(not(feature = "std"))]
pub struct GpioWaitFut<'a, P> {
    _doorbell: &'a GpioDoorbell<P>,
    armed: bool,
}

#[cfg(not(feature = "std"))]
impl<'a, P> GpioWaitFut<'a, P> {
    fn new(doorbell: &'a GpioDoorbell<P>) -> Self {
        Self {
            _doorbell: doorbell,
            armed: false,
        }
    }
}

#[cfg(not(feature = "std"))]
impl<'a, P: OutputPin> Future for GpioWaitFut<'a, P> {
    type Output = Result<Chan, P::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.armed {
            // Register waker BEFORE checking PENDING to close the race:
            // if the ISR fires (sets PENDING + calls WAKER.wake()) between the
            // PENDING check and the register call, the wake will still be delivered.
            WAKER.register(cx.waker());
            self.armed = true;
        }

        if PENDING.load(Ordering::Acquire) {
            // Signal arrived. Clear the flag and return.
            PENDING.store(false, Ordering::Release);
            compiler_fence(Ordering::Acquire);
            // SAFETY: 0 is a valid channel ID for single-channel GPIO.
            return Poll::Ready(Ok(unsafe { Chan::new_unchecked(0) }));
        }

        // No signal yet. Re-register waker and return Pending.
        // The executor may have changed the waker between polls.
        WAKER.register(cx.waker());
        Poll::Pending
    }
}

// ── std (A-core / Linux) ─────────────────────────────────────────────────────

#[cfg(feature = "std")]
use std::sync::Mutex;

/// GPIO doorbell for A-core (Linux, std).
///
/// Holds the output pin in a `Mutex` for thread-safe interior mutability,
/// and a `tokio::sync::Notify` that external code must call `notify_waiters()`
/// on when a GPIO interrupt arrives.
#[cfg(feature = "std")]
pub struct GpioDoorbell<P> {
    pin: Mutex<P>,
    notify: std::sync::Arc<tokio::sync::Notify>,
}

#[cfg(feature = "std")]
impl<P: OutputPin> GpioDoorbell<P> {
    /// Create a new GPIO doorbell with the given output pin.
    pub fn new(pin: P) -> Self {
        Self {
            pin: Mutex::new(pin),
            notify: std::sync::Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Returns an Arc to the internal Notify for wiring to the interrupt source.
    ///
    /// When the GPIO interrupt fires, the interrupt handler must call
    /// `notify.notify_waiters()` to wake any tasks waiting on `wait()`.
    pub fn notifier(&self) -> std::sync::Arc<tokio::sync::Notify> {
        std::sync::Arc::clone(&self.notify)
    }
}

#[cfg(feature = "std")]
impl<P: OutputPin + Send> Doorbell for GpioDoorbell<P> {
    type Error = P::Error;
    type WaitFut<'a> = GpioWaitFut<'a, P> where Self: 'a;

    fn ring(&self, _ch: Chan) -> Result<(), Self::Error> {
        // Channel argument ignored — single-channel GPIO.
        compiler_fence(Ordering::Release);

        let mut pin = self.pin.lock().unwrap();
        pin.set_high()?;
        pin.set_low()?;

        Ok(())
    }

    fn wait<'a>(&'a self, _ch: Chan) -> Self::WaitFut<'a>
    where
        Self: 'a,
    {
        GpioWaitFut::new(self)
    }

    fn pending(&self, _ch: Chan) -> bool {
        // No hardware status register on Linux; pending() is always false.
        // The Notify and executor handle the pending/woken state implicitly.
        false
    }
}

/// Future returned by `GpioDoorbell::wait()` on A-core.
///
/// Wraps a `tokio::sync::Notify` to provide async wakeup when
/// the GPIO interrupt arrives.
#[cfg(feature = "std")]
pub struct GpioWaitFut<'a, P> {
    doorbell: &'a GpioDoorbell<P>,
    armed: bool,
    /// Lifetime-erased Notified future.
    ///
    /// # Safety
    /// The `doorbell` is borrowed for `'a`, which outlives this future.
    /// The Arc inside the doorbell keeps the Notify allocation alive.
    /// The transmute only erases the borrow lifetime; the Arc guarantees soundness.
    notified: Option<tokio::sync::futures::Notified<'static>>,
}

#[cfg(feature = "std")]
impl<'a, P> GpioWaitFut<'a, P> {
    fn new(doorbell: &'a GpioDoorbell<P>) -> Self {
        Self {
            doorbell,
            armed: false,
            notified: None,
        }
    }
}

#[cfg(feature = "std")]
impl<'a, P: OutputPin + Send> Future for GpioWaitFut<'a, P> {
    type Output = Result<Chan, P::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: GpioWaitFut has no self-referential fields before `notified`
        // is stored; after that we never move the future, so it remains valid.
        let this = unsafe { self.get_unchecked_mut() };

        if !this.armed {
            this.armed = true;
            // SAFETY: `doorbell` is borrowed for `'a` which outlives this future.
            // The Arc inside keeps the Notify allocation alive.
            let notified = this.doorbell.notify.notified();
            let notified = unsafe {
                std::mem::transmute::<
                    tokio::sync::futures::Notified<'_>,
                    tokio::sync::futures::Notified<'static>,
                >(notified)
            };
            this.notified = Some(notified);
        }

        if let Some(notified) = this.notified.as_mut() {
            // SAFETY: `notified` lives inside `this` which is pinned via the Pin<&mut Self>.
            if unsafe { Pin::new_unchecked(notified) }.poll(cx).is_ready() {
                compiler_fence(Ordering::Acquire);
                // SAFETY: 0 is a valid channel ID for single-channel GPIO.
                return Poll::Ready(Ok(unsafe { Chan::new_unchecked(0) }));
            }
        }

        Poll::Pending
    }
}
