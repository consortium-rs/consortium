//! SPI master driver backed by an STM32MP2 SPI (full-feature "SPI2S v2")
//! instance.
//!
//! Select an instance by handing its peripheral singleton to [`Spi::new`].
//! Blocking transfers ([`embedded_hal::spi::SpiBus`]) work without interrupts.
//! Async transfers ([`embedded_hal_async::spi::SpiBus`]) require the caller to
//! route the instance's interrupt to [`on_interrupt`]. A transaction that fails,
//! or an async one dropped mid-flight, is suspended and the peripheral disabled
//! before control returns, so no half-finished transfer outlives the call.
//!
//! This is a byte-oriented full-duplex master. Each `SpiBus` method programs
//! `CR2.TSIZE` with the frame count and issues one `CR1.CSTART`, so it reads as
//! a single hardware transaction; buffers longer than the 16-bit `TSIZE` field
//! are split into back-to-back transactions. Software slave management
//! (`CFG2.SSM` + `CR1.SSI`) keeps the peripheral a master without driving a
//! hardware `NSS`, so chip-select is the caller's to manage.
//!
//! DMA readiness: the byte primitives push through `TXDR` and drain `RXDR`, so
//! a DMA-backed path can be layered on top by setting `CFG1.TXDMAEN` /
//! `CFG1.RXDMAEN` and streaming through
//! [`crate::dma::DmaChannel::write_peripheral`] /
//! [`crate::dma::DmaChannel::read_peripheral`] without changing this driver's
//! public shape. The interrupt/polling path is the default and only path today.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal::spi::{ErrorKind, ErrorType, MODE_0, Mode, Phase, Polarity};
use pac::peripherals::spi::SPI;

use crate::pac;

/// Fixed 8-bit frame size, expressed as `CFG1.DSIZE` (bits minus one).
const DSIZE_8BIT: u8 = 7;

/// Largest frame count a single `CR2.TSIZE` transaction can request.
const MAX_TSIZE: usize = u16::MAX as usize;

/// SPI transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The receive FIFO overran (a received frame was dropped).
    Overrun,
    /// A mode fault was detected (`NSS` pulled low while master).
    ModeFault,
    /// A TI-mode frame format error occurred.
    Frame,
}

impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Overrun => ErrorKind::Overrun,
            Error::ModeFault => ErrorKind::ModeFault,
            Error::Frame => ErrorKind::FrameFormat,
        }
    }
}

/// SPI master configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Target SCK frequency in Hz.
    pub frequency: u32,
    /// Clock polarity and phase.
    pub mode: Mode,
    /// Transmit the least-significant bit first when `true`.
    pub lsb_first: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: MODE_0,
            lsb_first: false,
        }
    }
}

/// Compute `CFG1.MBR` for a target SCK frequency.
///
/// The master baud generator divides the kernel clock by `2^(MBR + 1)`, so this
/// picks the smallest `MBR` (3-bit field) whose resulting clock does not exceed
/// the requested frequency.
const fn mbr(clock_hz: u32, frequency: u32) -> u8 {
    assert!(frequency > 0);
    let mut mbr = 0u8;
    while mbr < 7 && clock_hz / (1 << (mbr + 1)) > frequency {
        mbr += 1;
    }
    mbr
}

/// Per-instance interrupt state shared between a [`Spi`] and its IRQ handler.
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

    /// Handle the SPI instance's interrupt.
    ///
    /// Masks every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-arms the ones it still needs on the next poll.
    fn on_interrupt(&self, r: SPI) {
        let sr = r.SR().read();
        let ier = r.IER().read();
        let fire = (ier.TXPIE() && sr.TXP())
            || (ier.RXPIE() && sr.RXP())
            || (ier.EOTIE() && sr.EOT())
            || (ier.OVRIE() && sr.OVR())
            || (ier.MODFIE() && sr.MODF())
            || (ier.TIFREIE() && sr.TIFRE());
        if fire {
            r.IER().modify(|w| {
                w.set_TXPIE(false);
                w.set_RXPIE(false);
                w.set_EOTIE(false);
                w.set_OVRIE(false);
                w.set_MODFIE(false);
                w.set_TIFREIE(false);
            });
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> SPI;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An SPI instance the [`Spi`] driver can drive.
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
macro_rules! impl_spi {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::spi::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::spi::SPI {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::spi::SPI::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::spi::State {
                static STATE: crate::spi::State = crate::spi::State::new();
                &STATE
            }
        }

        impl crate::spi::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_spi;

/// Handle the interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// SPI master over one SPI instance.
pub struct Spi {
    regs: SPI,
    state: &'static State,
}

impl Spi {
    /// Initialize an SPI instance as a master.
    ///
    /// `clock_hz` is the instance's kernel clock; the platform must leave the
    /// instance clocked and pinned out. For async transfers, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, config: Config) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();

        // Configure only while the peripheral is disabled.
        r.CR1().modify(|w| w.set_SPE(false));

        let (cpol, cpha) = mode_bits(config.mode);
        r.CFG1().modify(|w| {
            w.set_DSIZE(DSIZE_8BIT);
            w.set_FTHLV(0);
            w.set_MBR(mbr(clock_hz, config.frequency));
        });
        r.CFG2().modify(|w| {
            w.set_MASTER(true);
            w.set_CPOL(cpol);
            w.set_CPHA(cpha);
            w.set_LSBFRST(config.lsb_first);
            // Software slave management: keep the peripheral a master without
            // driving a hardware NSS. `SSI` high avoids a mode fault.
            w.set_SSM(true);
            w.set_COMM(0);
            w.set_AFCNTR(true);
        });
        r.CR1().modify(|w| w.set_SSI(true));

        Self {
            regs: r,
            state: T::state(),
        }
    }

    /// Byte-wide view of `TXDR` so each store pushes exactly one 8-bit frame.
    fn txdr(&self) -> *mut u8 {
        self.regs.TXDR().as_ptr() as *mut u8
    }

    /// Byte-wide view of `RXDR` so each load pops exactly one 8-bit frame.
    fn rxdr(&self) -> *const u8 {
        self.regs.RXDR().as_ptr() as *const u8
    }

    /// Translate and clear a latched transfer error, if any.
    fn check_error(&self) -> Result<(), Error> {
        let sr = self.regs.SR().read();
        if sr.OVR() {
            self.regs.IFCR().write(|w| w.set_OVRC(true));
            return Err(Error::Overrun);
        }
        if sr.MODF() {
            self.regs.IFCR().write(|w| w.set_MODFC(true));
            return Err(Error::ModeFault);
        }
        if sr.TIFRE() {
            self.regs.IFCR().write(|w| w.set_TIFREC(true));
            return Err(Error::Frame);
        }
        Ok(())
    }

    /// Arm a transaction of `len` frames (`1..=MAX_TSIZE`) and assert `CSTART`.
    fn start(&self, len: usize) {
        self.regs.CR1().modify(|w| w.set_SPE(false));
        self.regs.CR2().modify(|w| w.set_TSIZE(len as u16));
        self.regs.CR1().modify(|w| w.set_SPE(true));
        self.regs.CR1().modify(|w| w.set_CSTART(true));
    }

    /// Clear end-of-transfer flags and drop `SPE` after a transaction.
    fn finish(&self) {
        self.regs.IFCR().write(|w| {
            w.set_EOTC(true);
            w.set_TXTFC(true);
        });
        self.regs.CR1().modify(|w| w.set_SPE(false));
    }

    // --- blocking primitives ------------------------------------------------

    fn chunk_blocking(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let len = read.len().max(write.len());
        self.start(len);
        let guard = AbortGuard::new(self.regs);
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            loop {
                self.check_error()?;
                if self.regs.SR().read().TXP() {
                    break;
                }
            }
            // SAFETY: `txdr` is this instance's transmit data register; a byte
            // store pushes one 8-bit frame with `DSIZE == 8`.
            unsafe { self.txdr().write_volatile(tx) };
            loop {
                self.check_error()?;
                if self.regs.SR().read().RXP() {
                    break;
                }
            }
            // SAFETY: `rxdr` is this instance's receive data register; a byte
            // load pops one received 8-bit frame.
            let rx = unsafe { self.rxdr().read_volatile() };
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        self.finish();
        guard.defuse();
        Ok(())
    }

    fn transfer_blocking(&mut self, mut read: &mut [u8], mut write: &[u8]) -> Result<(), Error> {
        loop {
            let len = read.len().max(write.len());
            if len == 0 {
                return Ok(());
            }
            let chunk = len.min(MAX_TSIZE);
            let (r_head, r_tail) = read.split_at_mut(chunk.min(read.len()));
            let (w_head, w_tail) = write.split_at(chunk.min(write.len()));
            self.chunk_blocking(r_head, w_head)?;
            read = r_tail;
            write = w_tail;
        }
    }

    // --- async primitives ---------------------------------------------------

    async fn wait_flag(
        &self,
        arm: fn(&SPI),
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

    fn arm_errors(r: &SPI) {
        r.IER().modify(|w| {
            w.set_OVRIE(true);
            w.set_MODFIE(true);
            w.set_TIFREIE(true);
        });
    }

    async fn wait_txp(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_errors(r);
                r.IER().modify(|w| w.set_TXPIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.SR().read().TXP() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_rxp(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_errors(r);
                r.IER().modify(|w| w.set_RXPIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.SR().read().RXP() => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn chunk_async(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let len = read.len().max(write.len());
        self.start(len);
        let guard = AbortGuard::new(self.regs);
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            self.wait_txp().await?;
            // SAFETY: byte store to this instance's `TXDR` pushes one frame.
            unsafe { self.txdr().write_volatile(tx) };
            self.wait_rxp().await?;
            // SAFETY: byte load from this instance's `RXDR` pops one frame.
            let rx = unsafe { self.rxdr().read_volatile() };
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        self.finish();
        guard.defuse();
        Ok(())
    }

    async fn transfer_async(&mut self, mut read: &mut [u8], mut write: &[u8]) -> Result<(), Error> {
        loop {
            let len = read.len().max(write.len());
            if len == 0 {
                return Ok(());
            }
            let chunk = len.min(MAX_TSIZE);
            let (r_head, r_tail) = read.split_at_mut(chunk.min(read.len()));
            let (w_head, w_tail) = write.split_at(chunk.min(write.len()));
            self.chunk_async(r_head, w_head).await?;
            read = r_tail;
            write = w_tail;
        }
    }
}

/// Upper bound on status polls while waiting for a suspend request to take
/// effect. Drop cannot await or read a clock, so this is a spin count: well
/// beyond one frame at the slowest `MBR` divider.
const ABORT_SPINS: u32 = 200_000;

/// Stop a transaction that ended early, by error or by a dropped future.
///
/// RM0457 has a master stop an ongoing transaction by suspending it (`CSUSP`,
/// then `SUSP`) before clearing `SPE`; clearing `SPE` resets the FIFOs. Every
/// interrupt enable and status flag is cleared too, so the next transfer starts
/// from the same state [`Spi::new`] leaves.
fn abort(r: SPI) {
    critical_section::with(|_| {
        r.IER().write(|w| {
            w.set_TXPIE(false);
            w.set_RXPIE(false);
            w.set_EOTIE(false);
            w.set_OVRIE(false);
            w.set_MODFIE(false);
            w.set_TIFREIE(false);
        })
    });
    if r.CR1().read().CSTART() {
        r.CR1().modify(|w| w.set_CSUSP(true));
        let mut spins = ABORT_SPINS;
        while spins > 0 && !r.SR().read().SUSP() && !r.SR().read().EOT() {
            spins -= 1;
        }
    }
    r.IFCR().write(|w| {
        w.set_EOTC(true);
        w.set_TXTFC(true);
        w.set_UDRC(true);
        w.set_OVRC(true);
        w.set_TIFREC(true);
        w.set_MODFC(true);
        w.set_SUSPC(true);
    });
    r.CR1().modify(|w| w.set_SPE(false));
}

/// Runs [`abort`] when dropped unless [`defuse`](Self::defuse)d, so an error
/// propagated with `?` and an async transfer dropped mid-flight both leave the
/// peripheral disabled with empty FIFOs.
struct AbortGuard {
    regs: SPI,
    armed: bool,
}

impl AbortGuard {
    fn new(regs: SPI) -> Self {
        Self { regs, armed: true }
    }

    /// The transaction completed cleanly; skip the abort.
    fn defuse(mut self) {
        self.armed = false;
    }
}

impl Drop for AbortGuard {
    fn drop(&mut self) {
        if self.armed {
            abort(self.regs);
        }
    }
}

/// Split an [`embedded_hal`] [`Mode`] into `(CPOL, CPHA)` register bits.
fn mode_bits(mode: Mode) -> (bool, bool) {
    (
        matches!(mode.polarity, Polarity::IdleHigh),
        matches!(mode.phase, Phase::CaptureOnSecondTransition),
    )
}

impl ErrorType for Spi {
    type Error = Error;
}

impl embedded_hal::spi::SpiBus<u8> for Spi {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.transfer_blocking(words, &[])
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.transfer_blocking(&mut [], words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        self.transfer_blocking(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        let len = words.len();
        let mut offset = 0;
        while offset < len {
            let chunk = (len - offset).min(MAX_TSIZE);
            let end = offset + chunk;
            self.chunk_in_place_blocking(&mut words[offset..end])?;
            offset = end;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl Spi {
    fn chunk_in_place_blocking(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.start(words.len());
        let guard = AbortGuard::new(self.regs);
        for word in words.iter_mut() {
            loop {
                self.check_error()?;
                if self.regs.SR().read().TXP() {
                    break;
                }
            }
            // SAFETY: byte store to this instance's `TXDR` pushes one frame.
            unsafe { self.txdr().write_volatile(*word) };
            loop {
                self.check_error()?;
                if self.regs.SR().read().RXP() {
                    break;
                }
            }
            // SAFETY: byte load from this instance's `RXDR` pops one frame.
            *word = unsafe { self.rxdr().read_volatile() };
        }
        self.finish();
        guard.defuse();
        Ok(())
    }

    async fn chunk_in_place_async(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.start(words.len());
        let guard = AbortGuard::new(self.regs);
        for word in words.iter_mut() {
            self.wait_txp().await?;
            // SAFETY: byte store to this instance's `TXDR` pushes one frame.
            unsafe { self.txdr().write_volatile(*word) };
            self.wait_rxp().await?;
            // SAFETY: byte load from this instance's `RXDR` pops one frame.
            *word = unsafe { self.rxdr().read_volatile() };
        }
        self.finish();
        guard.defuse();
        Ok(())
    }
}

impl embedded_hal_async::spi::SpiBus<u8> for Spi {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.transfer_async(words, &[]).await
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Error> {
        self.transfer_async(&mut [], words).await
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        self.transfer_async(read, write).await
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        let len = words.len();
        let mut offset = 0;
        while offset < len {
            let chunk = (len - offset).min(MAX_TSIZE);
            let end = offset + chunk;
            self.chunk_in_place_async(&mut words[offset..end]).await?;
            offset = end;
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::mbr;

    #[test]
    fn mbr_picks_divider_within_frequency() {
        // 100 MHz kernel, 1 MHz target: /128 (MBR 6) = 781 kHz <= 1 MHz;
        // /64 (MBR 5) = 1.5 MHz > 1 MHz, so MBR 6 is chosen.
        assert_eq!(mbr(100_000_000, 1_000_000), 6);
    }

    #[test]
    fn mbr_saturates_and_floors() {
        assert_eq!(mbr(1_000_000, 100_000_000), 0);
        assert_eq!(mbr(u32::MAX, 1), 7);
    }
}
