//! HPDMA channel driver for STM32MP2 devices.
//!
//! Any HPDMA instance/channel pair can back a [`DmaChannel`]; select it by
//! passing the instance MMIO base and channel index to [`DmaChannel::new`]
//! together with a `&'static` [`DmaState`] per channel. Transfers are single
//! linked-list-free blocks with byte-wide elements: memory-to-memory
//! transfers use the channel's software request, and peripheral transfers
//! move one byte per hardware request line. Completion can be awaited (route
//! the channel interrupt to [`DmaState::on_interrupt`]) or polled blocking.
//!
//! Buffers must be DMA-coherent; this driver performs no cache maintenance.
//! RCC clocking, RIF/secure configuration, and request-line numbers are
//! platform setup owned by the caller.

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::dma_channel::DMA_Channel;

use crate::pac;

const CCR_EN: u32 = 1 << 0;
const CCR_RESET: u32 = 1 << 1;
const CCR_SUSP: u32 = 1 << 2;
const CCR_TCIE: u32 = 1 << 8;
const CCR_DTEIE: u32 = 1 << 10;
const CCR_ULEIE: u32 = 1 << 11;
const CCR_USEIE: u32 = 1 << 12;
const CCR_IRQ_MASK: u32 = CCR_TCIE | CCR_DTEIE | CCR_ULEIE | CCR_USEIE;

const CSR_IDLEF: u32 = 1 << 0;
const CSR_TCF: u32 = 1 << 8;
const CSR_DTEF: u32 = 1 << 10;
const CSR_ULEF: u32 = 1 << 11;
const CSR_USEF: u32 = 1 << 12;
const CSR_SUSPF: u32 = 1 << 13;
const CSR_ERROR_MASK: u32 = CSR_DTEF | CSR_ULEF | CSR_USEF;
// CxFCR clear bits share the CxSR flag positions.
const FCR_CLEAR_ALL: u32 = 0x7f00;

const CTR1_SINC: u32 = 1 << 3;
const CTR1_DINC: u32 = 1 << 19;
const CTR2_SWREQ: u32 = 1 << 9;
const CTR2_DREQ: u32 = 1 << 10;
const CHANNEL_STRIDE: usize = 0x80;

/// Maximum length of one block transfer (16-bit `BNDT` byte count).
pub const MAX_TRANSFER: usize = 0xffff;

/// DMA transfer error, carrying the raw `CxSR` status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    /// Raw `CxSR` value captured when the error was observed.
    pub raw_sr: u32,
}

/// Per-channel interrupt state shared between a [`DmaChannel`] and its IRQ
/// handler.
pub struct DmaState {
    ch: AtomicPtr<()>,
    waker: AtomicWaker,
}

impl DmaState {
    /// Create an empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            ch: AtomicPtr::new(core::ptr::null_mut()),
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the channel's interrupt: mask the channel's interrupt enables
    /// and wake the waiter, which consumes and clears the flags.
    pub fn on_interrupt(&self) {
        let ptr = self.ch.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `DmaChannel::new` installs the channel block pointer for
        // the claimed channel before any interrupt is routed here.
        let ch = unsafe { DMA_Channel::from_ptr(ptr) };
        let sr = ch.CSR().read();
        if sr & (CSR_TCF | CSR_ERROR_MASK) != 0 {
            critical_section::with(|_| {
                ch.CCR().write_value(ch.CCR().read() & !CCR_IRQ_MASK);
            });
            self.waker.wake();
        }
    }
}

impl Default for DmaState {
    fn default() -> Self {
        Self::new()
    }
}

/// Exclusive handle to one HPDMA channel.
pub struct DmaChannel {
    ch: DMA_Channel,
    state: &'static DmaState,
}

impl DmaChannel {
    /// Claim channel `channel` of the HPDMA instance at `base`.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid HPDMA register block, clocked and
    /// RIF-configured by the caller, and `channel` must be exclusively owned
    /// by this handle for the lifetime of the program. `state` must be
    /// dedicated to this channel; for async use, route the channel's
    /// interrupt to [`DmaState::on_interrupt`] on the same state.
    pub unsafe fn new(base: *mut (), channel: usize, state: &'static DmaState) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned
        // HPDMA register block; the channel block reuses its layout at the
        // architectural 0x80 channel stride.
        let ch = unsafe {
            DMA_Channel::from_ptr((base as *mut u8).wrapping_add(channel * CHANNEL_STRIDE) as _)
        };
        state.ch.store(ch.as_ptr(), Ordering::Release);
        Self { ch, state }
    }

    fn setup(&mut self, src: u32, dst: u32, len: usize, tr1: u32, tr2: u32) {
        let ch = self.ch;
        ch.CFCR().write_value(FCR_CLEAR_ALL);
        ch.CTR1().write_value(tr1);
        ch.CTR2().write_value(tr2);
        ch.CBR1().write_value(len as u32);
        ch.CSAR().write_value(src);
        ch.CDAR().write_value(dst);
        ch.CLLR().write_value(0);
        ch.CCR().write_value(CCR_IRQ_MASK | CCR_EN);
    }

    /// Copy `src` into `dst` as one software-requested block transfer.
    ///
    /// Panics if the slice lengths differ, are zero, or exceed
    /// [`MAX_TRANSFER`].
    pub fn mem_to_mem<'a>(&'a mut self, src: &'a [u8], dst: &'a mut [u8]) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() == dst.len());
        assert!(src.len() <= MAX_TRANSFER);
        self.setup(
            src.as_ptr() as usize as u32,
            dst.as_mut_ptr() as usize as u32,
            src.len(),
            CTR1_SINC | CTR1_DINC,
            CTR2_SWREQ,
        );
        Transfer::new(self)
    }

    /// Stream `src` to a peripheral data register, one byte per hardware
    /// request on request line `request`.
    ///
    /// Panics if `src` is empty or longer than [`MAX_TRANSFER`].
    ///
    /// # Safety
    ///
    /// `dst` must be the DMA-capable data register of the peripheral that
    /// drives request line `request`, with its DMA request enabled.
    pub unsafe fn write_peripheral<'a>(
        &'a mut self,
        src: &'a [u8],
        dst: *mut u8,
        request: u8,
    ) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() <= MAX_TRANSFER);
        self.setup(
            src.as_ptr() as usize as u32,
            dst as usize as u32,
            src.len(),
            CTR1_SINC,
            CTR2_DREQ | request as u32,
        );
        Transfer::new(self)
    }

    /// Fill `dst` from a peripheral data register, one byte per hardware
    /// request on request line `request`.
    ///
    /// Panics if `dst` is empty or longer than [`MAX_TRANSFER`].
    ///
    /// # Safety
    ///
    /// `src` must be the DMA-capable data register of the peripheral that
    /// drives request line `request`, with its DMA request enabled.
    pub unsafe fn read_peripheral<'a>(
        &'a mut self,
        src: *const u8,
        dst: &'a mut [u8],
        request: u8,
    ) -> Transfer<'a> {
        assert!(!dst.is_empty() && dst.len() <= MAX_TRANSFER);
        self.setup(
            src as usize as u32,
            dst.as_mut_ptr() as usize as u32,
            dst.len(),
            CTR1_DINC,
            request as u32,
        );
        Transfer::new(self)
    }
}

/// In-flight DMA transfer. Dropping it halts the channel before the borrowed
/// buffers are released.
#[must_use = "a transfer makes no progress observation unless awaited or waited on"]
pub struct Transfer<'a> {
    ch: DMA_Channel,
    state: &'static DmaState,
    _channel: PhantomData<&'a mut DmaChannel>,
}

impl<'a> Transfer<'a> {
    fn new(channel: &'a mut DmaChannel) -> Self {
        Self {
            ch: channel.ch,
            state: channel.state,
            _channel: PhantomData,
        }
    }

    fn status(&self) -> Option<Result<(), Error>> {
        let sr = self.ch.CSR().read();
        if sr & CSR_ERROR_MASK != 0 {
            self.ch.CFCR().write_value(FCR_CLEAR_ALL);
            return Some(Err(Error { raw_sr: sr }));
        }
        if sr & CSR_TCF != 0 {
            self.ch.CFCR().write_value(FCR_CLEAR_ALL);
            return Some(Ok(()));
        }
        None
    }

    /// Busy-wait for completion.
    pub fn wait_blocking(self) -> Result<(), Error> {
        loop {
            if let Some(res) = self.status() {
                return res;
            }
        }
    }

    fn halt(&self) {
        let ch = self.ch;
        let ccr = ch.CCR().read();
        if ccr & CCR_EN != 0 && ch.CSR().read() & CSR_IDLEF == 0 {
            ch.CCR().write_value(ccr | CCR_SUSP);
            while ch.CSR().read() & (CSR_SUSPF | CSR_TCF) == 0 {}
            ch.CCR().write_value(ccr | CCR_SUSP | CCR_RESET);
        }
        ch.CFCR().write_value(FCR_CLEAR_ALL);
        ch.CCR().write_value(0);
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        if let Some(res) = self.status() {
            return Poll::Ready(res);
        }
        self.state.waker.register(cx.waker());
        // Re-arm the interrupt enables (the IRQ handler masks them) and
        // re-check to close the race against a completion in between.
        critical_section::with(|_| {
            self.ch
                .CCR()
                .write_value(self.ch.CCR().read() | CCR_IRQ_MASK);
        });
        if let Some(res) = self.status() {
            return Poll::Ready(res);
        }
        Poll::Pending
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        // Halt the hardware before the borrowed buffers become reusable.
        self.halt();
    }
}
