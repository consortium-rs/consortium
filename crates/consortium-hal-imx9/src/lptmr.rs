//! i.MX9 low-power timer (LPTMR).
//!
//! LPTMR is a compare timer or external pulse counter. It has no waveform
//! output channel, so PWM belongs to TPM rather than this module.

use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::lptmr::LPTMR;

use crate::pac;

#[cfg(feature = "dma")]
use crate::dma::{
    DmaChannel, DmaRequest, RingError, Transfer, WritableRingBuffer, request as dma_request,
};

/// LPTMR peripheral instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instance {
    Lptmr1,
    Lptmr2,
}

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
pub struct LptmrState {
    regs: AtomicPtr<()>,
    pending: AtomicBool,
    waker: AtomicWaker,
}

impl LptmrState {
    /// Create empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            regs: AtomicPtr::new(core::ptr::null_mut()),
            pending: AtomicBool::new(false),
            waker: AtomicWaker::new(),
        }
    }

    /// Forward the LPTMR interrupt here.
    pub fn on_interrupt(&self) {
        let ptr = self.regs.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: installed by `Lptmr::new` for the state dedicated to this
        // exclusively owned peripheral.
        let regs = unsafe { LPTMR::from_ptr(ptr) };
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

impl Default for LptmrState {
    fn default() -> Self {
        Self::new()
    }
}

/// Exclusive LPTMR driver.
pub struct Lptmr {
    regs: LPTMR,
    #[cfg(feature = "dma")]
    instance: Instance,
    state: &'static LptmrState,
}

impl Lptmr {
    /// Bind and configure an LPTMR.
    ///
    /// # Safety
    ///
    /// `base` must identify a valid, clocked, exclusively owned LPTMR.
    /// `state` must be dedicated to it and its IRQ must call
    /// [`LptmrState::on_interrupt`] for asynchronous waits.
    pub unsafe fn new(
        base: *mut (),
        instance: Instance,
        state: &'static LptmrState,
        config: Config,
    ) -> Self {
        assert!(config.prescaler <= 15);
        #[cfg(not(feature = "dma"))]
        let _ = instance;
        // SAFETY: guaranteed by the caller.
        let regs = unsafe { LPTMR::from_ptr(base) };
        state.regs.store(regs.as_ptr(), Ordering::Release);
        regs.CSR().write_value(Default::default());
        regs.PSR().write(|w| {
            w.set_PCS(config.clock_source as u8);
            w.set_PBYP(config.bypass_prescaler);
            w.set_PRESCALE(config.prescaler);
        });
        regs.CSR().modify(|w| w.set_TFC(config.free_running));
        Self {
            regs,
            #[cfg(feature = "dma")]
            instance,
            state,
        }
    }

    /// Infer the instance from its standard secure/non-secure address.
    ///
    /// # Safety
    ///
    /// Same requirements as [`Lptmr::new`].
    pub unsafe fn from_raw(base: *mut (), state: &'static LptmrState, config: Config) -> Self {
        let instance = match (base as usize) & !0x1000_0000 {
            0x4430_0000 => Instance::Lptmr1,
            0x424d_0000 => Instance::Lptmr2,
            _ => panic!("unknown LPTMR address; use Lptmr::new"),
        };
        // SAFETY: forwarded caller contract.
        unsafe { Self::new(base, instance, state, config) }
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
        match self.instance {
            Instance::Lptmr1 => dma_request::LPTMR1,
            Instance::Lptmr2 => dma_request::LPTMR2,
        }
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
        static STATE: LptmrState = LptmrState::new();
        let mut registers = [0u32; 4];
        // SAFETY: fake MMIO is exclusively owned by the test.
        let mut timer = unsafe {
            Lptmr::new(
                registers.as_mut_ptr().cast(),
                Instance::Lptmr1,
                &STATE,
                Config {
                    clock_source: ClockSource::Source2,
                    bypass_prescaler: false,
                    prescaler: 3,
                    free_running: true,
                },
            )
        };
        assert_eq!(registers[1] & 0x7f, 2 | (3 << 3));
        timer.configure_pulse_counter(PulseInput::Input2, PulsePolarity::Falling);
        assert_eq!(
            registers[0] & 0x3e,
            (1 << 1) | (1 << 2) | (1 << 3) | (2 << 4)
        );
    }

    #[test]
    fn start_stop_and_compare() {
        static STATE: LptmrState = LptmrState::new();
        let mut registers = [0u32; 4];
        // SAFETY: fake MMIO is exclusively owned by the test.
        let mut timer = unsafe {
            Lptmr::new(
                registers.as_mut_ptr().cast(),
                Instance::Lptmr1,
                &STATE,
                Config::default(),
            )
        };
        timer.set_compare(1234);
        timer.start();
        assert_eq!(registers[2], 1234);
        assert!(timer.is_running());
        timer.stop();
        assert!(!timer.is_running());
    }
}
