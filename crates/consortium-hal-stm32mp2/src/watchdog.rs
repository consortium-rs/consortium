//! Independent watchdog (IWDG) for STM32MP21 and STM32MP25.
//!
//! The platform must grant non-secure access and enable the peripheral bus clock.
//! Starting the watchdog enables its LSI clock in hardware. It cannot be stopped
//! until reset, including when this handle is dropped. Hardware-started watchdogs
//! continue running when claimed with [`Watchdog::new`].
//!
//! This driver follows STM32CubeMP2's IWDG startup sequence, disables the window
//! and early-warning interrupt, and requires no interrupt handler. WWDG is a
//! separate peripheral and is not handled by this module.

use crate::pac::peripherals::iwdg::IWDG;
use embassy_hal_internal::{Peri, PeripheralType};

const START: u16 = 0xcccc;
const WRITE_ACCESS: u16 = 0x5555;
const RELOAD: u16 = 0xaaaa;
const SYNC_POLLS: usize = 1_000_000;

/// Divider applied to the LSI clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    Divide4 = 0,
    Divide8 = 1,
    Divide16 = 2,
    Divide32 = 3,
    Divide64 = 4,
    Divide128 = 5,
    Divide256 = 6,
    Divide512 = 7,
    Divide1024 = 8,
}

/// Independent watchdog timeout. Window and early-warning modes are disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub prescaler: Prescaler,
    /// Twelve-bit reload value (`0..=4095`). Nominal timeout is
    /// `(reload + 1) * prescaler / LSI_Hz` seconds; account for LSI tolerance.
    pub reload: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler: Prescaler::Divide256,
            reload: 4095,
        }
    }
}

/// Startup failure. The watchdog may already be active and still needs feeding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidReload,
    /// Register synchronization exceeded one million status reads. This is a
    /// polling budget, not a wall-clock deadline; the handle remains usable.
    SyncTimeout,
}

pub(crate) trait SealedInstance {
    fn regs() -> IWDG;
}

/// An IWDG singleton with its database-resolved non-secure register window.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {}

macro_rules! impl_watchdog {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::watchdog::SealedInstance for crate::peripherals::$instance {
            fn regs() -> crate::pac::peripherals::iwdg::IWDG {
                // SAFETY: The generator supplies this IWDG's non-secure base;
                // the singleton is consumed for exclusive ownership.
                unsafe { crate::pac::peripherals::iwdg::IWDG::from_ptr($base as *mut ()) }
            }
        }
        impl crate::watchdog::Instance for crate::peripherals::$instance {}
    };
}
pub(crate) use impl_watchdog;

/// Exclusive IWDG handle. Drop never stops or refreshes the watchdog.
pub struct Watchdog {
    regs: IWDG,
}

impl Watchdog {
    /// Claim an IWDG without modifying it or starting a stopped watchdog.
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        let _ = peri;
        Self { regs: T::regs() }
    }

    /// Configure and start reset-on-expiry operation, then reload the counter.
    ///
    /// Rejects an invalid reload before starting hardware. On synchronization
    /// failure the watchdog remains active; the caller retains this handle.
    /// Do not use this to take over a peer's watchdog.
    pub fn start(&mut self, config: Config) -> Result<(), Error> {
        if config.reload > 4095 {
            return Err(Error::InvalidReload);
        }
        self.key(START);
        self.key(WRITE_ACCESS);
        // Wait for any updates left by a previous owner before writing again.
        self.wait_ready()?;
        self.regs.PR().write(|w| w.set_PR(config.prescaler as u8));
        self.regs.RLR().write(|w| w.set_RL(config.reload));
        self.wait_ready()?;
        self.regs.ICR().write(|w| w.set_EWIC(true));
        self.regs.EWCR().write_value(Default::default());
        self.wait_ready()?;
        // WINR writes also reload the counter. Do not immediately feed again:
        // the new window value may still be crossing into the LSI domain.
        if self.regs.WINR().read().WIN() != 4095 {
            self.regs.WINR().write(|w| w.set_WIN(4095));
            self.wait_ready()?;
        } else {
            self.feed();
        }
        // Close configuration writes. Refresh/start keys remain accepted.
        self.key(0);
        Ok(())
    }

    /// Reload an active IWDG. A previous owner's window restrictions apply until
    /// [`Self::start`] has successfully disabled window mode.
    pub fn feed(&mut self) {
        self.key(RELOAD);
    }

    /// Whether hardware reports the watchdog enabled.
    pub fn is_running(&self) -> bool {
        self.regs.SR().read().ONF()
    }

    fn key(&self, key: u16) {
        self.regs.KR().write(|w| w.set_KEY(key));
    }

    fn wait_ready(&self) -> Result<(), Error> {
        for _ in 0..SYNC_POLLS {
            let status = self.regs.SR().read();
            if !status.PVU() && !status.RVU() && !status.WVU() && !status.EWU() {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::SyncTimeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configures_maximum_divider_and_disables_window_and_interrupt() {
        let mut words = [0u32; 7];
        // SAFETY: This aligned array covers all registers accessed by the driver
        // and outlives the handle. Zero SR models completed synchronization.
        let regs = unsafe { IWDG::from_ptr(words.as_mut_ptr().cast()) };
        let mut watchdog = Watchdog { regs };
        watchdog
            .start(Config {
                prescaler: Prescaler::Divide1024,
                reload: 1234,
            })
            .unwrap();
        assert_eq!(words[1], 8);
        assert_eq!(words[2], 1234);
        assert_eq!(words[4], 4095);
        assert_eq!(words[5], 0);
        assert_eq!(words[6], 1 << 15);
        assert_eq!(words[0], 0); // Configuration access locked again.
        watchdog.feed();
        assert_eq!(words[0], 0xaaaa);
    }

    #[test]
    fn rejects_out_of_range_reload_without_starting() {
        let mut words = [0u32; 7];
        // SAFETY: The aligned array covers IWDG's accessed registers and lives
        // longer than the handle.
        let regs = unsafe { IWDG::from_ptr(words.as_mut_ptr().cast()) };
        let mut watchdog = Watchdog { regs };
        assert_eq!(
            watchdog.start(Config {
                reload: 4096,
                ..Config::default()
            }),
            Err(Error::InvalidReload)
        );
        assert_eq!(words, [0; 7]);
    }

    #[test]
    fn stuck_synchronization_returns_handle_that_can_still_feed() {
        let mut words = [0u32; 7];
        words[3] = 1 | (1 << 8); // PVU stuck, ONF set.
        // SAFETY: The aligned array covers IWDG's accessed registers and lives
        // longer than the handle.
        let regs = unsafe { IWDG::from_ptr(words.as_mut_ptr().cast()) };
        let mut watchdog = Watchdog { regs };
        assert_eq!(watchdog.start(Config::default()), Err(Error::SyncTimeout));
        assert!(watchdog.is_running());
        watchdog.feed();
        assert_eq!(words[0], 0xaaaa);
    }
}
