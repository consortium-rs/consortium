//! eDMA channel driver for i.MX9 devices.
//!
//! Any eDMA instance/channel pair can back a [`DmaChannel`]; select it by
//! passing the instance MMIO base and channel index to [`DmaChannel::new`]
//! together with a `&'static` [`DmaState`] per channel. Transfers are single
//! TCD blocks with byte-wide elements: memory-to-memory transfers run as one
//! software-started major loop, and peripheral transfers move one byte per
//! hardware request. Completion can be awaited (route the channel interrupt
//! to [`DmaState::on_interrupt`]) or polled blocking.
//!
//! Buffers must be DMA-coherent (e.g. TCM or non-cacheable regions); this
//! driver performs no cache maintenance. Hardware request routing (which
//! peripheral drives the channel) and clocking are platform setup owned by
//! the caller.

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::dma::{DMA, DMA_CH};

use crate::pac;

const CH_CSR_ERQ: u32 = 1 << 0;
const CH_CSR_DONE: u32 = 1 << 30;
const CH_CSR_ACTIVE: u32 = 1 << 31;
const CH_ES_ERR: u32 = 1 << 31;
const CH_INT_INT: u32 = 1 << 0;
const TCD_CSR_START: u16 = 1 << 0;
const TCD_CSR_INTMAJOR: u16 = 1 << 1;
const TCD_CSR_DREQ: u16 = 1 << 3;

/// Maximum length of a peripheral transfer (15-bit major loop counter).
pub const MAX_PERIPHERAL_TRANSFER: usize = 0x7fff;
/// Maximum length of a memory-to-memory transfer (30-bit minor byte count).
pub const MAX_MEM_TRANSFER: usize = 0x3fff_ffff;

/// DMA transfer error, carrying the raw `CH_ES` error status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    /// Raw `CH_ES` value captured when the error was observed.
    pub raw_es: u32,
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

    /// Handle the channel's interrupt: clear the request and wake the waiter.
    pub fn on_interrupt(&self) {
        let ptr = self.ch.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: `DmaChannel::new` installs the channel page pointer for the
        // claimed channel before any interrupt is routed here.
        let ch = unsafe { DMA_CH::from_ptr(ptr) };
        if ch.CH_INT().read() & CH_INT_INT != 0 {
            ch.CH_INT().write_value(CH_INT_INT);
            self.waker.wake();
        }
    }
}

impl Default for DmaState {
    fn default() -> Self {
        Self::new()
    }
}

/// Exclusive handle to one eDMA channel.
pub struct DmaChannel {
    ch: DMA_CH,
    state: &'static DmaState,
}

/// Transfer-control-descriptor parameters for one [`DmaChannel::setup`].
struct Tcd {
    src: u32,
    soff: u16,
    dst: u32,
    doff: u16,
    nbytes: u32,
    major_count: u16,
    tcd_csr: u16,
    erq: bool,
}

impl DmaChannel {
    /// Claim channel `channel` of the eDMA instance at `base`.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid eDMA register block, clocked by the
    /// caller, and `channel` must be exclusively owned by this handle for the
    /// lifetime of the program. `state` must be dedicated to this channel;
    /// for async use, route the channel's interrupt to
    /// [`DmaState::on_interrupt`] on the same state.
    pub unsafe fn new(base: *mut (), channel: usize, state: &'static DmaState) -> Self {
        // SAFETY: Caller guarantees `base` is a valid, exclusively owned eDMA
        // register block.
        let dma = unsafe { DMA::from_ptr(base) };
        let ch = dma.channel(channel);
        state.ch.store(ch.as_ptr(), Ordering::Release);
        Self { ch, state }
    }

    fn setup(&mut self, tcd: Tcd) {
        let ch = self.ch;
        ch.CH_CSR().write_value(CH_CSR_DONE);
        ch.CH_ES().write_value(CH_ES_ERR);
        ch.CH_INT().write_value(CH_INT_INT);
        ch.TCD_SADDR().write_value(tcd.src);
        ch.TCD_SOFF().write_value(tcd.soff);
        // 8-bit source and destination elements, no address modulo.
        ch.TCD_ATTR().write_value(0);
        ch.TCD_NBYTES_MLOFFNO().write_value(tcd.nbytes);
        ch.TCD_SLAST_SDA().write_value(0);
        ch.TCD_DADDR().write_value(tcd.dst);
        ch.TCD_DOFF().write_value(tcd.doff);
        ch.TCD_CITER_ELINKNO().write_value(tcd.major_count);
        ch.TCD_DLAST_SGA().write_value(0);
        ch.TCD_BITER_ELINKNO().write_value(tcd.major_count);
        ch.TCD_CSR().write_value(tcd.tcd_csr);
        if tcd.erq {
            ch.CH_CSR().write_value(CH_CSR_ERQ);
        }
    }

    /// Copy `src` into `dst` as one software-started block transfer.
    ///
    /// Panics if the slice lengths differ, are zero, or exceed
    /// [`MAX_MEM_TRANSFER`].
    pub fn mem_to_mem<'a>(&'a mut self, src: &'a [u8], dst: &'a mut [u8]) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() == dst.len());
        assert!(src.len() <= MAX_MEM_TRANSFER);
        self.setup(Tcd {
            src: src.as_ptr() as usize as u32,
            soff: 1,
            dst: dst.as_mut_ptr() as usize as u32,
            doff: 1,
            nbytes: src.len() as u32,
            major_count: 1,
            tcd_csr: TCD_CSR_INTMAJOR | TCD_CSR_START,
            erq: false,
        });
        Transfer::new(self)
    }

    /// Stream `src` to a peripheral data register, one byte per hardware
    /// request.
    ///
    /// Panics if `src` is empty or longer than [`MAX_PERIPHERAL_TRANSFER`].
    ///
    /// # Safety
    ///
    /// `dst` must be the DMA-capable data register of a peripheral whose
    /// hardware request is routed to this channel and enabled.
    pub unsafe fn write_peripheral<'a>(&'a mut self, src: &'a [u8], dst: *mut u8) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() <= MAX_PERIPHERAL_TRANSFER);
        self.setup(Tcd {
            src: src.as_ptr() as usize as u32,
            soff: 1,
            dst: dst as usize as u32,
            doff: 0,
            nbytes: 1,
            major_count: src.len() as u16,
            tcd_csr: TCD_CSR_INTMAJOR | TCD_CSR_DREQ,
            erq: true,
        });
        Transfer::new(self)
    }

    /// Fill `dst` from a peripheral data register, one byte per hardware
    /// request.
    ///
    /// Panics if `dst` is empty or longer than [`MAX_PERIPHERAL_TRANSFER`].
    ///
    /// # Safety
    ///
    /// `src` must be the DMA-capable data register of a peripheral whose
    /// hardware request is routed to this channel and enabled.
    pub unsafe fn read_peripheral<'a>(
        &'a mut self,
        src: *const u8,
        dst: &'a mut [u8],
    ) -> Transfer<'a> {
        assert!(!dst.is_empty() && dst.len() <= MAX_PERIPHERAL_TRANSFER);
        self.setup(Tcd {
            src: src as usize as u32,
            soff: 0,
            dst: dst.as_mut_ptr() as usize as u32,
            doff: 1,
            nbytes: 1,
            major_count: dst.len() as u16,
            tcd_csr: TCD_CSR_INTMAJOR | TCD_CSR_DREQ,
            erq: true,
        });
        Transfer::new(self)
    }
}

/// In-flight DMA transfer. Dropping it halts the channel before the borrowed
/// buffers are released.
#[must_use = "a transfer makes no progress observation unless awaited or waited on"]
pub struct Transfer<'a> {
    ch: DMA_CH,
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
        let es = self.ch.CH_ES().read();
        if es & CH_ES_ERR != 0 {
            self.ch.CH_ES().write_value(CH_ES_ERR);
            return Some(Err(Error { raw_es: es }));
        }
        if self.ch.CH_CSR().read() & CH_CSR_DONE != 0 {
            return Some(Ok(()));
        }
        None
    }

    /// Busy-wait for completion.
    pub fn wait_blocking(mut self) -> Result<(), Error> {
        loop {
            if let Some(res) = self.status() {
                self.finish();
                core::mem::forget(self);
                return res;
            }
        }
    }

    fn finish(&mut self) {
        let ch = self.ch;
        ch.CH_CSR().write_value(CH_CSR_DONE);
        ch.CH_INT().write_value(CH_INT_INT);
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        // INTMAJOR is set in every TCD, so completion raises the channel
        // interrupt; errors are surfaced by the DONE-less status check on the
        // wake that follows any polled progress.
        if let Some(res) = self.status() {
            let this = self.get_mut();
            this.finish();
            return Poll::Ready(res);
        }
        self.state.waker.register(cx.waker());
        if let Some(res) = self.status() {
            let this = self.get_mut();
            this.finish();
            return Poll::Ready(res);
        }
        Poll::Pending
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        let ch = self.ch;
        // Halt the hardware before the borrowed buffers become reusable:
        // disable requests, then wait out any in-flight minor loop.
        ch.CH_CSR().write_value(0);
        while ch.CH_CSR().read() & CH_CSR_ACTIVE != 0 {}
        self.finish();
        ch.CH_ES().write_value(CH_ES_ERR);
    }
}
