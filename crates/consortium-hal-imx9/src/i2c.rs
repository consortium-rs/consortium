//! I2C master driver backed by an i.MX9 LPI2C instance.
//!
//! Select an instance by passing its MMIO base to [`I2c::new`] together with a
//! `&'static` [`I2cState`]. Blocking transfers ([`embedded_hal::i2c::I2c`])
//! work without interrupts. Async transfers ([`embedded_hal_async::i2c::I2c`])
//! require the caller to route the instance's interrupt to
//! [`I2cState::on_interrupt`].
//!
//! DMA readiness: the byte primitives below are factored around the LPI2C FIFO
//! so a DMA-backed transmit/receive can be layered on top using
//! [`crate::dma::DmaChannel::write_peripheral`] /
//! [`crate::dma::DmaChannel::read_peripheral`] against `MTDR`/`MRDR` without
//! changing this driver's public shape. The FIFO/interrupt path is the default
//! and only path today.

use core::future::poll_fn;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::Poll;

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
pub struct I2cState {
    base: AtomicPtr<()>,
    waker: AtomicWaker,
}

impl I2cState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            base: AtomicPtr::new(core::ptr::null_mut()),
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the LPI2C instance's interrupt.
    ///
    /// Disables every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-enables the ones it still needs on the next
    /// poll.
    pub fn on_interrupt(&self) {
        let ptr = self.base.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `I2c::new` installs the MMIO base for the selected LPI2C
        // instance before any interrupt is routed here.
        let r = unsafe { LPI2C::from_ptr(ptr) };
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

impl Default for I2cState {
    fn default() -> Self {
        Self::new()
    }
}

/// I2C master over one LPI2C instance.
pub struct I2c {
    regs: LPI2C,
    state: &'static I2cState,
}

impl I2c {
    /// Initialize an LPI2C instance as a master at `frequency` Hz.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid LPI2C register block, clocked and pinned
    /// out by the caller, that is exclusively owned by this driver for the
    /// lifetime of the program. `clock_hz` is the instance's functional clock.
    /// `state` must be dedicated to this instance; for async use, route the
    /// instance's interrupt to [`I2cState::on_interrupt`] on the same state.
    pub unsafe fn new(
        base: *mut (),
        clock_hz: u32,
        frequency: u32,
        state: &'static I2cState,
    ) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned LPI2C
        // register block.
        let r = unsafe { LPI2C::from_ptr(base) };

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

        state.base.store(base, Ordering::Release);

        Self { regs: r, state }
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
