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
//! A transaction follows the `embedded-hal` contract: each run of consecutive
//! same-direction operations goes out as one segment, with a repeated START
//! only where the direction changes, and a single STOP at the end. A transfer
//! that fails, or an async one that is dropped mid-flight, flushes the FIFOs and
//! releases the bus before returning, so the next transaction starts from idle.
//!
//! A `Read` of zero bytes is rejected with [`Error::ZeroLengthTransfer`] before
//! anything goes on the bus: LPI2C has no way to address a target for reading
//! without then clocking at least one byte out of it. Zero-length writes are
//! fine and make a plain address probe.
//!
//! Every blocking wait is bounded by the driver's [`Timeout`] (see
//! [`I2c::set_timeout`]) and fails with [`Error::Timeout`] if the bus stalls.
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

use crate::hal::timeout::Timeout;
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
    /// The pin-low timeout expired, or a blocking wait ran out of its
    /// [`Timeout`] budget.
    Timeout,
    /// A `Read` operation had an empty buffer.
    ZeroLengthTransfer,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Error::Fifo => ErrorKind::Overrun,
            Error::Timeout | Error::ZeroLengthTransfer => ErrorKind::Other,
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
    /// The CCM gate the constructor opens before the first register access
    /// (see [`crate::clock`]); `None` where this core need not open one.
    const LPCG: Option<u32>;
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
            const LPCG: ::core::option::Option<u32> = crate::clock::lpcg(stringify!($instance));

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
    timeout: Timeout,
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
        // On i.MX 93 Linux closes the gate of every peripheral no kernel
        // driver claims; the first register access would BusFault without this.
        crate::clock::ungate(T::LPCG);
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
            timeout: Timeout::DEFAULT,
        }
    }

    /// Bound every blocking wait (one FIFO slot, one byte, the STOP) to
    /// `timeout` polls. Defaults to [`Timeout::DEFAULT`].
    pub fn set_timeout(&mut self, timeout: Timeout) {
        self.timeout = timeout;
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
        let mut deadline = self.timeout.start();
        loop {
            self.check_error()?;
            if self.regs.MSR().read().TDF() {
                return Ok(());
            }
            deadline.tick().map_err(|_| Error::Timeout)?;
        }
    }

    /// Block until a received byte is available, returning it.
    fn wait_rx(&self) -> Result<u8, Error> {
        let mut deadline = self.timeout.start();
        loop {
            self.check_error()?;
            let rdr = self.regs.MRDR().read();
            if !rdr.RXEMPTY() {
                return Ok(rdr.DATA());
            }
            deadline.tick().map_err(|_| Error::Timeout)?;
        }
    }

    /// Queue the commands that receive `buffer.len()` bytes, draining each
    /// byte as it arrives. A run longer than one `RECEIVE` command's reach is
    /// split across several; no START separates them.
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
        let mut deadline = self.timeout.start();
        loop {
            self.check_error()?;
            if self.regs.MSR().read().SDF() {
                self.regs.MSR().write(|w| w.set_SDF(true));
                return Ok(());
            }
            deadline.tick().map_err(|_| Error::Timeout)?;
        }
    }

    /// Run a transaction as a sequence of operations (blocking).
    ///
    /// Each run of consecutive same-direction operations is one segment: a
    /// single START, then every operation's bytes back to back.
    fn transaction_blocking(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        check_operations(operations)?;
        let guard = RecoverGuard::new(self.regs);
        self.clear_status();
        let mut start = 0;
        while start < operations.len() {
            let end = group_end(operations, start);
            let read = is_read(&operations[start]);
            self.wait_tx_ready()?;
            self.push(cmd::START, (address << 1) | u8::from(read));
            for op in &mut operations[start..end] {
                match op {
                    Operation::Write(bytes) => {
                        for &b in bytes.iter() {
                            self.wait_tx_ready()?;
                            self.push(cmd::TRANSMIT, b);
                        }
                    }
                    Operation::Read(buffer) => self.read_bytes(buffer)?,
                }
            }
            start = end;
        }
        self.stop()?;
        guard.defuse();
        Ok(())
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

    async fn stop_async(&self) -> Result<(), Error> {
        self.wait_tx_ready_async().await?;
        self.push(cmd::STOP, 0);
        self.wait_for(
            |r| {
                r.MIER().modify(|w| {
                    w.set_SDIE(true);
                    w.set_NDIE(true);
                    w.set_ALIE(true);
                });
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.MSR().read().SDF() => {
                    s.regs.MSR().write(|w| w.set_SDF(true));
                    Some(Ok(()))
                }
                Ok(()) => None,
            },
        )
        .await
    }

    async fn transaction_async(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        check_operations(operations)?;
        // Covers an error returned through `?` and a future dropped mid-transfer.
        let guard = RecoverGuard::new(self.regs);
        self.clear_status();
        let mut start = 0;
        while start < operations.len() {
            let end = group_end(operations, start);
            let read = is_read(&operations[start]);
            self.wait_tx_ready_async().await?;
            self.push(cmd::START, (address << 1) | u8::from(read));
            for op in &mut operations[start..end] {
                match op {
                    Operation::Write(bytes) => {
                        for &b in bytes.iter() {
                            self.wait_tx_ready_async().await?;
                            self.push(cmd::TRANSMIT, b);
                        }
                    }
                    Operation::Read(buffer) => {
                        for chunk in buffer.chunks_mut(MAX_RX_RUN) {
                            self.wait_tx_ready_async().await?;
                            self.push(cmd::RECEIVE, (chunk.len() - 1) as u8);
                            for slot in chunk.iter_mut() {
                                *slot = self.wait_rx_async().await?;
                            }
                        }
                    }
                }
            }
            start = end;
        }
        self.stop_async().await?;
        guard.defuse();
        Ok(())
    }
}

/// Reject a transaction LPI2C cannot express, before touching the bus.
///
/// A read segment needs at least one `RECEIVE` command: after a START with the
/// read bit the target owns SDA until the master NACKs a byte, so a read
/// address followed directly by STOP leaves the bus in an undefined state.
/// `embassy-mcxa` rejects the same case.
fn check_operations(operations: &[Operation<'_>]) -> Result<(), Error> {
    if operations
        .iter()
        .any(|op| matches!(op, Operation::Read(buffer) if buffer.is_empty()))
    {
        return Err(Error::ZeroLengthTransfer);
    }
    Ok(())
}

/// Whether `op` reads from the target.
fn is_read(op: &Operation<'_>) -> bool {
    matches!(op, Operation::Read(_))
}

/// One past the last operation of the run starting at `start` whose operations
/// all move data in the same direction.
///
/// `embedded-hal` requires such a run to go out as one segment, with no
/// repeated START between its operations.
fn group_end(operations: &[Operation<'_>], start: usize) -> usize {
    let read = is_read(&operations[start]);
    let mut end = start + 1;
    while end < operations.len() && is_read(&operations[end]) == read {
        end += 1;
    }
    end
}

/// Upper bound on status polls while waiting for a recovery STOP to reach the
/// bus. Drop cannot await or read a clock, so this is a spin count: comfortably
/// more than one byte time plus STOP at 100 kHz on a 300 MHz+ core.
const RECOVERY_SPINS: u32 = 200_000;

/// Return the controller to idle after a failed or abandoned transfer.
///
/// Follows `embassy-mcxa`'s LPI2C recovery: discard whatever commands and data
/// are still queued, clear the sticky flags (the master starts nothing new
/// while an error flag is set), and if this master still holds the bus — after
/// a NACK with an empty FIFO, or a transfer dropped mid-flight — issue a STOP
/// and give it time to complete. A final FIFO reset drops a STOP the hardware
/// had no bus to act on.
fn recover(r: LPI2C) {
    critical_section::with(|_| {
        r.MIER().write(|w| {
            w.set_TDIE(false);
            w.set_RDIE(false);
            w.set_NDIE(false);
            w.set_EPIE(false);
            w.set_SDIE(false);
            w.set_ALIE(false);
        })
    });
    let reset_fifos = || {
        r.MCR().modify(|w| {
            w.set_RTF(true);
            w.set_RRF(true);
        })
    };
    let clear_flags = || {
        r.MSR().write(|w| {
            w.set_EPF(true);
            w.set_SDF(true);
            w.set_NDF(true);
            w.set_ALF(true);
            w.set_FEF(true);
            w.set_PLTF(true);
            w.set_DMF(true);
        })
    };
    reset_fifos();
    clear_flags();
    if r.MSR().read().MBF() {
        r.MTDR().write_value(LPI2C_MTDR(u32::from(cmd::STOP) << 8));
        let mut spins = RECOVERY_SPINS;
        while spins > 0 {
            let msr = r.MSR().read();
            if msr.SDF() || !msr.MBF() {
                break;
            }
            spins -= 1;
        }
    }
    reset_fifos();
    clear_flags();
}

/// Runs [`recover`] when dropped unless [`defuse`](Self::defuse)d, so an
/// error propagated with `?` and an async transfer dropped mid-flight both
/// leave the controller idle and the bus released.
struct RecoverGuard {
    regs: LPI2C,
    armed: bool,
}

impl RecoverGuard {
    fn new(regs: LPI2C) -> Self {
        Self { regs, armed: true }
    }

    /// The transfer completed cleanly; skip recovery.
    fn defuse(mut self) {
        self.armed = false;
    }
}

impl Drop for RecoverGuard {
    fn drop(&mut self) {
        if self.armed {
            recover(self.regs);
        }
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
    use embedded_hal::i2c::Operation;

    use super::{Error, check_operations, clock_config, group_end};

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

    #[test]
    fn same_direction_operations_form_one_segment() {
        let (a, b) = ([1u8], [2u8]);
        let (mut r1, mut r2) = ([0u8; 1], [0u8; 1]);
        let ops = [
            Operation::Read(&mut r1),
            Operation::Read(&mut r2),
            Operation::Write(&a),
            Operation::Write(&b),
        ];
        assert_eq!(group_end(&ops, 0), 2);
        assert_eq!(group_end(&ops, 2), 4);
    }

    #[test]
    fn zero_length_read_is_rejected() {
        let mut empty = [0u8; 0];
        let mut one = [0u8; 1];
        assert_eq!(
            check_operations(&[Operation::Write(&[1]), Operation::Read(&mut empty)]),
            Err(Error::ZeroLengthTransfer)
        );
        assert_eq!(
            check_operations(&[Operation::Write(&[]), Operation::Read(&mut one)]),
            Ok(())
        );
    }
}
