//! i.MX9 low-power timer (LPTMR).
//!
//! LPTMR is a compare timer or external pulse counter. It has no waveform
//! output channel, so PWM belongs to TPM rather than this module.
//!
//! Select an instance by handing its peripheral singleton to [`Lptmr::new`]:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let timer = Lptmr::new(p.LPTMR1, Config::default());
//! ```

use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll};

use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::lptmr::LPTMR;

use crate::pac;

#[cfg(feature = "dma")]
use crate::dma::{DmaChannel, DmaRequest, RingError, Transfer, WritableRingBuffer};

/// Prescaler clock source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockSource {
    Source0 = 0,
    Source1 = 1,
    Source2 = 2,
    Source3 = 3,
}

/// Pulse-counter input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PulseInput {
    Input0 = 0,
    Input1 = 1,
    Input2 = 2,
    Input3 = 3,
}

/// Pulse-counter edge polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulsePolarity {
    Rising,
    Falling,
}

/// LPTMR operating configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Timer-clock source.
    pub clock_source: ClockSource,
    /// Bypass the prescaler/glitch filter.
    pub bypass_prescaler: bool,
    /// Prescaler exponent `0..=15`; division is `2^(value + 1)`.
    pub prescaler: u8,
    /// Continue counting instead of resetting on compare.
    pub free_running: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock_source: ClockSource::Source0,
            bypass_prescaler: true,
            prescaler: 0,
            free_running: false,
        }
    }
}

/// IRQ state dedicated to one LPTMR.
///
/// One `static` per LPTMR is created by `impl_lptmr` and reached through the
/// instance type, so callers never supply or name it.
pub struct State {
    pending: AtomicBool,
    waker: AtomicWaker,
}

impl State {
    /// Create empty state suitable for a `static`.
    pub(crate) const fn new() -> Self {
        Self {
            pending: AtomicBool::new(false),
            waker: AtomicWaker::new(),
        }
    }

    /// Handle the interrupt of the LPTMR at `regs`.
    fn on_interrupt(&self, regs: LPTMR) {
        if regs.CSR().read().TCF() {
            self.pending.store(true, Ordering::Release);
            regs.CSR().modify(|w| {
                w.set_TIE(false);
                w.set_TCF(true);
            });
            self.waker.wake();
        }
    }
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> LPTMR;
    /// This instance's dedicated interrupt state.
    fn state() -> &'static State;
}

/// An LPTMR the [`Lptmr`] driver can drive.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address, DMA request line, and interrupt state stay single-sourced.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
    /// eDMA request line this instance drives on compare events.
    #[cfg(feature = "dma")]
    const DMA_REQUEST: DmaRequest;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_lptmr {
    ($instance:ident, $base:expr, $irq:expr) => {
        impl crate::lptmr::SealedInstance for crate::peripherals::$instance {
            #[inline]
            fn regs() -> crate::pac::peripherals::lptmr::LPTMR {
                // SAFETY: `$base` is this instance's LPTMR base as extracted
                // from the chip database, and the singleton this impl is keyed
                // to can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::lptmr::LPTMR::from_ptr($base as *mut ()) }
            }

            #[inline]
            fn state() -> &'static crate::lptmr::State {
                static STATE: crate::lptmr::State = crate::lptmr::State::new();
                &STATE
            }
        }

        impl crate::lptmr::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
            // The request constants are named after the instances, so the
            // singleton's own identifier selects the right line.
            #[cfg(feature = "dma")]
            const DMA_REQUEST: crate::dma::DmaRequest = crate::dma::request::$instance;
        }
    };
}
pub(crate) use impl_lptmr;

/// Handle the LPTMR interrupt of instance `T`.
///
/// Call this from the application's `#[interrupt]` handler for `T`'s vector;
/// the HAL installs no handlers of its own.
pub fn on_interrupt<T: Instance>() {
    T::state().on_interrupt(T::regs());
}

/// Program `regs` from `config`, leaving the timer stopped.
fn program(regs: LPTMR, config: Config) {
    assert!(config.prescaler <= 15);
    regs.CSR().write_value(Default::default());
    regs.PSR().write(|w| {
        w.set_PCS(config.clock_source as u8);
        w.set_PBYP(config.bypass_prescaler);
        w.set_PRESCALE(config.prescaler);
    });
    regs.CSR().modify(|w| w.set_TFC(config.free_running));
}

/// Exclusive LPTMR driver.
pub struct Lptmr {
    regs: LPTMR,
    #[cfg(feature = "dma")]
    dma_request: DmaRequest,
    state: &'static State,
}

impl Lptmr {
    /// Bind and configure an LPTMR from its peripheral singleton.
    ///
    /// The platform must leave the instance clocked. For asynchronous waits,
    /// forward its interrupt to [`on_interrupt::<T>`](on_interrupt).
    pub fn new<T: Instance>(peri: Peri<'static, T>, config: Config) -> Self {
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        let regs = T::regs();
        program(regs, config);
        Self {
            regs,
            #[cfg(feature = "dma")]
            dma_request: T::DMA_REQUEST,
            state: T::state(),
        }
    }

    /// Build a driver over an arbitrary register block, for tests that stand a
    /// plain array in for the LPTMR.
    #[cfg(test)]
    fn over(regs: LPTMR, state: &'static State, config: Config) -> Self {
        program(regs, config);
        Self {
            regs,
            #[cfg(feature = "dma")]
            dma_request: crate::dma::request::LPTMR1,
            state,
        }
    }

    /// Configure time-counter mode.
    pub fn configure_timer(&mut self) {
        self.stop();
        self.regs.CSR().modify(|w| w.set_TMS(false));
    }

    /// Configure external pulse-counter mode.
    pub fn configure_pulse_counter(&mut self, input: PulseInput, polarity: PulsePolarity) {
        self.stop();
        self.regs.CSR().modify(|w| {
            w.set_TMS(true);
            w.set_TPS(input as u8);
            w.set_TPP(matches!(polarity, PulsePolarity::Falling));
        });
    }

    /// Set the compare value.
    pub fn set_compare(&mut self, compare: u32) {
        let running = self.is_running();
        if running {
            self.stop();
        }
        self.regs.CMR().write(|w| w.set_COMPARE(compare));
        if running {
            self.start();
        }
    }

    /// Current compare value.
    pub fn compare(&self) -> u32 {
        self.regs.CMR().read().COMPARE()
    }

    /// Read the synchronized counter value.
    pub fn counter(&self) -> u32 {
        self.regs.CNR().write(|w| w.set_COUNTER(0));
        self.regs.CNR().read().COUNTER()
    }

    /// Start counting.
    pub fn start(&mut self) {
        self.regs.CSR().modify(|w| w.set_TEN(true));
    }

    /// Stop counting.
    pub fn stop(&mut self) {
        self.regs.CSR().modify(|w| {
            w.set_TEN(false);
            // Do not acknowledge a pending flag accidentally.
            w.set_TCF(false);
        });
    }

    /// Whether the timer is enabled.
    pub fn is_running(&self) -> bool {
        self.regs.CSR().read().TEN()
    }

    /// Whether compare has occurred.
    pub fn is_compare_pending(&self) -> bool {
        self.regs.CSR().read().TCF()
    }

    /// Acknowledge compare.
    pub fn clear_compare(&mut self) {
        self.regs.CSR().modify(|w| w.set_TCF(true));
        self.state.pending.store(false, Ordering::Release);
    }

    /// Wait for the next compare interrupt.
    pub fn wait_compare(&mut self) -> WaitCompare<'_> {
        self.clear_compare();
        self.regs.CSR().modify(|w| {
            w.set_TCF(false);
            w.set_TIE(true);
        });
        WaitCompare {
            timer: self,
            complete: false,
        }
    }

    #[cfg(feature = "dma")]
    fn dma_request(&self) -> DmaRequest {
        self.dma_request
    }

    /// Write a finite sequence of future compare values at compare events.
    #[cfg(feature = "dma")]
    pub fn write_compare_dma<'a>(
        &'a mut self,
        dma: &'a mut DmaChannel,
        values: &'a [u32],
    ) -> CompareDmaTransfer<'a> {
        let request = self.dma_request();
        let destination = self.regs.CMR().as_ptr().cast::<u32>();
        // SAFETY: CMR is the destination associated with the LPTMR request.
        let transfer = unsafe { dma.write_peripheral_with_request(values, destination, request) };
        self.regs.CSR().modify(|w| {
            w.set_TCF(false);
            w.set_TDRE(true);
        });
        CompareDmaTransfer {
            timer: self,
            transfer: Some(transfer),
        }
    }

    /// Construct a circular compare-value stream.
    #[cfg(feature = "dma")]
    pub fn into_ring_buffered_compare<'a>(
        &'a mut self,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u32],
    ) -> RingBufferedCompare<'a> {
        let request = self.dma_request();
        let destination = self.regs.CMR().as_ptr().cast::<u32>();
        // SAFETY: CMR is the destination associated with the LPTMR request.
        let ring = unsafe { dma.write_circular(buffer, destination, request) };
        RingBufferedCompare {
            timer: self,
            ring: Some(ring),
            started: false,
        }
    }

    fn disable_irq(&mut self) {
        self.regs.CSR().modify(|w| {
            w.set_TCF(false);
            w.set_TIE(false);
        });
    }

    #[cfg(feature = "dma")]
    fn disable_dma(&mut self) {
        self.regs.CSR().modify(|w| {
            w.set_TCF(false);
            w.set_TDRE(false);
        });
    }
}

/// Future returned by [`Lptmr::wait_compare`].
pub struct WaitCompare<'a> {
    timer: &'a mut Lptmr,
    complete: bool,
}

impl Future for WaitCompare<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.timer.state.pending.swap(false, Ordering::AcqRel)
            || self.timer.regs.CSR().read().TCF()
        {
            self.timer.clear_compare();
            self.timer.disable_irq();
            self.complete = true;
            return Poll::Ready(());
        }
        self.timer.state.waker.register(cx.waker());
        if self.timer.state.pending.swap(false, Ordering::AcqRel) {
            self.timer.disable_irq();
            self.complete = true;
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for WaitCompare<'_> {
    fn drop(&mut self) {
        if !self.complete {
            self.timer.disable_irq();
        }
    }
}

/// Finite LPTMR compare-update DMA transfer.
#[cfg(feature = "dma")]
pub struct CompareDmaTransfer<'a> {
    timer: &'a mut Lptmr,
    transfer: Option<Transfer<'a>>,
}

#[cfg(feature = "dma")]
impl Future for CompareDmaTransfer<'_> {
    type Output = Result<(), crate::dma::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let result = Pin::new(
            this.transfer
                .as_mut()
                .expect("transfer polled after completion"),
        )
        .poll(cx);
        if result.is_ready() {
            drop(this.transfer.take());
            this.timer.disable_dma();
        }
        result
    }
}

#[cfg(feature = "dma")]
impl CompareDmaTransfer<'_> {
    /// Busy-wait for the final compare update.
    pub fn wait_blocking(mut self) -> Result<(), crate::dma::Error> {
        let result = self
            .transfer
            .take()
            .expect("transfer already consumed")
            .wait_blocking();
        self.timer.disable_dma();
        result
    }
}

#[cfg(feature = "dma")]
impl Drop for CompareDmaTransfer<'_> {
    fn drop(&mut self) {
        drop(self.transfer.take());
        self.timer.disable_dma();
    }
}

/// Circular LPTMR compare-value producer.
#[cfg(feature = "dma")]
pub struct RingBufferedCompare<'a> {
    timer: &'a mut Lptmr,
    ring: Option<WritableRingBuffer<'a, u32>>,
    started: bool,
}

#[cfg(feature = "dma")]
impl<'a> RingBufferedCompare<'a> {
    /// Prime unused ring slots.
    pub fn write_immediate(&mut self, values: &[u32]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write_immediate(values)
    }

    /// Enable compare-event DMA and start the timer.
    pub fn start(&mut self) {
        if !self.started {
            self.timer.regs.CSR().modify(|w| {
                w.set_TCF(false);
                w.set_TDRE(true);
            });
            self.ring_mut().start();
            self.timer.start();
            self.started = true;
        }
    }

    /// Queue compare values.
    pub fn write(&mut self, values: &[u32]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write(values)
    }

    /// Queue all values asynchronously.
    pub async fn write_exact(&mut self, values: &[u32]) -> Result<usize, RingError> {
        self.ring_mut().write_exact(values).await
    }

    /// Number of queued compare values.
    pub fn len(&self) -> Result<usize, RingError> {
        self.ring.as_ref().expect("ring has been released").len()
    }

    /// Whether no compare values are queued.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .is_empty()
    }

    /// Ring capacity in compare values.
    pub fn capacity(&self) -> usize {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .capacity()
    }

    /// Pause DMA requests.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume DMA requests.
    pub fn resume(&mut self) {
        self.ring_mut().resume();
    }

    /// Whether the DMA channel is active.
    pub fn is_running(&self) -> bool {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .is_running()
    }

    /// Stop DMA and disable compare requests.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        self.timer.disable_dma();
        self.started = false;
    }

    fn ring_mut(&mut self) -> &mut WritableRingBuffer<'a, u32> {
        self.ring.as_mut().expect("ring has been released")
    }
}

#[cfg(feature = "dma")]
impl Drop for RingBufferedCompare<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        self.timer.disable_dma();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configures_timer_and_pulse_modes() {
        static STATE: State = State::new();
        let mut registers = [0u32; 4];
        // SAFETY: `registers` is a word-aligned array covering every offset the
        // driver touches, and it outlives the driver built over it.
        let regs = unsafe { LPTMR::from_ptr(registers.as_mut_ptr().cast()) };
        let mut timer = Lptmr::over(
            regs,
            &STATE,
            Config {
                clock_source: ClockSource::Source2,
                bypass_prescaler: false,
                prescaler: 3,
                free_running: true,
            },
        );
        assert_eq!(registers[1] & 0x7f, 2 | (3 << 3));
        timer.configure_pulse_counter(PulseInput::Input2, PulsePolarity::Falling);
        assert_eq!(
            registers[0] & 0x3e,
            (1 << 1) | (1 << 2) | (1 << 3) | (2 << 4)
        );
    }

    #[test]
    fn start_stop_and_compare() {
        static STATE: State = State::new();
        let mut registers = [0u32; 4];
        // SAFETY: `registers` is a word-aligned array covering every offset the
        // driver touches, and it outlives the driver built over it.
        let regs = unsafe { LPTMR::from_ptr(registers.as_mut_ptr().cast()) };
        let mut timer = Lptmr::over(regs, &STATE, Config::default());
        timer.set_compare(1234);
        timer.start();
        assert_eq!(registers[2], 1234);
        assert!(timer.is_running());
        timer.stop();
        assert!(!timer.is_running());
    }
}
