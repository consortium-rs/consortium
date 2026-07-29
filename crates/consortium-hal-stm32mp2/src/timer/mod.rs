//! STM32MP2 general-purpose timers and their operating modes.
//!
//! A [`Timer`] owns one timer register block, taken from its peripheral
//! singleton. Consume it with one of the mode drivers, following
//! `embassy-stm32`'s low-level timer plus mode-driver layering:
//!
//! ```ignore
//! let p = consortium_hal_stm32mp2::init();
//! let pwm = Timer::new(p.TIM3, 64_000_000).into_pwm(1_000);
//! ```
//!
//! The mode drivers are polling/synchronous; DMA-backed ring-buffered modes
//! will be added with the HAL's DMA/IRQ ownership layer.

use embassy_hal_internal::{Peri, PeripheralType};
use pac::peripherals::timer::TIM_GP;

use crate::pac;

pub mod complementary_pwm;
pub mod hall;
pub mod input_capture;
pub mod low_level;
pub mod one_pulse;
pub mod output_compare;
pub mod pwm;
pub mod pwm_input;
pub mod qei;
#[cfg(feature = "dma")]
pub mod ringbuffered;

pub use complementary_pwm::{ComplementaryPwm, ComplementaryPwmChannel};
pub use hall::HallSensor;
pub use input_capture::{Capture, InputCapture};
pub use one_pulse::OnePulse;
pub use output_compare::{OutputCompare, OutputCompareChannel};
pub use pwm::{Pwm, PwmChannel};
pub use pwm_input::{PwmInput, PwmMeasurement};
pub use qei::Qei;
#[cfg(feature = "dma")]
pub use ringbuffered::{
    RingBufferedCapture, RingBufferedPwmBurst, RingBufferedPwmChannel, TimerDmaWord,
};

/// A timer capture/compare channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Channel 1.
    Ch1,
    /// Channel 2.
    Ch2,
    /// Channel 3.
    Ch3,
    /// Channel 4.
    Ch4,
}

impl Channel {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Ch1 => 0,
            Self::Ch2 => 1,
            Self::Ch3 => 2,
            Self::Ch4 => 3,
        }
    }
}

/// Output polarity of a timer channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Output is high while the channel is active.
    ActiveHigh,
    /// Output is low while the channel is active.
    ActiveLow,
}

/// Width of a timer's counter and auto-reload register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerWidth {
    /// 16-bit timer.
    Bits16,
    /// 32-bit timer, such as TIM2 or TIM5.
    Bits32,
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> TIM_GP;
}

/// A timer the [`Timer`] driver can drive.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address and counter width stay single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
    /// Whether this timer's counter and auto-reload register are 32 bits wide.
    const WIDTH: TimerWidth;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_tim {
    ($instance:ident, $base:expr, $width:ident, $irq:expr) => {
        impl crate::timer::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::timer::TIM_GP {
                // SAFETY: `$base` is this instance's timer base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::timer::TIM_GP::from_ptr($base as *mut ()) }
            }
        }

        impl crate::timer::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
            const WIDTH: crate::timer::TimerWidth = crate::timer::TimerWidth::$width;
        }
    };
}
pub(crate) use impl_tim;

/// Exclusive ownership of one STM32MP2 timer.
pub struct Timer {
    tim: TIM_GP,
    clock_hz: u32,
    width: TimerWidth,
}

impl Timer {
    /// Bind a timer from its peripheral singleton.
    ///
    /// The counter width comes from the chip database, so 32-bit timers such as
    /// TIM2 and TIM5 are recognized without the caller declaring it. The
    /// platform must leave the instance's RCC kernel clock enabled.
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32) -> Self {
        assert!(clock_hz > 0, "timer clock must be non-zero");
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        Self {
            tim: T::regs(),
            clock_hz,
            width: T::WIDTH,
        }
    }

    /// Build a timer over an arbitrary register block, for tests that stand a
    /// plain array in for the timer.
    #[cfg(test)]
    pub(crate) fn over(tim: TIM_GP, clock_hz: u32, width: TimerWidth) -> Self {
        Self {
            tim,
            clock_hz,
            width,
        }
    }

    /// Consume this timer and configure it for PWM output.
    pub fn into_pwm(self, freq_hz: u32) -> Pwm {
        Pwm::new(self, freq_hz)
    }

    /// Consume this timer and configure explicitly aligned PWM output.
    pub fn into_pwm_with_mode(self, freq_hz: u32, counting_mode: low_level::CountingMode) -> Pwm {
        Pwm::new_with_mode(self, freq_hz, counting_mode)
    }

    /// Timer kernel-clock frequency.
    pub const fn clock_hz(&self) -> u32 {
        self.clock_hz
    }

    /// Counter width for this timer.
    pub const fn width(&self) -> TimerWidth {
        self.width
    }

    pub(crate) const fn regs(&self) -> TIM_GP {
        self.tim
    }
}
