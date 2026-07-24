//! STM32MP2 general-purpose timers and their operating modes.
//!
//! A [`Timer`] owns one timer register block. Consume it with
//! one of the mode drivers, following `embassy-stm32`'s low-level timer plus
//! mode-driver layering. The mode drivers are polling/synchronous; DMA-backed
//! ring-buffered modes will be added with the HAL's DMA/IRQ ownership layer.

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

/// Exclusive ownership of one STM32MP2 timer.
pub struct Timer {
    tim: TIM_GP,
    clock_hz: u32,
    width: TimerWidth,
}

impl Timer {
    /// Bind a general-purpose or advanced-control timer register block.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid, exclusively owned STM32MP2 timer register
    /// block. The caller must enable its RCC kernel clock.
    pub unsafe fn from_raw(base: *mut (), clock_hz: u32) -> Self {
        // SAFETY: forwarded from the caller.
        unsafe { Self::from_raw_with_width(base, clock_hz, TimerWidth::Bits16) }
    }

    /// Bind a timer and specify whether its counter is 16 or 32 bits wide.
    ///
    /// # Safety
    ///
    /// The requirements are the same as [`Timer::from_raw`], and `width` must
    /// match the selected hardware timer.
    pub unsafe fn from_raw_with_width(base: *mut (), clock_hz: u32, width: TimerWidth) -> Self {
        assert!(clock_hz > 0, "timer clock must be non-zero");
        // SAFETY: upheld by the caller.
        let tim = unsafe { TIM_GP::from_ptr(base) };
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
