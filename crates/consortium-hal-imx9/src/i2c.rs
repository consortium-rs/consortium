//! I2C master driver backed by an i.MX9 LPI2C instance.
//!
//! Select an instance by handing its peripheral singleton to [`I2c::new`]:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let i2c = I2c::new(p.LPI2C1, 24_000_000, 400_000);
//! ```
//!
//! The singleton carries the instance's MMIO base and its interrupt state, so
//! construction is safe and cannot address the wrong block. Blocking transfers
//! ([`embedded_hal::i2c::I2c`]) work without interrupts. Async transfers
//! ([`embedded_hal_async::i2c::I2c`]) require the application to forward the
//! instance's interrupt to [`on_interrupt`] parameterized with the same
//! singleton type.
//!
//! DMA readiness: the byte primitives below are factored around the LPI2C FIFO
//! so a DMA-backed transmit/receive can be layered on top using
//! [`crate::dma::DmaChannel::write_peripheral`] /
//! [`crate::dma::DmaChannel::read_peripheral`] against `MTDR`/`MRDR` without
//! changing this driver's public shape. The FIFO/interrupt path is the default
//! and only path today.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress};
use pac::peripherals::lpi2c::{LPI2C, LPI2C_MTDR};

use crate::pac;

/// LPI2C `MTDR` command field values (master mode).
mod cmd {
    /// Transmit `DATA[7:0]`.
    pub const TRANSMIT: u8 = 0b000;
    /// Receive `DATA + 1` bytes.
    pub const RECEIVE: u8 = 0b001;
    /// Generate a STOP condition.
    pub const STOP: u8 = 0b010;
    /// Generate a (repeated) START and transmit the address byte in `DATA`.
    pub const START: u8 = 0b100;
}

/// Maximum byte count a single `RECEIVE` command can request.
const MAX_RX_RUN: usize = 256;

/// I2C transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A byte was not acknowledged by the target.
    Nack,
    /// Arbitration was lost to another master.
    ArbitrationLoss,
    /// A FIFO error (underrun/overrun) occurred.
    Fifo,
    /// The pin-low timeout expired.
    Timeout,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Error::Fifo => ErrorKind::Overrun,
            Error::Timeout => ErrorKind::Other,
        }
    }
}

/// LPI2C clock configuration: prescaler exponent and the high/low SCL counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ClockConfig {
    prescale: u8,
    clkhi: u8,
    clklo: u8,
}

/// Derive an LPI2C clock configuration from the functional clock and target
/// bus frequency.
///
/// The LPI2C prescales the functional clock by `2^prescale`; the SCL high and
/// low phases are then `CLKHI + 1` and `CLKLO + 1` prescaled cycles. Both
/// counts are six bits. This picks the smallest prescaler that keeps the counts
/// in range and splits the period evenly.
const fn clock_config(clock_hz: u32, frequency: u32) -> ClockConfig {
    assert!(frequency > 0);
    let mut prescale: u8 = 0;
    loop {
        let presc_clock = clock_hz >> prescale;
        // Full SCL period in prescaled cycles, split into high + low halves.
        let period = presc_clock / frequency;
        let half = period / 2;
        // CLKHI/CLKLO are the half-period minus one; must fit in six bits.
        if half >= 1 && half <= 64 {
            let count = (half - 1) as u8;
            return ClockConfig {
                prescale,
                clkhi: count,
                clklo: count,
            };
        }
        if prescale >= 7 {
            // Clamp to the slowest achievable configuration.
            return ClockConfig {
                prescale: 7,
                clkhi: 63,
                clklo: 63,
            };
        }
        prescale += 1;
    }
}

/// Per-instance interrupt state shared between an [`I2c`] and its IRQ handler.
///
/// One `static` per LPI2C instance is created by `impl_i2c` and reached
/// through `Instance::state`, so callers never supply or name it.
pub struct State {
    waker: AtomicWaker,
}

impl State {
    /// Create an empty state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the interrupt of the LPI2C instance at `r`.
    ///
    /// Disables every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-enables the ones it still needs on the next
    /// poll.
    fn on_interrupt(&self, r: LPI2C) {
        let msr = r.MSR().read();
        let mier = r.MIER().read();
        // Any enabled-and-asserted source: mask its enable and wake the task.
        let fire = (mier.TDIE() && msr.TDF())
            || (mier.RDIE() && msr.RDF())
            || (mier.NDIE() && msr.NDF())
            || (mier.EPIE() && msr.EPF())
            || (mier.SDIE() && msr.SDF())
            || (mier.ALIE() && msr.ALF());
        if fire {
            r.MIER().write(|w| {
                w.set_TDIE(false);
                w.set_RDIE(false);
                w.set_NDIE(false);
                w.set_EPIE(false);
                w.set_SDIE(false);
                w.set_ALIE(false);
            });
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> LPI2C;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An LPI2C instance the [`I2c`] driver can drive.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address and interrupt state stay single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_i2c {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::i2c::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::lpi2c::LPI2C {
                // SAFETY: `$base` is this instance's LPI2C base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::lpi2c::LPI2C::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::i2c::State {
                static STATE: crate::i2c::State = crate::i2c::State::new();
                &STATE
            }
        }

        impl crate::i2c::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_i2c;

/// Handle the LPI2C interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// I2C master over one LPI2C instance.
pub struct I2c {
    regs: LPI2C,
    state: &'static State,
}

impl I2c {
    /// Initialize an LPI2C instance as a master at `frequency` Hz.
    ///
    /// `clock_hz` is the instance's functional clock; the platform must leave
    /// the instance clocked and pinned out. For async transfers, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, frequency: u32) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();

        // Software reset, then configure while disabled.
        r.MCR().write(|w| w.set_RST(true));
        r.MCR().write(|w| w.set_RST(false));

        let cfg = clock_config(clock_hz, frequency);
        r.MCFGR1().modify(|w| w.set_PRESCALE(cfg.prescale));
        r.MCCR0().write(|w| {
            w.set_CLKHI(cfg.clkhi);
            w.set_CLKLO(cfg.clklo);
            // Data valid delay / setup-hold: half the high time is a safe default.
            w.set_DATAVD(cfg.clkhi / 2);
            w.set_SETHOLD(cfg.clkhi);
        });
        // Assert TX/RX FIFO flags as soon as any space/data exists.
        r.MFCR().write(|w| {
            w.set_TXWATER(0);
            w.set_RXWATER(0);
        });

        r.MCR().write(|w| w.set_MEN(true));

        Self {
            regs: r,
            state: T::state(),
        }
    }

    /// Clear the sticky master status flags before a transfer.
    fn clear_status(&self) {
        // Status flags are write-1-to-clear.
        self.regs.MSR().write(|w| {
            w.set_NDF(true);
            w.set_ALF(true);
            w.set_FEF(true);
            w.set_EPF(true);
            w.set_SDF(true);
        });
    }

    /// Translate the current error flags into an [`Error`], if any.
    fn check_error(&self) -> Result<(), Error> {
        let msr = self.regs.MSR().read();
        if msr.NDF() {
            self.regs.MSR().write(|w| w.set_NDF(true));
            return Err(Error::Nack);
        }
        if msr.ALF() {
            self.regs.MSR().write(|w| w.set_ALF(true));
            return Err(Error::ArbitrationLoss);
        }
        if msr.FEF() {
            self.regs.MSR().write(|w| w.set_FEF(true));
            return Err(Error::Fifo);
        }
        if msr.PLTF() {
            self.regs.MSR().write(|w| w.set_PLTF(true));
            return Err(Error::Timeout);
        }
        Ok(())
    }

    #[inline]
    fn push(&self, command: u8, data: u8) {
        self.regs
            .MTDR()
            .write_value(LPI2C_MTDR((u32::from(command) << 8) | u32::from(data)));
    }

    /// Block until the TX FIFO can accept another command word.
    fn wait_tx_ready(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.MSR().read().TDF() {
                return Ok(());
            }
        }
    }

    /// Block until a received byte is available, returning it.
    fn wait_rx(&self) -> Result<u8, Error> {
        loop {
            self.check_error()?;
            let rdr = self.regs.MRDR().read();
            if !rdr.RXEMPTY() {
                return Ok(rdr.DATA());
            }
        }
    }

    fn start_write(&self, address: u8) -> Result<(), Error> {
        self.wait_tx_ready()?;
        self.push(cmd::START, address << 1);
        Ok(())
    }

    fn start_read(&self, address: u8) -> Result<(), Error> {
        self.wait_tx_ready()?;
        self.push(cmd::START, (address << 1) | 1);
        Ok(())
    }

    fn write_bytes(&self, bytes: &[u8]) -> Result<(), Error> {
        for &b in bytes {
            self.wait_tx_ready()?;
            self.push(cmd::TRANSMIT, b);
        }
        Ok(())
    }

    fn read_bytes(&self, buffer: &mut [u8]) -> Result<(), Error> {
        for chunk in buffer.chunks_mut(MAX_RX_RUN) {
            self.wait_tx_ready()?;
            self.push(cmd::RECEIVE, (chunk.len() - 1) as u8);
            for slot in chunk.iter_mut() {
                *slot = self.wait_rx()?;
            }
        }
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.wait_tx_ready()?;
        self.push(cmd::STOP, 0);
        // Wait for the STOP to complete on the bus.
        loop {
            self.check_error()?;
            if self.regs.MSR().read().SDF() {
                self.regs.MSR().write(|w| w.set_SDF(true));
                return Ok(());
            }
        }
    }

    /// Run a transaction as a sequence of operations (blocking).
    fn transaction_blocking(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        self.clear_status();
        let mut prev_read: Option<bool> = None;
        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    if prev_read != Some(false) {
                        self.start_write(address)?;
                    }
                    self.write_bytes(bytes)?;
                    prev_read = Some(false);
                }
                Operation::Read(buffer) => {
                    self.start_read(address)?;
                    self.read_bytes(buffer)?;
                    prev_read = Some(true);
                }
            }
        }
        self.stop()
    }

    // --- async primitives ---------------------------------------------------

    /// Await a condition, arming the requested interrupt enables while pending.
    async fn wait_for<F>(&self, arm: fn(&LPI2C), mut ready: F) -> Result<(), Error>
    where
        F: FnMut(&Self) -> Option<Result<(), Error>>,
    {
        poll_fn(|cx| {
            if let Some(res) = ready(self) {
                return Poll::Ready(res);
            }
            self.state.waker.register(cx.waker());
            critical_section::with(|_| arm(&self.regs));
            if let Some(res) = ready(self) {
                return Poll::Ready(res);
            }
            Poll::Pending
        })
        .await
    }

    async fn wait_tx_ready_async(&self) -> Result<(), Error> {
        self.wait_for(
            |r| {
                r.MIER().modify(|w| {
                    w.set_TDIE(true);
                    w.set_NDIE(true);
                    w.set_ALIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.MSR().read().TDF() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_rx_async(&self) -> Result<u8, Error> {
        let mut byte = 0u8;
        self.wait_for(
            |r| {
                r.MIER().modify(|w| {
                    w.set_RDIE(true);
                    w.set_NDIE(true);
                    w.set_ALIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) => {
                    let rdr = s.regs.MRDR().read();
                    if rdr.RXEMPTY() {
                        None
                    } else {
                        byte = rdr.DATA();
                        Some(Ok(()))
                    }
                }
            },
        )
        .await?;
        Ok(byte)
    }

    async fn transaction_async(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        self.clear_status();
        let mut prev_read: Option<bool> = None;
        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    if prev_read != Some(false) {
                        self.wait_tx_ready_async().await?;
                        self.push(cmd::START, address << 1);
                    }
                    for &b in bytes.iter() {
                        self.wait_tx_ready_async().await?;
                        self.push(cmd::TRANSMIT, b);
                    }
                    prev_read = Some(false);
                }
                Operation::Read(buffer) => {
                    self.wait_tx_ready_async().await?;
                    self.push(cmd::START, (address << 1) | 1);
                    for chunk in buffer.chunks_mut(MAX_RX_RUN) {
                        self.wait_tx_ready_async().await?;
                        self.push(cmd::RECEIVE, (chunk.len() - 1) as u8);
                        for slot in chunk.iter_mut() {
                            *slot = self.wait_rx_async().await?;
                        }
                    }
                    prev_read = Some(true);
                }
            }
        }
        self.wait_tx_ready_async().await?;
        self.push(cmd::STOP, 0);
        Ok(())
    }
}

impl ErrorType for I2c {
    type Error = Error;
}

impl embedded_hal::i2c::I2c<SevenBitAddress> for I2c {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        self.transaction_blocking(address, operations)
    }
}

impl embedded_hal_async::i2c::I2c<SevenBitAddress> for I2c {
    async fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        self.transaction_async(address, operations).await
    }
}

#[cfg(test)]
mod tests {
    use super::clock_config;

    #[test]
    fn clock_config_standard_mode() {
        // 24 MHz functional clock, 100 kHz bus: ~120 prescaled cycles/period.
        let cfg = clock_config(24_000_000, 100_000);
        assert_eq!(cfg.prescale, 1);
        assert_eq!(cfg.clkhi, cfg.clklo);
        assert!(cfg.clkhi <= 63);
    }

    #[test]
    fn clock_config_fast_mode_fits() {
        let cfg = clock_config(80_000_000, 400_000);
        assert!(cfg.clkhi <= 63 && cfg.clklo <= 63);
    }

    #[test]
    fn clock_config_clamps_slow_clock() {
        // Unreachable target: clamps rather than looping forever.
        let cfg = clock_config(1_000_000, 10);
        assert_eq!(cfg.prescale, 7);
    }
}
