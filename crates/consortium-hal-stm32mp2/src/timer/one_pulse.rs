//! One-pulse timer mode.

use super::low_level::{CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter};
use super::low_level::{SlaveMode, TriggerSource};
use super::{Channel, Timer};

/// Retriggerable hardware one-shot.
pub struct OnePulse {
    timer: Timer,
}

impl OnePulse {
    /// Configure a one-shot with `delay` inactive ticks followed by `pulse`
    /// active ticks. PWM mode 2 is used so the output becomes active at the
    /// compare event and returns inactive at the update event.
    pub fn new(mut timer: Timer, prescaler: u16, channel: Channel, delay: u32, pulse: u32) -> Self {
        assert!(pulse > 0);
        timer.stop();
        timer.set_prescaler(prescaler);
        timer.set_auto_reload(delay.checked_add(pulse).expect("one-pulse period overflow") - 1);
        timer.configure_output(
            channel,
            super::low_level::OutputCompareMode::PwmMode2,
            true,
            false,
        );
        timer.set_compare(channel, delay);
        timer.set_channel_enabled(channel, true);
        timer.set_one_pulse(true);
        timer.generate_update();
        timer.clear_update_flag();
        Self { timer }
    }

    /// Optionally arm a hardware trigger input.
    pub fn set_trigger(
        &mut self,
        channel: Channel,
        selection: InputCaptureSelection,
        edge: CaptureEdge,
        filter: InputFilter,
    ) {
        self.timer
            .configure_input(channel, selection, edge, CapturePrescaler::Div1, filter);
        self.timer.set_channel_enabled(channel, true);
        let trigger = match channel {
            Channel::Ch1 => TriggerSource::Ti1Filtered,
            Channel::Ch2 => TriggerSource::Ti2Filtered,
            Channel::Ch3 | Channel::Ch4 => {
                panic!("only channels 1 and 2 can trigger one-pulse mode")
            }
        };
        self.timer.set_slave_mode(SlaveMode::Trigger, trigger);
    }

    /// Trigger the pulse in software.
    pub fn trigger(&mut self) {
        self.timer.clear_update_flag();
        self.timer.start();
    }

    /// Whether the one-shot has completed.
    pub fn is_complete(&self) -> bool {
        self.timer.update_flag()
    }

    pub fn clear_complete(&mut self) {
        self.timer.clear_update_flag();
    }

    pub fn free(self) -> Timer {
        self.timer
    }
}
