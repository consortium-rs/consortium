//! Three-input Hall-sensor interface mode.

use super::low_level::{
    CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter, SlaveMode, TriggerSource,
};
use super::{Channel, Timer};

/// Hall-sensor edge timestamping using the XOR of channels 1-3.
pub struct HallSensor {
    timer: Timer,
}

impl HallSensor {
    pub fn new(mut timer: Timer, prescaler: u16, max_period: u32, filter: InputFilter) -> Self {
        timer.stop();
        timer.set_prescaler(prescaler);
        timer.set_auto_reload(max_period);
        timer.set_ti1_xor(true);
        timer.configure_input(
            Channel::Ch1,
            InputCaptureSelection::TriggerController,
            CaptureEdge::Rising,
            CapturePrescaler::Div1,
            filter,
        );
        timer.set_slave_mode(SlaveMode::Reset, TriggerSource::Ti1Filtered);
        timer.set_channel_enabled(Channel::Ch1, true);
        timer.generate_update();
        timer.clear_capture_flag(Channel::Ch1);
        timer.start();
        Self { timer }
    }

    /// Return elapsed ticks since the preceding Hall transition.
    pub fn try_period(&mut self) -> Option<u32> {
        if !self.timer.capture_flag(Channel::Ch1) {
            return None;
        }
        let period = self.timer.compare(Channel::Ch1);
        self.timer.clear_capture_flag(Channel::Ch1);
        Some(period)
    }

    pub fn free(self) -> Timer {
        self.timer
    }
}
