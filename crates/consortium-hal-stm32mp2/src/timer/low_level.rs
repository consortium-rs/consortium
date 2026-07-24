//! Low-level STM32 TIM register control shared by the mode drivers.

use super::{Channel, Polarity, Timer, TimerWidth};

/// Counter alignment and update-event behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountingMode {
    /// Edge-aligned, up-counting.
    EdgeAlignedUp,
    /// Edge-aligned, down-counting.
    EdgeAlignedDown,
    /// Center-aligned; update on down-counting transitions.
    CenterAlignedDownInterrupts,
    /// Center-aligned; update on up-counting transitions.
    CenterAlignedUpInterrupts,
    /// Center-aligned; update on both transitions.
    CenterAlignedBothInterrupts,
}

/// Output-compare function selected for a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputCompareMode {
    Frozen = 0,
    ActiveOnMatch = 1,
    InactiveOnMatch = 2,
    Toggle = 3,
    ForceInactive = 4,
    ForceActive = 5,
    PwmMode1 = 6,
    PwmMode2 = 7,
}

/// Input signal connected to a capture channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InputCaptureSelection {
    Direct = 1,
    Indirect = 2,
    TriggerController = 3,
}

/// Edge(s) which cause a capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureEdge {
    Rising,
    Falling,
    Both,
}

/// Capture prescaler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CapturePrescaler {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

/// Digital filter setting (`ICxF`/`ETF` encoding from the reference manual).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputFilter(u8);

impl InputFilter {
    /// No input filtering.
    pub const NONE: Self = Self(0);

    /// Construct a filter from its four-bit hardware encoding.
    pub const fn new(bits: u8) -> Self {
        assert!(bits < 16, "input filter encoding must fit four bits");
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

/// Timer slave-mode controller operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SlaveMode {
    Disabled = 0,
    Encoder1 = 1,
    Encoder2 = 2,
    Encoder3 = 3,
    Reset = 4,
    Gated = 5,
    Trigger = 6,
    ExternalClock = 7,
}

/// Trigger input for the slave-mode controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TriggerSource {
    Internal0 = 0,
    Internal1 = 1,
    Internal2 = 2,
    Internal3 = 3,
    Ti1EdgeDetector = 4,
    Ti1Filtered = 5,
    Ti2Filtered = 6,
    ExternalTrigger = 7,
}

/// Timer input clock division used by digital filters and dead-time logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivision {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
}

/// Master trigger-output source (`MMS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MasterMode {
    Reset = 0,
    Enable = 1,
    Update = 2,
    ComparePulse = 3,
    Compare1 = 4,
    Compare2 = 5,
    Compare3 = 6,
    Compare4 = 7,
}

/// Advanced timer register lock level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LockLevel {
    Off = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}

impl Timer {
    /// Largest counter value supported by this timer.
    pub const fn max_counter(&self) -> u32 {
        match self.width() {
            TimerWidth::Bits16 => u16::MAX as u32,
            TimerWidth::Bits32 => u32::MAX,
        }
    }

    /// Start counting.
    pub fn start(&mut self) {
        self.regs().CR1().modify(|w| w.set_CEN(true));
    }

    /// Stop counting.
    pub fn stop(&mut self) {
        self.regs().CR1().modify(|w| w.set_CEN(false));
    }

    /// Whether the counter is enabled.
    pub fn is_running(&self) -> bool {
        self.regs().CR1().read().CEN()
    }

    /// Read the current counter.
    pub fn counter(&self) -> u32 {
        self.regs().CNT().read().CNT()
    }

    /// Set the current counter.
    pub fn set_counter(&mut self, value: u32) {
        assert!(value <= self.max_counter());
        self.regs().CNT().write(|w| w.set_CNT(value));
    }

    /// Set the prescaler divider (`divider = PSC + 1`).
    pub fn set_prescaler(&mut self, prescaler: u16) {
        self.regs().PSC().write(|w| w.set_PSC(prescaler as u32));
    }

    /// Set the auto-reload value.
    pub fn set_auto_reload(&mut self, value: u32) {
        assert!(value <= self.max_counter());
        self.regs().ARR().write(|w| w.set_ARR(value));
    }

    /// Read the auto-reload value.
    pub fn auto_reload(&self) -> u32 {
        self.regs().ARR().read().ARR()
    }

    /// Configure the counter from a desired tick frequency and period.
    ///
    /// Returns the realized tick frequency.
    pub fn configure_ticks(&mut self, tick_hz: u32, period_ticks: u32) -> u32 {
        assert!(tick_hz > 0 && tick_hz <= self.clock_hz());
        assert!(period_ticks > 0 && period_ticks - 1 <= self.max_counter());
        let divider = self.clock_hz().div_ceil(tick_hz).min(u16::MAX as u32 + 1);
        self.set_prescaler((divider - 1) as u16);
        self.set_auto_reload(period_ticks - 1);
        self.generate_update();
        self.clock_hz() / divider
    }

    /// Configure edge- or center-aligned counting.
    pub fn set_counting_mode(&mut self, mode: CountingMode) {
        let (dir, cms) = match mode {
            CountingMode::EdgeAlignedUp => (false, 0),
            CountingMode::EdgeAlignedDown => (true, 0),
            CountingMode::CenterAlignedDownInterrupts => (false, 1),
            CountingMode::CenterAlignedUpInterrupts => (false, 2),
            CountingMode::CenterAlignedBothInterrupts => (false, 3),
        };
        self.regs().CR1().modify(|w| {
            w.0 = (w.0 & !((1 << 4) | (3 << 5))) | ((dir as u32) << 4) | (cms << 5);
        });
    }

    /// Enable or disable one-pulse mode.
    pub fn set_one_pulse(&mut self, enabled: bool) {
        self.regs().CR1().modify(|w| set_bit(&mut w.0, 3, enabled));
    }

    /// Enable buffered updates of `ARR`.
    pub fn set_auto_reload_preload(&mut self, enabled: bool) {
        self.regs().CR1().modify(|w| w.set_ARPE(enabled));
    }

    /// Set the timer clock division used by filters and the dead-time unit.
    pub fn set_clock_division(&mut self, division: ClockDivision) {
        self.regs().CR1().modify(|w| {
            w.0 = (w.0 & !(3 << 8)) | ((division as u32) << 8);
        });
    }

    /// Route an internal event to the timer's master trigger output.
    pub fn set_master_mode(&mut self, mode: MasterMode) {
        self.regs().CR2().modify(|w| {
            *w = (*w & !(7 << 4)) | ((mode as u32) << 4);
        });
    }

    /// Set an advanced timer's repetition counter.
    pub fn set_repetition_counter(&mut self, repetitions: u16) {
        self.regs().RCR().write_value(repetitions as u32);
    }

    /// Generate an update event without turning it into an update request.
    pub fn generate_update(&mut self) {
        let urs = self.regs().CR1().read().0 & (1 << 2) != 0;
        self.regs().CR1().modify(|w| set_bit(&mut w.0, 2, true));
        self.regs().EGR().write(|w| w.set_UG(true));
        self.regs().CR1().modify(|w| set_bit(&mut w.0, 2, urs));
    }

    /// Configure a channel as an output compare.
    pub fn configure_output(
        &mut self,
        channel: Channel,
        mode: OutputCompareMode,
        preload: bool,
        fast: bool,
    ) {
        let index = channel.index();
        self.regs().CCMR_OUTPUT(index / 2).modify(|w| {
            let local = index % 2;
            let shift = local * 8;
            w.set_CCS(local, 0);
            set_bit(&mut w.0, shift + 2, fast);
            w.set_OCPE(local, preload);
            w.set_OCM(local, mode as u8);
        });
    }

    /// Configure a channel as an input capture.
    pub fn configure_input(
        &mut self,
        channel: Channel,
        selection: InputCaptureSelection,
        edge: CaptureEdge,
        prescaler: CapturePrescaler,
        filter: InputFilter,
    ) {
        let index = channel.index();
        self.set_channel_enabled(channel, false);
        self.regs().CCMR_OUTPUT(index / 2).modify(|w| {
            let shift = (index % 2) * 8;
            w.0 = (w.0 & !(0xff << shift))
                | (selection as u32) << shift
                | (prescaler as u32) << (shift + 2)
                | (filter.bits() as u32) << (shift + 4);
        });
        self.regs().CCER().modify(|w| {
            let shift = index * 4;
            w.0 &= !(3 << (shift + 1));
            match edge {
                CaptureEdge::Rising => {}
                CaptureEdge::Falling => w.0 |= 1 << (shift + 1),
                CaptureEdge::Both => w.0 |= 1 << (shift + 1) | 1 << (shift + 3),
            }
        });
    }

    /// Enable or disable a channel's primary output/capture.
    pub fn set_channel_enabled(&mut self, channel: Channel, enabled: bool) {
        self.regs()
            .CCER()
            .modify(|w| w.set_CCE(channel.index(), enabled));
    }

    /// Enable or disable an advanced timer's complementary output.
    pub fn set_complementary_enabled(&mut self, channel: Channel, enabled: bool) {
        self.regs()
            .CCER()
            .modify(|w| set_bit(&mut w.0, channel.index() * 4 + 2, enabled));
    }

    /// Set the primary output polarity.
    pub fn set_channel_polarity(&mut self, channel: Channel, polarity: Polarity) {
        self.regs().CCER().modify(|w| {
            w.set_CCP(channel.index(), matches!(polarity, Polarity::ActiveLow));
        });
    }

    /// Set an advanced timer's complementary output polarity.
    pub fn set_complementary_polarity(&mut self, channel: Channel, polarity: Polarity) {
        self.regs().CCER().modify(|w| {
            set_bit(
                &mut w.0,
                channel.index() * 4 + 3,
                matches!(polarity, Polarity::ActiveLow),
            );
        });
    }

    /// Read a channel's capture/compare register.
    pub fn compare(&self, channel: Channel) -> u32 {
        self.regs().CCR(channel.index()).read().CCR()
    }

    /// Write a channel's capture/compare register.
    pub fn set_compare(&mut self, channel: Channel, value: u32) {
        assert!(value <= self.max_counter());
        self.regs().CCR(channel.index()).write(|w| w.set_CCR(value));
    }

    /// Whether a capture/compare event is pending.
    pub fn capture_flag(&self, channel: Channel) -> bool {
        self.regs().SR().read().CCIF(channel.index())
    }

    /// Clear a channel's capture/compare event.
    pub fn clear_capture_flag(&mut self, channel: Channel) {
        self.regs()
            .SR()
            .modify(|w| w.set_CCIF(channel.index(), false));
    }

    /// Whether a capture overrun occurred.
    pub fn overcapture_flag(&self, channel: Channel) -> bool {
        self.regs().SR().read().0 & (1 << (channel.index() + 9)) != 0
    }

    /// Clear a capture overrun.
    pub fn clear_overcapture_flag(&mut self, channel: Channel) {
        self.regs()
            .SR()
            .modify(|w| set_bit(&mut w.0, channel.index() + 9, false));
    }

    /// Enable or disable a channel interrupt request.
    pub fn set_capture_interrupt(&mut self, channel: Channel, enabled: bool) {
        self.regs()
            .DIER()
            .modify(|w| w.set_CCIE(channel.index(), enabled));
    }

    /// Whether an update event is pending.
    pub fn update_flag(&self) -> bool {
        self.regs().SR().read().UIF()
    }

    /// Clear the update event.
    pub fn clear_update_flag(&mut self) {
        self.regs().SR().modify(|w| w.set_UIF(false));
    }

    /// Enable or disable update interrupts.
    pub fn set_update_interrupt(&mut self, enabled: bool) {
        self.regs().DIER().modify(|w| w.set_UIE(enabled));
    }

    /// Configure the slave-mode controller.
    pub fn set_slave_mode(&mut self, mode: SlaveMode, trigger: TriggerSource) {
        self.regs().SMCR().modify(|w| {
            *w = (*w & !0x77) | mode as u32 | ((trigger as u32) << 4);
        });
    }

    /// Configure the external trigger input and optional external-clock mode.
    pub fn configure_external_trigger(
        &mut self,
        filter: InputFilter,
        prescaler: CapturePrescaler,
        polarity: Polarity,
        external_clock: bool,
    ) {
        self.regs().SMCR().modify(|w| {
            *w &= !((0xf << 8) | (3 << 12) | (1 << 14) | (1 << 15));
            *w |= (filter.bits() as u32) << 8;
            *w |= (prescaler as u32) << 12;
            *w |= (external_clock as u32) << 14;
            *w |= (matches!(polarity, Polarity::ActiveLow) as u32) << 15;
        });
    }

    /// XOR channels 1-3 into the TI1 input (Hall-sensor mode).
    pub fn set_ti1_xor(&mut self, enabled: bool) {
        self.regs().CR2().modify(|w| set_bit(w, 7, enabled));
    }

    /// Enable or disable the main output gate on an advanced timer.
    pub fn set_main_output(&mut self, enabled: bool) {
        self.regs().BDTR().modify(|w| w.set_MOE(enabled));
    }

    /// Program the advanced timer's encoded dead time (`DTG`).
    pub fn set_dead_time(&mut self, encoded: u8) {
        self.regs()
            .BDTR()
            .modify(|w| w.0 = (w.0 & !0xff) | encoded as u32);
    }

    /// Set the idle level of a primary advanced-timer output.
    pub fn set_output_idle_state(&mut self, channel: Channel, active: bool) {
        self.regs()
            .CR2()
            .modify(|w| set_bit(w, 8 + channel.index() * 2, active));
    }

    /// Set the idle level of a complementary advanced-timer output.
    pub fn set_complementary_idle_state(&mut self, channel: Channel, active: bool) {
        self.regs()
            .CR2()
            .modify(|w| set_bit(w, 9 + channel.index() * 2, active));
    }

    /// Select whether idle-state levels are driven when the timer is idle.
    pub fn set_off_state_idle(&mut self, enabled: bool) {
        self.regs()
            .BDTR()
            .modify(|w| set_bit(&mut w.0, 10, enabled));
    }

    /// Select whether idle-state levels are driven while the timer is running.
    pub fn set_off_state_run(&mut self, enabled: bool) {
        self.regs()
            .BDTR()
            .modify(|w| set_bit(&mut w.0, 11, enabled));
    }

    /// Lock advanced-timer configuration until the next peripheral reset.
    pub fn set_lock_level(&mut self, level: LockLevel) {
        self.regs().BDTR().modify(|w| {
            w.0 = (w.0 & !(3 << 8)) | ((level as u32) << 8);
        });
    }

    /// Configure the primary break input of an advanced timer.
    pub fn configure_break(&mut self, enabled: bool, polarity: Polarity, automatic_output: bool) {
        self.regs().BDTR().modify(|w| {
            set_bit(&mut w.0, 12, enabled);
            set_bit(&mut w.0, 13, matches!(polarity, Polarity::ActiveHigh));
            set_bit(&mut w.0, 14, automatic_output);
        });
    }

    /// Configure the secondary break input of an advanced timer.
    pub fn configure_break2(&mut self, enabled: bool, polarity: Polarity) {
        self.regs().BDTR().modify(|w| {
            set_bit(&mut w.0, 24, enabled);
            set_bit(&mut w.0, 25, matches!(polarity, Polarity::ActiveHigh));
        });
    }

    /// Program the primary break-input digital filter.
    pub fn set_break_filter(&mut self, filter: InputFilter) {
        self.regs().BDTR().modify(|w| {
            w.0 = (w.0 & !(0xf << 16)) | ((filter.bits() as u32) << 16);
        });
    }

    /// Program the secondary break-input digital filter.
    pub fn set_break2_filter(&mut self, filter: InputFilter) {
        self.regs().BDTR().modify(|w| {
            w.0 = (w.0 & !(0xf << 20)) | ((filter.bits() as u32) << 20);
        });
    }

    /// Generate a software break event (`break = 1`, `break2 = 2`).
    pub fn generate_break(&mut self, break_number: u8) {
        let bit = match break_number {
            1 => 7,
            2 => 8,
            _ => panic!("break number must be 1 or 2"),
        };
        self.regs()
            .EGR()
            .write_value(crate::pac::peripherals::timer::TIM_EGR(1 << bit));
    }

    /// Enable an update DMA request.
    pub fn set_update_dma(&mut self, enabled: bool) {
        self.regs().DIER().modify(|w| set_bit(&mut w.0, 8, enabled));
    }

    /// Enable a channel DMA request.
    pub fn set_capture_dma(&mut self, channel: Channel, enabled: bool) {
        self.regs()
            .DIER()
            .modify(|w| set_bit(&mut w.0, channel.index() + 9, enabled));
    }

    /// Configure the hardware DMA burst base register and transfer length.
    ///
    /// Values use the reference-manual `DBA` and `DBL` encodings. This only
    /// configures TIM; a DMA channel must be owned and started separately.
    pub fn configure_dma_burst(&mut self, base: u8, transfers: u8) {
        assert!(base < 64);
        assert!((1..=32).contains(&transfers));
        self.regs()
            .DCR()
            .write_value((base as u32 & 0x3f) | (((transfers - 1) as u32 & 0x1f) << 8));
    }

    /// Consume the low-level wrapper and return the raw PAC timer.
    pub fn free(self) -> crate::pac::peripherals::timer::TIM_GP {
        self.tim
    }
}

fn set_bit(value: &mut u32, bit: usize, enabled: bool) {
    let mask = 1u32 << bit;
    *value = (*value & !mask) | ((enabled as u32) << bit);
}

#[cfg(test)]
mod tests {
    use super::{
        CaptureEdge, CapturePrescaler, InputCaptureSelection, InputFilter, OutputCompareMode,
        SlaveMode, TriggerSource,
    };
    use crate::timer::{Channel, Timer, TimerWidth};

    fn timer(registers: &mut [u32; 256]) -> Timer {
        // SAFETY: the test buffer is exclusively borrowed and large enough for
        // every register exposed by TIM_GP.
        unsafe {
            Timer::from_raw_with_width(
                registers.as_mut_ptr().cast(),
                200_000_000,
                TimerWidth::Bits16,
            )
        }
    }

    #[test]
    fn accepts_all_filter_encodings() {
        for value in 0..16 {
            assert_eq!(InputFilter::new(value).bits(), value);
        }
    }

    #[test]
    #[should_panic]
    fn rejects_invalid_filter_encoding() {
        InputFilter::new(16);
    }

    #[test]
    fn output_compare_programs_channel_pair() {
        let mut registers = [0u32; 256];
        let mut timer = timer(&mut registers);
        timer.configure_output(Channel::Ch3, OutputCompareMode::PwmMode2, true, true);
        // CCMR2: CC3S=0, OC3FE=1, OC3PE=1, OC3M=111.
        assert_eq!(registers[0x1c / 4] & 0xff, 0x7c);
    }

    #[test]
    fn both_edge_capture_programs_ccmr_and_ccer() {
        let mut registers = [0u32; 256];
        let mut timer = timer(&mut registers);
        timer.configure_input(
            Channel::Ch2,
            InputCaptureSelection::Indirect,
            CaptureEdge::Both,
            CapturePrescaler::Div4,
            InputFilter::new(9),
        );
        assert_eq!((registers[0x18 / 4] >> 8) & 0xff, 0x9a);
        assert_eq!((registers[0x20 / 4] >> 4) & 0x0a, 0x0a);
    }

    #[test]
    fn slave_mode_programs_mode_and_trigger() {
        let mut registers = [0u32; 256];
        let mut timer = timer(&mut registers);
        timer.set_slave_mode(SlaveMode::Encoder3, TriggerSource::Ti2Filtered);
        assert_eq!(registers[0x08 / 4] & 0x77, 0x63);
    }

    #[test]
    fn dma_burst_encodes_length_minus_one() {
        let mut registers = [0u32; 256];
        let mut timer = timer(&mut registers);
        timer.configure_dma_burst(13, 8);
        assert_eq!(registers[0x3dc / 4], 13 | (7 << 8));
    }
}
