//! Output-compare timer mode.

use super::low_level::{CountingMode, OutputCompareMode};
use super::{Channel, Polarity, Timer};

/// Multi-channel output-compare driver.
pub struct OutputCompare {
    timer: Timer,
}

impl OutputCompare {
    /// Configure a stopped output-compare timer.
    pub fn new(mut timer: Timer, prescaler: u16, period: u32, mode: CountingMode) -> Self {
        timer.stop();
        timer.set_counting_mode(mode);
        timer.set_prescaler(prescaler);
        timer.set_auto_reload(period);
        timer.generate_update();
        Self { timer }
    }

    /// Configure and borrow one channel.
    pub fn channel(
        &mut self,
        channel: Channel,
        mode: OutputCompareMode,
        compare: u32,
    ) -> OutputCompareChannel<'_> {
        self.timer.configure_output(channel, mode, false, false);
        self.timer.set_compare(channel, compare);
        OutputCompareChannel {
            driver: self,
            channel,
        }
    }

    /// Start the counter.
    pub fn start(&mut self) {
        self.timer.start();
    }

    /// Stop the counter.
    pub fn stop(&mut self) {
        self.timer.stop();
    }

    /// Recover the underlying timer.
    pub fn free(self) -> Timer {
        self.timer
    }
}

/// Borrowed output-compare channel.
pub struct OutputCompareChannel<'a> {
    driver: &'a mut OutputCompare,
    channel: Channel,
}

impl OutputCompareChannel<'_> {
    pub fn enable(&mut self) {
        self.driver.timer.set_channel_enabled(self.channel, true);
    }

    pub fn disable(&mut self) {
        self.driver.timer.set_channel_enabled(self.channel, false);
    }

    pub fn set_compare(&mut self, compare: u32) {
        self.driver.timer.set_compare(self.channel, compare);
    }

    pub fn set_mode(&mut self, mode: OutputCompareMode) {
        self.driver
            .timer
            .configure_output(self.channel, mode, false, false);
    }

    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.driver
            .timer
            .set_channel_polarity(self.channel, polarity);
    }
}
