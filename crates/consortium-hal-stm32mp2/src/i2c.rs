//! I2C master driver backed by an STM32MP2 I2C (v2) instance.
//!
//! Select an instance by passing its MMIO base to [`I2c::new`] together with a
//! `&'static` [`State`]. Blocking transfers ([`embedded_hal::i2c::I2c`])
//! work without interrupts. Async transfers ([`embedded_hal_async::i2c::I2c`])
//! require the caller to route the instance's interrupt to
//! `State::on_interrupt`.
//!
//! DMA readiness: the byte primitives below drive `TXDR`/`RXDR` one byte at a
//! time. A DMA-backed path can be layered on top by setting `CR1.TXDMAEN` /
//! `CR1.RXDMAEN` and streaming through
//! [`crate::dma::DmaChannel::write_peripheral`] /
//! [`crate::dma::DmaChannel::read_peripheral`] without changing this driver's
//! public shape. The interrupt/polling path is the default and only path today.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal::i2c::{ErrorKind, ErrorType, NoAcknowledgeSource, Operation, SevenBitAddress};
use pac::peripherals::i2c::{I2C, I2C_TXDR};

use crate::pac;

/// `CR2.RD_WRN` (bit 10) selects a read transfer; the PAC does not name it.
const CR2_RD_WRN: u32 = 1 << 10;

/// Maximum byte count a single transfer setup can request (`NBYTES` is 8-bit).
const MAX_NBYTES: usize = 255;

/// I2C transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A byte was not acknowledged by the target.
    Nack,
    /// Arbitration was lost to another master.
    ArbitrationLoss,
    /// A bus error (misplaced START/STOP) was detected.
    Bus,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Error::Bus => ErrorKind::Bus,
        }
    }
}

/// I2C `TIMINGR` field values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Timing {
    presc: u8,
    scll: u8,
    sclh: u8,
    scldel: u8,
    sdadel: u8,
}

/// Derive `TIMINGR` fields from the kernel clock and target bus frequency.
///
/// The I2C prescales the kernel clock by `PRESC + 1`; SCL low and high phases
/// are `SCLL + 1` and `SCLH + 1` prescaled cycles. This picks the smallest
/// prescaler that keeps the phase counts in the 8-bit range and splits the
/// period evenly, with conservative data setup/hold delays. It is an
/// approximation suitable for standard/fast-mode bring-up, not a rise-time
/// calibrated table.
const fn timing(clock_hz: u32, frequency: u32) -> Timing {
    assert!(frequency > 0);
    let period = clock_hz / frequency;
    let mut presc: u32 = 0;
    while presc < 15 && period / (presc + 1) > 512 {
        presc += 1;
    }
    let ticks = period / (presc + 1);
    let half = ticks / 2;
    let count = if half >= 1 {
        let c = half - 1;
        if c > 255 { 255 } else { c as u8 }
    } else {
        0
    };
    Timing {
        presc: presc as u8,
        scll: count,
        sclh: count,
        // Conservative fixed delays: a few prescaled cycles each.
        scldel: 4,
        sdadel: 2,
    }
}

/// Per-instance interrupt state shared between an [`I2c`] and its IRQ handler.
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

    /// Handle the I2C instance's interrupt.
    ///
    /// Disables every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-enables the ones it still needs on the next
    /// poll.
    fn on_interrupt(&self, r: I2C) {
        let isr = r.ISR().read();
        let cr1 = r.CR1().read();
        let fire = (cr1.TXIE() && isr.TXIS())
            || (cr1.RXIE() && isr.RXNE())
            || (cr1.TCIE() && (isr.TC() || isr.TCR()))
            || (cr1.STOPIE() && isr.STOPF())
            || (cr1.NACKIE() && isr.NACKF())
            || (cr1.ERRIE() && (isr.ARLO() || isr.BERR()));
        if fire {
            r.CR1().modify(|w| {
                w.set_TXIE(false);
                w.set_RXIE(false);
                w.set_TCIE(false);
                w.set_STOPIE(false);
                w.set_NACKIE(false);
                w.set_ERRIE(false);
            });
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> I2C;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An I2C instance the [`I2c`] driver can drive.
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
            fn regs() -> crate::pac::peripherals::i2c::I2C {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::i2c::I2C::from_ptr($base as *mut ()) }
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

/// Handle the interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// I2C master over one I2C instance.
pub struct I2c {
    regs: I2C,
    state: &'static State,
}

impl I2c {
    /// Initialize an I2C instance as a master at `frequency` Hz.
    ///
    /// `clock_hz` is the instance's kernel clock; the platform must leave the
    /// instance clocked and pinned out. For async transfers, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, frequency: u32) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();

        // Configure while the peripheral is disabled.
        r.CR1().modify(|w| w.set_PE(false));
        let t = timing(clock_hz, frequency);
        r.TIMINGR().write(|w| {
            w.set_PRESC(t.presc);
            w.set_SCLL(t.scll);
            w.set_SCLH(t.sclh);
            w.set_SCLDEL(t.scldel);
            w.set_SDADEL(t.sdadel);
        });
        r.CR1().modify(|w| w.set_PE(true));

        Self {
            regs: r,
            state: T::state(),
        }
    }

    /// Translate the current error flags into an [`Error`], clearing them.
    fn check_error(&self) -> Result<(), Error> {
        let isr = self.regs.ISR().read();
        if isr.NACKF() {
            self.regs.ICR().write(|w| w.set_NACKCF(true));
            return Err(Error::Nack);
        }
        if isr.ARLO() {
            self.regs.ICR().write(|w| w.set_ARLOCF(true));
            return Err(Error::ArbitrationLoss);
        }
        if isr.BERR() {
            self.regs.ICR().write(|w| w.set_BERRCF(true));
            return Err(Error::Bus);
        }
        Ok(())
    }

    /// Program `CR2` to start (or reload) a transfer chunk.
    fn setup(&self, address: u8, read: bool, nbytes: u8, reload: bool, start: bool) {
        self.regs.CR2().write_value({
            let mut cr2 = pac::peripherals::i2c::I2C_CR2(0);
            cr2.set_SADD(u16::from(address) << 1);
            cr2.set_NBYTES(nbytes);
            cr2.set_RELOAD(reload);
            cr2.set_AUTOEND(false);
            cr2.set_START(start);
            if read {
                cr2.0 |= CR2_RD_WRN;
            }
            cr2
        });
    }

    /// Number of chunks and the final chunk length for `len` bytes.
    fn transfer_bytes(
        &self,
        address: u8,
        read: bool,
        data_len: usize,
        start: bool,
        mut byte: impl FnMut(&Self) -> Result<(), Error>,
        wait_tcr: impl Fn(&Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut remaining = data_len;
        let mut first = true;
        loop {
            let chunk = remaining.min(MAX_NBYTES);
            let reload = remaining > MAX_NBYTES;
            if first {
                self.setup(address, read, chunk as u8, reload, start);
            } else {
                wait_tcr(self)?;
                self.regs.CR2().modify(|w| {
                    w.set_NBYTES(chunk as u8);
                    w.set_RELOAD(reload);
                });
            }
            for _ in 0..chunk {
                byte(self)?;
            }
            remaining -= chunk;
            first = false;
            if remaining == 0 {
                break;
            }
        }
        Ok(())
    }

    // --- blocking primitives ------------------------------------------------

    fn wait_txis(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.ISR().read().TXIS() {
                return Ok(());
            }
        }
    }

    fn wait_rxne(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.ISR().read().RXNE() {
                return Ok(());
            }
        }
    }

    fn wait_tcr(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.ISR().read().TCR() {
                return Ok(());
            }
        }
    }

    fn stop(&self) -> Result<(), Error> {
        self.regs.CR2().modify(|w| w.set_STOP(true));
        loop {
            self.check_error()?;
            if self.regs.ISR().read().STOPF() {
                self.regs.ICR().write(|w| w.set_STOPCF(true));
                return Ok(());
            }
        }
    }

    fn transaction_blocking(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    let mut iter = bytes.iter();
                    self.transfer_bytes(
                        address,
                        false,
                        bytes.len(),
                        true,
                        |s| {
                            s.wait_txis()?;
                            let b = *iter.next().unwrap();
                            s.regs.TXDR().write_value(I2C_TXDR(u32::from(b)));
                            Ok(())
                        },
                        |s| s.wait_tcr(),
                    )?;
                }
                Operation::Read(buffer) => {
                    let len = buffer.len();
                    let mut iter = buffer.iter_mut();
                    self.transfer_bytes(
                        address,
                        true,
                        len,
                        true,
                        |s| {
                            s.wait_rxne()?;
                            *iter.next().unwrap() = s.regs.RXDR().read().RXDATA();
                            Ok(())
                        },
                        |s| s.wait_tcr(),
                    )?;
                }
            }
        }
        self.stop()
    }

    // --- async primitives ---------------------------------------------------

    async fn wait_flag(
        &self,
        arm: fn(&I2C),
        ready: impl Fn(&Self) -> Option<Result<(), Error>>,
    ) -> Result<(), Error> {
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

    fn arm_common(r: &I2C) {
        r.CR1().modify(|w| {
            w.set_NACKIE(true);
            w.set_ERRIE(true);
        });
    }

    async fn wait_txis_async(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_common(r);
                r.CR1().modify(|w| w.set_TXIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.ISR().read().TXIS() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_rxne_async(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_common(r);
                r.CR1().modify(|w| w.set_RXIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.ISR().read().RXNE() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_tcr_async(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_common(r);
                r.CR1().modify(|w| w.set_TCIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.ISR().read().TCR() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn stop_async(&self) -> Result<(), Error> {
        self.regs.CR2().modify(|w| w.set_STOP(true));
        self.wait_flag(
            |r| {
                Self::arm_common(r);
                r.CR1().modify(|w| w.set_STOPIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.ISR().read().STOPF() => {
                    s.regs.ICR().write(|w| w.set_STOPCF(true));
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
        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    let mut remaining = bytes.len();
                    let mut first = true;
                    let mut iter = bytes.iter();
                    loop {
                        let chunk = remaining.min(MAX_NBYTES);
                        let reload = remaining > MAX_NBYTES;
                        if first {
                            self.setup(address, false, chunk as u8, reload, true);
                        } else {
                            self.wait_tcr_async().await?;
                            self.regs.CR2().modify(|w| {
                                w.set_NBYTES(chunk as u8);
                                w.set_RELOAD(reload);
                            });
                        }
                        for _ in 0..chunk {
                            self.wait_txis_async().await?;
                            let b = *iter.next().unwrap();
                            self.regs.TXDR().write_value(I2C_TXDR(u32::from(b)));
                        }
                        remaining -= chunk;
                        first = false;
                        if remaining == 0 {
                            break;
                        }
                    }
                }
                Operation::Read(buffer) => {
                    let mut remaining = buffer.len();
                    let mut first = true;
                    let mut iter = buffer.iter_mut();
                    loop {
                        let chunk = remaining.min(MAX_NBYTES);
                        let reload = remaining > MAX_NBYTES;
                        if first {
                            self.setup(address, true, chunk as u8, reload, true);
                        } else {
                            self.wait_tcr_async().await?;
                            self.regs.CR2().modify(|w| {
                                w.set_NBYTES(chunk as u8);
                                w.set_RELOAD(reload);
                            });
                        }
                        for _ in 0..chunk {
                            self.wait_rxne_async().await?;
                            *iter.next().unwrap() = self.regs.RXDR().read().RXDATA();
                        }
                        remaining -= chunk;
                        first = false;
                        if remaining == 0 {
                            break;
                        }
                    }
                }
            }
        }
        self.stop_async().await
    }
}

/// DMA-backed transfers.
///
/// These reuse the shared [`crate::dma`] channel primitives: the I2C DMA
/// request lines (`CR1.TXDMAEN` / `CR1.RXDMAEN`) are enabled here, and the
/// bytes are streamed through the channel against `TXDR`/`RXDR`. The transfers
/// are blocking today; an async variant can be layered on the same primitives.
#[cfg(feature = "dma")]
impl I2c {
    fn map_dma(res: Result<(), crate::dma::Error>) -> Result<(), Error> {
        res.map_err(|_| Error::Bus)
    }

    /// Block until the current transfer completes (`TC`).
    fn wait_tc(&self) -> Result<(), Error> {
        loop {
            self.check_error()?;
            if self.regs.ISR().read().TC() {
                return Ok(());
            }
        }
    }

    /// Write `bytes` to `address` using DMA, then generate a STOP (blocking).
    ///
    /// `tx_request` is the DMAMUX request line for this I2C instance's transmit
    /// request; the caller supplies it because the driver is bound to a raw base
    /// and does not know its instance identity.
    ///
    /// # Panics
    ///
    /// Panics if `bytes` is empty or longer than 255 (a single-shot `NBYTES`).
    pub fn write_dma(
        &mut self,
        address: SevenBitAddress,
        bytes: &[u8],
        dma: &mut crate::dma::DmaChannel,
        tx_request: u8,
    ) -> Result<(), Error> {
        assert!(!bytes.is_empty() && bytes.len() <= MAX_NBYTES);
        self.setup(address, false, bytes.len() as u8, false, true);
        self.regs.CR1().modify(|w| w.set_TXDMAEN(true));
        let txdr = self.regs.TXDR().as_ptr() as *mut u8;
        // SAFETY: `txdr` is this instance's transmit data register and
        // `tx_request` names the matching hardware request line.
        let result =
            Self::map_dma(unsafe { dma.write_peripheral(bytes, txdr, tx_request) }.wait_blocking());
        self.regs.CR1().modify(|w| w.set_TXDMAEN(false));
        result?;
        self.wait_tc()?;
        self.stop()
    }

    /// Read `buffer.len()` bytes from `address` using DMA, then generate a STOP
    /// (blocking).
    ///
    /// `rx_request` is the DMAMUX request line for this I2C instance's receive
    /// request; the caller supplies it because the driver is bound to a raw base
    /// and does not know its instance identity.
    ///
    /// # Panics
    ///
    /// Panics if `buffer` is empty or longer than 255 (a single-shot `NBYTES`).
    pub fn read_dma(
        &mut self,
        address: SevenBitAddress,
        buffer: &mut [u8],
        dma: &mut crate::dma::DmaChannel,
        rx_request: u8,
    ) -> Result<(), Error> {
        assert!(!buffer.is_empty() && buffer.len() <= MAX_NBYTES);
        self.setup(address, true, buffer.len() as u8, false, true);
        self.regs.CR1().modify(|w| w.set_RXDMAEN(true));
        let rxdr = self.regs.RXDR().as_ptr() as *const u8;
        // SAFETY: `rxdr` is this instance's receive data register and
        // `rx_request` names the matching hardware request line.
        let result =
            Self::map_dma(unsafe { dma.read_peripheral(rxdr, buffer, rx_request) }.wait_blocking());
        self.regs.CR1().modify(|w| w.set_RXDMAEN(false));
        result?;
        self.wait_tc()?;
        self.stop()
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
    use super::timing;

    #[test]
    fn timing_standard_mode_fits() {
        let t = timing(64_000_000, 100_000);
        assert!(t.scll == t.sclh);
        // Fields are 8/4-bit; ensure they are in range.
        assert!(t.presc <= 15 && t.scldel <= 15 && t.sdadel <= 15);
    }

    #[test]
    fn timing_fast_mode_fits() {
        let t = timing(64_000_000, 400_000);
        assert!(t.presc <= 15);
    }
}
