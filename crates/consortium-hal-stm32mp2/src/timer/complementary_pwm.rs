//! Advanced-control PWM with complementary outputs, dead time, and break.

use core::convert::Infallible;

use super::low_level::{CountingMode, InputFilter, LockLevel, OutputCompareMode};
use super::pwm::compute_psc_arr_with_mode;
use super::{Channel, Polarity, Timer};

/// PWM driver for advanced-control timers (TIM1/TIM8/TIM20).
pub struct ComplementaryPwm {
    timer: Timer,
    arr: u16,
    counting_mode: CountingMode,
}

impl ComplementaryPwm {
    /// Configure channels 1-4 for PWM mode 1 and start the counter.
    ///
    /// GPIO alternate functions and break-input muxing remain application
    /// responsibilities.
    pub fn new(timer: Timer, frequency_hz: u32) -> Self {
        Self::new_with_mode(timer, frequency_hz, CountingMode::EdgeAlignedUp)
    }

    /// Configure complementary PWM with an explicit counter alignment.
    pub fn new_with_mode(mut timer: Timer, frequency_hz: u32, counting_mode: CountingMode) -> Self {
        timer.stop();
        timer.set_counting_mode(counting_mode);
        for channel in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4] {
            timer.configure_output(channel, OutputCompareMode::PwmMode1, true, false);
            timer.set_compare(channel, 0);
            timer.set_channel_enabled(channel, false);
            timer.set_complementary_enabled(channel, false);
        }
        timer.set_auto_reload_preload(true);
        let mut pwm = Self {
            timer,
            arr: 0,
            counting_mode,
        };
        pwm.set_frequency(frequency_hz);
        pwm.timer.set_main_output(true);
        pwm.timer.start();
        pwm
    }

    pub fn set_frequency(&mut self, frequency_hz: u32) {
        let (psc, arr) =
            compute_psc_arr_with_mode(self.timer.clock_hz(), frequency_hz, self.counting_mode);
        let old_period = self.arr as u64 + 1;
        self.timer.set_prescaler(psc);
        self.timer.set_auto_reload(arr as u32);
        if self.arr != 0 {
            for channel in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4] {
                let scaled = self.timer.compare(channel) as u64 * (arr as u64 + 1) / old_period;
                self.timer.set_compare(channel, scaled as u32);
            }
        }
        self.arr = arr;
        self.timer.generate_update();
    }

    /// Program the raw `DTG` encoding. This exposes the complete nonlinear
    /// hardware range without assuming a particular timer clock division.
    pub fn set_dead_time(&mut self, encoded: u8) {
        self.timer.set_dead_time(encoded);
    }

    pub fn configure_break(
        &mut self,
        enabled: bool,
        active_polarity: Polarity,
        automatic_output: bool,
    ) {
        self.timer
            .configure_break(enabled, active_polarity, automatic_output);
    }

    pub fn set_break_filter(&mut self, filter: InputFilter) {
        self.timer.set_break_filter(filter);
    }

    pub fn configure_break2(&mut self, enabled: bool, active_polarity: Polarity) {
        self.timer.configure_break2(enabled, active_polarity);
    }

    pub fn set_break2_filter(&mut self, filter: InputFilter) {
        self.timer.set_break2_filter(filter);
    }

    pub fn trigger_software_break(&mut self, break_number: u8) {
        self.timer.generate_break(break_number);
    }

    pub fn set_off_state_idle(&mut self, enabled: bool) {
        self.timer.set_off_state_idle(enabled);
    }

    pub fn set_off_state_run(&mut self, enabled: bool) {
        self.timer.set_off_state_run(enabled);
    }

    pub fn set_lock_level(&mut self, level: LockLevel) {
        self.timer.set_lock_level(level);
    }

    pub fn set_main_output(&mut self, enabled: bool) {
        self.timer.set_main_output(enabled);
    }

    pub fn max_duty_cycle(&self) -> u16 {
        self.arr + 1
    }

    pub fn channel(&mut self, channel: Channel) -> ComplementaryPwmChannel<'_> {
        ComplementaryPwmChannel { pwm: self, channel }
    }

    pub fn free(mut self) -> Timer {
        self.timer.stop();
        self.timer.set_main_output(false);
        self.timer
    }

    #[cfg(feature = "dma")]
    pub(crate) fn into_parts(self) -> (Timer, u16) {
        (self.timer, self.arr)
    }

    #[cfg(feature = "dma")]
    pub(crate) fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

/// Borrowed main/complementary output pair.
pub struct ComplementaryPwmChannel<'a> {
    pwm: &'a mut ComplementaryPwm,
    channel: Channel,
}

impl ComplementaryPwmChannel<'_> {
    pub fn enable_main(&mut self) {
        self.pwm.timer.set_channel_enabled(self.channel, true);
    }

    pub fn disable_main(&mut self) {
        self.pwm.timer.set_channel_enabled(self.channel, false);
    }

    pub fn enable_complementary(&mut self) {
        self.pwm.timer.set_complementary_enabled(self.channel, true);
    }

    pub fn disable_complementary(&mut self) {
        self.pwm
            .timer
            .set_complementary_enabled(self.channel, false);
    }

    pub fn set_polarities(&mut self, main: Polarity, complementary: Polarity) {
        self.pwm.timer.set_channel_polarity(self.channel, main);
        self.pwm
            .timer
            .set_complementary_polarity(self.channel, complementary);
    }

    pub fn set_idle_states(&mut self, main_active: bool, complementary_active: bool) {
        self.pwm
            .timer
            .set_output_idle_state(self.channel, main_active);
        self.pwm
            .timer
            .set_complementary_idle_state(self.channel, complementary_active);
    }

    pub fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty_cycle()
    }

    pub fn set_duty_cycle(&mut self, duty: u16) {
        self.pwm
            .timer
            .set_compare(self.channel, duty.min(self.pwm.max_duty_cycle()) as u32);
    }
}

impl embedded_hal::pwm::ErrorType for ComplementaryPwmChannel<'_> {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for ComplementaryPwmChannel<'_> {
    fn max_duty_cycle(&self) -> u16 {
        ComplementaryPwmChannel::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        ComplementaryPwmChannel::set_duty_cycle(self, duty);
        Ok(())
    }
}
