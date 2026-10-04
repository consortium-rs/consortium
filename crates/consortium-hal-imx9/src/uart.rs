//! UART driver backed by an i.MX9 LPUART instance.
//!
//! Any LPUART instance can back a [`Uart`]; select it by handing its
//! peripheral singleton to [`Uart::new`]:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let uart = Uart::new(p.LPUART1, 24_000_000, Config::default());
//! ```
//!
//! Blocking I/O ([`embedded_io`], [`embedded_hal_nb::serial`]) works without
//! interrupts. Async I/O ([`embedded_io_async`]) requires the application to
//! forward the instance's interrupt to [`on_interrupt`], or to
//! [`on_buffered_interrupt`] for a [`BufferedUart`].
//!
//! The async reads and writes are cancel-safe: a future moves bytes only in the
//! poll that completes it, so dropping one loses no data, and the one interrupt
//! enable it may leave armed is masked by the handler the first time it fires.

use core::future::poll_fn;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;

use embassy_hal_internal::atomic_ring_buffer::RingBuffer;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::lpuart::{LPUART, LPUART_DATA};

use crate::pac;

// The LPUART receiver oversamples each bit 16 times.
const OVERSAMPLING: u32 = 16;
const SBR_MAX: u64 = 0x1fff;

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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Overrun => f.write_str("receive overrun"),
            Error::Framing => f.write_str("framing error"),
            Error::Parity => f.write_str("parity error"),
            Error::Noise => f.write_str("noise error"),
        }
    }
}

impl core::error::Error for Error {}

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

/// Per-instance interrupt state shared between a UART driver and its IRQ
/// handler.
///
/// One `static` per instance is created by `impl_uart` and reached through
/// the instance type, so callers never supply or name it. The receive ring is only initialized by [`BufferedUart::new`]; the plain [`Uart`] leaves it empty.
pub struct State {
    rx_waker: AtomicWaker,
    tx_waker: AtomicWaker,
    /// Receive ring, filled by [`on_buffered_interrupt`] and drained by
    /// [`BufferedUartRx`]. Left uninitialized for the plain [`Uart`].
    rx_ring: RingBuffer,
    /// Latched when the ring overflowed or a line error was seen. Corrupt
    /// bytes are caught by the transport's frame CRC, so reads never fail on
    /// this; integrators can poll [`BufferedUartRx::take_overrun`].
    overrun: AtomicBool,
}

impl State {
    /// Create an empty state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            rx_waker: AtomicWaker::new(),
            tx_waker: AtomicWaker::new(),
            rx_ring: RingBuffer::new(),
            overrun: AtomicBool::new(false),
        }
    }

    /// Read and clear the latched overrun/line-error flag.
    fn take_overrun(&self) -> bool {
        self.overrun.swap(false, Ordering::Relaxed)
    }

    /// Handle the interrupt of the instance at `r` in buffered mode.
    ///
    /// Drains the receive FIFO into the ring and wakes the read future when
    /// bytes arrived; then services the transmit half exactly as
    /// `State::on_interrupt` does.
    fn on_buffered_interrupt(&self, r: LPUART) {
        critical_section::with(|_| {
            let stat = r.STAT().read();
            if stat.PF() || stat.FE() || stat.NF() || stat.OR() {
                // Error flags are write-1-to-clear; `write` zeroes the other
                // fields, which are no-ops for the read-only status bits.
                r.STAT().write(|w| {
                    w.set_PF(true);
                    w.set_FE(true);
                    w.set_NF(true);
                    w.set_OR(true);
                });
                self.overrun.store(true, Ordering::Relaxed);
            }

            // SAFETY: the ring has a single writer, exercised only in this IRQ
            // path (the reader lives in `BufferedUartRx`).
            let mut writer = unsafe { self.rx_ring.writer() };
            let mut pushed = false;
            while r.STAT().read().RDRF() {
                let byte = r.DATA().read().0 as u8;
                if writer.push_one(byte) {
                    pushed = true;
                } else {
                    // Ring full: the byte is dropped. Reading DATA already
                    // advanced the FIFO, so the loop still makes progress.
                    self.overrun.store(true, Ordering::Relaxed);
                    break;
                }
            }
            if pushed {
                self.rx_waker.wake();
            }

            service_tx(r, &self.tx_waker);
        });
    }

    /// Handle the interrupt of the instance at `r`.
    ///
    /// Disables the interrupt-enable bit that fired and wakes the pending
    /// future; the future re-enables it on the next poll if still waiting.
    fn on_interrupt(&self, r: LPUART) {
        let stat = r.STAT().read();
        critical_section::with(|_| {
            let ctrl = r.CTRL().read();
            if ctrl.RIE() && (stat.RDRF() || stat.OR()) {
                r.CTRL().modify(|w| w.set_RIE(false));
                self.rx_waker.wake();
            }
            service_tx(r, &self.tx_waker);
        });
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> LPUART;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An instance the [`Uart`] and [`BufferedUart`] drivers can drive.
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
macro_rules! impl_uart {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::uart::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::lpuart::LPUART {
                // SAFETY: `$base` is this instance's base as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe { crate::pac::peripherals::lpuart::LPUART::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::uart::State {
                static STATE: crate::uart::State = crate::uart::State::new();
                &STATE
            }
        }

        impl crate::uart::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_uart;

/// Handle the interrupt of instance `T` driven by a plain [`Uart`].
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own. Use [`on_buffered_interrupt`]
/// instead when the instance is driven by a [`BufferedUart`].
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// Handle the interrupt of instance `T` driven by a [`BufferedUart`].
pub fn on_buffered_interrupt<T: Instance>() {
    T::state().on_buffered_interrupt(T::regs());
}

/// Handle the transmit half of an LPUART interrupt: disable whichever of the
/// TX interrupt-enable bits fired and wake the TX future so it can refill.
///
/// Shared by [`on_interrupt`] and [`on_buffered_interrupt`]; must run inside a
/// critical section.
fn service_tx(r: LPUART, tx_waker: &AtomicWaker) {
    let stat = r.STAT().read();
    let ctrl = r.CTRL().read();
    if ctrl.TIE() && stat.TDRE() {
        r.CTRL().modify(|w| w.set_TIE(false));
        tx_waker.wake();
    }
    if ctrl.TCIE() && stat.TC() {
        r.CTRL().modify(|w| w.set_TCIE(false));
        tx_waker.wake();
    }
}

fn check_and_clear_errors(r: LPUART) -> Result<(), Error> {
    let stat = r.STAT().read();
    // STAT error flags are write-1-to-clear; `write` starts from the zeroed
    // register value, so only the targeted flag is affected.
    if stat.PF() {
        r.STAT().write(|w| w.set_PF(true));
        return Err(Error::Parity);
    }
    if stat.FE() {
        r.STAT().write(|w| w.set_FE(true));
        return Err(Error::Framing);
    }
    if stat.NF() {
        r.STAT().write(|w| w.set_NF(true));
        return Err(Error::Noise);
    }
    if stat.OR() {
        r.STAT().write(|w| w.set_OR(true));
        return Err(Error::Overrun);
    }
    Ok(())
}

/// Receive half of a [`Uart`].
pub struct UartRx {
    regs: LPUART,
    state: &'static State,
}

/// Transmit half of a [`Uart`].
pub struct UartTx {
    regs: LPUART,
    state: &'static State,
}

/// Full-duplex UART over one LPUART instance.
pub struct Uart {
    rx: UartRx,
    tx: UartTx,
}

impl Uart {
    /// Initialize an LPUART instance for 8N1 operation at `config.baudrate`.
    ///
    /// `clock_hz` is the instance's functional clock; the platform must leave
    /// the instance clocked and pinned out. For async I/O, forward the
    /// instance's interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32, config: Config) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();
        let state = T::state();

        let sbr = sbr_value(clock_hz, config.baudrate);

        r.GLOBAL().write(|w| w.set_RST(true));
        r.GLOBAL().write(|w| w.set_RST(false));
        r.BAUD().write(|w| {
            // OSR field value is the oversampling ratio minus one.
            w.set_OSR((OVERSAMPLING - 1) as u8);
            w.set_SBR(sbr);
        });
        r.STAT().write(|w| {
            w.set_PF(true);
            w.set_FE(true);
            w.set_NF(true);
            w.set_OR(true);
            w.set_IDLE(true);
        });
        r.CTRL().write(|w| {
            w.set_TE(true);
            w.set_RE(true);
        });

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

const fn sbr_value(clock_hz: u32, baudrate: u32) -> u16 {
    assert!(baudrate > 0);
    let divisor = baudrate as u64 * OVERSAMPLING as u64;
    let sbr = (clock_hz as u64 + divisor / 2) / divisor;
    assert!(sbr >= 1 && sbr <= SBR_MAX);
    sbr as u16
}

impl UartRx {
    fn try_read(&self, buf: &mut [u8]) -> Option<Result<usize, Error>> {
        if let Err(e) = check_and_clear_errors(self.regs) {
            return Some(Err(e));
        }
        if !self.regs.STAT().read().RDRF() {
            return None;
        }
        let mut n = 0;
        while n < buf.len() && self.regs.STAT().read().RDRF() {
            buf[n] = self.regs.DATA().read().0 as u8;
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
            critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_RIE(true)));
            if let Some(res) = self.try_read(buf) {
                critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_RIE(false)));
                return Poll::Ready(res);
            }
            Poll::Pending
        })
        .await
    }
}

impl UartTx {
    fn try_write(&self, buf: &[u8]) -> Option<usize> {
        if !self.regs.STAT().read().TDRE() {
            return None;
        }
        let mut n = 0;
        while n < buf.len() && self.regs.STAT().read().TDRE() {
            self.regs.DATA().write_value(LPUART_DATA(buf[n] as u32));
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
            critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_TIE(true)));
            if let Some(n) = self.try_write(buf) {
                critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_TIE(false)));
                return Poll::Ready(Ok(n));
            }
            Poll::Pending
        })
        .await
    }

    /// Wait until all queued data has left the shift register.
    pub async fn flush(&mut self) -> Result<(), Error> {
        poll_fn(|cx| {
            if self.regs.STAT().read().TC() {
                return Poll::Ready(Ok(()));
            }
            self.state.tx_waker.register(cx.waker());
            critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_TCIE(true)));
            if self.regs.STAT().read().TC() {
                critical_section::with(|_| self.regs.CTRL().modify(|w| w.set_TCIE(false)));
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
        Ok(self.regs.STAT().read().RDRF())
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
        while !self.regs.STAT().read().TC() {}
        Ok(())
    }
}

impl embedded_io::WriteReady for UartTx {
    fn write_ready(&mut self) -> Result<bool, Error> {
        Ok(self.regs.STAT().read().TDRE())
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
        if self.regs.STAT().read().RDRF() {
            Ok(self.regs.DATA().read().0 as u8)
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }
}

impl embedded_hal_nb::serial::Write<u8> for UartTx {
    fn write(&mut self, word: u8) -> embedded_hal_nb::nb::Result<(), Error> {
        if self.regs.STAT().read().TDRE() {
            self.regs.DATA().write_value(LPUART_DATA(word as u32));
            Ok(())
        } else {
            Err(embedded_hal_nb::nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> embedded_hal_nb::nb::Result<(), Error> {
        if self.regs.STAT().read().TC() {
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

/// Receive half of a [`BufferedUart`]: pops bytes the IRQ handler buffered
/// into the ring. Pass it to `Channel::<Rx, …>` via the UART transport.
pub struct BufferedUartRx {
    state: &'static State,
}

/// Full-duplex UART over one LPUART instance with an interrupt-buffered
/// receiver and a direct interrupt-driven transmitter.
pub struct BufferedUart {
    rx: BufferedUartRx,
    tx: UartTx,
}

impl BufferedUart {
    /// Initialize an LPUART instance for buffered 8N1 operation.
    ///
    /// Like [`Uart::new`] but the receive interrupt is left enabled so the IRQ
    /// handler can fill `rx_buf`, which backs the receive ring. `rx_buf` must
    /// be dedicated to this instance, and its interrupt must be forwarded to
    /// [`on_buffered_interrupt::<T>`](on_buffered_interrupt).
    pub fn new<T: Instance>(
        peri: Peri<'static, T>,
        clock_hz: u32,
        config: Config,
        rx_buf: &'static mut [u8],
    ) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let r = T::regs();
        let state = T::state();

        let sbr = sbr_value(clock_hz, config.baudrate);

        r.GLOBAL().write(|w| w.set_RST(true));
        r.GLOBAL().write(|w| w.set_RST(false));
        r.BAUD().write(|w| {
            w.set_OSR((OVERSAMPLING - 1) as u8);
            w.set_SBR(sbr);
        });
        r.STAT().write(|w| {
            w.set_PF(true);
            w.set_FE(true);
            w.set_NF(true);
            w.set_OR(true);
            w.set_IDLE(true);
        });

        // SAFETY: `rx_buf` is exclusively owned by this instance's ring and
        // initialized exactly once here, before any interrupt is routed.
        unsafe { state.rx_ring.init(rx_buf.as_mut_ptr(), rx_buf.len()) };

        // Enable the receiver interrupt last, once the ring is live.
        r.CTRL().write(|w| {
            w.set_TE(true);
            w.set_RE(true);
            w.set_RIE(true);
        });

        Self {
            rx: BufferedUartRx { state },
            tx: UartTx { regs: r, state },
        }
    }

    /// Split into independently owned transmit and receive halves.
    pub fn split(self) -> (UartTx, BufferedUartRx) {
        (self.tx, self.rx)
    }
}

impl BufferedUartRx {
    /// Read and clear the latched overrun/line-error flag.
    ///
    /// Set when the receive ring filled or the LPUART reported a line error.
    /// Corrupt bytes are caught by the transport's frame CRC, so reads never
    /// fail on this; poll it when you want to observe the loss.
    pub fn take_overrun(&self) -> bool {
        self.state.take_overrun()
    }

    /// Copy from the ring into `buf`, popping the bytes.
    ///
    /// Returns 0 when the ring is empty.
    fn drain_ring(&self, buf: &mut [u8]) -> usize {
        // SAFETY: the ring has a single reader, held only by this half.
        let mut reader = unsafe { self.state.rx_ring.reader() };
        let data = reader.pop_slice();
        let n = data.len().min(buf.len());
        if n > 0 {
            buf[..n].copy_from_slice(&data[..n]);
            reader.pop_done(n);
        }
        n
    }

    /// Read buffered bytes, waiting until at least one is available.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        poll_fn(|cx| {
            let n = self.drain_ring(buf);
            if n > 0 {
                return Poll::Ready(Ok(n));
            }
            self.state.rx_waker.register(cx.waker());
            // Re-check after registering so a byte buffered in the gap is not
            // missed until the next interrupt.
            let n = self.drain_ring(buf);
            if n > 0 {
                Poll::Ready(Ok(n))
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl embedded_io::ErrorType for BufferedUart {
    type Error = Error;
}
impl embedded_io::ErrorType for BufferedUartRx {
    type Error = Error;
}

impl embedded_io::Read for BufferedUartRx {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let n = self.drain_ring(buf);
            if n > 0 {
                return Ok(n);
            }
        }
    }
}

impl embedded_io::ReadReady for BufferedUartRx {
    fn read_ready(&mut self) -> Result<bool, Error> {
        Ok(!self.state.rx_ring.is_empty())
    }
}

impl embedded_io_async::Read for BufferedUartRx {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        BufferedUartRx::read(self, buf).await
    }
}

impl embedded_io::Read for BufferedUart {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        embedded_io::Read::read(&mut self.rx, buf)
    }
}

impl embedded_io::ReadReady for BufferedUart {
    fn read_ready(&mut self) -> Result<bool, Error> {
        embedded_io::ReadReady::read_ready(&mut self.rx)
    }
}

impl embedded_io::Write for BufferedUart {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        embedded_io::Write::write(&mut self.tx, buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        embedded_io::Write::flush(&mut self.tx)
    }
}

impl embedded_io::WriteReady for BufferedUart {
    fn write_ready(&mut self) -> Result<bool, Error> {
        embedded_io::WriteReady::write_ready(&mut self.tx)
    }
}

impl embedded_io_async::Read for BufferedUart {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        BufferedUartRx::read(&mut self.rx, buf).await
    }
}

impl embedded_io_async::Write for BufferedUart {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        UartTx::write(&mut self.tx, buf).await
    }

    async fn flush(&mut self) -> Result<(), Error> {
        UartTx::flush(&mut self.tx).await
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::boxed::Box;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Wake, Waker};

    use super::{State, sbr_value};
    use crate::pac::peripherals::lpuart::{LPUART, LPUART_DATA};

    #[test]
    fn sbr_rounds_to_nearest() {
        assert_eq!(sbr_value(80_000_000, 115_200), 43);
        assert_eq!(sbr_value(24_000_000, 115_200), 13);
    }

    #[test]
    #[should_panic]
    fn sbr_rejects_too_fast_baud() {
        sbr_value(100_000, 115_200);
    }

    #[test]
    #[should_panic]
    fn sbr_rejects_divider_overflow() {
        sbr_value(2_000_000_000, 110);
    }

    /// Word-aligned backing store standing in for the LPUART register block.
    /// Covers offsets through DATA (0x1c).
    struct FakeRegs {
        mem: Box<[u32; 0x40]>,
    }

    impl FakeRegs {
        fn new() -> Self {
            Self {
                mem: Box::new([0; 0x40]),
            }
        }

        fn regs(&mut self) -> LPUART {
            // SAFETY: `mem` is word-aligned and large enough for every register
            // the buffered driver touches; it outlives the returned handle.
            unsafe { LPUART::from_ptr(self.mem.as_mut_ptr() as *mut ()) }
        }
    }

    struct CountingWaker(AtomicUsize);
    impl Wake for CountingWaker {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn on_interrupt_drains_fifo_into_ring() {
        let mut fake = FakeRegs::new();
        let r = fake.regs();
        // A persistent RDRF with a fixed data word models a FIFO the fake
        // never empties; the drain stops once the ring fills.
        r.STAT().write(|w| w.set_RDRF(true));
        r.DATA().write_value(LPUART_DATA(0xab));

        let state = State::new();
        let mut ring = [0u8; 4];
        // SAFETY: single init, `ring` outlives all ring access below.
        unsafe { state.rx_ring.init(ring.as_mut_ptr(), ring.len()) };

        let counter = Arc::new(CountingWaker(AtomicUsize::new(0)));
        let waker = Waker::from(counter.clone());
        state.rx_waker.register(&waker);

        state.on_buffered_interrupt(r);

        // The ring filled to capacity, so the reader sees four bytes.
        assert_eq!(state.rx_ring.available(), 4);
        // The read future was woken.
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        // Filling the ring latched an overrun.
        assert!(state.take_overrun());
        assert!(!state.take_overrun());
    }

    #[test]
    fn on_interrupt_noop_without_rx_data() {
        let mut fake = FakeRegs::new();
        let r = fake.regs();

        let state = State::new();
        let mut ring = [0u8; 4];
        // SAFETY: single init, `ring` outlives all ring access below.
        unsafe { state.rx_ring.init(ring.as_mut_ptr(), ring.len()) };

        state.on_buffered_interrupt(r);

        assert_eq!(state.rx_ring.available(), 0);
        assert!(!state.take_overrun());
    }
}
