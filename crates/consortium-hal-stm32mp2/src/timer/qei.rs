//! Quadrature-encoder interface mode.

use super::low_level::{
    CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter, SlaveMode, TriggerSource,
};
use super::{Channel, Timer};

/// Hardware quadrature-encoder interface using channels 1 and 2.
pub struct Qei {
    timer: Timer,
}

impl Qei {
    pub fn new(mut timer: Timer, max_count: u32, filter: InputFilter) -> Self {
        timer.stop();
        timer.set_auto_reload(max_count);
        for channel in [Channel::Ch1, Channel::Ch2] {
            timer.configure_input(
                channel,
                InputCaptureSelection::Direct,
                CaptureEdge::Rising,
                CapturePrescaler::Div1,
                filter,
            );
            timer.set_channel_enabled(channel, true);
        }
        timer.set_slave_mode(SlaveMode::Encoder3, TriggerSource::Internal0);
        timer.generate_update();
        timer.start();
        Self { timer }
    }

    pub fn count(&self) -> u32 {
        self.timer.counter()
    }

    pub fn set_count(&mut self, count: u32) {
        self.timer.set_counter(count);
    }

    /// `true` while the timer is counting down.
    pub fn direction_down(&self) -> bool {
        self.timer.regs().CR1().read().0 & (1 << 4) != 0
    }

    pub fn free(self) -> Timer {
        self.timer
    }
}
