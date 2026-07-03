//! GPIO driver backed by an i.MX9 RGPIO port instance.
//!
//! Select a port by passing its MMIO base to [`Gpio::new`], then take
//! individual [`Pin`]s with [`Gpio::pin`]. Pins implement the standard
//! [`embedded_hal::digital`] output/input traits. Pin operations are
//! infallible on this hardware.
//!
//! Edge interrupts (async [`embedded_hal_async::digital::Wait`]) are not wired
//! yet. The [`GpioState`] scaffold below is the seam a future revision will use:
//! the port's `GPIOn_0`/`GPIOn_1` NVIC handlers read the RGPIO `ISFR` pending
//! register and forward the pending-pin mask to [`GpioState::on_interrupt`].

use core::convert::Infallible;

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::rgpio::{RGPIO, RGPIO_PCOR, RGPIO_PSOR, RGPIO_PTOR};

use crate::pac;

/// Number of pins in an RGPIO port.
const PIN_COUNT: u8 = 32;

/// Output level of a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Logic low.
    Low,
    /// Logic high.
    High,
}

/// Per-port interrupt state shared between a [`Gpio`] port and its IRQ handler.
///
/// This is the seam for future async edge support: it is `const`-constructible
/// for a `static`, holds one waker per pin, and wakes the tasks selected by a
/// pending-pin mask. Nothing routes an interrupt here yet; the RGPIO `ISFR`
/// wiring and the [`embedded_hal_async::digital::Wait`] impl on [`Pin`] are
/// deliberately left for a later change so this can land without an API break.
pub struct GpioState {
    wakers: [AtomicWaker; PIN_COUNT as usize],
}

impl GpioState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            wakers: [const { AtomicWaker::new() }; PIN_COUNT as usize],
        }
    }

    /// Wake the tasks waiting on the pins set in `pending_mask`.
    ///
    /// A future RGPIO interrupt handler reads the port's `ISFR`, clears the
    /// serviced flags, and passes the pending-pin mask here.
    pub fn on_interrupt(&self, pending_mask: u32) {
        let mut mask = pending_mask;
        while mask != 0 {
            let pin = mask.trailing_zeros() as usize;
            self.wakers[pin].wake();
            mask &= mask - 1;
        }
    }
}

impl Default for GpioState {
    fn default() -> Self {
        Self::new()
    }
}

/// An RGPIO port. Hand out [`Pin`]s with [`Gpio::pin`].
#[derive(Clone, Copy)]
pub struct Gpio {
    port: RGPIO,
}

impl Gpio {
    /// Bind an RGPIO port at `base`.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid RGPIO register block that is clocked by the
    /// caller and exclusively owned by this driver for the lifetime of the
    /// program.
    pub unsafe fn new(base: *mut ()) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned RGPIO
        // register block.
        let port = unsafe { RGPIO::from_ptr(base) };
        Self { port }
    }

    /// Take the [`Pin`] at index `pin` (`0..32`), unconfigured.
    ///
    /// # Panics
    ///
    /// Panics if `pin >= 32`.
    pub fn pin(&self, pin: u8) -> Pin {
        assert!(pin < PIN_COUNT, "RGPIO pin index out of range");
        Pin {
            port: self.port,
            pin,
        }
    }
}

/// A single RGPIO pin.
#[derive(Clone, Copy)]
pub struct Pin {
    port: RGPIO,
    pin: u8,
}

impl Pin {
    #[inline]
    fn mask(&self) -> u32 {
        1u32 << self.pin
    }

    /// Configure the pin as a push-pull output driving `initial`.
    pub fn into_output(self, initial: Level) -> Self {
        match initial {
            Level::Low => self.write_low(),
            Level::High => self.write_high(),
        }
        // PDDR bit set = output direction.
        self.port.PDDR().modify(|w| w.0 |= self.mask());
        self
    }

    /// Configure the pin as a high-impedance input.
    pub fn into_input(self) -> Self {
        // PDDR bit clear = input direction.
        self.port.PDDR().modify(|w| w.0 &= !self.mask());
        self
    }

    #[inline]
    fn write_high(&self) {
        // PSOR is write-1-to-set; other pins are unaffected.
        self.port.PSOR().write_value(RGPIO_PSOR(self.mask()));
    }

    #[inline]
    fn write_low(&self) {
        // PCOR is write-1-to-clear; other pins are unaffected.
        self.port.PCOR().write_value(RGPIO_PCOR(self.mask()));
    }

    /// Drive the output high.
    pub fn set_high(&mut self) {
        self.write_high();
    }

    /// Drive the output low.
    pub fn set_low(&mut self) {
        self.write_low();
    }

    /// Toggle the output.
    pub fn toggle(&mut self) {
        self.port.PTOR().write_value(RGPIO_PTOR(self.mask()));
    }

    /// Read back the latched output level.
    pub fn is_set_high(&self) -> bool {
        self.port.PDOR().read().0 & self.mask() != 0
    }

    /// Read the input level at the pad.
    pub fn is_high(&self) -> bool {
        self.port.PDIR().read().0 & self.mask() != 0
    }
}

impl embedded_hal::digital::ErrorType for Pin {
    type Error = Infallible;
}

impl embedded_hal::digital::OutputPin for Pin {
    fn set_high(&mut self) -> Result<(), Infallible> {
        Pin::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Infallible> {
        Pin::set_low(self);
        Ok(())
    }
}

impl embedded_hal::digital::StatefulOutputPin for Pin {
    fn is_set_high(&mut self) -> Result<bool, Infallible> {
        Ok(Pin::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Infallible> {
        Ok(!Pin::is_set_high(self))
    }

    fn toggle(&mut self) -> Result<(), Infallible> {
        Pin::toggle(self);
        Ok(())
    }
}

impl embedded_hal::digital::InputPin for Pin {
    fn is_high(&mut self) -> Result<bool, Infallible> {
        Ok(Pin::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Infallible> {
        Ok(!Pin::is_high(self))
    }
}
