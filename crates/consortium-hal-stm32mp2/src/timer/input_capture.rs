//! Polling input-capture timer mode.

use super::low_level::{CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter};
use super::{Channel, Timer};

/// One captured counter value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capture {
    pub value: u32,
    /// A newer edge arrived before the preceding value was consumed.
    pub overcaptured: bool,
}

/// Multi-channel polling input-capture driver.
pub struct InputCapture {
    timer: Timer,
}

impl InputCapture {
    pub fn new(mut timer: Timer, prescaler: u16, max_period: u32) -> Self {
        timer.stop();
        timer.set_prescaler(prescaler);
        timer.set_auto_reload(max_period);
        timer.generate_update();
        Self { timer }
    }

    pub fn configure_channel(
        &mut self,
        channel: Channel,
        selection: InputCaptureSelection,
        edge: CaptureEdge,
        prescaler: CapturePrescaler,
        filter: InputFilter,
    ) {
        self.timer
            .configure_input(channel, selection, edge, prescaler, filter);
        self.timer.clear_capture_flag(channel);
        self.timer.clear_overcapture_flag(channel);
        self.timer.set_channel_enabled(channel, true);
    }

    /// Return a pending capture, or `None` if no edge has arrived.
    pub fn try_capture(&mut self, channel: Channel) -> Option<Capture> {
        if !self.timer.capture_flag(channel) {
            return None;
        }
        let capture = Capture {
            value: self.timer.compare(channel),
            overcaptured: self.timer.overcapture_flag(channel),
        };
        self.timer.clear_capture_flag(channel);
        self.timer.clear_overcapture_flag(channel);
        Some(capture)
    }

    pub fn start(&mut self) {
        self.timer.start();
    }

    pub fn stop(&mut self) {
        self.timer.stop();
    }

    pub fn free(self) -> Timer {
        self.timer
    }

    #[cfg(feature = "dma")]
    pub(crate) fn timer_mut(&mut self) -> &mut Timer {
        &mut self.timer
    }

    #[cfg(feature = "dma")]
    pub(crate) fn into_timer(self) -> Timer {
        self.timer
    }
}
