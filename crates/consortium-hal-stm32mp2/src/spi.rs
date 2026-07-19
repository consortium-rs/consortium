//! SPI master driver backed by an STM32MP2 SPI (full-feature "SPI2S v2")
//! instance.
//!
//! Select an instance by passing its MMIO base to [`Spi::new`] together with a
//! `&'static` [`SpiState`]. Blocking transfers ([`embedded_hal::spi::SpiBus`])
//! work without interrupts. Async transfers
//! ([`embedded_hal_async::spi::SpiBus`]) require the caller to route the
//! instance's interrupt to [`SpiState::on_interrupt`].
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
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::Poll;

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
pub struct SpiState {
    base: AtomicPtr<()>,
    waker: AtomicWaker,
}

impl SpiState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            base: AtomicPtr::new(core::ptr::null_mut()),
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the SPI instance's interrupt.
    ///
    /// Masks every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-arms the ones it still needs on the next poll.
    pub fn on_interrupt(&self) {
        let ptr = self.base.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `Spi::new` installs the MMIO base for the selected SPI
        // instance before any interrupt is routed here.
        let r = unsafe { SPI::from_ptr(ptr) };
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

impl Default for SpiState {
    fn default() -> Self {
        Self::new()
    }
}

/// SPI master over one SPI instance.
pub struct Spi {
    regs: SPI,
    state: &'static SpiState,
}

impl Spi {
    /// Initialize an SPI instance as a master.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid SPI register block, clocked and pinned out
    /// by the caller, that is exclusively owned by this driver for the lifetime
    /// of the program. `clock_hz` is the instance's kernel clock. `state` must
    /// be dedicated to this instance; for async use, route the instance's
    /// interrupt to [`SpiState::on_interrupt`] on the same state.
    pub unsafe fn new(
        base: *mut (),
        clock_hz: u32,
        config: Config,
        state: &'static SpiState,
    ) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned SPI
        // register block.
        let r = unsafe { SPI::from_ptr(base) };

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

        state.base.store(base, Ordering::Release);

        Self { regs: r, state }
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
        Ok(())
    }

    async fn chunk_in_place_async(&mut self, words: &mut [u8]) -> Result<(), Error> {
        self.start(words.len());
        for word in words.iter_mut() {
            self.wait_txp().await?;
            // SAFETY: byte store to this instance's `TXDR` pushes one frame.
            unsafe { self.txdr().write_volatile(*word) };
            self.wait_rxp().await?;
            // SAFETY: byte load from this instance's `RXDR` pops one frame.
            *word = unsafe { self.rxdr().read_volatile() };
        }
        self.finish();
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
