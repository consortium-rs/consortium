//! GPIO driver backed by an STM32MP2 GPIO port instance.
//!
//! Select a port by passing its MMIO base to [`Gpio::new`], then take
//! individual [`Pin`]s with [`Gpio::pin`]. Pins implement the standard
//! [`embedded_hal::digital`] output/input traits. Pin operations are
//! infallible on this hardware.
//!
//! Edge interrupts (async [`embedded_hal_async::digital::Wait`]) are not wired
//! yet. The [`GpioState`] scaffold below is the seam a future revision will use:
//! the EXTI line handlers read the pending register and forward the pending-pin
//! mask to [`GpioState::on_interrupt`].

use core::convert::Infallible;

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::gpio::{GPIO, GPIO_BRR, GPIO_BSRR};

use crate::pac;

/// Number of pins in an STM32 GPIO port.
const PIN_COUNT: u8 = 16;

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
/// pending-pin mask. Nothing routes an interrupt here yet; the EXTI wiring and
/// the [`embedded_hal_async::digital::Wait`] impl on [`Pin`] are deliberately
/// left for a later change so this can land without an API break.
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
    /// A future EXTI interrupt handler reads the pending register, clears the
    /// serviced lines, and passes the pending-pin mask here.
    pub fn on_interrupt(&self, pending_mask: u32) {
        let mut mask = pending_mask;
        while mask != 0 {
            let pin = mask.trailing_zeros() as usize;
            if pin < PIN_COUNT as usize {
                self.wakers[pin].wake();
            }
            mask &= mask - 1;
        }
    }
}

impl Default for GpioState {
    fn default() -> Self {
        Self::new()
    }
}

/// A GPIO port. Hand out [`Pin`]s with [`Gpio::pin`].
#[derive(Clone, Copy)]
pub struct Gpio {
    port: GPIO,
}

impl Gpio {
    /// Bind a GPIO port at `base`.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid GPIO register block that is clocked by the
    /// caller and exclusively owned by this driver for the lifetime of the
    /// program.
    pub unsafe fn new(base: *mut ()) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned GPIO
        // register block.
        let port = unsafe { GPIO::from_ptr(base) };
        Self { port }
    }

    /// Take the [`Pin`] at index `pin` (`0..16`), unconfigured.
    ///
    /// # Panics
    ///
    /// Panics if `pin >= 16`.
    pub fn pin(&self, pin: u8) -> Pin {
        assert!(pin < PIN_COUNT, "GPIO pin index out of range");
        Pin {
            port: self.port,
            pin,
        }
    }
}

/// A single GPIO pin.
#[derive(Clone, Copy)]
pub struct Pin {
    port: GPIO,
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
        // MODER holds two bits per pin: 0b01 selects general-purpose output.
        let shift = u32::from(self.pin) * 2;
        self.port
            .MODER()
            .modify(|w| w.0 = (w.0 & !(0b11 << shift)) | (0b01 << shift));
        self
    }

    /// Configure the pin as a high-impedance input.
    pub fn into_input(self) -> Self {
        // MODER 0b00 selects input mode.
        let shift = u32::from(self.pin) * 2;
        self.port.MODER().modify(|w| w.0 &= !(0b11 << shift));
        self
    }

    #[inline]
    fn write_high(&self) {
        // BSRR low half is atomic bit-set; other pins are unaffected.
        self.port.BSRR().write_value(GPIO_BSRR(self.mask()));
    }

    #[inline]
    fn write_low(&self) {
        // BRR is atomic bit-reset; other pins are unaffected.
        self.port.BRR().write_value(GPIO_BRR(self.mask()));
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
        if self.is_set_high() {
            self.write_low();
        } else {
            self.write_high();
        }
    }

    /// Read back the latched output level.
    pub fn is_set_high(&self) -> bool {
        self.port.ODR().read().0 & self.mask() != 0
    }

    /// Read the input level at the pad.
    pub fn is_high(&self) -> bool {
        self.port.IDR().read().0 & self.mask() != 0
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
