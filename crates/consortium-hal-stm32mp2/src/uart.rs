//! UART driver backed by an STM32MP2 USART instance.
//!
//! Any USART/UART instance can back a [`Uart`]; select it by passing its MMIO
//! base to [`Uart::new`] together with a `&'static` [`UartState`] per
//! instance. Blocking I/O ([`embedded_io`], [`embedded_hal_nb::serial`])
//! works without interrupts. Async I/O ([`embedded_io_async`]) requires the
//! caller to route the instance's interrupt to [`UartState::on_interrupt`].

use core::future::poll_fn;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::usart::USART;

use crate::pac;

/// UART line error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Received data was lost because the receive register was full.
    Overrun,
    /// A stop bit was sampled low.
    Framing,
    /// The received parity bit did not match.
    Parity,
    /// Noise was detected on a received character.
    Noise,
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl embedded_hal_nb::serial::Error for Error {
    fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
        match self {
            Error::Overrun => embedded_hal_nb::serial::ErrorKind::Overrun,
            Error::Framing => embedded_hal_nb::serial::ErrorKind::FrameFormat,
            Error::Parity => embedded_hal_nb::serial::ErrorKind::Parity,
            Error::Noise => embedded_hal_nb::serial::ErrorKind::Noise,
        }
    }
}

/// UART configuration. The frame format is fixed at 8N1.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Baud rate in bits per second.
    pub baudrate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { baudrate: 115_200 }
    }
}

/// Per-instance interrupt state shared between a [`Uart`] and its IRQ handler.
pub struct UartState {
    base: AtomicPtr<()>,
    rx_waker: AtomicWaker,
    tx_waker: AtomicWaker,
}

impl UartState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            base: AtomicPtr::new(core::ptr::null_mut()),
            rx_waker: AtomicWaker::new(),
            tx_waker: AtomicWaker::new(),
        }
    }

    /// Handle the USART instance's interrupt.
    ///
    /// Disables the interrupt-enable bit that fired and wakes the pending
    /// future; the future re-enables it on the next poll if still waiting.
    pub fn on_interrupt(&self) {
        let ptr = self.base.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `Uart::new` installs the MMIO base for the selected USART
        // instance before any interrupt is routed here.
        let r = unsafe { USART::from_ptr(ptr) };

        let isr = r.ISR().read();
        critical_section::with(|_| {
            let cr1 = r.CR1().read();
            if cr1.RXFNEIE() && (isr.RXFNE() || isr.ORE()) {
                r.CR1().modify(|w| w.set_RXFNEIE(false));
                self.rx_waker.wake();
            }
            if cr1.TXFNFIE() && isr.TXFNF() {
                r.CR1().modify(|w| w.set_TXFNFIE(false));
                self.tx_waker.wake();
            }
            if cr1.TCIE() && isr.TC() {
                r.CR1().modify(|w| w.set_TCIE(false));
                self.tx_waker.wake();
            }
        });
    }
}

impl Default for UartState {
    fn default() -> Self {
        Self::new()
    }
}

fn check_and_clear_errors(r: USART) -> Result<(), Error> {
    let isr = r.ISR().read();
    if isr.PE() {
        r.ICR().write(|w| w.set_PECF(true));
        return Err(Error::Parity);
    }
    if isr.FE() {
        r.ICR().write(|w| w.set_FECF(true));
        return Err(Error::Framing);
    }
    if isr.NE() {
        r.ICR().write(|w| w.set_NECF(true));
        return Err(Error::Noise);
    }
    if isr.ORE() {
        r.ICR().write(|w| w.set_ORECF(true));
        return Err(Error::Overrun);
    }
    Ok(())
}

/// Receive half of a [`Uart`].
pub struct UartRx {
    regs: USART,
    state: &'static UartState,
}

/// Transmit half of a [`Uart`].
pub struct UartTx {
    regs: USART,
    state: &'static UartState,
}

/// Full-duplex UART over one USART instance.
pub struct Uart {
    rx: UartRx,
    tx: UartTx,
}

impl Uart {
    /// Initialize a USART instance for 8N1 operation at `config.baudrate`.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid USART register block, clocked and pinned
    /// out by the caller, that is exclusively owned by this driver for the
    /// lifetime of the program. `clock_hz` is the instance's kernel clock.
    /// `state` must be dedicated to this instance; for async use, route the
    /// instance's interrupt to [`UartState::on_interrupt`] on the same state.
    pub unsafe fn new(
        base: *mut (),
        clock_hz: u32,
        config: Config,
        state: &'static UartState,
    ) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned
        // USART register block.
        let r = unsafe { USART::from_ptr(base) };

        let brr = brr_value(clock_hz, config.baudrate);

        r.CR1().write(|w| w.set_UE(false));
        r.BRR().write(|w| w.set_BRR(brr));
        r.PRESC().write(|w| w.set_PRESCALER(0));
        r.ICR().write(|w| {
            w.set_PECF(true);
            w.set_FECF(true);
            w.set_NECF(true);
            w.set_ORECF(true);
            w.set_TCCF(true);
        });
        r.CR1().write(|w| {
            w.set_FIFOEN(true);
            w.set_TE(true);
            w.set_RE(true);
            w.set_UE(true);
        });

        state.base.store(base, Ordering::Release);

        Self {
            rx: UartRx { regs: r, state },
            tx: UartTx { regs: r, state },
        }
    }

    /// Split into independently owned transmit and receive halves.
    pub fn split(self) -> (UartTx, UartRx) {
        (self.tx, self.rx)
    }
}

const fn brr_value(clock_hz: u32, baudrate: u32) -> u16 {
    assert!(baudrate > 0);
    let div = (clock_hz as u64 + baudrate as u64 / 2) / baudrate as u64;
    // Oversampling by 16 requires BRR >= 16; the register is 16 bits wide.
    assert!(div >= 16 && div <= u16::MAX as u64);
    div as u16
}

impl UartRx {
    fn try_read(&self, buf: &mut [u8]) -> Option<Result<usize, Error>> {
        if let Err(e) = check_and_clear_errors(self.regs) {
            return Some(Err(e));
        }
        if !self.regs.ISR().read().RXFNE() {
            return None;
        }
        let mut n = 0;
        while n < buf.len() && self.regs.ISR().read().RXFNE() {
            buf[n] = self.regs.RDR().read().RDR() as u8;
            n += 1;
        }
        Some(Ok(n))
    }

    /// Read into `buf`, waiting until at least one byte is available.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            if let Some(res) = self.try_read(buf) {
                return Poll::Ready(res);
            }
            self.state.rx_waker.register(cx.waker());
            critical_section::with(|_| self.regs.CR1().modify(|w| w.set_RXFNEIE(true)));
            if let Some(res) = self.try_read(buf) {
                critical_section::with(|_| self.regs.CR1().modify(|w| w.set_RXFNEIE(false)));
                return Poll::Ready(res);
            }
            Poll::Pending
        })
        .await
    }
}

impl UartTx {
    fn try_write(&self, buf: &[u8]) -> Option<usize> {
        if !self.regs.ISR().read().TXFNF() {
            return None;
        }
        let mut n = 0;
        while n < buf.len() && self.regs.ISR().read().TXFNF() {
            self.regs.TDR().write(|w| w.set_TDR(buf[n] as u16));
            n += 1;
        }
        Some(n)
    }

    /// Write from `buf`, waiting until at least one byte is accepted.
    pub async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            if let Some(n) = self.try_write(buf) {
                return Poll::Ready(Ok(n));
            }
            self.state.tx_waker.register(cx.waker());
            critical_section::with(|_| self.regs.CR1().modify(|w| w.set_TXFNFIE(true)));
            if let Some(n) = self.try_write(buf) {
                critical_section::with(|_| self.regs.CR1().modify(|w| w.set_TXFNFIE(false)));
                return Poll::Ready(Ok(n));
            }
            Poll::Pending
        })
        .await
    }

    /// Wait until all queued data has left the shift register.
    pub async fn flush(&mut self) -> Result<(), Error> {
        poll_fn(|cx| {
            if self.regs.ISR().read().TC() {
                return Poll::Ready(Ok(()));
            }
            self.state.tx_waker.register(cx.waker());
            critical_section::with(|_| self.regs.CR1().modify(|w| w.set_TCIE(true)));
            if self.regs.ISR().read().TC() {
                critical_section::with(|_| self.regs.CR1().modify(|w| w.set_TCIE(false)));
                return Poll::Ready(Ok(()));
            }
            Poll::Pending
        })
        .await
    }
}

impl embedded_io::ErrorType for Uart {
    type Error = Error;
}
impl embedded_io::ErrorType for UartRx {
    type Error = Error;
}
impl embedded_io::ErrorType for UartTx {
    type Error = Error;
}

impl embedded_io::Read for UartRx {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if let Some(res) = self.try_read(buf) {
                return res;
            }
        }
    }
}

impl embedded_io::ReadReady for UartRx {
    fn read_ready(&mut self) -> Result<bool, Error> {
        check_and_clear_errors(self.regs)?;
        Ok(self.regs.ISR().read().RXFNE())
    }
}

impl embedded_io::Write for UartTx {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            if let Some(n) = self.try_write(buf) {
                return Ok(n);
            }
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        while !self.regs.ISR().read().TC() {}
        Ok(())
    }
}

impl embedded_io::WriteReady for UartTx {
    fn write_ready(&mut self) -> Result<bool, Error> {
        Ok(self.regs.ISR().read().TXFNF())
    }
}

impl embedded_io::Read for Uart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        embedded_io::Read::read(&mut self.rx, buf)
    }
}

impl embedded_io::ReadReady for Uart {
    fn read_ready(&mut self) -> Result<bool, Error> {
        embedded_io::ReadReady::read_ready(&mut self.rx)
    }
}

impl embedded_io::Write for Uart {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        embedded_io::Write::write(&mut self.tx, buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        embedded_io::Write::flush(&mut self.tx)
    }
}

impl embedded_io::WriteReady for Uart {
    fn write_ready(&mut self) -> Result<bool, Error> {
        embedded_io::WriteReady::write_ready(&mut self.tx)
    }
}

impl embedded_io_async::Read for UartRx {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        UartRx::read(self, buf).await
    }
}

impl embedded_io_async::Write for UartTx {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        UartTx::write(self, buf).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        UartTx::flush(self).await
    }
}

impl embedded_io_async::Read for Uart {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        UartRx::read(&mut self.rx, buf).await
    }
}

impl embedded_io_async::Write for Uart {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        UartTx::write(&mut self.tx, buf).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        UartTx::flush(&mut self.tx).await
    }
}

impl embedded_hal_nb::serial::ErrorType for Uart {
    type Error = Error;
}
impl embedded_hal_nb::serial::ErrorType for UartRx {
    type Error = Error;
}
impl embedded_hal_nb::serial::ErrorType for UartTx {
    type Error = Error;
}

impl embedded_hal_nb::serial::Read<u8> for UartRx {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Error> {
        check_and_clear_errors(self.regs)?;
        if self.regs.ISR().read().RXFNE() {
            Ok(self.regs.RDR().read().RDR() as u8)
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for UartTx {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Error> {
        if self.regs.ISR().read().TXFNF() {
            self.regs.TDR().write(|w| w.set_TDR(word as u16));
            Ok(())
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Error> {
        if self.regs.ISR().read().TC() {
            Ok(())
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal_nb::serial::Read<u8> for Uart {
    fn read(&mut self) -> embedded_hal_nb::nb::Result<u8, Error> {
        embedded_hal_nb::serial::Read::read(&mut self.rx)
    }
}

impl embedded_hal_nb::serial::Write<u8> for Uart {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Error> {
        embedded_hal_nb::serial::Write::write(&mut self.tx, word)
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Error> {
        embedded_hal_nb::serial::Write::flush(&mut self.tx)
    }
}

#[cfg(test)]
mod tests {
    use super::brr_value;

    #[test]
    fn brr_rounds_to_nearest() {
        assert_eq!(brr_value(64_000_000, 115_200), 556);
        assert_eq!(brr_value(1_000_000, 9_600), 104);
    }

    #[test]
    #[should_panic]
    fn brr_rejects_too_fast_baud() {
        brr_value(100_000, 115_200);
    }

    #[test]
    #[should_panic]
    fn brr_rejects_divider_overflow() {
        brr_value(2_000_000_000, 300);
    }
}
