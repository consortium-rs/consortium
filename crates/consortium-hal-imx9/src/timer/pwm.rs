//! PWM output mode for an i.MX 9 [`Timer`].
//!
//! This follows the shape of the STM32MP2 PWM driver: the caller supplies the
//! timer and desired output frequency. IOMUX routing remains the caller's
//! responsibility.

use core::convert::Infallible;

use super::{Channel, Polarity, Timer};

const MAX_MOD: u32 = u16::MAX as u32 - 1;

/// Select the smallest power-of-two prescaler that keeps the modulo value in
/// the 16-bit duty-cycle range.
const fn compute_prescale_mod(clock_hz: u32, freq_hz: u32) -> (u8, u16) {
    assert!(freq_hz > 0, "PWM frequency must be non-zero");
    let ticks = clock_hz / freq_hz;
    assert!(ticks > 0, "PWM frequency exceeds the TPM clock");

    let mut ps = 0u8;
    while ps < 7 && ticks.div_ceil(1u32 << ps) > MAX_MOD + 1 {
        ps += 1;
    }
    let period = ticks / (1u32 << ps);
    assert!(
        period > 0 && period <= MAX_MOD + 1,
        "PWM frequency is below the TPM range"
    );
    (ps, (period - 1) as u16)
}

/// PWM driver over one TPM instance and its four channels.
pub struct Pwm {
    pub(crate) timer: Timer,
    modulo: u16,
}

impl Pwm {
    /// Configure a timer in edge-aligned PWM mode.
    ///
    /// All channels begin disabled with 0% duty.
    pub fn new(timer: Timer, freq_hz: u32) -> Self {
        let tpm = timer.regs();
        tpm.SC().modify(|w| w.set_CMOD(0));
        tpm.CNT().write(|w| w.set_COUNT(0));

        for channel in 0..4 {
            tpm.CONTROLS_CnSC(channel).write_value(Default::default());
            tpm.CONTROLS_CnV(channel).write(|w| w.set_VAL(0));
        }

        let mut pwm = Self { timer, modulo: 0 };
        pwm.set_frequency(freq_hz);
        pwm.timer.regs().SC().modify(|w| w.set_CMOD(1));
        pwm
    }

    /// Change the output frequency while preserving channel duty ratios.
    pub fn set_frequency(&mut self, freq_hz: u32) {
        let (prescale, modulo) = compute_prescale_mod(self.timer.clock_hz(), freq_hz);
        let old_modulo = self.modulo;
        let tpm = self.timer.regs();

        tpm.SC().modify(|w| w.set_CMOD(0));
        tpm.SC().modify(|w| w.set_PS(prescale));
        tpm.MOD().write(|w| w.set_MOD(modulo as u32));

        if old_modulo != 0 {
            for channel in 0..4 {
                let old = tpm.CONTROLS_CnV(channel).read().VAL();
                let scaled = old as u64 * (modulo as u64 + 1) / (old_modulo as u64 + 1);
                tpm.CONTROLS_CnV(channel)
                    .write(|w| w.set_VAL(scaled as u32));
            }
        }

        self.modulo = modulo;
        tpm.CNT().write(|w| w.set_COUNT(0));
        tpm.SC().modify(|w| w.set_CMOD(1));
    }

    /// The duty value corresponding to 100%.
    pub fn max_duty_cycle(&self) -> u16 {
        self.modulo + 1
    }

    /// Borrow one output channel.
    pub fn channel(&mut self, channel: Channel) -> PwmChannel<'_> {
        PwmChannel { pwm: self, channel }
    }
}

/// A borrowed TPM PWM channel.
pub struct PwmChannel<'a> {
    pwm: &'a mut Pwm,
    channel: Channel,
}

impl PwmChannel<'_> {
    /// Enable edge-aligned high-true pulses on this channel.
    pub fn enable(&mut self) {
        self.pwm
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| {
                w.set_MSA(false);
                w.set_MSB(true);
                w.set_ELSA(false);
                w.set_ELSB(true);
            });
    }

    /// Disable this channel's output.
    pub fn disable(&mut self) {
        self.pwm
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| {
                w.set_ELSA(false);
                w.set_ELSB(false);
            });
    }

    /// Set channel polarity using TPM's output-polarity register.
    pub fn set_polarity(&mut self, polarity: Polarity) {
        let mask = 1u32 << self.channel.index();
        let active_low = matches!(polarity, Polarity::ActiveLow);
        self.pwm.timer.regs().POL().modify(|w| {
            if active_low {
                w.0 |= mask;
            } else {
                w.0 &= !mask;
            }
        });
    }

    /// The duty value corresponding to 100%.
    pub fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty_cycle()
    }

    /// Set a raw duty-cycle value, clamped to 100%.
    pub fn set_duty_cycle(&mut self, duty: u16) {
        let duty = duty.min(self.pwm.max_duty_cycle());
        self.pwm
            .timer
            .regs()
            .CONTROLS_CnV(self.channel.index())
            .write(|w| w.set_VAL(duty as u32));
    }
}

impl embedded_hal::pwm::ErrorType for PwmChannel<'_> {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmChannel<'_> {
    fn max_duty_cycle(&self) -> u16 {
        PwmChannel::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        PwmChannel::set_duty_cycle(self, duty);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::compute_prescale_mod;

    #[test]
    fn selects_smallest_prescaler() {
        assert_eq!(compute_prescale_mod(24_000_000, 20_000), (0, 1199));
        let (ps, modulo) = compute_prescale_mod(24_000_000, 100);
        assert_eq!(ps, 2);
        assert_eq!(modulo, 59_999);
    }

    #[test]
    #[should_panic]
    fn rejects_unachievable_low_frequency() {
        compute_prescale_mod(24_000_000, 1);
    }
}
