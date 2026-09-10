//! Independent WDOG driver for i.MX93 and i.MX95 (WDOG1–5).
//!
//! The platform must grant this core access and supply the selected watchdog
//! clock and reset routing. Clock-source meanings depend on the SoC integration.
//! [`Watchdog::new`] only takes ownership; [`Watchdog::start`] enables reset on
//! expiry, with window mode and interrupts disabled. Dropping the handle does
//! not stop the watchdog. A bootloader-enabled watchdog continues running.
//!
//! Configuration and refresh follow NXP's MCUXpresso `fsl_rtwdog` sequences.
//! A critical-section implementation that masks this core's interrupts is
//! required: the unlock/write sequence has a hardware time limit.

use crate::pac::peripherals::wdog::{WDOG, WDOG_CNT, WDOG_CS, WDOG_TOVAL, WDOG_WIN};
use embassy_hal_internal::{Peri, PeripheralType};

const UNLOCK: u32 = 0xd928_c520;
const REFRESH: u32 = 0xb480_a602;
const SYNC_POLLS: usize = 1_000_000;

/// SoC-specific watchdog clock selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    Source0 = 0,
    Source1 = 1,
    Source2 = 2,
    Source3 = 3,
}

/// Reset watchdog configuration. No window or interrupt is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Timeout in selected-clock ticks after prescaling; must be at least 3.
    pub timeout_ticks: u16,
    pub clock_source: ClockSource,
    /// Divide the selected watchdog clock by 256 instead of 1.
    pub divide_by_256: bool,
    pub run_in_wait: bool,
    pub run_in_stop: bool,
    pub run_in_debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout_ticks: u16::MAX,
            clock_source: ClockSource::Source1,
            divide_by_256: false,
            run_in_wait: true,
            run_in_stop: false,
            run_in_debug: false,
        }
    }
}

/// Failure to configure the watchdog. The handle remains available to feed it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidTimeout,
    /// A previous owner disabled register updates until reset.
    Locked,
    /// Unlock or reconfiguration did not finish within one million status reads.
    /// This is a bounded polling budget, not a wall-clock deadline. Hardware may
    /// already be running when this error is returned.
    SyncTimeout,
}

pub(crate) trait SealedInstance {
    fn regs() -> WDOG;
}

/// A chip's WDOG singleton, with its database-resolved register window.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

macro_rules! impl_watchdog {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::watchdog::SealedInstance for crate::peripherals::$instance {
            fn regs() -> crate::pac::peripherals::wdog::WDOG {
                // SAFETY: The generator supplies this WDOG's base; its singleton
                // is consumed by the driver for exclusive ownership.
                unsafe { crate::pac::peripherals::wdog::WDOG::from_ptr($base as *mut ()) }
            }
        }
        impl crate::watchdog::Instance for crate::peripherals::$instance {}
    };
}
pub(crate) use impl_watchdog;

/// Exclusive watchdog handle. Drop never disables or refreshes the hardware.
pub struct Watchdog {
    regs: WDOG,
}

impl Watchdog {
    /// Claim a watchdog without changing its current configuration.
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        let _ = peri;
        Self { regs: T::regs() }
    }

    /// Start or reconfigure the watchdog, keeping future updates enabled.
    ///
    /// Invalid configuration is rejected before any register write. Interrupts
    /// are masked through unlock, programming and the completion handshake.
    /// Do not use this to take over a peer's active watchdog.
    pub fn start(&mut self, config: Config) -> Result<(), Error> {
        if config.timeout_ticks < 3 {
            return Err(Error::InvalidTimeout);
        }
        let configured = control(config);
        critical_section::with(|_| {
            let cs = self.regs.CS().read();
            if !cs.UPDATE() {
                return Err(Error::Locked);
            }
            command(
                |word| self.regs.CNT().write_value(WDOG_CNT(word)),
                UNLOCK,
                cs.CMD32EN(),
            );
            wait_until(|| self.regs.CS().read().ULK())?;
            // Configuration must be prepared before unlocking: the unlocked
            // write window is short. These three writes follow the vendor order.
            self.regs.WIN().write_value(WDOG_WIN(0));
            self.regs
                .TOVAL()
                .write_value(WDOG_TOVAL(u32::from(config.timeout_ticks)));
            self.regs.CS().write_value(configured);
            wait_until(|| self.regs.CS().read().RCS())
        })
    }

    /// Reload an active watchdog. Also handles a bootloader's 16-bit command mode.
    /// If a previous owner enabled window mode, its refresh window still applies.
    pub fn feed(&mut self) {
        critical_section::with(|_| {
            command(
                |word| self.regs.CNT().write_value(WDOG_CNT(word)),
                REFRESH,
                self.regs.CS().read().CMD32EN(),
            );
        });
    }

    /// Whether the hardware is currently enabled.
    pub fn is_running(&self) -> bool {
        self.regs.CS().read().EN()
    }
}

fn control(config: Config) -> WDOG_CS {
    let mut cs = WDOG_CS::default();
    cs.set_EN(true);
    cs.set_UPDATE(true);
    cs.set_CMD32EN(true);
    cs.set_CLK(config.clock_source as u8);
    cs.set_PRES(config.divide_by_256);
    cs.set_WAIT(config.run_in_wait);
    cs.set_STOP(config.run_in_stop);
    cs.set_DBG(config.run_in_debug);
    cs
}

fn command(mut write: impl FnMut(u32), key: u32, command32: bool) {
    if command32 {
        write(key);
    } else {
        write(key & 0xffff);
        write(key >> 16);
    }
}

fn wait_until(mut ready: impl FnMut() -> bool) -> Result<(), Error> {
    for _ in 0..SYNC_POLLS {
        if ready() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::SyncTimeout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_modes_preserve_vendor_key_order() {
        for (key, words) in [(UNLOCK, [0xc520, 0xd928]), (REFRESH, [0xa602, 0xb480])] {
            let mut actual = std::vec::Vec::new();
            command(|word| actual.push(word), key, false);
            assert_eq!(actual, words);
            actual.clear();
            command(|word| actual.push(word), key, true);
            assert_eq!(actual, [key]);
        }
    }

    #[test]
    fn control_enables_reset_without_window_interrupt_or_test_mode() {
        let cs = control(Config {
            clock_source: ClockSource::Source3,
            divide_by_256: true,
            run_in_wait: false,
            run_in_stop: true,
            run_in_debug: true,
            ..Config::default()
        });
        assert_eq!(cs.0, 0x33a5);
        assert!(!cs.FLG()); // Never acknowledge a W1C status accidentally.
    }

    #[test]
    fn invalid_or_locked_start_leaves_registers_untouched() {
        let mut words = [0u32; 4];
        // SAFETY: The aligned backing array covers WDOG and outlives the handle.
        let regs = unsafe { WDOG::from_ptr(words.as_mut_ptr().cast()) };
        let mut watchdog = Watchdog { regs };
        assert_eq!(
            watchdog.start(Config {
                timeout_ticks: 2,
                ..Config::default()
            }),
            Err(Error::InvalidTimeout)
        );
        assert_eq!(watchdog.start(Config::default()), Err(Error::Locked));
        assert_eq!(words, [0; 4]);
    }

    #[test]
    fn synchronization_has_a_bounded_failure_path() {
        let mut reads = 0;
        assert_eq!(
            wait_until(|| {
                reads += 1;
                false
            }),
            Err(Error::SyncTimeout)
        );
        assert_eq!(reads, SYNC_POLLS);
        assert_eq!(wait_until(|| true), Ok(()));
    }
}
