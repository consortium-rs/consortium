//! SPI master driver backed by an i.MX9 LPSPI instance.
//!
//! Select an instance by handing its peripheral singleton to [`Spi::new`].
//! Blocking transfers ([`embedded_hal::spi::SpiBus`]) work without interrupts.
//! Async transfers ([`embedded_hal_async::spi::SpiBus`]) require the caller to
//! route the instance's interrupt to [`on_interrupt`].
//!
//! This is a byte-oriented full-duplex master: every transferred word clocks
//! eight bits out on the shifter and eight bits in. The driver drives one
//! hardware chip-select (`PCS`) and holds it asserted for the duration of each
//! `SpiBus` method call (via the `TCR.CONT` flag), releasing it on the final
//! word so a bus method reads as one framed transaction. A transfer that
//! fails, or an async one dropped mid-flight, soft-resets the module so `PCS`
//! is released and the FIFOs are empty before control returns.
//!
//! DMA readiness: the byte primitives push through `TDR` and drain `RDR`, so a
//! DMA-backed path can be layered on top by enabling `DER.TDDE`/`DER.RDDE` and
//! streaming through [`crate::dma::DmaChannel::write_peripheral`] /
//! [`crate::dma::DmaChannel::read_peripheral`] without changing this driver's
//! public shape. The FIFO/interrupt path is the default and only path today.

use core::future::poll_fn;
use core::task::Poll;

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal::spi::{ErrorKind, ErrorType, MODE_0, Mode, Phase, Polarity};
use pac::peripherals::lpspi::{LPSPI, LPSPI_TDR};

use crate::pac;

/// Fixed 8-bit frame size, expressed as `FRAMESZ` (bits minus one).
const FRAMESZ_8BIT: u16 = 7;

/// SPI transfer error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The receive FIFO overflowed (a returned word was dropped).
    Overrun,
    /// The transmit FIFO underflowed during a frame.
    Underrun,
}

impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Overrun => ErrorKind::Overrun,
            Error::Underrun => ErrorKind::Other,
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
    /// Peripheral chip-select line (`PCS[n]`) asserted during transfers.
    pub pcs: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            mode: MODE_0,
            lsb_first: false,
            pcs: 0,
        }
    }
}

/// Compute `CCR.SCKDIV` for a target SCK frequency.
///
/// LPSPI derives `f_sck = f_clk / (SCKDIV + 2)`, so `SCKDIV = f_clk/f_sck - 2`,
/// clamped to the 8-bit field. The result is rounded up so the actual clock
/// never exceeds the requested frequency.
const fn sckdiv(clock_hz: u32, frequency: u32) -> u8 {
    assert!(frequency > 0);
    let div = clock_hz.div_ceil(frequency);
    if div < 2 {
        0
    } else {
        let d = div - 2;
        if d > 255 { 255 } else { d as u8 }
    }
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

    /// Handle the LPSPI instance's interrupt.
    ///
    /// Masks every enabled interrupt whose flag is asserted and wakes the
    /// pending transfer, which re-arms the ones it still needs on the next poll.
    fn on_interrupt(&self, r: LPSPI) {
        let sr = r.SR().read();
        let ier = r.IER().read();
        let fire = (ier.TDIE() && sr.TDF())
            || (ier.RDIE() && sr.RDF())
            || (ier.TEIE() && sr.TEF())
            || (ier.REIE() && sr.REF());
        if fire {
            r.IER().modify(|w| {
                w.set_TDIE(false);
                w.set_RDIE(false);
                w.set_TEIE(false);
                w.set_REIE(false);
            });
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> LPSPI;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An LPSPI instance the [`Spi`] driver can drive.
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
            fn regs() -> crate::pac::peripherals::lpspi::LPSPI {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::lpspi::LPSPI::from_ptr($base as *mut ()) }
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

/// SPI master over one LPSPI instance.
pub struct Spi {
    regs: LPSPI,
    state: &'static State,
    cpol: bool,
    cpha: bool,
    lsb_first: bool,
    pcs: u8,
}

impl Spi {
    /// Initialize an LPSPI instance as a master.
    ///
    /// `clock_hz` is the instance's functional clock; the platform must leave
    /// the instance clocked and pinned out. For async transfers, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, config: Config) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();

        // Software-reset the logic and FIFOs, then release the reset.
        r.CR().write(|w| w.set_RST(true));
        r.CR().write(|w| {
            w.set_RST(false);
            w.set_RTF(true);
            w.set_RRF(true);
        });

        // Master mode; leave PCS active-low (`PCSPOL` clear).
        r.CFGR1().modify(|w| w.set_MASTER(true));

        // Serial clock divider for the requested SCK frequency.
        r.CCR()
            .modify(|w| w.set_SCKDIV(sckdiv(clock_hz, config.frequency)));

        // Watermarks at zero: `TDF` asserts when the TX FIFO is empty and `RDF`
        // asserts as soon as a word is received, which suits word-at-a-time
        // full-duplex framing.
        r.FCR().write(|w| {
            w.set_TXWATER(0);
            w.set_RXWATER(0);
        });

        // Enable the module.
        r.CR().modify(|w| w.set_MEN(true));

        let (cpol, cpha) = mode_bits(config.mode);
        Self {
            regs: r,
            state: T::state(),
            cpol,
            cpha,
            lsb_first: config.lsb_first,
            pcs: config.pcs,
        }
    }

    /// TX FIFO depth (`PARAM.TXFIFO` encodes `2^n` words).
    fn tx_fifo_len(&self) -> u32 {
        1 << self.regs.PARAM().read().TXFIFO()
    }

    /// Program `TCR` for one 8-bit frame, choosing whether to keep `PCS`
    /// asserted afterward (`cont`).
    fn set_frame(&self, cont: bool) {
        self.regs.TCR().write(|w| {
            w.set_FRAMESZ(FRAMESZ_8BIT);
            w.set_CPOL(self.cpol);
            w.set_CPHA(self.cpha);
            w.set_LSBF(self.lsb_first);
            w.set_PCS(self.pcs);
            w.set_CONT(cont);
        });
    }

    /// Translate and clear a latched FIFO error, if any.
    fn check_error(&self) -> Result<(), Error> {
        let sr = self.regs.SR().read();
        if sr.REF() {
            self.regs.SR().write(|w| w.set_REF(true));
            return Err(Error::Overrun);
        }
        if sr.TEF() {
            self.regs.SR().write(|w| w.set_TEF(true));
            return Err(Error::Underrun);
        }
        Ok(())
    }

    // --- blocking primitives ------------------------------------------------

    /// Transfer one word, holding `PCS` afterward when `cont` is set (blocking).
    fn transfer_word(&mut self, tx: u8, cont: bool) -> Result<u8, Error> {
        // Wait for transmit-FIFO space.
        let depth = self.tx_fifo_len();
        loop {
            self.check_error()?;
            if u32::from(self.regs.FSR().read().TXCOUNT()) < depth {
                break;
            }
        }
        self.set_frame(cont);
        self.regs.TDR().write_value(LPSPI_TDR(u32::from(tx)));
        // Wait for the received word.
        loop {
            self.check_error()?;
            if self.regs.FSR().read().RXCOUNT() > 0 {
                return Ok(self.regs.RDR().read().DATA() as u8);
            }
        }
    }

    fn transfer_blocking(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        let guard = AbortGuard::new(self.regs);
        let len = read.len().max(write.len());
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            let cont = i + 1 < len;
            let rx = self.transfer_word(tx, cont)?;
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        guard.defuse();
        Ok(())
    }

    // --- async primitives ---------------------------------------------------

    async fn wait_flag(
        &self,
        arm: fn(&LPSPI),
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

    fn arm_errors(r: &LPSPI) {
        r.IER().modify(|w| {
            w.set_TEIE(true);
            w.set_REIE(true);
        });
    }

    async fn wait_tx_space(&self) -> Result<(), Error> {
        let depth = self.tx_fifo_len();
        self.wait_flag(
            |r| {
                Self::arm_errors(r);
                r.IER().modify(|w| w.set_TDIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if u32::from(s.regs.FSR().read().TXCOUNT()) < depth => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn wait_rx(&self) -> Result<(), Error> {
        self.wait_flag(
            |r| {
                Self::arm_errors(r);
                r.IER().modify(|w| w.set_RDIE(true));
            },
            |s| match s.check_error() {
                Err(e) => Some(Err(e)),
                Ok(()) if s.regs.FSR().read().RXCOUNT() > 0 => Some(Ok(())),
                Ok(()) => None,
            },
        )
        .await
    }

    async fn transfer_word_async(&mut self, tx: u8, cont: bool) -> Result<u8, Error> {
        self.wait_tx_space().await?;
        self.set_frame(cont);
        self.regs.TDR().write_value(LPSPI_TDR(u32::from(tx)));
        self.wait_rx().await?;
        Ok(self.regs.RDR().read().DATA() as u8)
    }

    async fn transfer_async(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        // Covers an error returned through `?` and a future dropped mid-transfer.
        let guard = AbortGuard::new(self.regs);
        let len = read.len().max(write.len());
        for i in 0..len {
            let tx = write.get(i).copied().unwrap_or(0);
            let cont = i + 1 < len;
            let rx = self.transfer_word_async(tx, cont).await?;
            if let Some(slot) = read.get_mut(i) {
                *slot = rx;
            }
        }
        guard.defuse();
        Ok(())
    }
}

/// Stop a transfer that ended early, by error or by a dropped future.
///
/// A word-at-a-time transfer holds `PCS` asserted between words (`TCR.CONT`),
/// so stopping partway would leave the target selected. A module software
/// reset is the one step that negates `PCS`, empties both FIFOs, and clears the
/// error flags together; it also clears every register but `CR`, so the
/// configuration [`Spi::new`] programmed is saved and restored around it.
fn abort(r: LPSPI) {
    critical_section::with(|_| {
        r.IER().write(|w| {
            w.set_TDIE(false);
            w.set_RDIE(false);
            w.set_TEIE(false);
            w.set_REIE(false);
        })
    });
    let cfgr1 = r.CFGR1().read();
    let ccr = r.CCR().read();
    let fcr = r.FCR().read();
    r.CR().write(|w| w.set_RST(true));
    r.CR().write(|w| {
        w.set_RST(false);
        w.set_RTF(true);
        w.set_RRF(true);
    });
    r.CFGR1().write_value(cfgr1);
    r.CCR().write_value(ccr);
    r.FCR().write_value(fcr);
    r.CR().modify(|w| w.set_MEN(true));
}

/// Runs [`abort`] when dropped unless [`defuse`](Self::defuse)d, so an error
/// propagated with `?` and an async transfer dropped mid-flight both release
/// `PCS` and leave the FIFOs empty.
struct AbortGuard {
    regs: LPSPI,
    armed: bool,
}

impl AbortGuard {
    fn new(regs: LPSPI) -> Self {
        Self { regs, armed: true }
    }

    /// The transfer completed cleanly; skip the abort.
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
        // A throwaway read side keeps the receive FIFO drained so a write does
        // not trip an overrun; `transfer_blocking` handles the empty read slice.
        self.transfer_blocking(&mut [], words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Error> {
        self.transfer_blocking(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Error> {
        let guard = AbortGuard::new(self.regs);
        let len = words.len();
        for (i, word) in words.iter_mut().enumerate() {
            let cont = i + 1 < len;
            *word = self.transfer_word(*word, cont)?;
        }
        guard.defuse();
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Error> {
        while self.regs.SR().read().MBF() {
            self.check_error()?;
        }
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
        let guard = AbortGuard::new(self.regs);
        let len = words.len();
        for (i, word) in words.iter_mut().enumerate() {
            let cont = i + 1 < len;
            *word = self.transfer_word_async(*word, cont).await?;
        }
        guard.defuse();
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Error> {
        while self.regs.SR().read().MBF() {
            self.check_error()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::sckdiv;

    #[test]
    fn sckdiv_rounds_up_to_stay_within_frequency() {
        // 100 MHz / 1 MHz => divider 100 => SCKDIV 98.
        assert_eq!(sckdiv(100_000_000, 1_000_000), 98);
    }

    #[test]
    fn sckdiv_saturates_and_floors() {
        assert_eq!(sckdiv(1_000_000, 100_000_000), 0);
        assert_eq!(sckdiv(u32::MAX, 1), 255);
    }
}
