//! I2C master driver backed by an STM32MP2 I2C (v2) instance.
//!
//! Select an instance by handing its peripheral singleton to [`I2c::new`].
//! Blocking transfers ([`embedded_hal::i2c::I2c`]) work without interrupts.
//! Async transfers ([`embedded_hal_async::i2c::I2c`]) require the caller to
//! route the instance's interrupt to [`on_interrupt`].
//!
//! A transaction follows the `embedded-hal` contract: each run of consecutive
//! same-direction operations goes out as one segment, with a repeated START
//! only where the direction changes, and a single STOP at the end. A transfer
//! that fails, or an async one that is dropped mid-flight, releases the bus and
//! resets the controller before returning, so the next transaction starts from
//! idle.
//!
//! Every blocking wait, including the DMA paths, is bounded by the driver's
//! [`Timeout`] (see [`I2c::set_timeout`]) and fails with [`Error::Timeout`] if
//! the bus stalls.
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

use crate::hal::timeout::Timeout;
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
    /// A blocking wait ran out of its [`Timeout`] budget.
    Timeout,
    /// The DMA channel reported a transfer error.
    #[cfg(feature = "dma")]
    Dma(crate::dma::Error),
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Nack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            Error::Bus => ErrorKind::Bus,
            Error::Timeout => ErrorKind::Other,
            #[cfg(feature = "dma")]
            Error::Dma(_) => ErrorKind::Other,
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
    timeout: Timeout,
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
            timeout: Timeout::DEFAULT,
        }
    }

    /// Bound every blocking wait (one byte, the free bus, the STOP, a DMA
    /// transfer) to `timeout` polls. Defaults to [`Timeout::DEFAULT`].
    pub fn set_timeout(&mut self, timeout: Timeout) {
        self.timeout = timeout;
    }

    /// Translate the current error flags into an [`Error`].
    fn check_error(&self) -> Result<(), Error> {
        // Flags are left set: the transfer's `RecoverGuard` clears them, and a
        // still-set `NACKF` tells `recover` the hardware is already sending
        // the STOP.
        let isr = self.regs.ISR().read();
        if isr.NACKF() {
            return Err(Error::Nack);
        }
        if isr.ARLO() {
            return Err(Error::ArbitrationLoss);
        }
        if isr.BERR() {
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

    // --- blocking primitives ------------------------------------------------

    /// Spin until `ready` holds, surfacing any error flag first.
    fn wait_blocking(&self, ready: impl Fn(&I2C) -> bool) -> Result<(), Error> {
        let mut deadline = self.timeout.start();
        loop {
            self.check_error()?;
            if ready(&self.regs) {
                return Ok(());
            }
            deadline.tick().map_err(|_| Error::Timeout)?;
        }
    }

    /// Run one segment: a START (or repeated START) addressing `address`, then
    /// `total` bytes moved one at a time by `byte`, reloading `NBYTES` every
    /// [`MAX_NBYTES`].
    ///
    /// A first segment waits for the bus to be free; a later one waits for the
    /// previous segment's `TC` so the repeated START is not requested while its
    /// last byte is still on the wire.
    fn segment_blocking(
        &self,
        address: u8,
        read: bool,
        total: usize,
        restart: bool,
        mut byte: impl FnMut(&Self) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if restart {
            self.wait_blocking(|r| r.ISR().read().TC())?;
        } else {
            self.wait_blocking(|r| !r.CR2().read().START() && !r.ISR().read().BUSY())?;
        }
        let mut remaining = total;
        let mut chunk = remaining.min(MAX_NBYTES);
        self.setup(address, read, chunk as u8, remaining > MAX_NBYTES, true);
        loop {
            for _ in 0..chunk {
                byte(self)?;
            }
            remaining -= chunk;
            if remaining == 0 {
                return Ok(());
            }
            self.wait_blocking(|r| r.ISR().read().TCR())?;
            chunk = remaining.min(MAX_NBYTES);
            self.regs.CR2().modify(|w| {
                w.set_NBYTES(chunk as u8);
                w.set_RELOAD(remaining > MAX_NBYTES);
            });
        }
    }

    /// Wait for the last segment to complete, then generate a STOP.
    fn finish_blocking(&self) -> Result<(), Error> {
        self.wait_blocking(|r| r.ISR().read().TC())?;
        self.regs.CR2().modify(|w| w.set_STOP(true));
        self.wait_blocking(|r| r.ISR().read().STOPF())?;
        self.regs.ICR().write(|w| w.set_STOPCF(true));
        Ok(())
    }

    fn transaction_blocking(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        let guard = RecoverGuard::new(self.regs);
        let mut start = 0;
        while start < operations.len() {
            let end = group_end(operations, start);
            let group = &mut operations[start..end];
            let total = group_len(group);
            let restart = start > 0;
            if is_read(&group[0]) {
                let mut bytes = read_bytes(group);
                self.segment_blocking(address, true, total, restart, |s| {
                    s.wait_blocking(|r| r.ISR().read().RXNE())?;
                    *bytes.next().unwrap() = s.regs.RXDR().read().RXDATA();
                    Ok(())
                })?;
            } else {
                let mut bytes = write_bytes(group);
                self.segment_blocking(address, false, total, restart, |s| {
                    s.wait_blocking(|r| r.ISR().read().TXIS())?;
                    let b = *bytes.next().unwrap();
                    s.regs.TXDR().write_value(I2C_TXDR(u32::from(b)));
                    Ok(())
                })?;
            }
            start = end;
        }
        self.finish_blocking()?;
        guard.defuse();
        Ok(())
    }

    // --- async primitives ---------------------------------------------------

    /// Wait until `ready` holds, parking on the instance interrupt in between.
    ///
    /// `enable` arms the interrupt that signals `ready`; the error interrupts
    /// are armed alongside it so a NACK or bus fault also wakes the task.
    async fn wait_async(&self, enable: fn(&I2C), ready: fn(&I2C) -> bool) -> Result<(), Error> {
        poll_fn(|cx| {
            self.check_error()?;
            if ready(&self.regs) {
                return Poll::Ready(Ok(()));
            }
            self.state.waker.register(cx.waker());
            critical_section::with(|_| {
                self.regs.CR1().modify(|w| {
                    w.set_NACKIE(true);
                    w.set_ERRIE(true);
                });
                enable(&self.regs);
            });
            self.check_error()?;
            if ready(&self.regs) {
                return Poll::Ready(Ok(()));
            }
            Poll::Pending
        })
        .await
    }

    async fn wait_txis_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| r.CR1().modify(|w| w.set_TXIE(true)),
            |r| r.ISR().read().TXIS(),
        )
        .await
    }

    async fn wait_rxne_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| r.CR1().modify(|w| w.set_RXIE(true)),
            |r| r.ISR().read().RXNE(),
        )
        .await
    }

    async fn wait_tc_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| r.CR1().modify(|w| w.set_TCIE(true)),
            |r| r.ISR().read().TC(),
        )
        .await
    }

    async fn wait_tcr_async(&self) -> Result<(), Error> {
        self.wait_async(
            |r| r.CR1().modify(|w| w.set_TCIE(true)),
            |r| r.ISR().read().TCR(),
        )
        .await
    }

    /// Async counterpart of [`Self::segment_blocking`]. Waiting for a free bus
    /// before the first START has no interrupt to park on, so it yields between
    /// polls instead.
    async fn begin_segment_async(&self, restart: bool) -> Result<(), Error> {
        if restart {
            self.wait_tc_async().await
        } else {
            poll_fn(|cx| {
                self.check_error()?;
                if !self.regs.CR2().read().START() && !self.regs.ISR().read().BUSY() {
                    return Poll::Ready(Ok(()));
                }
                cx.waker().wake_by_ref();
                Poll::Pending
            })
            .await
        }
    }

    /// Reload `NBYTES` for the next chunk of a segment longer than
    /// [`MAX_NBYTES`], once the hardware has drained the previous one.
    async fn reload_async(&self, remaining: usize) -> Result<usize, Error> {
        self.wait_tcr_async().await?;
        let chunk = remaining.min(MAX_NBYTES);
        self.regs.CR2().modify(|w| {
            w.set_NBYTES(chunk as u8);
            w.set_RELOAD(remaining > MAX_NBYTES);
        });
        Ok(chunk)
    }

    async fn finish_async(&self) -> Result<(), Error> {
        self.wait_tc_async().await?;
        self.regs.CR2().modify(|w| w.set_STOP(true));
        self.wait_async(
            |r| r.CR1().modify(|w| w.set_STOPIE(true)),
            |r| r.ISR().read().STOPF(),
        )
        .await?;
        self.regs.ICR().write(|w| w.set_STOPCF(true));
        Ok(())
    }

    async fn transaction_async(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        if operations.is_empty() {
            return Ok(());
        }
        // Covers an error returned through `?` and a future dropped mid-transfer.
        let guard = RecoverGuard::new(self.regs);
        let mut start = 0;
        while start < operations.len() {
            let end = group_end(operations, start);
            let group = &mut operations[start..end];
            let total = group_len(group);
            let read = is_read(&group[0]);
            self.begin_segment_async(start > 0).await?;
            let mut remaining = total;
            let mut chunk = remaining.min(MAX_NBYTES);
            self.setup(address, read, chunk as u8, remaining > MAX_NBYTES, true);
            if read {
                let mut bytes = read_bytes(group);
                loop {
                    for _ in 0..chunk {
                        self.wait_rxne_async().await?;
                        *bytes.next().unwrap() = self.regs.RXDR().read().RXDATA();
                    }
                    remaining -= chunk;
                    if remaining == 0 {
                        break;
                    }
                    chunk = self.reload_async(remaining).await?;
                }
            } else {
                let mut bytes = write_bytes(group);
                loop {
                    for _ in 0..chunk {
                        self.wait_txis_async().await?;
                        let b = *bytes.next().unwrap();
                        self.regs.TXDR().write_value(I2C_TXDR(u32::from(b)));
                    }
                    remaining -= chunk;
                    if remaining == 0 {
                        break;
                    }
                    chunk = self.reload_async(remaining).await?;
                }
            }
            start = end;
        }
        self.finish_async().await?;
        guard.defuse();
        Ok(())
    }
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

/// Total byte count of a run of operations.
fn group_len(group: &[Operation<'_>]) -> usize {
    group
        .iter()
        .map(|op| match op {
            Operation::Read(buf) => buf.len(),
            Operation::Write(buf) => buf.len(),
        })
        .sum()
}

/// The bytes of a run of write operations, in bus order.
fn write_bytes<'a>(group: &'a [Operation<'_>]) -> impl Iterator<Item = &'a u8> {
    group.iter().flat_map(|op| match op {
        Operation::Write(buf) => buf.iter(),
        Operation::Read(_) => [].iter(),
    })
}

/// The buffer slots of a run of read operations, in bus order.
fn read_bytes<'a>(group: &'a mut [Operation<'_>]) -> impl Iterator<Item = &'a mut u8> {
    group.iter_mut().flat_map(|op| match op {
        Operation::Read(buf) => buf.iter_mut(),
        Operation::Write(_) => [].iter_mut(),
    })
}

/// Upper bound on status polls while waiting for a recovery STOP to reach the
/// bus. Drop cannot await or read a clock, so this is a spin count: comfortably
/// more than one byte time plus STOP at 100 kHz on a CM33 at 400 MHz.
const RECOVERY_SPINS: u32 = 200_000;

/// Return the controller to idle after a failed or abandoned transfer.
///
/// Follows STM32CubeMP2's `I2C_IsErrorOccurred`: after a NACK the hardware
/// sends STOP by itself, so wait for it, and request one by hand only if the
/// bus is still held — which is also the state a cancelled transfer leaves.
/// Then toggle `PE`, which resets the state machine, releases SCL/SDA, and
/// clears the status flags whatever the failure was, and reset `CR2` so no
/// half-programmed transfer survives.
fn recover(r: I2C) {
    critical_section::with(|_| {
        r.CR1().modify(|w| {
            w.set_TXIE(false);
            w.set_RXIE(false);
            w.set_TCIE(false);
            w.set_STOPIE(false);
            w.set_NACKIE(false);
            w.set_ERRIE(false);
            w.set_TXDMAEN(false);
            w.set_RXDMAEN(false);
        })
    });
    let isr = r.ISR().read();
    if isr.BUSY() && !isr.ARLO() && !isr.BERR() {
        let mut spins = RECOVERY_SPINS;
        if !isr.NACKF() && !r.CR2().read().STOP() {
            r.CR2().modify(|w| w.set_STOP(true));
        }
        while spins > 0 && !r.ISR().read().STOPF() {
            spins -= 1;
        }
    }
    r.CR1().modify(|w| w.set_PE(false));
    // PE takes a few APB cycles to read back as cleared.
    let mut spins = RECOVERY_SPINS;
    while spins > 0 && r.CR1().read().PE() {
        spins -= 1;
    }
    r.ICR().write(|w| {
        w.set_STOPCF(true);
        w.set_NACKCF(true);
        w.set_BERRCF(true);
        w.set_ARLOCF(true);
        w.set_OVRCF(true);
    });
    r.CR2().write_value(pac::peripherals::i2c::I2C_CR2(0));
    r.CR1().modify(|w| w.set_PE(true));
}

/// Runs [`recover`] when dropped unless [`defuse`](Self::defuse)d, so an
/// error propagated with `?` and an async transfer dropped mid-flight both
/// leave the controller idle and the bus released.
struct RecoverGuard {
    regs: I2C,
    armed: bool,
}

impl RecoverGuard {
    fn new(regs: I2C) -> Self {
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

/// DMA-backed transfers.
///
/// These reuse the shared [`crate::dma`] channel primitives: the I2C DMA
/// request lines (`CR1.TXDMAEN` / `CR1.RXDMAEN`) are enabled here, and the
/// bytes are streamed through the channel against `TXDR`/`RXDR`. The transfers
/// are blocking today; an async variant can be layered on the same primitives.
///
/// A NACKed target never raises another DMA request, so the channel would never
/// complete; the wait therefore polls the I2C error flags alongside the
/// channel status and gives up on either an error or the [`Timeout`].
#[cfg(feature = "dma")]
impl I2c {
    /// Wait for `transfer` to complete, failing early on an I2C error flag.
    fn wait_dma(&self, transfer: &crate::dma::Transfer<'_>) -> Result<(), Error> {
        let mut deadline = self.timeout.start();
        loop {
            self.check_error()?;
            if let Some(result) = transfer.status() {
                return result.map_err(Error::Dma);
            }
            deadline.tick().map_err(|_| Error::Timeout)?;
        }
    }

    /// Write `bytes` to `address` using DMA, then generate a STOP (blocking).
    ///
    /// `tx_request` is the HPDMA request line for this I2C instance's transmit
    /// request; the caller supplies it because the HAL has no per-instance DMA
    /// request table yet.
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
        let guard = RecoverGuard::new(self.regs);
        self.wait_blocking(|r| !r.CR2().read().START() && !r.ISR().read().BUSY())?;
        self.setup(address, false, bytes.len() as u8, false, true);
        self.regs.CR1().modify(|w| w.set_TXDMAEN(true));
        let txdr = self.regs.TXDR().as_ptr() as *mut u8;
        // SAFETY: `txdr` is this instance's transmit data register and
        // `tx_request` names the matching hardware request line.
        let transfer = unsafe { dma.write_peripheral(bytes, txdr, tx_request) };
        let result = self.wait_dma(&transfer);
        // Dropping the transfer halts the channel, also on the error path.
        drop(transfer);
        self.regs.CR1().modify(|w| w.set_TXDMAEN(false));
        result?;
        self.finish_blocking()?;
        guard.defuse();
        Ok(())
    }

    /// Read `buffer.len()` bytes from `address` using DMA, then generate a STOP
    /// (blocking).
    ///
    /// `rx_request` is the HPDMA request line for this I2C instance's receive
    /// request; the caller supplies it because the HAL has no per-instance DMA
    /// request table yet.
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
        let guard = RecoverGuard::new(self.regs);
        self.wait_blocking(|r| !r.CR2().read().START() && !r.ISR().read().BUSY())?;
        self.setup(address, true, buffer.len() as u8, false, true);
        self.regs.CR1().modify(|w| w.set_RXDMAEN(true));
        let rxdr = self.regs.RXDR().as_ptr() as *const u8;
        // SAFETY: `rxdr` is this instance's receive data register and
        // `rx_request` names the matching hardware request line.
        let transfer = unsafe { dma.read_peripheral(rxdr, buffer, rx_request) };
        let result = self.wait_dma(&transfer);
        // Dropping the transfer halts the channel, also on the error path.
        drop(transfer);
        self.regs.CR1().modify(|w| w.set_RXDMAEN(false));
        result?;
        self.finish_blocking()?;
        guard.defuse();
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
    use embedded_hal::i2c::Operation;

    use super::{group_end, group_len, read_bytes, timing, write_bytes};

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

    #[test]
    fn same_direction_operations_form_one_segment() {
        let (a, b, c) = ([1u8, 2], [3u8], [4u8, 5, 6]);
        let (mut r1, mut r2) = ([0u8; 2], [0u8; 1]);
        let ops = [
            Operation::Write(&a),
            Operation::Write(&b),
            Operation::Read(&mut r1),
            Operation::Read(&mut r2),
            Operation::Write(&c),
        ];
        assert_eq!(group_end(&ops, 0), 2);
        assert_eq!(group_end(&ops, 2), 4);
        assert_eq!(group_end(&ops, 4), 5);
        assert_eq!(group_len(&ops[0..2]), 3);
        assert_eq!(group_len(&ops[2..4]), 3);
    }

    #[test]
    fn segment_bytes_span_operations_in_order() {
        let (a, b) = ([1u8, 2], [3u8]);
        let writes = [Operation::Write(&a), Operation::Write(&b)];
        assert!(write_bytes(&writes).copied().eq([1, 2, 3]));

        let (mut r1, mut r2) = ([0u8; 2], [0u8; 1]);
        {
            let mut reads = [Operation::Read(&mut r1), Operation::Read(&mut r2)];
            for (slot, value) in read_bytes(&mut reads).zip([7u8, 8, 9]) {
                *slot = value;
            }
        }
        assert_eq!((r1, r2), ([7, 8], [9]));
    }
}
