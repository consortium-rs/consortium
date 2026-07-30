//! eDMA transfers for i.MX9 devices.
//!
//! This module provides typed one-shot transfers and allocation-free circular
//! producer/consumer rings. Circular transfers use the eDMA major-loop address
//! adjustments (`SLAST`/`DLAST`) and half/major interrupts; no descriptor
//! allocation is required. Applications own clocks, RIF/TRDC setup, request
//! routing, interrupt vectors, and DMA-coherent memory placement.
//!
//! A channel is a peripheral singleton of its own, named `<CONTROLLER>_CH<n>`:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let channel = DmaChannel::new(p.DMA3_CH7);
//! ```

use core::future::{Future, poll_fn};
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{AtomicPtr, AtomicU8, AtomicU32, AtomicUsize, Ordering, fence};
use core::task::{Context, Poll};

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

const CH_CSR_ERQ: u32 = 1 << 0;
const CH_CSR_DONE: u32 = 1 << 30;
const CH_CSR_ACTIVE: u32 = 1 << 31;
const CH_ES_ERR: u32 = 1 << 31;
const CH_INT_INT: u32 = 1 << 0;
const TCD_CSR_START: u16 = 1 << 0;
const TCD_CSR_INTMAJOR: u16 = 1 << 1;
const TCD_CSR_INTHALF: u16 = 1 << 2;
const TCD_CSR_DREQ: u16 = 1 << 3;

const MODE_IDLE: u8 = 0;
const MODE_ONESHOT: u8 = 1;
const MODE_CIRCULAR: u8 = 2;
const LAYOUT_DMA3: u8 = 0;
const LAYOUT_DMA4: u8 = 1;
const LAYOUT_DMA5: u8 = 2;

/// Maximum major-loop count when channel linking is disabled.
pub const MAX_PERIPHERAL_TRANSFER: usize = 0x7fff;
/// Maximum byte count in one software-started minor loop.
pub const MAX_MEM_TRANSFER: usize = 0x3fff_ffff;

mod sealed {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}

/// An element type supported by eDMA.
pub trait DmaWord: sealed::Sealed + Copy + Default {
    #[doc(hidden)]
    const WIDTH_LOG2: u16;
}

impl DmaWord for u8 {
    const WIDTH_LOG2: u16 = 0;
}
impl DmaWord for u16 {
    const WIDTH_LOG2: u16 = 1;
}
impl DmaWord for u32 {
    const WIDTH_LOG2: u16 = 2;
}

/// eDMA controller class on which a peripheral request is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaController {
    /// Always-on/secure eDMA3.
    Dma3,
    /// i.MX93 eDMA4.
    Dma4,
    /// i.MX95 eDMA5.
    Dma5,
}

/// Chip-selected eDMA request source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaRequest {
    controller: DmaController,
    source: u8,
}

impl DmaRequest {
    /// Construct a request selector.
    pub const fn new(controller: DmaController, source: u8) -> Self {
        Self { controller, source }
    }

    /// Required eDMA controller.
    pub const fn controller(self) -> DmaController {
        self.controller
    }

    /// Raw channel-mux request source.
    pub const fn source(self) -> u8 {
        self.source
    }
}

/// ADC and TPM request selectors for the selected i.MX9 chip.
pub mod request {
    use super::{DmaController, DmaRequest};

    macro_rules! requests {
        ($controller:ident; $($name:ident = $value:expr),+ $(,)?) => {$(
            #[doc = concat!("eDMA request `", stringify!($name), "`.")]
            pub const $name: DmaRequest =
                DmaRequest::new(DmaController::$controller, $value);
        )+};
    }

    #[cfg(feature = "mimx93")]
    requests! { Dma3;
        LPTMR1 = 15,
        TPM1_CH0_CH2 = 23, TPM1_CH1_CH3 = 24, TPM1_OVERFLOW = 25,
        TPM2_CH0_CH2 = 26, TPM2_CH1_CH3 = 27, TPM2_OVERFLOW = 28,
        ADC1 = 30
    }
    #[cfg(feature = "mimx93")]
    requests! { Dma4;
        LPTMR2 = 16,
        TPM3_CH0_CH2 = 25, TPM3_CH1_CH3 = 26, TPM3_OVERFLOW = 27,
        TPM4_CH0_CH2 = 28, TPM4_CH1_CH3 = 29, TPM4_OVERFLOW = 30,
        TPM5_CH0_CH2 = 31, TPM5_CH1_CH3 = 32, TPM5_OVERFLOW = 33,
        TPM6_CH0_CH2 = 34, TPM6_CH1_CH3 = 35, TPM6_OVERFLOW = 36
    }

    #[cfg(feature = "mimx95")]
    requests! { Dma3;
        LPTMR1 = 2, TPM1_OVERFLOW = 4, TPM2_OVERFLOW = 5, ADC1 = 7,
        TPM1_CH0_CH2 = 26, TPM1_CH1_CH3 = 27,
        TPM2_CH0_CH2 = 28, TPM2_CH1_CH3 = 29
    }
    #[cfg(feature = "mimx95")]
    requests! { Dma5;
        LPTMR2 = 16,
        TPM3_CH0_CH2 = 25, TPM3_CH1_CH3 = 26, TPM3_OVERFLOW = 27,
        TPM4_CH0_CH2 = 28, TPM4_CH1_CH3 = 29, TPM4_OVERFLOW = 30,
        TPM5_CH0_CH2 = 31, TPM5_CH1_CH3 = 32, TPM5_OVERFLOW = 33,
        TPM6_CH0_CH2 = 34, TPM6_CH1_CH3 = 35, TPM6_OVERFLOW = 36
    }
}

/// DMA transfer error, carrying the raw `CH_ES` error status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    /// Raw channel error status.
    pub raw_es: u32,
}

/// Ring-buffer synchronization error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingError {
    /// Hardware DMA failure.
    Dma(Error),
    /// The producer lapped the consumer.
    Overrun,
    /// The consumer reached data not yet supplied by the producer.
    Underrun,
    /// The requested operation requires a started ring.
    NotStarted,
}

/// Per-channel state shared between a [`DmaChannel`] and its IRQ handler.
///
/// One `static` per channel singleton is created by `impl_dma_channel` and
/// reached through the instance type, so callers never supply or name it.
pub struct DmaState {
    ch: AtomicPtr<()>,
    layout: AtomicU8,
    mode: AtomicU8,
    half_irqs: AtomicUsize,
    error_status: AtomicU32,
    waker: AtomicWaker,
}

impl DmaState {
    /// Create empty state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            ch: AtomicPtr::new(core::ptr::null_mut()),
            layout: AtomicU8::new(LAYOUT_DMA3),
            mode: AtomicU8::new(MODE_IDLE),
            half_irqs: AtomicUsize::new(0),
            error_status: AtomicU32::new(0),
            waker: AtomicWaker::new(),
        }
    }

    /// Clear a channel interrupt, account for circular progress, and wake.
    pub fn on_interrupt(&self) {
        let ptr = self.ch.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        let ch = RawChannel {
            ptr: ptr.cast(),
            layout: self.layout.load(Ordering::Acquire),
        };
        let es = ch.read32(0x04);
        if es & CH_ES_ERR != 0 {
            self.error_status.store(es, Ordering::Release);
        }
        if ch.read32(0x08) & CH_INT_INT != 0 {
            ch.write32(0x08, CH_INT_INT);
            if self.mode.load(Ordering::Acquire) == MODE_CIRCULAR {
                self.half_irqs.fetch_add(1, Ordering::AcqRel);
            }
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// Management-page base of the controller this channel belongs to.
    fn management() -> *mut ();
    /// This channel's dedicated interrupt state.
    fn state() -> &'static DmaState;
}

/// One eDMA channel the [`DmaChannel`] driver can claim.
///
/// Implemented by the generated `<CONTROLLER>_CH<n>` singletons; sealed so the
/// controller base, class, and channel index stay single-sourced from the chip
/// database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// Controller class, which selects the TCD layout and mux scheme.
    const CONTROLLER: DmaController;
    /// Zero-based channel index within the controller.
    const CHANNEL: usize;
    /// NVIC line this channel raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
}

/// Bind a generated channel singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_dma_channel {
    ($instance:ident, $management:expr, $controller:ident, $channel:expr, $irq:expr) => {
        impl crate::dma::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn management() -> *mut () {
                $management as *mut ()
            }

            #[inline]
            fn state() -> &'static crate::dma::DmaState {
                static STATE: crate::dma::DmaState = crate::dma::DmaState::new();
                &STATE
            }
        }

        impl crate::dma::Instance for crate::peripherals::$instance {
            const CONTROLLER: crate::dma::DmaController = crate::dma::DmaController::$controller;
            const CHANNEL: usize = $channel;
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_dma_channel;

/// Handle the eDMA interrupt of channel `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt();
}

/// Exclusive handle to one eDMA channel.
pub struct DmaChannel {
    ch: RawChannel,
    channel: usize,
    controller: DmaController,
    mux: Option<*mut u32>,
    state: &'static DmaState,
}

#[derive(Clone, Copy)]
struct RawChannel {
    ptr: *mut u8,
    layout: u8,
}

// SAFETY: the channel page denotes hardware state, not Rust-owned memory.
// Exclusive mutation is enforced by the unsafe `DmaChannel` constructors.
unsafe impl Send for RawChannel {}
// SAFETY: IRQ and thread contexts may access disjoint/status registers through
// volatile operations; higher-level ownership serializes transfer setup.
unsafe impl Sync for RawChannel {}

impl RawChannel {
    fn read32(self, offset: usize) -> u32 {
        // SAFETY: constructors validate the controller layout and the caller
        // guarantees that its register block remains mapped.
        // SAFETY: `offset` is within the selected channel-page layout.
        let register = unsafe { self.ptr.add(offset) }.cast::<u32>();
        // SAFETY: the register is mapped, aligned, and readable.
        unsafe { register.read_volatile() }
    }

    fn write32(self, offset: usize, value: u32) {
        // SAFETY: same register-block invariant as `read32`.
        // SAFETY: `offset` is within the selected channel-page layout.
        let register = unsafe { self.ptr.add(offset) }.cast::<u32>();
        // SAFETY: the register is mapped, aligned, and writable.
        unsafe { register.write_volatile(value) }
    }

    fn read16(self, offset: usize) -> u16 {
        // SAFETY: same register-block invariant as `read32`; all supplied
        // offsets are halfword aligned.
        // SAFETY: `offset` is within the selected channel-page layout.
        let register = unsafe { self.ptr.add(offset) }.cast::<u16>();
        // SAFETY: the register is mapped, aligned, and readable.
        unsafe { register.read_volatile() }
    }

    fn write16(self, offset: usize, value: u16) {
        // SAFETY: same register-block invariant as `read16`.
        // SAFETY: `offset` is within the selected channel-page layout.
        let register = unsafe { self.ptr.add(offset) }.cast::<u16>();
        // SAFETY: the register is mapped, aligned, and writable.
        unsafe { register.write_volatile(value) }
    }

    const fn soff_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x28
        } else {
            0x24
        }
    }

    const fn attr_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x2a
        } else {
            0x26
        }
    }

    const fn nbytes_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x2c
        } else {
            0x28
        }
    }

    const fn slast_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x30
        } else {
            0x2c
        }
    }

    const fn daddr_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x38
        } else {
            0x30
        }
    }

    const fn doff_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x48
        } else {
            0x34
        }
    }

    const fn citer_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x4a
        } else {
            0x36
        }
    }

    const fn dlast_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x40
        } else {
            0x38
        }
    }

    const fn tcd_csr_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x4c
        } else {
            0x3c
        }
    }

    const fn biter_offset(self) -> usize {
        if self.layout == LAYOUT_DMA5 {
            0x4e
        } else {
            0x3e
        }
    }
}

struct Tcd {
    src: u32,
    soff: u16,
    attr: u16,
    slast: u32,
    dst: u32,
    doff: u16,
    dlast: u32,
    nbytes: u32,
    major_count: u16,
    csr: u16,
    erq: bool,
}

impl DmaChannel {
    /// Claim an eDMA channel from its peripheral singleton.
    ///
    /// The platform must leave the controller clocked and RIF/TRDC-configured.
    /// Route the channel's IRQ to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // channel access safe; it is consumed here and never reissued.
        let _ = peri;
        Self::claim(T::CONTROLLER, T::management(), T::CHANNEL, T::state())
    }

    /// Derive the channel page, TCD layout, and request mux from the
    /// controller's management page.
    ///
    /// `pub(crate)` so tests, and drivers that stand an array in for a
    /// controller, can build a channel without a singleton.
    pub(crate) fn claim(
        controller: DmaController,
        base: *mut (),
        channel: usize,
        state: &'static DmaState,
    ) -> Self {
        let (layout, count, stride) = match controller {
            DmaController::Dma3 => (LAYOUT_DMA3, 31, 0x1_0000),
            DmaController::Dma4 => (LAYOUT_DMA4, 64, 0x1000),
            DmaController::Dma5 => (LAYOUT_DMA5, 64, 0x8000),
        };
        assert!(channel < count, "eDMA channel index out of range");
        let management = base.cast::<u8>();
        let ch = RawChannel {
            ptr: management.wrapping_add(0x1_0000 + channel * stride),
            layout,
        };
        let mux = match controller {
            DmaController::Dma3 => None,
            DmaController::Dma4 => Some(ch.ptr.wrapping_add(0x14).cast()),
            DmaController::Dma5 => Some(management.wrapping_add(0x200 + channel * 4).cast()),
        };
        state.layout.store(layout, Ordering::Release);
        state.ch.store(ch.ptr.cast(), Ordering::Release);
        Self {
            ch,
            channel,
            controller,
            mux,
            state,
        }
    }

    /// Controller class of this channel.
    pub const fn controller(&self) -> DmaController {
        self.controller
    }

    fn check_request(&self, request: DmaRequest) {
        assert_eq!(
            self.controller, request.controller,
            "DMA request is routed through a different eDMA controller"
        );
        if self.controller == DmaController::Dma3 {
            assert_eq!(
                self.channel, request.source as usize,
                "eDMA3 has fixed request-to-channel routing"
            );
        } else if let Some(mux) = self.mux {
            // SAFETY: `new_on` derived the controller-specific mux register
            // from the caller-validated management page.
            unsafe { mux.write_volatile(u32::from(request.source)) };
        }
    }

    fn setup(&mut self, tcd: Tcd, mode: u8) {
        self.halt();
        let ch = self.ch;
        ch.write32(0x04, CH_ES_ERR);
        ch.write32(0x08, CH_INT_INT);
        self.state.half_irqs.store(0, Ordering::Release);
        self.state.error_status.store(0, Ordering::Release);
        ch.write32(0x20, tcd.src);
        if ch.layout == LAYOUT_DMA5 {
            ch.write32(0x24, 0);
        }
        ch.write16(ch.soff_offset(), tcd.soff);
        ch.write16(ch.attr_offset(), tcd.attr);
        ch.write32(ch.nbytes_offset(), tcd.nbytes);
        ch.write32(ch.slast_offset(), tcd.slast);
        if ch.layout == LAYOUT_DMA5 {
            ch.write32(0x34, if tcd.slast == 0 { 0 } else { u32::MAX });
        }
        ch.write32(ch.daddr_offset(), tcd.dst);
        if ch.layout == LAYOUT_DMA5 {
            ch.write32(0x3c, 0);
        }
        ch.write16(ch.doff_offset(), tcd.doff);
        ch.write16(ch.citer_offset(), tcd.major_count);
        ch.write32(ch.dlast_offset(), tcd.dlast);
        if ch.layout == LAYOUT_DMA5 {
            ch.write32(0x44, if tcd.dlast == 0 { 0 } else { u32::MAX });
        }
        ch.write16(ch.biter_offset(), tcd.major_count);
        self.state.mode.store(mode, Ordering::Release);
        fence(Ordering::SeqCst);
        ch.write16(ch.tcd_csr_offset(), tcd.csr);
        if tcd.erq {
            ch.write32(0x00, CH_CSR_ERQ);
        }
    }

    /// Copy one equal-length typed block.
    pub fn mem_to_mem<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: &'a mut [W],
    ) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() == dst.len());
        let bytes = byte_len::<W>(src.len(), MAX_MEM_TRANSFER);
        self.setup(
            Tcd {
                src: address(src.as_ptr()),
                soff: size_of_word::<W>(),
                attr: attributes::<W>(),
                slast: 0,
                dst: address(dst.as_mut_ptr()),
                doff: size_of_word::<W>(),
                dlast: 0,
                nbytes: bytes as u32,
                major_count: 1,
                csr: TCD_CSR_INTMAJOR | TCD_CSR_START,
                erq: false,
            },
            MODE_ONESHOT,
        );
        Transfer::new(self)
    }

    /// Write a finite typed block to a peripheral register.
    ///
    /// # Safety
    ///
    /// `dst` must be the data register corresponding to `request`, and the
    /// platform must route that request to this channel.
    pub unsafe fn write_peripheral<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: *mut W,
    ) -> Transfer<'a> {
        // SAFETY: the caller owns request routing and validates the register.
        unsafe { self.write_peripheral_inner(src, dst) }
    }

    /// Write a finite typed block using a validated request selector.
    ///
    /// # Safety
    ///
    /// `dst` must be the data register corresponding to `request`, and the
    /// platform must configure any programmable channel mux.
    pub unsafe fn write_peripheral_with_request<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: *mut W,
        request: DmaRequest,
    ) -> Transfer<'a> {
        self.check_request(request);
        // SAFETY: forwarded caller contract after request validation.
        unsafe { self.write_peripheral_inner(src, dst) }
    }

    unsafe fn write_peripheral_inner<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: *mut W,
    ) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() <= MAX_PERIPHERAL_TRANSFER);
        self.setup(
            Tcd {
                src: address(src.as_ptr()),
                soff: size_of_word::<W>(),
                attr: attributes::<W>(),
                slast: 0,
                dst: address(dst),
                doff: 0,
                dlast: 0,
                nbytes: core::mem::size_of::<W>() as u32,
                major_count: src.len() as u16,
                csr: TCD_CSR_INTMAJOR | TCD_CSR_DREQ,
                erq: true,
            },
            MODE_ONESHOT,
        );
        Transfer::new(self)
    }

    /// Read a finite typed peripheral stream into memory.
    ///
    /// # Safety
    ///
    /// `src` must be the data register corresponding to `request`, and the
    /// platform must route that request to this channel.
    pub unsafe fn read_peripheral<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        dst: &'a mut [W],
    ) -> Transfer<'a> {
        // SAFETY: the caller owns request routing and validates the register.
        unsafe { self.read_peripheral_inner(src, dst) }
    }

    /// Read a finite typed stream using a validated request selector.
    ///
    /// # Safety
    ///
    /// `src` must be the data register corresponding to `request`, and the
    /// platform must configure any programmable channel mux.
    pub unsafe fn read_peripheral_with_request<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        dst: &'a mut [W],
        request: DmaRequest,
    ) -> Transfer<'a> {
        self.check_request(request);
        // SAFETY: forwarded caller contract after request validation.
        unsafe { self.read_peripheral_inner(src, dst) }
    }

    unsafe fn read_peripheral_inner<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        dst: &'a mut [W],
    ) -> Transfer<'a> {
        assert!(!dst.is_empty() && dst.len() <= MAX_PERIPHERAL_TRANSFER);
        self.setup(
            Tcd {
                src: address(src),
                soff: 0,
                attr: attributes::<W>(),
                slast: 0,
                dst: address(dst.as_mut_ptr()),
                doff: size_of_word::<W>(),
                dlast: 0,
                nbytes: core::mem::size_of::<W>() as u32,
                major_count: dst.len() as u16,
                csr: TCD_CSR_INTMAJOR | TCD_CSR_DREQ,
                erq: true,
            },
            MODE_ONESHOT,
        );
        Transfer::new(self)
    }

    /// Construct a circular peripheral-to-memory ring.
    ///
    /// # Safety
    ///
    /// `src` must be the data register corresponding to `request`.
    pub unsafe fn read_circular<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        buffer: &'a mut [W],
        request: DmaRequest,
    ) -> ReadableRingBuffer<'a, W> {
        self.check_request(request);
        validate_ring(buffer.len());
        ReadableRingBuffer {
            channel: self,
            src,
            buffer,
            request,
            consumed: 0,
            started: false,
        }
    }

    /// Construct a circular memory-to-peripheral ring.
    ///
    /// # Safety
    ///
    /// `dst` must be the data register corresponding to `request`.
    pub unsafe fn write_circular<'a, W: DmaWord>(
        &'a mut self,
        buffer: &'a mut [W],
        dst: *mut W,
        request: DmaRequest,
    ) -> WritableRingBuffer<'a, W> {
        self.check_request(request);
        validate_ring(buffer.len());
        WritableRingBuffer {
            channel: self,
            buffer,
            dst,
            request,
            produced: 0,
            started: false,
        }
    }

    fn start_circular<W: DmaWord>(
        &mut self,
        memory: *mut W,
        peripheral: *mut W,
        len: usize,
        request: DmaRequest,
        direction: Direction,
    ) {
        self.check_request(request);
        let bytes = byte_len::<W>(len, MAX_PERIPHERAL_TRANSFER * core::mem::size_of::<W>());
        let rewind = 0u32.wrapping_sub(bytes as u32);
        let (src, soff, slast, dst, doff, dlast) = match direction {
            Direction::MemoryToPeripheral => (
                address(memory),
                size_of_word::<W>(),
                rewind,
                address(peripheral),
                0,
                0,
            ),
            Direction::PeripheralToMemory => (
                address(peripheral),
                0,
                0,
                address(memory),
                size_of_word::<W>(),
                rewind,
            ),
        };
        self.setup(
            Tcd {
                src,
                soff,
                attr: attributes::<W>(),
                slast,
                dst,
                doff,
                dlast,
                nbytes: core::mem::size_of::<W>() as u32,
                major_count: len as u16,
                csr: TCD_CSR_INTHALF | TCD_CSR_INTMAJOR,
                erq: true,
            },
            MODE_CIRCULAR,
        );
    }

    fn progress(&self, capacity: usize) -> usize {
        critical_section::with(|_| {
            let half_irqs = self.state.half_irqs.load(Ordering::Acquire);
            let half = capacity / 2;
            let remaining = (self.ch.read16(self.ch.citer_offset()) as usize).min(capacity);
            let within = capacity - remaining;
            // IRQ count is the stable lap source. Clamp the live position to
            // the half indicated by the last serviced IRQ to avoid counting a
            // concurrently arriving half twice.
            let base = half_irqs.saturating_mul(half);
            let phase_start = (half_irqs % 2) * half;
            base.saturating_add(within.saturating_sub(phase_start).min(half))
        })
    }

    fn circular_error(&self) -> Option<RingError> {
        let es = self.state.error_status.load(Ordering::Acquire);
        (es & CH_ES_ERR != 0).then_some(RingError::Dma(Error { raw_es: es }))
    }

    fn halt(&self) {
        self.ch.write32(0x00, 0);
        while self.ch.read32(0x00) & CH_CSR_ACTIVE != 0 {}
        self.ch.write32(0x00, CH_CSR_DONE);
        self.ch.write32(0x08, CH_INT_INT);
        self.ch.write32(0x04, CH_ES_ERR);
        self.state.mode.store(MODE_IDLE, Ordering::Release);
    }

    fn pause(&self) {
        self.ch.write32(0x00, 0);
        while self.ch.read32(0x00) & CH_CSR_ACTIVE != 0 {}
    }

    fn resume(&self) {
        self.ch.write32(0x00, CH_CSR_ERQ);
    }

    fn is_running(&self) -> bool {
        self.ch.read32(0x00) & CH_CSR_ERQ != 0
    }
}

enum Direction {
    MemoryToPeripheral,
    PeripheralToMemory,
}

/// In-flight one-shot DMA transfer.
#[must_use]
pub struct Transfer<'a> {
    ch: RawChannel,
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
        let es = self.ch.read32(0x04);
        if es & CH_ES_ERR != 0 {
            return Some(Err(Error { raw_es: es }));
        }
        (self.ch.read32(0x00) & CH_CSR_DONE != 0).then_some(Ok(()))
    }

    /// Busy-wait for completion.
    pub fn wait_blocking(self) -> Result<(), Error> {
        loop {
            if let Some(result) = self.status() {
                return result;
            }
        }
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(result) = self.status() {
            return Poll::Ready(result);
        }
        self.state.waker.register(cx.waker());
        self.status().map_or(Poll::Pending, Poll::Ready)
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        self.ch.write32(0x00, 0);
        while self.ch.read32(0x00) & CH_CSR_ACTIVE != 0 {}
        self.ch.write32(0x00, CH_CSR_DONE);
        self.ch.write32(0x08, CH_INT_INT);
        self.ch.write32(0x04, CH_ES_ERR);
        self.state.mode.store(MODE_IDLE, Ordering::Release);
    }
}

/// Circular peripheral-to-memory consumer.
pub struct ReadableRingBuffer<'a, W: DmaWord> {
    channel: &'a mut DmaChannel,
    src: *const W,
    buffer: &'a mut [W],
    request: DmaRequest,
    consumed: usize,
    started: bool,
}

impl<W: DmaWord> ReadableRingBuffer<'_, W> {
    /// Start hardware requests.
    pub fn start(&mut self) {
        if !self.started {
            self.channel.start_circular(
                self.buffer.as_mut_ptr(),
                self.src.cast_mut(),
                self.buffer.len(),
                self.request,
                Direction::PeripheralToMemory,
            );
            self.consumed = 0;
            self.started = true;
        }
    }

    fn produced(&self) -> Result<usize, RingError> {
        if !self.started {
            return Err(RingError::NotStarted);
        }
        if let Some(error) = self.channel.circular_error() {
            return Err(error);
        }
        Ok(self.channel.progress(self.buffer.len()))
    }

    /// Number of unread elements.
    pub fn len(&self) -> Result<usize, RingError> {
        let available = self.produced()?.saturating_sub(self.consumed);
        if available > self.buffer.len() {
            Err(RingError::Overrun)
        } else {
            Ok(available)
        }
    }

    /// Whether no samples are available.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.len().map(|len| len == 0)
    }

    /// Read immediately available elements.
    pub fn read(&mut self, output: &mut [W]) -> Result<(usize, usize), RingError> {
        let available = self.len()?;
        let count = available.min(output.len());
        copy_from_ring(self.buffer, self.consumed, &mut output[..count]);
        self.consumed += count;
        Ok((count, available - count))
    }

    /// Wait until all of `output` is filled.
    pub async fn read_exact(&mut self, output: &mut [W]) -> Result<usize, RingError> {
        let mut offset = 0;
        while offset < output.len() {
            let read = poll_fn(|cx| {
                self.channel.state.waker.register(cx.waker());
                match self.read(&mut output[offset..]) {
                    Ok((0, _)) => Poll::Pending,
                    result => Poll::Ready(result),
                }
            })
            .await?
            .0;
            offset += read;
        }
        self.len()
    }

    /// Return the newest samples, discarding older unread data.
    pub fn read_latest(&mut self, output: &mut [W]) -> Result<usize, RingError> {
        let produced = self.produced()?;
        let count = output.len().min(self.buffer.len()).min(produced);
        self.consumed = produced - count;
        copy_from_ring(self.buffer, self.consumed, &mut output[..count]);
        self.consumed = produced;
        Ok(count)
    }

    /// Ring capacity.
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Pause after the current minor transfer.
    pub fn pause(&mut self) {
        self.channel.pause();
    }

    /// Resume hardware requests.
    pub fn resume(&mut self) {
        self.channel.resume();
    }

    /// Whether requests are enabled.
    pub fn is_running(&self) -> bool {
        self.channel.is_running()
    }

    /// Stop the ring safely.
    pub async fn stop(&mut self) {
        self.channel.pause();
        self.channel.halt();
        self.started = false;
    }
}

impl<W: DmaWord> Drop for ReadableRingBuffer<'_, W> {
    fn drop(&mut self) {
        self.channel.halt();
    }
}

/// Circular memory-to-peripheral producer.
pub struct WritableRingBuffer<'a, W: DmaWord> {
    channel: &'a mut DmaChannel,
    buffer: &'a mut [W],
    dst: *mut W,
    request: DmaRequest,
    produced: usize,
    started: bool,
}

impl<W: DmaWord> WritableRingBuffer<'_, W> {
    /// Fill free ring space before or during operation.
    pub fn write_immediate(&mut self, input: &[W]) -> Result<(usize, usize), RingError> {
        self.write_inner(input, false)
    }

    /// Start after the complete ring has been primed.
    pub fn start(&mut self) {
        assert!(
            self.produced >= self.buffer.len(),
            "prime the complete DMA ring before starting"
        );
        if !self.started {
            self.channel.start_circular(
                self.buffer.as_mut_ptr(),
                self.dst,
                self.buffer.len(),
                self.request,
                Direction::MemoryToPeripheral,
            );
            self.started = true;
        }
    }

    fn consumed(&self) -> Result<usize, RingError> {
        if !self.started {
            return Ok(0);
        }
        if let Some(error) = self.channel.circular_error() {
            return Err(error);
        }
        let consumed = self.channel.progress(self.buffer.len());
        if consumed > self.produced {
            Err(RingError::Underrun)
        } else {
            Ok(consumed)
        }
    }

    /// Number of queued elements.
    pub fn len(&self) -> Result<usize, RingError> {
        Ok(self.produced.saturating_sub(self.consumed()?))
    }

    /// Whether no elements are queued.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.len().map(|len| len == 0)
    }

    fn write_inner(
        &mut self,
        input: &[W],
        require_started: bool,
    ) -> Result<(usize, usize), RingError> {
        if require_started && !self.started {
            return Err(RingError::NotStarted);
        }
        let consumed = self.consumed()?;
        let queued = self.produced.saturating_sub(consumed);
        let writable = self.buffer.len().saturating_sub(queued);
        let count = writable.min(input.len());
        copy_into_ring(self.buffer, self.produced, &input[..count]);
        self.produced += count;
        Ok((count, writable - count))
    }

    /// Write to a started ring.
    pub fn write(&mut self, input: &[W]) -> Result<(usize, usize), RingError> {
        self.write_inner(input, true)
    }

    /// Wait until all input is queued.
    pub async fn write_exact(&mut self, input: &[W]) -> Result<usize, RingError> {
        let mut offset = 0;
        while offset < input.len() {
            let written = poll_fn(|cx| {
                self.channel.state.waker.register(cx.waker());
                match self.write(&input[offset..]) {
                    Ok((0, _)) => Poll::Pending,
                    result => Poll::Ready(result),
                }
            })
            .await?
            .0;
            offset += written;
        }
        self.len()
    }

    /// Ring capacity.
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// Pause after the current minor transfer.
    pub fn pause(&mut self) {
        self.channel.pause();
    }

    /// Resume hardware requests.
    pub fn resume(&mut self) {
        self.channel.resume();
    }

    /// Whether requests are enabled.
    pub fn is_running(&self) -> bool {
        self.channel.is_running()
    }

    /// Stop the ring safely.
    pub async fn stop(&mut self) {
        self.channel.pause();
        self.channel.halt();
        self.started = false;
    }
}

impl<W: DmaWord> Drop for WritableRingBuffer<'_, W> {
    fn drop(&mut self) {
        self.channel.halt();
    }
}

const fn attributes<W: DmaWord>() -> u16 {
    W::WIDTH_LOG2 | (W::WIDTH_LOG2 << 8)
}

const fn size_of_word<W: DmaWord>() -> u16 {
    core::mem::size_of::<W>() as u16
}

fn byte_len<W: DmaWord>(elements: usize, maximum: usize) -> usize {
    let bytes = elements
        .checked_mul(core::mem::size_of::<W>())
        .expect("DMA byte length overflow");
    assert!((1..=maximum).contains(&bytes));
    bytes
}

fn validate_ring(len: usize) {
    assert!(
        (2..=MAX_PERIPHERAL_TRANSFER).contains(&len) && len.is_multiple_of(2),
        "DMA ring length must be even and fit the major-loop counter"
    );
}

fn address<T>(pointer: *const T) -> u32 {
    pointer as usize as u32
}

fn copy_from_ring<W: Copy>(ring: &[W], absolute: usize, output: &mut [W]) {
    for (offset, value) in output.iter_mut().enumerate() {
        *value = ring[(absolute + offset) % ring.len()];
    }
}

fn copy_into_ring<W: Copy>(ring: &mut [W], absolute: usize, input: &[W]) {
    let len = ring.len();
    for (offset, value) in input.iter().copied().enumerate() {
        ring[(absolute + offset) % len] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn channel(registers: &mut [u32; 0x4010], state: &'static DmaState) -> DmaChannel {
        // `registers` stands in for the eDMA3 management page; the `&mut` borrow
        // keeps it alive and unaliased for the returned channel's lifetime.
        DmaChannel::claim(DmaController::Dma3, registers.as_mut_ptr().cast(), 0, state)
    }

    #[test]
    fn halfword_write_programs_width_and_count() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 0x4010];
        let mut dma = channel(&mut registers, &STATE);
        let data = [1u16, 2, 3];
        // Use a controller-compatible synthetic request.
        // SAFETY: fake destination is never accessed by hardware.
        let transfer = unsafe {
            dma.write_peripheral_with_request(
                &data,
                0x5000_0000usize as *mut u16,
                DmaRequest::new(DmaController::Dma3, 0),
            )
        };
        let channel = 0x1_0000 / 4;
        assert_eq!(registers[channel + 0x26 / 4] >> 16, 0x0101);
        assert_eq!(registers[channel + 0x28 / 4], 2);
        assert_eq!(registers[channel + 0x36 / 4] >> 16, 3);
        core::mem::forget(transfer);
    }

    #[test]
    #[should_panic(expected = "different eDMA controller")]
    fn rejects_request_from_other_controller() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 0x4010];
        let mut dma = channel(&mut registers, &STATE);
        let data = [1u16, 2];
        // SAFETY: the assertion fires before fake MMIO can start a transfer.
        drop(unsafe {
            dma.write_peripheral_with_request(
                &data,
                0x5000_0000usize as *mut u16,
                DmaRequest::new(DmaController::Dma5, 25),
            )
        });
    }

    #[test]
    fn circular_read_rewinds_and_accounts_half_irqs() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 0x4010];
        let mut dma = channel(&mut registers, &STATE);
        let mut samples = [0u16; 4];
        let request = DmaRequest::new(DmaController::Dma3, 0);
        // SAFETY: fake source is never accessed by hardware.
        let mut ring =
            unsafe { dma.read_circular(0x5000_0000usize as *const u16, &mut samples, request) };
        ring.start();

        let channel = 0x1_0000 / 4;
        // Model W1C error status after setup cleared it.
        registers[channel + 0x04 / 4] = 0;
        assert_eq!(registers[channel + 0x28 / 4], 2);
        assert_eq!(registers[channel + 0x38 / 4], 0u32.wrapping_sub(8));
        assert_eq!(
            registers[channel + 0x3c / 4] & 0xffff,
            u32::from(TCD_CSR_INTHALF | TCD_CSR_INTMAJOR)
        );

        // Half complete: CITER is the high halfword at offset 0x36.
        registers[channel + 0x34 / 4] = (registers[channel + 0x34 / 4] & 0xffff) | (2 << 16);
        registers[channel + 0x08 / 4] = CH_INT_INT;
        STATE.on_interrupt();
        assert_eq!(ring.len(), Ok(2));

        // Major loop reloads CITER before the completion interrupt.
        registers[channel + 0x34 / 4] = (registers[channel + 0x34 / 4] & 0xffff) | (4 << 16);
        assert_eq!(registers[channel + 0x34 / 4] >> 16, 4);
        STATE.on_interrupt();
        assert_eq!(ring.len(), Ok(4));
    }

    #[test]
    fn dma4_programs_channel_mux_and_stride() {
        static STATE: DmaState = DmaState::new();
        let channel_index = 3usize;
        let mut registers = std::vec![0u32; (0x1_0000 + 4 * 0x1000) / 4];
        // The allocation covers the management page plus every channel page.
        let mut dma = DmaChannel::claim(
            DmaController::Dma4,
            registers.as_mut_ptr().cast(),
            channel_index,
            &STATE,
        );
        let data = [1u16, 2];
        // SAFETY: fake destination is never accessed by hardware.
        let transfer = unsafe {
            dma.write_peripheral_with_request(
                &data,
                0x5000_0000usize as *mut u16,
                DmaRequest::new(DmaController::Dma4, 25),
            )
        };
        let channel = (0x1_0000 + channel_index * 0x1000) / 4;
        assert_eq!(registers[channel + 0x14 / 4], 25);
        assert_eq!(registers[channel + 0x28 / 4], 2);
        core::mem::forget(transfer);
    }

    #[test]
    fn dma5_programs_mux_and_extended_tcd_layout() {
        static STATE: DmaState = DmaState::new();
        let channel_index = 1usize;
        let mut registers = std::vec![0u32; (0x1_0000 + 2 * 0x8000) / 4];
        // The allocation covers the management page plus every channel page.
        let mut dma = DmaChannel::claim(
            DmaController::Dma5,
            registers.as_mut_ptr().cast(),
            channel_index,
            &STATE,
        );
        let data = [1u32, 2, 3];
        // SAFETY: fake destination is never accessed by hardware.
        let transfer = unsafe {
            dma.write_peripheral_with_request(
                &data,
                0x5000_0000usize as *mut u32,
                DmaRequest::new(DmaController::Dma5, 25),
            )
        };
        let channel = (0x1_0000 + channel_index * 0x8000) / 4;
        assert_eq!(registers[0x204 / 4], 25);
        assert_eq!(registers[channel + 0x2c / 4], 4);
        assert_eq!(registers[channel + 0x48 / 4] >> 16, 3);
        core::mem::forget(transfer);
    }
}
