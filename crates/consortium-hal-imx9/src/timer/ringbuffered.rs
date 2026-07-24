//! eDMA-backed TPM PWM and input-capture streams.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::dma::{
    DmaChannel, DmaRequest, ReadableRingBuffer, RingError, Transfer, WritableRingBuffer, request,
};

use super::{Channel, Instance, Pwm, Timer};

/// Input-capture edge selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureEdge {
    Rising,
    Falling,
    Both,
}

/// TPM input-capture mode.
pub struct InputCapture {
    timer: Timer,
}

impl InputCapture {
    /// Configure a TPM for free-running input capture.
    pub fn new(timer: Timer) -> Self {
        let tpm = timer.regs();
        tpm.SC().modify(|w| w.set_CMOD(0));
        tpm.CNT().write(|w| w.set_COUNT(0));
        tpm.MOD().write(|w| w.set_MOD(u16::MAX as u32));
        tpm.SC().modify(|w| w.set_CMOD(1));
        Self { timer }
    }

    /// Configure a capture channel.
    pub fn configure_channel(&mut self, channel: Channel, edge: CaptureEdge) {
        let (elsa, elsb) = match edge {
            CaptureEdge::Rising => (true, false),
            CaptureEdge::Falling => (false, true),
            CaptureEdge::Both => (true, true),
        };
        self.timer
            .regs()
            .CONTROLS_CnSC(channel.index())
            .modify(|w| {
                w.set_MSA(false);
                w.set_MSB(false);
                w.set_ELSA(elsa);
                w.set_ELSB(elsb);
                w.set_CHF(true);
            });
    }

    /// Consume capture mode into an eDMA-backed channel.
    pub fn into_ring_buffered_channel<'a>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u32],
    ) -> RingBufferedCapture<'a> {
        RingBufferedCapture::new(self, channel, dma, buffer)
    }

    /// Perform a finite input-capture DMA transfer.
    pub fn read_dma<'a>(
        &'a mut self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        values: &'a mut [u32],
    ) -> CaptureTransfer<'a> {
        let dma_request = channel_request(self.timer.instance(), channel);
        let register = self
            .timer
            .regs()
            .CONTROLS_CnV(channel.index())
            .as_ptr()
            .cast::<u32>();
        self.timer
            .regs()
            .CONTROLS_CnSC(channel.index())
            .modify(|w| {
                w.set_CHF(true);
                w.set_DMA(true);
            });
        // SAFETY: CnV is the TPM request's peripheral-to-memory data register.
        let transfer = unsafe { dma.read_peripheral_with_request(register, values, dma_request) };
        CaptureTransfer {
            transfer: Some(transfer),
            tpm: self.timer.regs(),
            channel,
        }
    }
}

impl Timer {
    /// Consume this timer into input-capture mode.
    pub fn into_input_capture(self) -> InputCapture {
        InputCapture::new(self)
    }
}

impl Pwm {
    /// Consume PWM mode into an eDMA-backed compare channel.
    pub fn into_ring_buffered_channel<'a>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u32],
    ) -> RingBufferedPwm<'a> {
        RingBufferedPwm::new(self, channel, dma, buffer)
    }

    /// Perform a finite sequence of compare-register writes.
    pub fn write_dma<'a>(
        &'a mut self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        values: &'a [u32],
    ) -> PwmTransfer<'a> {
        let dma_request = channel_request(self.timer.instance(), channel);
        let register = self
            .timer
            .regs()
            .CONTROLS_CnV(channel.index())
            .as_ptr()
            .cast::<u32>();
        self.timer
            .regs()
            .CONTROLS_CnSC(channel.index())
            .modify(|w| w.set_DMA(true));
        // SAFETY: CnV is the TPM request's memory-to-peripheral data register.
        let transfer = unsafe { dma.write_peripheral_with_request(values, register, dma_request) };
        PwmTransfer {
            transfer: Some(transfer),
            tpm: self.timer.regs(),
            channel,
        }
    }
}

/// Finite PWM DMA transfer which disables the TPM request on cancellation.
pub struct PwmTransfer<'a> {
    transfer: Option<Transfer<'a>>,
    tpm: crate::pac::peripherals::tpm::TPM,
    channel: Channel,
}

impl PwmTransfer<'_> {
    /// Busy-wait for completion.
    pub fn wait_blocking(mut self) -> Result<(), crate::dma::Error> {
        let result = self
            .transfer
            .take()
            .expect("transfer already consumed")
            .wait_blocking();
        self.disable_request();
        result
    }

    fn disable_request(&self) {
        self.tpm
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| w.set_DMA(false));
    }
}

impl Future for PwmTransfer<'_> {
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
            this.disable_request();
        }
        result
    }
}

impl Drop for PwmTransfer<'_> {
    fn drop(&mut self) {
        drop(self.transfer.take());
        self.disable_request();
    }
}

/// Finite input-capture DMA transfer.
pub struct CaptureTransfer<'a> {
    transfer: Option<Transfer<'a>>,
    tpm: crate::pac::peripherals::tpm::TPM,
    channel: Channel,
}

impl CaptureTransfer<'_> {
    /// Busy-wait for all timestamps.
    pub fn wait_blocking(mut self) -> Result<(), crate::dma::Error> {
        let result = self
            .transfer
            .take()
            .expect("transfer already consumed")
            .wait_blocking();
        self.disable_request();
        result
    }

    fn disable_request(&self) {
        self.tpm
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| w.set_DMA(false));
    }
}

impl Future for CaptureTransfer<'_> {
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
            this.disable_request();
        }
        result
    }
}

impl Drop for CaptureTransfer<'_> {
    fn drop(&mut self) {
        drop(self.transfer.take());
        self.disable_request();
    }
}

/// Circular TPM compare stream.
pub struct RingBufferedPwm<'a> {
    pwm: Pwm,
    channel: Channel,
    ring: Option<WritableRingBuffer<'a, u32>>,
}

impl<'a> RingBufferedPwm<'a> {
    fn new(pwm: Pwm, channel: Channel, dma: &'a mut DmaChannel, buffer: &'a mut [u32]) -> Self {
        let dma_request = channel_request(pwm.timer.instance(), channel);
        let register = pwm
            .timer
            .regs()
            .CONTROLS_CnV(channel.index())
            .as_ptr()
            .cast::<u32>();
        // SAFETY: CnV is the TPM request's memory-to-peripheral data register.
        let ring = unsafe { dma.write_circular(buffer, register, dma_request) };
        Self {
            pwm,
            channel,
            ring: Some(ring),
        }
    }

    /// Prime unused slots.
    pub fn write_immediate(&mut self, input: &[u32]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write_immediate(input)
    }

    /// Start compare updates.
    pub fn start(&mut self) {
        self.pwm
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| w.set_DMA(true));
        self.ring_mut().start();
    }

    /// Queue compare values.
    pub fn write(&mut self, input: &[u32]) -> Result<(usize, usize), RingError> {
        self.ring_mut().write(input)
    }

    /// Queue all compare values asynchronously.
    pub async fn write_exact(&mut self, input: &[u32]) -> Result<usize, RingError> {
        self.ring_mut().write_exact(input).await
    }

    /// Pause DMA requests.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume DMA requests.
    pub fn resume(&mut self) {
        self.ring_mut().resume();
    }

    /// Stop DMA and disable the TPM request.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        self.disable_request();
    }

    fn ring_mut(&mut self) -> &mut WritableRingBuffer<'a, u32> {
        self.ring.as_mut().expect("ring has been released")
    }

    fn disable_request(&self) {
        self.pwm
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| w.set_DMA(false));
    }
}

impl Drop for RingBufferedPwm<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        self.disable_request();
    }
}

/// Circular TPM input-capture stream.
pub struct RingBufferedCapture<'a> {
    capture: InputCapture,
    channel: Channel,
    ring: Option<ReadableRingBuffer<'a, u32>>,
}

impl<'a> RingBufferedCapture<'a> {
    fn new(
        capture: InputCapture,
        channel: Channel,
        dma: &'a mut DmaChannel,
        buffer: &'a mut [u32],
    ) -> Self {
        let dma_request = channel_request(capture.timer.instance(), channel);
        let register = capture
            .timer
            .regs()
            .CONTROLS_CnV(channel.index())
            .as_ptr()
            .cast::<u32>();
        // SAFETY: CnV is the TPM request's peripheral-to-memory data register.
        let ring = unsafe { dma.read_circular(register, buffer, dma_request) };
        Self {
            capture,
            channel,
            ring: Some(ring),
        }
    }

    /// Start capture requests.
    pub fn start(&mut self) {
        self.capture
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| {
                w.set_CHF(true);
                w.set_DMA(true);
            });
        self.ring_mut().start();
    }

    /// Read available capture timestamps.
    pub fn read(&mut self, output: &mut [u32]) -> Result<(usize, usize), RingError> {
        self.ring_mut().read(output)
    }

    /// Fill the complete output asynchronously.
    pub async fn read_exact(&mut self, output: &mut [u32]) -> Result<usize, RingError> {
        self.ring_mut().read_exact(output).await
    }

    /// Read only the newest timestamps.
    pub fn read_latest(&mut self, output: &mut [u32]) -> Result<usize, RingError> {
        self.ring_mut().read_latest(output)
    }

    /// Pause DMA requests.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume DMA requests.
    pub fn resume(&mut self) {
        self.ring_mut().resume();
    }

    /// Stop DMA and disable the TPM request.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        self.disable_request();
    }

    fn ring_mut(&mut self) -> &mut ReadableRingBuffer<'a, u32> {
        self.ring.as_mut().expect("ring has been released")
    }

    fn disable_request(&self) {
        self.capture
            .timer
            .regs()
            .CONTROLS_CnSC(self.channel.index())
            .modify(|w| w.set_DMA(false));
    }
}

impl Drop for RingBufferedCapture<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        self.disable_request();
    }
}

fn channel_request(instance: Instance, channel: Channel) -> DmaRequest {
    let even = matches!(channel, Channel::Ch0 | Channel::Ch2);
    match (instance, even) {
        (Instance::Tpm1, true) => request::TPM1_CH0_CH2,
        (Instance::Tpm1, false) => request::TPM1_CH1_CH3,
        (Instance::Tpm2, true) => request::TPM2_CH0_CH2,
        (Instance::Tpm2, false) => request::TPM2_CH1_CH3,
        (Instance::Tpm3, true) => request::TPM3_CH0_CH2,
        (Instance::Tpm3, false) => request::TPM3_CH1_CH3,
        (Instance::Tpm4, true) => request::TPM4_CH0_CH2,
        (Instance::Tpm4, false) => request::TPM4_CH1_CH3,
        (Instance::Tpm5, true) => request::TPM5_CH0_CH2,
        (Instance::Tpm5, false) => request::TPM5_CH1_CH3,
        (Instance::Tpm6, true) => request::TPM6_CH0_CH2,
        (Instance::Tpm6, false) => request::TPM6_CH1_CH3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dma::DmaState;

    #[cfg(feature = "mimx93")]
    const TPM_DMA_CHANNEL: usize = 23;
    #[cfg(feature = "mimx95")]
    const TPM_DMA_CHANNEL: usize = 26;

    #[test]
    fn finite_pwm_dma_enables_and_cleans_channel_request() {
        static STATE: DmaState = DmaState::new();
        let mut tpm_registers = [0u32; 0x40];
        let words = ((TPM_DMA_CHANNEL + 2) * 0x1_0000) / 4;
        let mut dma_registers = std::vec![0u32; words];

        // SAFETY: fake TPM registers are exclusively owned by this test.
        let timer = unsafe {
            Timer::from_raw_instance(
                tpm_registers.as_mut_ptr().cast(),
                24_000_000,
                Instance::Tpm1,
            )
        };
        let mut pwm = timer.into_pwm(20_000);
        // SAFETY: fake eDMA management and channel pages fit the allocation.
        let mut dma =
            unsafe { DmaChannel::new(dma_registers.as_mut_ptr().cast(), TPM_DMA_CHANNEL, &STATE) };
        let values = [100u32, 200, 300];

        {
            let transfer = pwm.write_dma(Channel::Ch0, &mut dma, &values);
            assert_ne!(tpm_registers[0x20 / 4] & 1, 0);
            drop(transfer);
        }
        assert_eq!(tpm_registers[0x20 / 4] & 1, 0);
    }

    #[test]
    fn later_tpm_instances_select_the_secondary_edma() {
        let request = channel_request(Instance::Tpm5, Channel::Ch3);
        #[cfg(feature = "mimx93")]
        assert_eq!(request.controller(), crate::dma::DmaController::Dma4);
        #[cfg(feature = "mimx95")]
        assert_eq!(request.controller(), crate::dma::DmaController::Dma5);
        assert_eq!(request.source(), 32);
    }
}
