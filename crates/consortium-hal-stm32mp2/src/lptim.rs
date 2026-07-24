//! STM32MP2 low-power timer (LPTIM).
//!
//! The v2 LPTIM block provides two PWM/capture channels, one-shot and
//! continuous counting, external trigger/counter/encoder modes, repetition,
//! interrupt-driven waits, and HPDMA capture/update streams. RCC, RIF,
//! low-power clock selection, and pin muxing remain application-owned.

use core::convert::Infallible;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicPtr, AtomicU32, Ordering};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;
use pac::peripherals::lptim::LPTIM;

use crate::pac;

#[cfg(feature = "dma")]
use crate::dma::{
    DmaChannel, DmaNode, DmaRequest, ReadableRingBuffer, RingError, Transfer, WritableRingBuffer,
    request as dma_request,
};

const CR_ENABLE: u32 = 1 << 0;
const CR_SNGSTRT: u32 = 1 << 1;
const CR_CNTSTRT: u32 = 1 << 2;
const CR_COUNTRST: u32 = 1 << 3;
const CR_RSTARE: u32 = 1 << 4;

#[cfg(feature = "dma")]
const DIER_CC1DE: u32 = 1 << 16;
#[cfg(feature = "dma")]
const DIER_UEDE: u32 = 1 << 23;
#[cfg(feature = "dma")]
const DIER_CC2DE: u32 = 1 << 25;

/// LPTIM peripheral instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instance {
    Lptim1,
    Lptim2,
    Lptim3,
    Lptim4,
    Lptim5,
}

/// Capture/compare channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Ch1,
    Ch2,
}

impl Channel {
    const fn event(self) -> Event {
        match self {
            Self::Ch1 => Event::CaptureCompare1,
            Self::Ch2 => Event::CaptureCompare2,
        }
    }
}

/// Counter event which may be awaited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Event {
    CaptureCompare1 = 0,
    AutoReloadMatch = 1,
    ExternalTrigger = 2,
    Update = 7,
    CaptureCompare2 = 9,
    OverCapture1 = 12,
    OverCapture2 = 13,
}

impl Event {
    const fn mask(self) -> u32 {
        1 << self as u8
    }
}

/// External trigger edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TriggerEdge {
    Software = 0,
    Rising = 1,
    Falling = 2,
    Both = 3,
}

/// Capture/output polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Edge {
    Rising = 0,
    Falling = 1,
    Both = 2,
}

/// PWM output polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}

/// LPTIM configuration error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    FrequencyOutOfRange,
    DmaUnavailable,
}

/// IRQ state dedicated to one LPTIM.
pub struct LptimState {
    regs: AtomicPtr<()>,
    pending: AtomicU32,
    waker: AtomicWaker,
}

impl LptimState {
    /// Create empty state suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            regs: AtomicPtr::new(core::ptr::null_mut()),
            pending: AtomicU32::new(0),
            waker: AtomicWaker::new(),
        }
    }

    /// Forward the LPTIM global interrupt here.
    pub fn on_interrupt(&self) {
        let ptr = self.regs.load(Ordering::Acquire);
        if ptr.is_null() {
            return;
        }
        // SAFETY: installed by `LowPowerTimer::new` for the state dedicated to
        // this exclusively owned peripheral.
        let regs = unsafe { LPTIM::from_ptr(ptr) };
        let pending = regs.ISR().read().0 & regs.DIER().read().0 & 0x0000_ffff;
        if pending != 0 {
            self.pending.fetch_or(pending, Ordering::AcqRel);
            regs.ICR().write(|w| w.0 = pending);
            regs.DIER().modify(|w| w.0 &= !pending);
            self.waker.wake();
        }
    }
}

impl Default for LptimState {
    fn default() -> Self {
        Self::new()
    }
}

/// Low-level LPTIM driver.
pub struct LowPowerTimer {
    regs: LPTIM,
    #[cfg(feature = "dma")]
    instance: Instance,
    clock_hz: u32,
    state: &'static LptimState,
}

impl LowPowerTimer {
    /// Bind an LPTIM register block.
    ///
    /// # Safety
    ///
    /// `base` must be a valid, clocked, RIF-configured, exclusively owned
    /// LPTIM. `instance` must match it. `state` must be dedicated to it.
    pub unsafe fn new(
        base: *mut (),
        instance: Instance,
        clock_hz: u32,
        state: &'static LptimState,
    ) -> Self {
        assert!(clock_hz > 0);
        #[cfg(not(feature = "dma"))]
        let _ = instance;
        // SAFETY: guaranteed by the caller.
        let regs = unsafe { LPTIM::from_ptr(base) };
        state.regs.store(regs.as_ptr(), Ordering::Release);
        regs.CR().write(|w| w.0 = 0);
        regs.DIER().write(|w| w.0 = 0);
        regs.ICR().write(|w| w.0 = 0x01ff_ffff);
        regs.CFGR().write(|w| w.0 = 0);
        regs.CCMR1().write(|w| w.0 = 0);
        regs.CR().write(|w| w.0 = CR_ENABLE);
        Self {
            regs,
            #[cfg(feature = "dma")]
            instance,
            clock_hz,
            state,
        }
    }

    /// Enable the peripheral.
    pub fn enable(&mut self) {
        self.regs.CR().modify(|w| w.0 |= CR_ENABLE);
    }

    /// Stop and disable the peripheral.
    pub fn disable(&mut self) {
        self.regs.CR().write(|w| w.0 = 0);
    }

    /// Whether the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        self.regs.CR().read().0 & CR_ENABLE != 0
    }

    /// Start continuous counting.
    pub fn start_continuous(&mut self) {
        self.enable();
        self.regs.CR().modify(|w| w.0 |= CR_CNTSTRT);
    }

    /// Start one-shot counting.
    pub fn start_single(&mut self) {
        self.enable();
        self.regs.CR().modify(|w| w.0 |= CR_SNGSTRT);
    }

    /// Reset the counter.
    pub fn reset_counter(&mut self) {
        self.regs.CR().modify(|w| w.0 |= CR_COUNTRST);
    }

    /// Reset the counter after each counter-register read.
    pub fn set_reset_after_read(&mut self, enabled: bool) {
        self.regs.CR().modify(|w| {
            if enabled {
                w.0 |= CR_RSTARE;
            } else {
                w.0 &= !CR_RSTARE;
            }
        });
    }

    /// Current counter value.
    pub fn counter(&self) -> u16 {
        self.regs.CNT().read().CNT()
    }

    /// Program raw prescaler and autoreload values.
    pub fn set_period(&mut self, prescaler_log2: u8, autoreload: u16) {
        assert!(prescaler_log2 <= 7);
        self.regs.CFGR().modify(|w| w.set_PRESC(prescaler_log2));
        self.regs.ARR().write(|w| w.set_ARR(autoreload));
    }

    /// Program the counter overflow frequency.
    pub fn set_frequency(&mut self, frequency_hz: u32) -> Result<(), Error> {
        let (prescaler, autoreload) = compute_period(self.clock_hz, frequency_hz)?;
        self.set_period(prescaler, autoreload);
        Ok(())
    }

    /// Autoreload value.
    pub fn autoreload(&self) -> u16 {
        self.regs.ARR().read().ARR()
    }

    /// Repetition count (`0` means every period).
    pub fn set_repetition(&mut self, repetitions: u8) {
        self.regs.RCR().write(|w| w.set_REP(repetitions));
    }

    /// Configure external-trigger start.
    pub fn configure_trigger(&mut self, source: u8, edge: TriggerEdge, filter: u8) {
        assert!(source <= 7 && filter <= 3);
        self.regs.CFGR().modify(|w| {
            w.set_TRIGSEL(source);
            w.set_TRIGEN(edge as u8);
            w.set_TRGFLT(filter);
        });
    }

    /// Enable timeout mode.
    pub fn set_timeout_mode(&mut self, enabled: bool) {
        self.regs.CFGR().modify(|w| w.set_TIMOUT(enabled));
    }

    /// Count external clock edges instead of the internal kernel clock.
    pub fn configure_external_counter(&mut self, polarity: Edge, filter: u8) {
        assert!(filter <= 3);
        self.regs.CFGR().modify(|w| {
            w.set_CKSEL(true);
            w.set_CKPOL(polarity as u8);
            w.set_CKFLT(filter);
            w.set_COUNTMODE(true);
            w.set_ENC(false);
        });
    }

    /// Enable quadrature encoder mode.
    pub fn configure_encoder(&mut self, filter: u8) {
        assert!(filter <= 3);
        self.regs.CFGR().modify(|w| {
            w.set_CKSEL(true);
            w.set_CKFLT(filter);
            w.set_COUNTMODE(true);
            w.set_ENC(true);
        });
    }

    /// Return to internal-clock timer mode.
    pub fn configure_internal_clock(&mut self) {
        self.regs.CFGR().modify(|w| {
            w.set_CKSEL(false);
            w.set_COUNTMODE(false);
            w.set_ENC(false);
        });
    }

    /// Wait asynchronously for a timer event.
    pub fn wait_event(&mut self, event: Event) -> WaitEvent<'_> {
        let mask = event.mask();
        self.state.pending.fetch_and(!mask, Ordering::AcqRel);
        self.regs.ICR().write(|w| w.0 = mask);
        self.regs.DIER().modify(|w| w.0 |= mask);
        WaitEvent {
            timer: self,
            event,
            complete: false,
        }
    }

    /// Consume the timer into two-channel PWM mode.
    pub fn into_pwm(mut self, frequency_hz: u32) -> Result<Pwm, Error> {
        self.set_frequency(frequency_hz)?;
        self.regs.CFGR().modify(|w| {
            w.set_WAVE(false);
            w.set_PRELOAD(true);
        });
        self.regs.CCMR1().modify(|w| {
            w.set_CC1SEL(false);
            w.set_CC2SEL(false);
        });
        self.start_continuous();
        Ok(Pwm { timer: self })
    }

    /// Consume the timer into input-capture mode.
    pub fn into_input_capture(mut self) -> InputCapture {
        self.set_period(0, u16::MAX);
        self.start_continuous();
        InputCapture { timer: self }
    }

    fn disable_event(&mut self, event: Event) {
        self.regs.DIER().modify(|w| w.0 &= !event.mask());
    }

    #[cfg(feature = "dma")]
    fn capture_request(&self, channel: Channel) -> Result<DmaRequest, Error> {
        match (self.instance, channel) {
            (Instance::Lptim1, Channel::Ch1) => Ok(dma_request::LPTIM1_IC1),
            (Instance::Lptim1, Channel::Ch2) => Ok(dma_request::LPTIM1_IC2),
            (Instance::Lptim2, Channel::Ch1) => Ok(dma_request::LPTIM2_IC1),
            (Instance::Lptim2, Channel::Ch2) => Ok(dma_request::LPTIM2_IC2),
            (Instance::Lptim3, Channel::Ch1) => Ok(dma_request::LPTIM3_IC1),
            (Instance::Lptim3, Channel::Ch2) => Ok(dma_request::LPTIM3_IC2),
            (Instance::Lptim4, Channel::Ch1) => Ok(dma_request::LPTIM4_IC1),
            (Instance::Lptim4, Channel::Ch2) => Ok(dma_request::LPTIM4_IC2),
            (Instance::Lptim5, _) => Err(Error::DmaUnavailable),
        }
    }

    #[cfg(feature = "dma")]
    fn update_request(&self) -> Result<DmaRequest, Error> {
        match self.instance {
            Instance::Lptim1 => Ok(dma_request::LPTIM1_UE),
            Instance::Lptim2 => Ok(dma_request::LPTIM2_UE),
            Instance::Lptim3 => Ok(dma_request::LPTIM3_UE),
            Instance::Lptim4 => Ok(dma_request::LPTIM4_UE),
            Instance::Lptim5 => Err(Error::DmaUnavailable),
        }
    }
}

/// Future returned by [`LowPowerTimer::wait_event`].
pub struct WaitEvent<'a> {
    timer: &'a mut LowPowerTimer,
    event: Event,
    complete: bool,
}

impl Future for WaitEvent<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let event = self.event;
        let mask = event.mask();
        if self.timer.state.pending.fetch_and(!mask, Ordering::AcqRel) & mask != 0
            || self.timer.regs.ISR().read().0 & mask != 0
        {
            self.timer.regs.ICR().write(|w| w.0 = mask);
            self.timer.disable_event(event);
            self.complete = true;
            return Poll::Ready(());
        }
        self.timer.state.waker.register(cx.waker());
        if self.timer.state.pending.fetch_and(!mask, Ordering::AcqRel) & mask != 0 {
            self.timer.disable_event(event);
            self.complete = true;
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for WaitEvent<'_> {
    fn drop(&mut self) {
        if !self.complete {
            self.timer.disable_event(self.event);
        }
    }
}

/// Two-channel LPTIM PWM.
pub struct Pwm {
    timer: LowPowerTimer,
}

impl Pwm {
    /// Maximum duty value.
    pub fn max_duty_cycle(&self) -> u16 {
        self.timer.autoreload().saturating_add(1)
    }

    /// Change PWM frequency.
    pub fn set_frequency(&mut self, frequency_hz: u32) -> Result<(), Error> {
        self.timer.set_frequency(frequency_hz)
    }

    /// Enable a PWM channel.
    pub fn enable(&mut self, channel: Channel) {
        self.timer.regs.CCMR1().modify(|w| match channel {
            Channel::Ch1 => w.set_CC1E(true),
            Channel::Ch2 => w.set_CC2E(true),
        });
    }

    /// Disable a PWM channel.
    pub fn disable(&mut self, channel: Channel) {
        self.timer.regs.CCMR1().modify(|w| match channel {
            Channel::Ch1 => w.set_CC1E(false),
            Channel::Ch2 => w.set_CC2E(false),
        });
    }

    /// Select output polarity.
    pub fn set_polarity(&mut self, channel: Channel, polarity: Polarity) {
        let value = u8::from(matches!(polarity, Polarity::ActiveLow));
        self.timer.regs.CCMR1().modify(|w| match channel {
            Channel::Ch1 => w.set_CC1P(value),
            Channel::Ch2 => w.set_CC2P(value),
        });
    }

    /// Set a raw duty value, clamped to the period.
    pub fn set_duty_cycle(&mut self, channel: Channel, duty: u16) {
        let duty = duty.min(self.max_duty_cycle());
        match channel {
            Channel::Ch1 => self.timer.regs.CCR1().write(|w| w.set_CCR1(duty)),
            Channel::Ch2 => self.timer.regs.CCR2().write(|w| w.set_CCR2(duty)),
        }
    }

    /// Read a channel duty value.
    pub fn duty_cycle(&self, channel: Channel) -> u16 {
        match channel {
            Channel::Ch1 => self.timer.regs.CCR1().read().CCR1(),
            Channel::Ch2 => self.timer.regs.CCR2().read().CCR2(),
        }
    }

    /// Borrow one output channel for `embedded-hal` PWM control.
    pub fn channel(&mut self, channel: Channel) -> PwmChannel<'_> {
        PwmChannel { pwm: self, channel }
    }

    /// Stop PWM and recover the low-power timer.
    pub fn free(mut self) -> LowPowerTimer {
        self.timer.disable();
        self.timer
    }

    /// Construct an update-event-driven circular duty stream.
    #[cfg(feature = "dma")]
    pub fn into_ring_buffered_channel<'a>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u16],
        node: &'a mut DmaNode,
    ) -> Result<RingBufferedPwm<'a>, Error> {
        let request = self.timer.update_request()?;
        let destination = compare_ptr(self.timer.regs, channel);
        // SAFETY: CCR is updated from memory on this LPTIM's UE request.
        let ring = unsafe { dma.write_circular(buffer, destination, request, node) };
        Ok(RingBufferedPwm {
            pwm: self,
            channel,
            ring: Some(ring),
        })
    }

    /// Perform a finite update-event-driven duty transfer.
    #[cfg(feature = "dma")]
    pub fn write_dma<'a>(
        &'a mut self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        values: &'a [u16],
    ) -> Result<LptimDmaTransfer<'a>, Error> {
        let request = self.timer.update_request()?;
        let destination = compare_ptr(self.timer.regs, channel);
        // SAFETY: CCR is updated from memory on this LPTIM's UE request.
        let transfer = unsafe { dma.write_peripheral(values, destination, request) };
        self.timer.regs.DIER().modify(|w| w.0 |= DIER_UEDE);
        Ok(LptimDmaTransfer {
            transfer: Some(transfer),
            regs: self.timer.regs,
            dma_enable: DIER_UEDE,
        })
    }
}

/// Borrowed handle to one LPTIM PWM output.
pub struct PwmChannel<'a> {
    pwm: &'a mut Pwm,
    channel: Channel,
}

impl PwmChannel<'_> {
    /// Enable the channel output.
    pub fn enable(&mut self) {
        self.pwm.enable(self.channel);
    }

    /// Disable the channel output.
    pub fn disable(&mut self) {
        self.pwm.disable(self.channel);
    }

    /// Select output polarity.
    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.pwm.set_polarity(self.channel, polarity);
    }

    /// Maximum raw duty value.
    pub fn max_duty_cycle(&self) -> u16 {
        self.pwm.max_duty_cycle()
    }

    /// Set the raw duty value.
    pub fn set_duty_cycle(&mut self, duty: u16) {
        self.pwm.set_duty_cycle(self.channel, duty);
    }
}

impl embedded_hal::pwm::ErrorType for PwmChannel<'_> {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmChannel<'_> {
    fn max_duty_cycle(&self) -> u16 {
        PwmChannel::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        PwmChannel::set_duty_cycle(self, duty);
        Ok(())
    }
}

/// LPTIM input capture.
pub struct InputCapture {
    timer: LowPowerTimer,
}

impl InputCapture {
    /// Configure a capture channel.
    pub fn configure_channel(
        &mut self,
        channel: Channel,
        edge: Edge,
        filter: u8,
        prescaler: u8,
        input_selection: u8,
    ) {
        assert!(filter <= 3 && prescaler <= 3 && input_selection <= 3);
        self.timer.regs.CCMR1().modify(|w| match channel {
            Channel::Ch1 => {
                w.set_CC1SEL(true);
                w.set_CC1E(true);
                w.set_CC1P(edge as u8);
                w.set_IC1F(filter);
                w.set_IC1PSC(prescaler);
            }
            Channel::Ch2 => {
                w.set_CC2SEL(true);
                w.set_CC2E(true);
                w.set_CC2P(edge as u8);
                w.set_IC2F(filter);
                w.set_IC2PSC(prescaler);
            }
        });
        self.timer.regs.CFGR2().modify(|w| match channel {
            Channel::Ch1 => w.set_IC1SEL(input_selection),
            Channel::Ch2 => w.set_IC2SEL(input_selection),
        });
    }

    /// Latest captured count.
    pub fn capture(&self, channel: Channel) -> u16 {
        match channel {
            Channel::Ch1 => self.timer.regs.CCR1().read().CCR1(),
            Channel::Ch2 => self.timer.regs.CCR2().read().CCR2(),
        }
    }

    /// Whether another edge arrived before the previous capture was consumed.
    pub fn overcaptured(&self, channel: Channel) -> bool {
        let status = self.timer.regs.ISR().read().0;
        status
            & match channel {
                Channel::Ch1 => Event::OverCapture1.mask(),
                Channel::Ch2 => Event::OverCapture2.mask(),
            }
            != 0
    }

    /// Wait for the next capture.
    pub fn wait_capture(&mut self, channel: Channel) -> WaitEvent<'_> {
        self.timer.wait_event(channel.event())
    }

    /// Construct a circular capture stream.
    #[cfg(feature = "dma")]
    pub fn into_ring_buffered_channel<'a>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u16],
        node: &'a mut DmaNode,
    ) -> Result<RingBufferedCapture<'a>, Error> {
        let request = self.timer.capture_request(channel)?;
        let source = compare_ptr(self.timer.regs, channel).cast_const();
        // SAFETY: CCR is the capture register associated with the IC request.
        let ring = unsafe { dma.read_circular(source, buffer, request, node) };
        Ok(RingBufferedCapture {
            capture: self,
            channel,
            ring: Some(ring),
        })
    }

    /// Perform a finite capture transfer.
    #[cfg(feature = "dma")]
    pub fn read_dma<'a>(
        &'a mut self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        values: &'a mut [u16],
    ) -> Result<LptimDmaTransfer<'a>, Error> {
        let request = self.timer.capture_request(channel)?;
        let source = compare_ptr(self.timer.regs, channel).cast_const();
        // SAFETY: CCR is the capture register associated with the IC request.
        let transfer = unsafe { dma.read_peripheral(source, values, request) };
        let enable = capture_dma_enable(channel);
        self.timer.regs.DIER().modify(|w| w.0 |= enable);
        Ok(LptimDmaTransfer {
            transfer: Some(transfer),
            regs: self.timer.regs,
            dma_enable: enable,
        })
    }
}

#[cfg(feature = "dma")]
fn compare_ptr(regs: LPTIM, channel: Channel) -> *mut u16 {
    match channel {
        Channel::Ch1 => regs.CCR1().as_ptr().cast(),
        Channel::Ch2 => regs.CCR2().as_ptr().cast(),
    }
}

#[cfg(feature = "dma")]
const fn capture_dma_enable(channel: Channel) -> u32 {
    match channel {
        Channel::Ch1 => DIER_CC1DE,
        Channel::Ch2 => DIER_CC2DE,
    }
}

/// Finite LPTIM DMA operation.
#[cfg(feature = "dma")]
pub struct LptimDmaTransfer<'a> {
    transfer: Option<Transfer<'a>>,
    regs: LPTIM,
    dma_enable: u32,
}

#[cfg(feature = "dma")]
impl Future for LptimDmaTransfer<'_> {
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
            this.regs.DIER().modify(|w| w.0 &= !this.dma_enable);
        }
        result
    }
}

#[cfg(feature = "dma")]
impl LptimDmaTransfer<'_> {
    /// Busy-wait for completion.
    pub fn wait_blocking(mut self) -> Result<(), crate::dma::Error> {
        let result = self
            .transfer
            .take()
            .expect("transfer already consumed")
            .wait_blocking();
        self.regs.DIER().modify(|w| w.0 &= !self.dma_enable);
        result
    }
}

#[cfg(feature = "dma")]
impl Drop for LptimDmaTransfer<'_> {
    fn drop(&mut self) {
        drop(self.transfer.take());
        self.regs.DIER().modify(|w| w.0 &= !self.dma_enable);
    }
}

/// Circular LPTIM PWM stream.
#[cfg(feature = "dma")]
pub struct RingBufferedPwm<'a> {
    pwm: Pwm,
    channel: Channel,
    ring: Option<WritableRingBuffer<'a, u16>>,
}

#[cfg(feature = "dma")]
impl<'a> RingBufferedPwm<'a> {
    /// Prime unused ring slots before starting.
    pub fn write_immediate(&mut self, values: &[u16]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write_immediate(values)
    }

    /// Enable update-event DMA and start the circular transfer.
    pub fn start(&mut self) {
        self.pwm.timer.regs.DIER().modify(|w| w.0 |= DIER_UEDE);
        self.ring_mut().start();
    }

    /// Queue duty values.
    pub fn write(&mut self, values: &[u16]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write(values)
    }

    /// Queue all duty values asynchronously.
    pub async fn write_exact(&mut self, values: &[u16]) -> Result<usize, RingError> {
        self.ring_mut().write_exact(values).await
    }

    /// Number of queued duty values.
    pub fn len(&self) -> Result<usize, RingError> {
        self.ring.as_ref().expect("ring has been released").len()
    }

    /// Whether no duty values are queued.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .is_empty()
    }

    /// Ring capacity in duty values.
    pub fn capacity(&self) -> usize {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .capacity()
    }

    /// Pause the DMA channel.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume the DMA channel.
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

    /// Stop DMA and disable update-event requests.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        self.pwm.timer.regs.DIER().modify(|w| w.0 &= !DIER_UEDE);
    }

    /// PWM channel driven by this stream.
    pub const fn channel(&self) -> Channel {
        self.channel
    }

    fn ring_mut(&mut self) -> &mut WritableRingBuffer<'a, u16> {
        self.ring.as_mut().expect("ring has been released")
    }
}

#[cfg(feature = "dma")]
impl Drop for RingBufferedPwm<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        self.pwm.timer.regs.DIER().modify(|w| w.0 &= !DIER_UEDE);
    }
}

/// Circular LPTIM capture stream.
#[cfg(feature = "dma")]
pub struct RingBufferedCapture<'a> {
    capture: InputCapture,
    channel: Channel,
    ring: Option<ReadableRingBuffer<'a, u16>>,
}

#[cfg(feature = "dma")]
impl<'a> RingBufferedCapture<'a> {
    /// Enable capture DMA and start the circular transfer.
    pub fn start(&mut self) {
        let enable = capture_dma_enable(self.channel);
        self.capture.timer.regs.DIER().modify(|w| w.0 |= enable);
        self.ring_mut().start();
    }

    /// Read available capture values.
    pub fn read(&mut self, values: &mut [u16]) -> Result<(usize, usize), RingError> {
        self.ring_mut().read(values)
    }

    /// Fill the complete output slice asynchronously.
    pub async fn read_exact(&mut self, values: &mut [u16]) -> Result<usize, RingError> {
        self.ring_mut().read_exact(values).await
    }

    /// Read the newest capture values, discarding older unread values.
    pub fn read_latest(&mut self, values: &mut [u16]) -> Result<usize, RingError> {
        self.ring_mut().read_latest(values)
    }

    /// Number of unread capture values.
    pub fn len(&self) -> Result<usize, RingError> {
        self.ring.as_ref().expect("ring has been released").len()
    }

    /// Whether there are no unread capture values.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .is_empty()
    }

    /// Ring capacity in capture values.
    pub fn capacity(&self) -> usize {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .capacity()
    }

    /// Pause the DMA channel.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume the DMA channel.
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

    /// Stop DMA and disable capture requests.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        let enable = capture_dma_enable(self.channel);
        self.capture.timer.regs.DIER().modify(|w| w.0 &= !enable);
    }

    fn ring_mut(&mut self) -> &mut ReadableRingBuffer<'a, u16> {
        self.ring.as_mut().expect("ring has been released")
    }
}

#[cfg(feature = "dma")]
impl Drop for RingBufferedCapture<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        let enable = capture_dma_enable(self.channel);
        self.capture.timer.regs.DIER().modify(|w| w.0 &= !enable);
    }
}

fn compute_period(clock_hz: u32, frequency_hz: u32) -> Result<(u8, u16), Error> {
    if frequency_hz == 0 || frequency_hz > clock_hz {
        return Err(Error::FrequencyOutOfRange);
    }
    let ticks = clock_hz / frequency_hz;
    for prescaler in 0..=7 {
        let divided = ticks / (1u32 << prescaler);
        if (1..=u16::MAX as u32 + 1).contains(&divided) {
            return Ok((prescaler, (divided - 1) as u16));
        }
    }
    Err(Error::FrequencyOutOfRange)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_smallest_prescaler() {
        assert_eq!(compute_period(32_768, 1), Ok((0, 32_767)));
        assert_eq!(compute_period(100_000_000, 100), Ok((4, 62_499)));
    }

    #[test]
    fn configures_pwm_and_capture_channels() {
        static STATE: LptimState = LptimState::new();
        let mut registers = [0u32; 256];
        // SAFETY: fake MMIO is exclusively owned by the test.
        let timer = unsafe {
            LowPowerTimer::new(
                registers.as_mut_ptr().cast(),
                Instance::Lptim1,
                32_768,
                &STATE,
            )
        };
        let mut pwm = timer.into_pwm(128).unwrap();
        pwm.enable(Channel::Ch2);
        pwm.set_duty_cycle(Channel::Ch2, 64);
        assert_ne!(registers[0x2c / 4] & (1 << 17), 0);
        assert_eq!(registers[0x34 / 4], 64);
    }
}
