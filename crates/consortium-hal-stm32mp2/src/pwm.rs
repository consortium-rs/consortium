//! PWM output driver backed by an STM32MP2 timer.
//!
//! Modeled on `embassy-stm32`'s `simple_pwm`, adapted to this HAL's raw-MMIO
//! convention: select a timer instance by passing its register base to
//! [`Pwm::new`] together with the instance kernel-clock frequency and the
//! desired output frequency. Like [`crate::gpio`] and [`crate::timer_driver`],
//! this driver does **not** touch the RCC or the GPIO alternate-function mux —
//! the caller must have enabled the timer's clock and routed each output pin to
//! the timer's alternate function before driving a channel.
//!
//! Every channel is configured for edge-aligned PWM mode 1 with output and
//! auto-reload preload, so [`PwmChannel::set_duty_cycle`] updates are latched
//! glitch-free at the next counter update event. General-purpose timers
//! (TIM2-5, TIM12-17) drive their outputs directly. Advanced-control timers
//! (TIM1/8/20) additionally gate their outputs behind the main-output-enable
//! bit; call [`Pwm::enable_advanced_outputs`] once for those instances.

use core::convert::Infallible;

use pac::peripherals::timer::TIM_GP;

use crate::pac;

/// Output-compare mode value selecting edge-aligned PWM mode 1.
const OCM_PWM_MODE1: u8 = 0b110;

/// The largest auto-reload value used. Capping one below `u16::MAX` keeps the
/// 100%-duty value (`ARR + 1`) inside a `u16` so it fits [`SetDutyCycle`].
const MAX_ARR: u32 = u16::MAX as u32 - 1;

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
    /// Zero-based channel index (`0..4`).
    const fn index(self) -> usize {
        match self {
            Channel::Ch1 => 0,
            Channel::Ch2 => 1,
            Channel::Ch3 => 2,
            Channel::Ch4 => 3,
        }
    }
}

/// Output polarity of a PWM channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Output is high while the channel is active (reset value).
    ActiveHigh,
    /// Output is low while the channel is active.
    ActiveLow,
}

/// Compute the `(PSC, ARR)` pair that best approximates `freq_hz` from a timer
/// clocked at `clock_hz`, for edge-aligned counting.
///
/// The prescaler is chosen as small as possible so the auto-reload value — and
/// therefore the duty-cycle resolution — is as large as the 16-bit `ARR` allows.
const fn compute_psc_arr(clock_hz: u32, freq_hz: u32) -> (u16, u16) {
    assert!(freq_hz > 0, "PWM frequency must be non-zero");
    // Total timer ticks per PWM period.
    let ticks = (clock_hz / freq_hz) as u64;
    assert!(ticks > 0, "PWM frequency exceeds the timer clock");

    // Smallest prescaler that brings `ARR + 1` within `MAX_ARR + 1`.
    let psc = ((ticks - 1) / (MAX_ARR as u64 + 1)) as u32;
    let arr = (ticks / (psc as u64 + 1)).saturating_sub(1);
    let arr = if arr > MAX_ARR as u64 {
        MAX_ARR
    } else {
        arr as u32
    };
    (psc as u16, arr as u16)
}

/// PWM driver over one timer instance and its four channels.
pub struct Pwm {
    tim: TIM_GP,
    clock_hz: u32,
    /// Auto-reload value currently programmed; 100% duty is `arr + 1`.
    arr: u16,
}

impl Pwm {
    /// Bind the timer at `base`, configure all four channels for PWM output, and
    /// start the counter at `freq_hz`.
    ///
    /// `clock_hz` is the timer instance's kernel-clock frequency. Channels start
    /// disabled at 0% duty; enable one with [`Pwm::channel`] then
    /// [`PwmChannel::enable`].
    ///
    /// # Safety
    ///
    /// `base` must point to a valid STM32MP2 timer register block that is
    /// exclusively owned by this driver. The caller must have enabled the
    /// instance clock (RCC) and routed the desired output pins to the timer's
    /// GPIO alternate function.
    pub unsafe fn new(base: *mut (), clock_hz: u32, freq_hz: u32) -> Self {
        // SAFETY: the caller guarantees `base` is a valid, exclusively owned TIM
        // register block.
        let tim = unsafe { TIM_GP::from_ptr(base) };

        tim.CR1().modify(|w| w.set_CEN(false));

        // Configure every channel for PWM mode 1 as an output with CCR preload.
        for reg in 0..2usize {
            tim.CCMR_OUTPUT(reg).modify(|w| {
                for ch in 0..2usize {
                    w.set_CCS(ch, 0);
                    w.set_OCM(ch, OCM_PWM_MODE1);
                    w.set_OCPE(ch, true);
                }
            });
        }

        // Start with all outputs disabled and 0% duty.
        tim.CCER().write_value(Default::default());
        for ch in 0..4usize {
            tim.CCR(ch).write(|w| w.set_CCR(0));
        }

        let mut pwm = Self {
            tim,
            clock_hz,
            arr: 0,
        };
        pwm.set_frequency(freq_hz);

        // Enable auto-reload preload and load the shadow registers.
        tim.CR1().modify(|w| w.set_ARPE(true));
        tim.EGR().write(|w| w.set_UG(true));
        tim.CR1().modify(|w| w.set_CEN(true));

        pwm
    }

    /// Reprogram the output frequency, preserving each channel's relative duty
    /// cycle as closely as the new resolution allows.
    pub fn set_frequency(&mut self, freq_hz: u32) {
        let (psc, arr) = compute_psc_arr(self.clock_hz, freq_hz);
        let old_arr = self.arr;

        self.tim.PSC().write(|w| w.set_PSC(psc as u32));
        self.tim.ARR().write(|w| w.set_ARR(arr as u32));

        // Rescale live compare values so duty ratios survive the ARR change.
        if old_arr != 0 {
            for ch in 0..4usize {
                let old = self.tim.CCR(ch).read().CCR();
                let scaled = (old as u64 * (arr as u64 + 1) / (old_arr as u64 + 1)) as u32;
                self.tim.CCR(ch).write(|w| w.set_CCR(scaled));
            }
        }

        self.arr = arr;
    }

    /// The duty value corresponding to 100% (fully on).
    pub fn max_duty_cycle(&self) -> u16 {
        self.arr + 1
    }

    /// Borrow a single channel for enable/disable and duty control.
    pub fn channel(&mut self, channel: Channel) -> PwmChannel<'_> {
        PwmChannel { pwm: self, channel }
    }

    /// Enable the main output for advanced-control timers (TIM1/8/20).
    ///
    /// General-purpose timers drive their outputs directly and do not need this;
    /// calling it on a GP timer writes a reserved register and should be
    /// avoided.
    pub fn enable_advanced_outputs(&mut self) {
        self.tim.BDTR().modify(|w| w.set_MOE(true));
    }
}

/// A borrowed handle to one channel of a [`Pwm`] timer.
pub struct PwmChannel<'a> {
    pwm: &'a mut Pwm,
    channel: Channel,
}

impl PwmChannel<'_> {
    /// Enable the channel output.
    pub fn enable(&mut self) {
        self.pwm
            .tim
            .CCER()
            .modify(|w| w.set_CCE(self.channel.index(), true));
    }

    /// Disable the channel output.
    pub fn disable(&mut self) {
        self.pwm
            .tim
            .CCER()
            .modify(|w| w.set_CCE(self.channel.index(), false));
    }

    /// Set the output polarity.
    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.pwm.tim.CCER().modify(|w| {
            w.set_CCP(
                self.channel.index(),
                matches!(polarity, Polarity::ActiveLow),
            )
        });
    }

    /// The duty value corresponding to 100% (fully on).
    pub fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty_cycle()
    }

    /// Set the raw compare value. A `duty` of `0` is always inactive and
    /// [`max_duty_cycle`](Self::max_duty_cycle) is always active.
    pub fn set_duty_cycle(&mut self, duty: u16) {
        let duty = duty.min(self.pwm.max_duty_cycle());
        self.pwm
            .tim
            .CCR(self.channel.index())
            .write(|w| w.set_CCR(duty as u32));
    }
}

impl embedded_hal::pwm::ErrorType for PwmChannel<'_> {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmChannel<'_> {
    fn max_duty_cycle(&self) -> u16 {
        PwmChannel::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Infallible> {
        PwmChannel::set_duty_cycle(self, duty);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_ARR, compute_psc_arr};

    fn realized_freq(clock_hz: u32, psc: u16, arr: u16) -> f64 {
        clock_hz as f64 / ((psc as f64 + 1.0) * (arr as f64 + 1.0))
    }

    #[test]
    fn low_frequency_needs_prescaler() {
        // 200 MHz / 1 kHz = 200_000 ticks — more than a 16-bit ARR can hold.
        let (psc, arr) = compute_psc_arr(200_000_000, 1_000);
        assert!(psc > 0);
        assert!(arr as u32 <= MAX_ARR);
        let f = realized_freq(200_000_000, psc, arr);
        assert!((f - 1_000.0).abs() / 1_000.0 < 0.01);
    }

    #[test]
    fn high_frequency_uses_no_prescaler_for_resolution() {
        // 64 MHz / 20 kHz = 3200 ticks fits directly, so PSC stays 0 for the
        // finest resolution.
        let (psc, arr) = compute_psc_arr(64_000_000, 20_000);
        assert_eq!(psc, 0);
        assert_eq!(arr, 3199);
    }

    #[test]
    fn arr_is_clamped_to_16_bit_range() {
        // Very low frequency: ARR must stay within the 16-bit register.
        let (_, arr) = compute_psc_arr(200_000_000, 1);
        assert!(arr as u32 <= MAX_ARR);
    }

    #[test]
    #[should_panic]
    fn rejects_zero_frequency() {
        compute_psc_arr(64_000_000, 0);
    }

    #[test]
    #[should_panic]
    fn rejects_frequency_above_clock() {
        compute_psc_arr(1_000, 2_000);
    }
}
