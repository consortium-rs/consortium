//! HPDMA transfers for STM32MP2.
//!
//! The driver supports byte, halfword, and word one-shot transfers plus
//! circular linked-list transfers. Applications own RCC/RIF setup, interrupt
//! vectors, and DMA-coherent buffer placement. Forward the selected channel
//! interrupt to [`DmaState::on_interrupt`].

use core::future::{Future, poll_fn};
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{AtomicPtr, AtomicU8, AtomicU32, AtomicUsize, Ordering, fence};
use core::task::{Context, Poll};

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::dma_channel::DMA_Channel;

use crate::pac;

const CCR_EN: u32 = 1 << 0;
const CCR_RESET: u32 = 1 << 1;
const CCR_SUSP: u32 = 1 << 2;
const CCR_TCIE: u32 = 1 << 8;
const CCR_HTIE: u32 = 1 << 9;
const CCR_DTEIE: u32 = 1 << 10;
const CCR_ULEIE: u32 = 1 << 11;
const CCR_USEIE: u32 = 1 << 12;
const CCR_SUSPIE: u32 = 1 << 13;
const CCR_IRQ_MASK: u32 = CCR_TCIE | CCR_HTIE | CCR_DTEIE | CCR_ULEIE | CCR_USEIE | CCR_SUSPIE;

const CSR_IDLEF: u32 = 1 << 0;
const CSR_TCF: u32 = 1 << 8;
const CSR_HTF: u32 = 1 << 9;
const CSR_DTEF: u32 = 1 << 10;
const CSR_ULEF: u32 = 1 << 11;
const CSR_USEF: u32 = 1 << 12;
const CSR_SUSPF: u32 = 1 << 13;
const CSR_ERROR_MASK: u32 = CSR_DTEF | CSR_ULEF | CSR_USEF;
const FCR_CLEAR_ALL: u32 = 0x7f00;

const CTR1_SINC: u32 = 1 << 3;
const CTR1_SAP_AHB: u32 = 1 << 14;
const CTR1_DINC: u32 = 1 << 19;
const CTR1_DAP_AHB: u32 = 1 << 30;
const CTR2_SWREQ: u32 = 1 << 9;
const CTR2_DREQ: u32 = 1 << 10;

const CLLR_LA_MASK: u32 = 0x0000_fffc;
const CLLR_ULL: u32 = 1 << 16;
const CLLR_UDA: u32 = 1 << 27;
const CLLR_USA: u32 = 1 << 28;
const CLLR_UB1: u32 = 1 << 29;
const CLLR_UT2: u32 = 1 << 30;
const CLLR_UT1: u32 = 1 << 31;
const CLLR_LINEAR_UPDATES: u32 = CLLR_ULL | CLLR_UDA | CLLR_USA | CLLR_UB1 | CLLR_UT2 | CLLR_UT1;

const MODE_IDLE: u8 = 0;
const MODE_ONESHOT: u8 = 1;
const MODE_CIRCULAR: u8 = 2;

/// Maximum byte length of one HPDMA block.
pub const MAX_TRANSFER: usize = 0xffff;

mod sealed {
    pub trait Sealed {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}

/// An element type supported by HPDMA.
pub trait DmaWord: sealed::Sealed + Copy + Default {
    #[doc(hidden)]
    const WIDTH_LOG2: u32;
}

impl DmaWord for u8 {
    const WIDTH_LOG2: u32 = 0;
}
impl DmaWord for u16 {
    const WIDTH_LOG2: u32 = 1;
}
impl DmaWord for u32 {
    const WIDTH_LOG2: u32 = 2;
}

/// HPDMA request selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmaRequest(u8);

impl DmaRequest {
    /// Construct a request from its vendor request number.
    pub const fn new(request: u8) -> Self {
        Self(request)
    }

    /// Raw HPDMA request number.
    pub const fn raw(self) -> u8 {
        self.0
    }
}

impl From<u8> for DmaRequest {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// Chip-selected STM32MP2 HPDMA request numbers.
///
/// Transcribed from STM32CubeMP2's `stm32mp2xx_hal_dma.h`, whose
/// `#if defined(LPDMA)` branch is STM32MP23/25 and whose `#else` branch is
/// STM32MP21.
pub mod request {
    use super::DmaRequest;

    macro_rules! requests {
        ($($name:ident = $value:expr),+ $(,)?) => {
            $(
                #[doc = concat!("HPDMA request `", stringify!($name), "`.")]
                pub const $name: DmaRequest = DmaRequest::new($value);
            )+

            /// Every request in this table, for consistency tests.
            #[cfg(test)]
            pub(crate) const ALL: &[(&str, DmaRequest)] = &[$((stringify!($name), $name)),+];
        };
    }

    #[cfg(feature = "stm32mp21x")]
    requests! {
        ADC1 = 58, ADC2 = 59,
        TIM1_CH1 = 60, TIM1_CH2 = 61, TIM1_CH3 = 62, TIM1_CH4 = 63,
        TIM1_UP = 64, TIM1_TRIG = 65, TIM1_COM = 66,
        TIM2_CH1 = 67, TIM2_CH2 = 68, TIM2_CH3 = 69, TIM2_CH4 = 70, TIM2_UP = 71,
        TIM3_CH1 = 72, TIM3_CH2 = 73, TIM3_CH3 = 74, TIM3_CH4 = 75,
        TIM3_UP = 76, TIM3_TRIG = 77,
        TIM4_CH1 = 78, TIM4_CH2 = 79, TIM4_CH3 = 80, TIM4_UP = 81,
        TIM5_CH1 = 82, TIM5_CH2 = 83, TIM5_CH3 = 84, TIM5_CH4 = 85,
        TIM5_UP = 86, TIM5_TRIG = 87, TIM6_UP = 88, TIM7_UP = 89,
        TIM8_CH1 = 90, TIM8_CH2 = 91, TIM8_CH3 = 92, TIM8_CH4 = 93,
        TIM8_UP = 94, TIM8_TRIG = 95, TIM8_COM = 96,
        TIM15_CH1 = 97, TIM15_UP = 98, TIM15_TRIG = 99, TIM15_COM = 100,
        TIM16_CH1 = 101, TIM16_UP = 102, TIM17_CH1 = 103, TIM17_UP = 104,
        TIM4_CH4 = 110, TIM2_TRIG = 117, TIM4_TRIG = 118, TIM15_CH2 = 119,
        TIM16_COM = 120, TIM17_COM = 121,
        // `LPTIM2_IC1` and `LPTIM2_IC2` both read 115 in STM32CubeMP2's
        // `stm32mp2xx_hal_dma.h`; the skipped 114 suggests a vendor typo, but
        // the header is the only source, so it is copied verbatim.
        LPTIM1_IC1 = 111, LPTIM1_IC2 = 112, LPTIM1_UE = 113,
        LPTIM2_IC1 = 115, LPTIM2_IC2 = 115, LPTIM2_UE = 116,
        LPTIM3_IC1 = 136, LPTIM3_IC2 = 137, LPTIM3_UE = 138,
        LPTIM4_IC1 = 139, LPTIM4_IC2 = 140, LPTIM4_UE = 141
    }

    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    requests! {
        ADC1 = 81, ADC2 = 82, ADC3 = 83,
        TIM1_CH1 = 84, TIM1_CH2 = 85, TIM1_CH3 = 86, TIM1_CH4 = 87,
        TIM1_UP = 88, TIM1_TRIG = 89, TIM1_COM = 90,
        TIM2_CH1 = 91, TIM2_CH2 = 92, TIM2_CH3 = 93, TIM2_CH4 = 94, TIM2_UP = 95,
        TIM3_CH1 = 96, TIM3_CH2 = 97, TIM3_CH3 = 98, TIM3_CH4 = 99,
        TIM3_UP = 100, TIM3_TRIG = 101,
        TIM4_CH1 = 102, TIM4_CH2 = 103, TIM4_CH3 = 104, TIM4_UP = 105,
        TIM5_CH1 = 106, TIM5_CH2 = 107, TIM5_CH3 = 108, TIM5_CH4 = 109,
        TIM5_UP = 110, TIM5_TRIG = 111, TIM6_UP = 112, TIM7_UP = 113,
        TIM8_CH1 = 114, TIM8_CH2 = 115, TIM8_CH3 = 116, TIM8_CH4 = 117,
        TIM8_UP = 118, TIM8_TRIG = 119, TIM8_COM = 120,
        TIM15_CH1 = 121, TIM15_UP = 122, TIM15_TRIG = 123, TIM15_COM = 124,
        TIM16_CH1 = 125, TIM16_UP = 126, TIM17_CH1 = 127, TIM17_UP = 128,
        TIM20_CH1 = 129, TIM20_CH2 = 130, TIM20_CH3 = 131, TIM20_CH4 = 132,
        TIM20_UP = 133, TIM20_TRIG = 134, TIM20_COM = 135,
        TIM4_CH4 = 142, TIM2_TRIG = 149, TIM4_TRIG = 150, TIM15_CH2 = 151,
        TIM16_COM = 152, TIM17_COM = 153,
        LPTIM1_IC1 = 143, LPTIM1_IC2 = 144, LPTIM1_UE = 145,
        LPTIM2_IC1 = 146, LPTIM2_IC2 = 147, LPTIM2_UE = 148,
        LPTIM3_IC1 = 174, LPTIM3_IC2 = 175, LPTIM3_UE = 176,
        LPTIM4_IC1 = 177, LPTIM4_IC2 = 178, LPTIM4_UE = 179
    }
}

/// DMA transfer error carrying the raw channel status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    pub raw_sr: u32,
}

/// Ring-buffer synchronization error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingError {
    Dma(Error),
    Overrun,
    Underrun,
    NotStarted,
}

/// Per-channel state shared with the channel IRQ.
///
/// One `static` per channel singleton is created by `impl_dma_channel` and
/// reached through the instance type, so callers never supply or name it.
pub struct DmaState {
    ch: AtomicPtr<()>,
    mode: AtomicU8,
    complete_count: AtomicUsize,
    error_status: AtomicU32,
    waker: AtomicWaker,
}

impl DmaState {
    /// Create empty state suitable for a `static`.
    ///
    /// No `Default` impl: the only states that exist are the per-channel
    /// `static`s the instance bindings create.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            ch: AtomicPtr::new(core::ptr::null_mut()),
            mode: AtomicU8::new(MODE_IDLE),
            complete_count: AtomicUsize::new(0),
            error_status: AtomicU32::new(0),
            waker: AtomicWaker::new(),
        }
    }

    /// Clear pending flags, account for circular laps, and wake the waiter.
    pub fn on_interrupt(&self) {
        let ptr = self.ch.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: installed by `DmaChannel::new` for the channel owning this state.
        let ch = unsafe { DMA_Channel::from_ptr(ptr) };
        let sr = ch.CSR().read();
        let pending = sr & (CSR_TCF | CSR_HTF | CSR_ERROR_MASK | CSR_SUSPF);
        if pending == 0 {
            return;
        }
        if self.mode.load(Ordering::Acquire) == MODE_CIRCULAR {
            if sr & CSR_ERROR_MASK != 0 {
                self.error_status.store(sr, Ordering::Release);
            }
            // Keep SUSPF asserted until the waiter observes that suspension
            // completed. Resume explicitly clears it.
            ch.CFCR().write_value(pending & !CSR_SUSPF);
            if sr & CSR_TCF != 0 {
                self.complete_count.fetch_add(1, Ordering::AcqRel);
            }
        } else {
            critical_section::with(|_| {
                ch.CCR().write_value(ch.CCR().read() & !CCR_IRQ_MASK);
            });
        }
        self.waker.wake();
    }
}

/// One stable HPDMA linked-list node supplied by the caller.
#[repr(C, align(4))]
#[derive(Debug, Default)]
pub struct DmaNode {
    tr1: u32,
    tr2: u32,
    br1: u32,
    sar: u32,
    dar: u32,
    llr: u32,
}

impl DmaNode {
    /// Create an empty node suitable for static initialization.
    pub const fn new() -> Self {
        Self {
            tr1: 0,
            tr2: 0,
            br1: 0,
            sar: 0,
            dar: 0,
            llr: 0,
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This channel's register page, at its database-resolved base.
    fn regs() -> DMA_Channel;
    /// This channel's dedicated interrupt state.
    fn state() -> &'static DmaState;
}

/// One HPDMA/LPDMA channel the [`DmaChannel`] driver can claim.
///
/// Implemented by the generated per-channel singletons; sealed so the channel
/// page stays single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this channel raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
}

/// Bind a generated channel singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_dma_channel {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::dma::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::dma_channel::DMA_Channel {
                // SAFETY: `$base` is this channel's page as extracted from the
                // chip database, and the singleton this impl is keyed to can be
                // held by only one driver at a time.
                unsafe {
                    crate::pac::peripherals::dma_channel::DMA_Channel::from_ptr($base as *mut ())
                }
            }

            #[inline]
            fn state() -> &'static crate::dma::DmaState {
                static STATE: crate::dma::DmaState = crate::dma::DmaState::new();
                &STATE
            }
        }

        impl crate::dma::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
        }
    };
}
pub(crate) use impl_dma_channel;

/// Handle the HPDMA interrupt of channel `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt();
}

/// Exclusive handle to one HPDMA channel.
pub struct DmaChannel {
    ch: DMA_Channel,
    state: &'static DmaState,
}

impl DmaChannel {
    /// Claim a channel from its peripheral singleton.
    ///
    /// The platform must leave the controller clocked and RIF-configured.
    /// Route the channel's IRQ to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // channel access safe; it is consumed here and never reissued.
        let _ = peri;
        Self::over(T::regs(), T::state())
    }

    /// Build a channel over an arbitrary register page, for tests and for
    /// drivers that stand an array in for the controller.
    pub(crate) fn over(ch: DMA_Channel, state: &'static DmaState) -> Self {
        state.ch.store(ch.as_ptr(), Ordering::Release);
        Self { ch, state }
    }

    fn prepare(&mut self) {
        self.halt();
        self.ch.CFCR().write_value(FCR_CLEAR_ALL);
        self.state.complete_count.store(0, Ordering::Release);
        self.state.error_status.store(0, Ordering::Release);
    }

    fn setup(&mut self, src: u32, dst: u32, bytes: usize, tr1: u32, tr2: u32, interrupts: u32) {
        assert!((1..=MAX_TRANSFER).contains(&bytes));
        self.prepare();
        self.ch.CTR1().write_value(tr1);
        self.ch.CTR2().write_value(tr2);
        self.ch.CBR1().write_value(bytes as u32);
        self.ch.CSAR().write_value(src);
        self.ch.CDAR().write_value(dst);
        self.ch.CLLR().write_value(0);
        self.state.mode.store(MODE_ONESHOT, Ordering::Release);
        fence(Ordering::SeqCst);
        self.ch.CCR().write_value(interrupts | CCR_EN);
    }

    /// Copy one equal-length block using a software request.
    pub fn mem_to_mem<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: &'a mut [W],
    ) -> Transfer<'a> {
        assert!(!src.is_empty() && src.len() == dst.len());
        let bytes = byte_len::<W>(src.len());
        let tr1 = widths::<W>() | CTR1_SINC | CTR1_DINC;
        self.setup(
            address(src.as_ptr()),
            address(dst.as_mut_ptr()),
            bytes,
            tr1,
            CTR2_SWREQ,
            CCR_TCIE | CCR_DTEIE | CCR_ULEIE | CCR_USEIE,
        );
        Transfer::new(self)
    }

    /// Write a finite memory block to a peripheral.
    ///
    /// # Safety
    ///
    /// `dst` must be a DMA-capable register matching `W` and `request`.
    pub unsafe fn write_peripheral<'a, W: DmaWord>(
        &'a mut self,
        src: &'a [W],
        dst: *mut W,
        request: impl Into<DmaRequest>,
    ) -> Transfer<'a> {
        assert!(!src.is_empty());
        self.setup(
            address(src.as_ptr()),
            address(dst),
            byte_len::<W>(src.len()),
            widths::<W>() | CTR1_SINC | CTR1_DAP_AHB,
            CTR2_DREQ | request.into().raw() as u32,
            CCR_TCIE | CCR_DTEIE | CCR_ULEIE | CCR_USEIE,
        );
        Transfer::new(self)
    }

    /// Read a finite peripheral block into memory.
    ///
    /// # Safety
    ///
    /// `src` must be a DMA-capable register matching `W` and `request`.
    pub unsafe fn read_peripheral<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        dst: &'a mut [W],
        request: impl Into<DmaRequest>,
    ) -> Transfer<'a> {
        assert!(!dst.is_empty());
        self.setup(
            address(src),
            address(dst.as_mut_ptr()),
            byte_len::<W>(dst.len()),
            widths::<W>() | CTR1_DINC | CTR1_SAP_AHB,
            request.into().raw() as u32,
            CCR_TCIE | CCR_DTEIE | CCR_ULEIE | CCR_USEIE,
        );
        Transfer::new(self)
    }

    /// Construct a circular peripheral-to-memory ring.
    ///
    /// # Safety
    ///
    /// `src` must be the data register associated with `request`.
    pub unsafe fn read_circular<'a, W: DmaWord>(
        &'a mut self,
        src: *const W,
        buffer: &'a mut [W],
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
    ) -> ReadableRingBuffer<'a, W> {
        assert!(buffer.len() >= 2 && buffer.len().is_multiple_of(2));
        ReadableRingBuffer {
            channel: self,
            src,
            buffer,
            request: request.into(),
            node,
            consumed: 0,
            started: false,
        }
    }

    /// Construct a circular memory-to-peripheral ring.
    ///
    /// # Safety
    ///
    /// `dst` must be the data register associated with `request`.
    pub unsafe fn write_circular<'a, W: DmaWord>(
        &'a mut self,
        buffer: &'a mut [W],
        dst: *mut W,
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
    ) -> WritableRingBuffer<'a, W> {
        assert!(buffer.len() >= 2 && buffer.len().is_multiple_of(2));
        WritableRingBuffer {
            channel: self,
            buffer,
            dst,
            request: request.into(),
            node,
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
        node: &mut DmaNode,
    ) {
        let bytes = byte_len::<W>(len);
        let (sar, dar, tr1, tr2) = match direction {
            Direction::MemoryToPeripheral => (
                address(memory),
                address(peripheral),
                widths::<W>() | CTR1_SINC | CTR1_DAP_AHB,
                CTR2_DREQ | request.raw() as u32,
            ),
            Direction::PeripheralToMemory => (
                address(peripheral),
                address(memory),
                widths::<W>() | CTR1_DINC | CTR1_SAP_AHB,
                request.raw() as u32,
            ),
        };
        let node_addr = address(node as *mut DmaNode);
        assert!((node_addr & 0xffff) as usize + core::mem::size_of::<DmaNode>() <= 0x1_0000);
        let link = (node_addr & CLLR_LA_MASK) | CLLR_LINEAR_UPDATES;
        *node = DmaNode {
            tr1,
            tr2,
            br1: bytes as u32,
            sar,
            dar,
            llr: link,
        };

        self.prepare();
        self.ch.CLBAR().write_value(node_addr & 0xffff_0000);
        self.ch.CTR1().write_value(tr1);
        self.ch.CTR2().write_value(tr2);
        self.ch.CBR1().write_value(bytes as u32);
        self.ch.CSAR().write_value(sar);
        self.ch.CDAR().write_value(dar);
        self.ch.CLLR().write_value(link);
        self.state.mode.store(MODE_CIRCULAR, Ordering::Release);
        fence(Ordering::SeqCst);
        self.ch.CCR().write_value(CCR_IRQ_MASK | CCR_EN);
    }

    fn remaining<W: DmaWord>(&self) -> usize {
        (self.ch.CBR1().read() as usize & 0xffff) / core::mem::size_of::<W>()
    }

    fn progress<W: DmaWord>(&self, capacity: usize) -> usize {
        critical_section::with(|_| {
            let laps = self.state.complete_count.load(Ordering::Acquire);
            let remaining = self.remaining::<W>().min(capacity);
            laps.saturating_mul(capacity)
                .saturating_add(capacity - remaining)
        })
    }

    fn register_waker(&self, cx: &Context<'_>) {
        self.state.waker.register(cx.waker());
    }

    fn circular_error(&self) -> Option<RingError> {
        let sr = self.state.error_status.load(Ordering::Acquire);
        (sr & CSR_ERROR_MASK != 0).then_some(RingError::Dma(Error { raw_sr: sr }))
    }

    fn pause(&self) {
        self.ch.CCR().write_value(self.ch.CCR().read() | CCR_SUSP);
    }

    fn resume(&self) {
        self.ch.CFCR().write_value(CSR_SUSPF);
        self.ch
            .CCR()
            .write_value((self.ch.CCR().read() & !CCR_SUSP) | CCR_EN | CCR_IRQ_MASK);
    }

    fn is_running(&self) -> bool {
        let sr = self.ch.CSR().read();
        self.ch.CCR().read() & CCR_EN != 0 && sr & (CSR_IDLEF | CSR_SUSPF) == 0
    }

    fn halt(&self) {
        let ccr = self.ch.CCR().read();
        if ccr & CCR_EN != 0 && self.ch.CSR().read() & CSR_IDLEF == 0 {
            self.ch.CCR().write_value(ccr | CCR_SUSP);
            while self.ch.CSR().read() & (CSR_SUSPF | CSR_TCF | CSR_IDLEF) == 0 {}
            self.ch.CCR().write_value(ccr | CCR_SUSP | CCR_RESET);
        }
        self.ch.CFCR().write_value(FCR_CLEAR_ALL);
        self.ch.CCR().write_value(0);
        self.state.mode.store(MODE_IDLE, Ordering::Release);
    }
}

enum Direction {
    MemoryToPeripheral,
    PeripheralToMemory,
}

/// In-flight one-shot DMA transfer.
#[must_use]
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
            return Some(Err(Error { raw_sr: sr }));
        }
        (sr & CSR_TCF != 0).then_some(Ok(()))
    }

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
        critical_section::with(|_| {
            self.ch
                .CCR()
                .write_value(self.ch.CCR().read() | CCR_IRQ_MASK);
        });
        self.status().map_or(Poll::Pending, Poll::Ready)
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        let channel = DmaChannel {
            ch: self.ch,
            state: self.state,
        };
        channel.halt();
    }
}

/// Circular peripheral-to-memory consumer.
pub struct ReadableRingBuffer<'a, W: DmaWord> {
    channel: &'a mut DmaChannel,
    src: *const W,
    buffer: &'a mut [W],
    request: DmaRequest,
    node: &'a mut DmaNode,
    consumed: usize,
    started: bool,
}

impl<W: DmaWord> ReadableRingBuffer<'_, W> {
    pub fn start(&mut self) {
        if !self.started {
            self.channel.start_circular(
                self.buffer.as_mut_ptr(),
                self.src.cast_mut(),
                self.buffer.len(),
                self.request,
                Direction::PeripheralToMemory,
                self.node,
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
        Ok(self.channel.progress::<W>(self.buffer.len()))
    }

    pub fn len(&self) -> Result<usize, RingError> {
        let available = self.produced()?.saturating_sub(self.consumed);
        if available > self.buffer.len() {
            Err(RingError::Overrun)
        } else {
            Ok(available)
        }
    }

    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.len().map(|length| length == 0)
    }

    pub fn read(&mut self, output: &mut [W]) -> Result<(usize, usize), RingError> {
        let available = self.len()?;
        let count = available.min(output.len());
        copy_from_ring(self.buffer, self.consumed, &mut output[..count]);
        self.consumed += count;
        Ok((count, available - count))
    }

    pub async fn read_exact(&mut self, output: &mut [W]) -> Result<usize, RingError> {
        let mut offset = 0;
        while offset < output.len() {
            let read = poll_fn(|cx| {
                self.channel.register_waker(cx);
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

    pub fn read_latest(&mut self, output: &mut [W]) -> Result<usize, RingError> {
        let produced = self.produced()?;
        let count = output.len().min(self.buffer.len()).min(produced);
        self.consumed = produced - count;
        copy_from_ring(self.buffer, self.consumed, &mut output[..count]);
        self.consumed = produced;
        Ok(count)
    }

    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn pause(&mut self) {
        self.channel.pause();
    }

    pub fn resume(&mut self) {
        self.channel.resume();
    }

    pub fn is_running(&self) -> bool {
        self.channel.is_running()
    }

    pub async fn stop(&mut self) {
        self.channel.pause();
        poll_fn(|cx| {
            self.channel.register_waker(cx);
            if self.channel.is_running() {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
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
    node: &'a mut DmaNode,
    produced: usize,
    started: bool,
}

impl<W: DmaWord> WritableRingBuffer<'_, W> {
    /// Fill unused ring slots before or during operation.
    pub fn write_immediate(&mut self, input: &[W]) -> Result<(usize, usize), RingError> {
        self.write_inner(input, false)
    }

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
                self.node,
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
        let consumed = self.channel.progress::<W>(self.buffer.len());
        if consumed > self.produced {
            Err(RingError::Underrun)
        } else {
            Ok(consumed)
        }
    }

    pub fn len(&self) -> Result<usize, RingError> {
        Ok(self.produced.saturating_sub(self.consumed()?))
    }

    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.len().map(|length| length == 0)
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

    pub fn write(&mut self, input: &[W]) -> Result<(usize, usize), RingError> {
        self.write_inner(input, true)
    }

    pub async fn write_exact(&mut self, input: &[W]) -> Result<usize, RingError> {
        let mut offset = 0;
        while offset < input.len() {
            let written = poll_fn(|cx| {
                self.channel.register_waker(cx);
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

    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn pause(&mut self) {
        self.channel.pause();
    }

    pub fn resume(&mut self) {
        self.channel.resume();
    }

    pub fn is_running(&self) -> bool {
        self.channel.is_running()
    }

    pub async fn stop(&mut self) {
        self.channel.pause();
        poll_fn(|cx| {
            self.channel.register_waker(cx);
            if self.channel.is_running() {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        self.channel.halt();
        self.started = false;
    }
}

impl<W: DmaWord> Drop for WritableRingBuffer<'_, W> {
    fn drop(&mut self) {
        self.channel.halt();
    }
}

const fn widths<W: DmaWord>() -> u32 {
    W::WIDTH_LOG2 | (W::WIDTH_LOG2 << 16)
}

fn byte_len<W: DmaWord>(elements: usize) -> usize {
    let bytes = elements
        .checked_mul(core::mem::size_of::<W>())
        .expect("DMA byte length overflow");
    assert!((1..=MAX_TRANSFER).contains(&bytes));
    bytes
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

    fn channel(registers: &mut [u32; 128], state: &'static DmaState) -> DmaChannel {
        // SAFETY: fake MMIO is exclusively owned by this test.
        // SAFETY: `registers` is word-aligned, covers every offset the driver
        // touches, and outlives the channel built over it.
        let ch = unsafe { DMA_Channel::from_ptr(registers.as_mut_ptr().cast()) };
        DmaChannel::over(ch, state)
    }

    #[test]
    fn halfword_write_sets_width_ports_and_byte_count() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 128];
        // Mark idle so setup can reset without waiting.
        registers[0x60 / 4] = CSR_IDLEF;
        let mut dma = channel(&mut registers, &STATE);
        let data = [1u16, 2, 3];
        // SAFETY: fake destination is never dereferenced by hardware.
        let transfer = unsafe {
            dma.write_peripheral(&data, 0x5000_0000usize as *mut u16, DmaRequest::new(84))
        };
        assert_eq!(
            registers[0x90 / 4],
            1 | (1 << 16) | CTR1_SINC | CTR1_DAP_AHB
        );
        assert_eq!(registers[0x94 / 4], CTR2_DREQ | 84);
        assert_eq!(registers[0x98 / 4], 6);
        core::mem::forget(transfer);
    }

    #[test]
    fn word_read_sets_source_ahb_port() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 128];
        registers[0x60 / 4] = CSR_IDLEF;
        let mut dma = channel(&mut registers, &STATE);
        let mut data = [0u32; 2];
        // SAFETY: fake source is never dereferenced by hardware.
        let transfer = unsafe {
            dma.read_peripheral(
                0x5000_0000usize as *const u32,
                &mut data,
                DmaRequest::new(90),
            )
        };
        assert_eq!(
            registers[0x90 / 4],
            2 | (2 << 16) | CTR1_DINC | CTR1_SAP_AHB
        );
        assert_eq!(registers[0x98 / 4], 8);
        core::mem::forget(transfer);
    }

    #[test]
    fn ring_copy_wraps() {
        let ring = [1u16, 2, 3, 4];
        let mut output = [0u16; 3];
        copy_from_ring(&ring, 3, &mut output);
        assert_eq!(output, [4, 1, 2]);
    }

    #[test]
    fn circular_transfer_programs_stable_self_link() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 128];
        registers[0x60 / 4] = CSR_IDLEF;
        let mut dma = channel(&mut registers, &STATE);
        let mut node = DmaNode::default();
        let mut data = [0u16; 4];
        // SAFETY: fake destination is not dereferenced.
        let mut ring = unsafe {
            dma.write_circular(
                &mut data,
                0x5000_0000usize as *mut u16,
                DmaRequest::new(84),
                &mut node,
            )
        };
        assert_eq!(ring.write_immediate(&[1, 2, 3, 4]), Ok((4, 0)));
        ring.start();
        assert_eq!(registers[0x98 / 4], 8);
        drop(ring);
        assert_eq!(node.br1, 8);
        assert_eq!(node.llr & !CLLR_LA_MASK, CLLR_LINEAR_UPDATES);
        assert_ne!(registers[0xcc / 4] & CLLR_LINEAR_UPDATES, 0);
    }

    #[test]
    fn circular_irq_counts_laps_without_masking_interrupts() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 128];
        let _dma = channel(&mut registers, &STATE);
        STATE.mode.store(MODE_CIRCULAR, Ordering::Release);
        registers[0x64 / 4] = CCR_IRQ_MASK | CCR_EN;
        registers[0x60 / 4] = CSR_TCF;
        STATE.on_interrupt();
        assert_eq!(STATE.complete_count.load(Ordering::Acquire), 1);
        assert_eq!(registers[0x64 / 4] & CCR_IRQ_MASK, CCR_IRQ_MASK);
    }

    #[test]
    fn one_shot_irq_preserves_completion_for_future() {
        static STATE: DmaState = DmaState::new();
        let mut registers = [0u32; 128];
        let _dma = channel(&mut registers, &STATE);
        STATE.mode.store(MODE_ONESHOT, Ordering::Release);
        registers[0x64 / 4] = CCR_IRQ_MASK | CCR_EN;
        registers[0x60 / 4] = CSR_TCF;
        STATE.on_interrupt();
        assert_eq!(registers[0x60 / 4] & CSR_TCF, CSR_TCF);
        assert_eq!(registers[0x64 / 4] & CCR_IRQ_MASK, 0);
    }

    /// Two requests sharing a number would route one peripheral's DMA onto
    /// another's line. STM32MP21 is exempt: ST's own table repeats 115 for
    /// `LPTIM2_IC1`/`LPTIM2_IC2`.
    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    #[test]
    fn request_numbers_are_unique() {
        let all = super::request::ALL;
        for (i, (a, ra)) in all.iter().enumerate() {
            for (b, rb) in &all[i + 1..] {
                assert_ne!(ra.raw(), rb.raw(), "{a} and {b} share a request number");
            }
        }
    }

    #[cfg(any(feature = "stm32mp23x", feature = "stm32mp25x"))]
    #[test]
    fn lptim_requests_match_the_mp25_table() {
        use super::request::*;
        assert_eq!(LPTIM1_IC1.raw(), 143);
        assert_eq!(LPTIM2_UE.raw(), 148);
        assert_eq!(LPTIM3_IC1.raw(), 174);
        assert_eq!(LPTIM4_UE.raw(), 179);
    }

    /// ST's MP21 table repeats 115 for `LPTIM2_IC1`/`LPTIM2_IC2`; nothing else
    /// may collide.
    #[cfg(feature = "stm32mp21x")]
    #[test]
    fn only_the_vendor_duplicate_shares_a_request_number() {
        let all = super::request::ALL;
        for (i, (a, ra)) in all.iter().enumerate() {
            for (b, rb) in &all[i + 1..] {
                if ra.raw() == rb.raw() {
                    assert_eq!((*a, *b), ("LPTIM2_IC1", "LPTIM2_IC2"));
                }
            }
        }
    }

    #[cfg(feature = "stm32mp21x")]
    #[test]
    fn lptim_requests_match_the_mp21_table() {
        use super::request::*;
        assert_eq!(LPTIM1_IC1.raw(), 111);
        assert_eq!(LPTIM2_UE.raw(), 116);
        assert_eq!(LPTIM3_IC1.raw(), 136);
        assert_eq!(LPTIM4_UE.raw(), 141);
    }
}
