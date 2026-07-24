//! PWM-input measurement mode.

use super::low_level::{
    CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter, SlaveMode, TriggerSource,
};
use super::{Channel, Timer};

/// One PWM period/high-time measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PwmMeasurement {
    pub period_ticks: u32,
    pub high_ticks: u32,
}

impl PwmMeasurement {
    /// Duty cycle scaled to 0..=65535.
    pub fn duty_u16(self) -> u16 {
        if self.period_ticks == 0 {
            0
        } else {
            ((self.high_ticks as u64 * u16::MAX as u64) / self.period_ticks as u64)
                .min(u16::MAX as u64) as u16
        }
    }
}

/// PWM period and high-time capture using channels 1 and 2.
pub struct PwmInput {
    timer: Timer,
}

impl PwmInput {
    pub fn new(mut timer: Timer, prescaler: u16, max_period: u32, filter: InputFilter) -> Self {
        timer.stop();
        timer.set_prescaler(prescaler);
        timer.set_auto_reload(max_period);
        timer.configure_input(
            Channel::Ch1,
            InputCaptureSelection::Direct,
            CaptureEdge::Rising,
            CapturePrescaler::Div1,
            filter,
        );
        timer.configure_input(
            Channel::Ch2,
            InputCaptureSelection::Indirect,
            CaptureEdge::Falling,
            CapturePrescaler::Div1,
            filter,
        );
        timer.set_channel_enabled(Channel::Ch1, true);
        timer.set_channel_enabled(Channel::Ch2, true);
        timer.set_slave_mode(SlaveMode::Reset, TriggerSource::Ti1Filtered);
        timer.generate_update();
        timer.clear_capture_flag(Channel::Ch1);
        timer.start();
        Self { timer }
    }

    pub fn try_measure(&mut self) -> Option<PwmMeasurement> {
        if !self.timer.capture_flag(Channel::Ch1) {
            return None;
        }
        let measurement = PwmMeasurement {
            period_ticks: self.timer.compare(Channel::Ch1),
            high_ticks: self.timer.compare(Channel::Ch2),
        };
        self.timer.clear_capture_flag(Channel::Ch1);
        Some(measurement)
    }

    pub fn free(self) -> Timer {
        self.timer
    }
}
