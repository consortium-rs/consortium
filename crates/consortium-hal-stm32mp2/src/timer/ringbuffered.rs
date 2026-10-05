//! DMA-backed TIM waveform and capture streams.
//!
//! For a hardware smoke test, route one PWM pin to a capture pin, stream a
//! known compare pattern, and verify the captured edge intervals. Place the
//! buffers and [`DmaNode`] in DMA-coherent SRAM and forward the HPDMA channel
//! IRQ to [`crate::dma::on_interrupt`].

use crate::dma::{
    DmaChannel, DmaNode, DmaRequest, DmaWord, Error as DmaError, ReadableRingBuffer, RingError,
    WritableRingBuffer,
};

use super::{Channel, ComplementaryPwm, InputCapture, Polarity, Pwm, Timer, TimerWidth};

mod sealed {
    pub trait Sealed {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}

/// Counter-sized word accepted by TIM DMA.
pub trait TimerDmaWord: DmaWord + sealed::Sealed {}
impl TimerDmaWord for u16 {}
impl TimerDmaWord for u32 {}

fn validate_width<W: TimerDmaWord>(timer: &Timer) {
    let expected = match timer.width() {
        TimerWidth::Bits16 => 2,
        TimerWidth::Bits32 => 4,
    };
    assert_eq!(
        core::mem::size_of::<W>(),
        expected,
        "DMA word must match the timer counter width"
    );
}

fn compare_pointer<W>(timer: &Timer, channel: Channel) -> *mut W {
    timer.regs().CCR(channel.index()).as_ptr().cast()
}

struct ChannelDmaGuard<'a> {
    timer: &'a mut Timer,
    channel: Channel,
}

impl Drop for ChannelDmaGuard<'_> {
    fn drop(&mut self) {
        self.timer.set_capture_dma(self.channel, false);
    }
}

struct UpdateDmaGuard<'a>(&'a mut Timer);

impl Drop for UpdateDmaGuard<'_> {
    fn drop(&mut self) {
        self.0.set_update_dma(false);
    }
}

impl Pwm {
    /// Stream compare values continuously into one PWM channel.
    pub fn into_ring_buffered_channel<'a, W: TimerDmaWord>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
        buffer: &'a mut [W],
    ) -> RingBufferedPwmChannel<'a, W> {
        let (mut timer, max_duty) = self.into_parts();
        validate_width::<W>(&timer);
        timer.set_capture_dma(channel, false);
        let pointer = compare_pointer(&timer, channel);
        // SAFETY: CCR is a DMA-capable TIM register, and the caller selected
        // the request associated with this timer/channel.
        let ring = unsafe { dma.write_circular(buffer, pointer, request, node) };
        RingBufferedPwmChannel {
            timer,
            ring,
            channel,
            max_duty: max_duty as u32 + 1,
            advanced: false,
        }
    }

    /// Send a finite compare sequence and wait asynchronously for completion.
    pub async fn write_dma<W: TimerDmaWord>(
        &mut self,
        channel: Channel,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        values: &[W],
    ) -> Result<(), DmaError> {
        validate_width::<W>(self.timer_mut());
        let pointer = compare_pointer(self.timer_mut(), channel);
        // SAFETY: CCR and its selected channel request form a valid DMA sink.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_capture_dma(channel, true);
        let _guard = ChannelDmaGuard {
            timer: self.timer_mut(),
            channel,
        };
        transfer.await
    }

    /// Send a finite compare sequence and busy-wait for completion.
    pub fn write_dma_blocking<W: TimerDmaWord>(
        &mut self,
        channel: Channel,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        values: &[W],
    ) -> Result<(), DmaError> {
        validate_width::<W>(self.timer_mut());
        let pointer = compare_pointer(self.timer_mut(), channel);
        // SAFETY: CCR and its selected channel request form a valid DMA sink.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_capture_dma(channel, true);
        let _guard = ChannelDmaGuard {
            timer: self.timer_mut(),
            channel,
        };
        transfer.wait_blocking()
    }

    /// Send a finite TIM update-burst sequence through `DMAR`.
    pub async fn write_dma_burst<W: TimerDmaWord>(
        &mut self,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        base_register: u8,
        transfers_per_update: u8,
        values: &[W],
    ) -> Result<(), DmaError> {
        assert!(transfers_per_update > 0);
        assert_eq!(values.len() % transfers_per_update as usize, 0);
        validate_width::<W>(self.timer_mut());
        self.timer_mut()
            .configure_dma_burst(base_register, transfers_per_update);
        let pointer = self.timer_mut().regs().DMAR().as_ptr().cast();
        // SAFETY: DMAR is the TIM DMA-burst data port.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_update_dma(true);
        let _guard = UpdateDmaGuard(self.timer_mut());
        transfer.await
    }

    /// Stream repeating multi-register frames through TIM `DMAR`.
    pub fn into_ring_buffered_burst<'a, W: TimerDmaWord>(
        self,
        dma: &'a mut DmaChannel,
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
        buffer: &'a mut [W],
        base_register: u8,
        transfers_per_update: u8,
    ) -> RingBufferedPwmBurst<'a, W> {
        assert!(transfers_per_update > 0);
        assert_eq!(buffer.len() % transfers_per_update as usize, 0);
        let (mut timer, _) = self.into_parts();
        validate_width::<W>(&timer);
        timer.configure_dma_burst(base_register, transfers_per_update);
        timer.set_update_dma(false);
        let pointer = timer.regs().DMAR().as_ptr().cast();
        // SAFETY: DMAR is the update-burst sink for the selected TIM request.
        let ring = unsafe { dma.write_circular(buffer, pointer, request, node) };
        RingBufferedPwmBurst {
            timer,
            ring,
            transfers_per_update,
        }
    }
}

impl ComplementaryPwm {
    /// Stream compare values continuously into an advanced PWM channel.
    pub fn into_ring_buffered_channel<'a, W: TimerDmaWord>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
        buffer: &'a mut [W],
    ) -> RingBufferedPwmChannel<'a, W> {
        let (mut timer, max_duty) = self.into_parts();
        validate_width::<W>(&timer);
        timer.set_capture_dma(channel, false);
        let pointer = compare_pointer(&timer, channel);
        // SAFETY: CCR is a DMA-capable advanced TIM register.
        let ring = unsafe { dma.write_circular(buffer, pointer, request, node) };
        RingBufferedPwmChannel {
            timer,
            ring,
            channel,
            max_duty: max_duty as u32 + 1,
            advanced: true,
        }
    }

    /// Send a finite compare sequence to an advanced timer channel.
    pub async fn write_dma<W: TimerDmaWord>(
        &mut self,
        channel: Channel,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        values: &[W],
    ) -> Result<(), DmaError> {
        validate_width::<W>(self.timer_mut());
        let pointer = compare_pointer(self.timer_mut(), channel);
        // SAFETY: CCR is the DMA sink for this advanced TIM channel.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_capture_dma(channel, true);
        let _guard = ChannelDmaGuard {
            timer: self.timer_mut(),
            channel,
        };
        transfer.await
    }

    /// Busy-waiting form of [`ComplementaryPwm::write_dma`].
    pub fn write_dma_blocking<W: TimerDmaWord>(
        &mut self,
        channel: Channel,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        values: &[W],
    ) -> Result<(), DmaError> {
        validate_width::<W>(self.timer_mut());
        let pointer = compare_pointer(self.timer_mut(), channel);
        // SAFETY: CCR is the DMA sink for this advanced TIM channel.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_capture_dma(channel, true);
        let _guard = ChannelDmaGuard {
            timer: self.timer_mut(),
            channel,
        };
        transfer.wait_blocking()
    }

    /// Send a finite advanced-TIM update-burst sequence.
    pub async fn write_dma_burst<W: TimerDmaWord>(
        &mut self,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        base_register: u8,
        transfers_per_update: u8,
        values: &[W],
    ) -> Result<(), DmaError> {
        assert!(transfers_per_update > 0);
        assert_eq!(values.len() % transfers_per_update as usize, 0);
        validate_width::<W>(self.timer_mut());
        self.timer_mut()
            .configure_dma_burst(base_register, transfers_per_update);
        let pointer = self.timer_mut().regs().DMAR().as_ptr().cast();
        // SAFETY: DMAR is the advanced TIM update-burst port.
        let transfer = unsafe { dma.write_peripheral(values, pointer, request) };
        self.timer_mut().set_update_dma(true);
        let _guard = UpdateDmaGuard(self.timer_mut());
        transfer.await
    }
}

/// DMA-driven PWM channel.
pub struct RingBufferedPwmChannel<'a, W: TimerDmaWord> {
    timer: Timer,
    ring: WritableRingBuffer<'a, W>,
    channel: Channel,
    max_duty: u32,
    advanced: bool,
}

impl<W: TimerDmaWord> RingBufferedPwmChannel<'_, W> {
    pub fn write_immediate(&mut self, values: &[W]) -> Result<(usize, usize), RingError> {
        self.ring.write_immediate(values)
    }

    pub fn start(&mut self) {
        self.ring.start();
        self.timer.set_capture_dma(self.channel, true);
        self.timer.set_channel_enabled(self.channel, true);
        if self.advanced {
            self.timer.set_main_output(true);
        }
    }

    pub fn write(&mut self, values: &[W]) -> Result<(usize, usize), RingError> {
        self.ring.write(values)
    }

    pub async fn write_exact(&mut self, values: &[W]) -> Result<usize, RingError> {
        self.ring.write_exact(values).await
    }

    pub fn set_polarity(&mut self, polarity: Polarity) {
        self.timer.set_channel_polarity(self.channel, polarity);
    }

    pub const fn max_duty_cycle(&self) -> u32 {
        self.max_duty
    }

    pub const fn capacity(&self) -> usize {
        self.ring.capacity()
    }

    pub fn pause(&mut self) {
        self.ring.pause();
    }

    pub fn resume(&mut self) {
        self.ring.resume();
    }

    pub async fn stop(&mut self) {
        self.ring.stop().await;
        self.timer.set_channel_enabled(self.channel, false);
        self.timer.set_capture_dma(self.channel, false);
    }
}

impl<W: TimerDmaWord> Drop for RingBufferedPwmChannel<'_, W> {
    fn drop(&mut self) {
        self.timer.set_channel_enabled(self.channel, false);
        self.timer.set_capture_dma(self.channel, false);
        if self.advanced {
            self.timer.set_main_output(false);
        }
    }
}

/// Circular TIM update-burst stream for synchronized register frames.
pub struct RingBufferedPwmBurst<'a, W: TimerDmaWord> {
    timer: Timer,
    ring: WritableRingBuffer<'a, W>,
    transfers_per_update: u8,
}

impl<W: TimerDmaWord> RingBufferedPwmBurst<'_, W> {
    pub fn write_immediate(&mut self, frames: &[W]) -> Result<(usize, usize), RingError> {
        assert_eq!(frames.len() % self.transfers_per_update as usize, 0);
        self.ring.write_immediate(frames)
    }

    pub fn start(&mut self) {
        self.ring.start();
        self.timer.set_update_dma(true);
    }

    pub fn write(&mut self, frames: &[W]) -> Result<(usize, usize), RingError> {
        assert_eq!(frames.len() % self.transfers_per_update as usize, 0);
        self.ring.write(frames)
    }

    pub async fn write_exact(&mut self, frames: &[W]) -> Result<usize, RingError> {
        assert_eq!(frames.len() % self.transfers_per_update as usize, 0);
        self.ring.write_exact(frames).await
    }

    pub const fn transfers_per_update(&self) -> u8 {
        self.transfers_per_update
    }

    pub async fn stop(&mut self) {
        self.ring.stop().await;
        self.timer.set_update_dma(false);
    }
}

impl<W: TimerDmaWord> Drop for RingBufferedPwmBurst<'_, W> {
    fn drop(&mut self) {
        self.timer.set_update_dma(false);
    }
}

impl InputCapture {
    /// Capture one finite sequence of edge timestamps using DMA.
    pub async fn read_dma<W: TimerDmaWord>(
        &mut self,
        channel: Channel,
        dma: &mut DmaChannel,
        request: impl Into<DmaRequest>,
        values: &mut [W],
    ) -> Result<(), DmaError> {
        validate_width::<W>(self.timer_mut());
        self.timer_mut().set_capture_dma(channel, true);
        let pointer = compare_pointer(self.timer_mut(), channel);
        let _guard = ChannelDmaGuard {
            timer: self.timer_mut(),
            channel,
        };
        // SAFETY: CCR is the source for this TIM channel's capture request.
        unsafe { dma.read_peripheral(pointer, values, request) }.await
    }

    /// Continuously capture timestamps from one channel.
    pub fn into_ring_buffered_channel<'a, W: TimerDmaWord>(
        self,
        channel: Channel,
        dma: &'a mut DmaChannel,
        request: impl Into<DmaRequest>,
        node: &'a mut DmaNode,
        buffer: &'a mut [W],
    ) -> RingBufferedCapture<'a, W> {
        let mut timer = self.into_timer();
        validate_width::<W>(&timer);
        timer.set_capture_dma(channel, false);
        let pointer = compare_pointer(&timer, channel);
        // SAFETY: CCR is the capture source for the selected TIM request.
        let ring = unsafe { dma.read_circular(pointer, buffer, request, node) };
        RingBufferedCapture {
            timer,
            ring,
            channel,
        }
    }
}

/// DMA-backed input-capture stream.
pub struct RingBufferedCapture<'a, W: TimerDmaWord> {
    timer: Timer,
    ring: ReadableRingBuffer<'a, W>,
    channel: Channel,
}

impl<W: TimerDmaWord> RingBufferedCapture<'_, W> {
    pub fn start(&mut self) {
        self.ring.start();
        self.timer.set_capture_dma(self.channel, true);
        self.timer.set_channel_enabled(self.channel, true);
        self.timer.start();
    }

    pub fn read(&mut self, values: &mut [W]) -> Result<(usize, usize), RingError> {
        self.ring.read(values)
    }

    pub async fn read_exact(&mut self, values: &mut [W]) -> Result<usize, RingError> {
        self.ring.read_exact(values).await
    }

    pub fn read_latest(&mut self, values: &mut [W]) -> Result<usize, RingError> {
        self.ring.read_latest(values)
    }

    pub const fn capacity(&self) -> usize {
        self.ring.capacity()
    }

    pub async fn stop(&mut self) {
        self.ring.stop().await;
        self.timer.stop();
        self.timer.set_capture_dma(self.channel, false);
    }
}

impl<W: TimerDmaWord> Drop for RingBufferedCapture<'_, W> {
    fn drop(&mut self) {
        self.timer.stop();
        self.timer.set_capture_dma(self.channel, false);
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, Timer, TimerWidth, compare_pointer, validate_width};
    use crate::pac::peripherals::timer::TIM_GP;

    #[test]
    fn compare_pointer_selects_ccr_channel() {
        let mut registers = [0u32; 256];
        // SAFETY: the fake MMIO array remains alive and exclusively owned.
        let timer = unsafe {
            Timer::over(
                TIM_GP::from_ptr(registers.as_mut_ptr().cast()),
                200_000_000,
                TimerWidth::Bits16,
            )
        };
        let offset =
            compare_pointer::<u16>(&timer, Channel::Ch4) as usize - registers.as_mut_ptr() as usize;
        assert_eq!(offset, 0x40);
        validate_width::<u16>(&timer);
    }

    #[test]
    #[should_panic]
    fn rejects_dma_word_different_from_timer_width() {
        let mut registers = [0u32; 256];
        // SAFETY: the fake MMIO array remains alive and exclusively owned.
        let timer = unsafe {
            Timer::over(
                TIM_GP::from_ptr(registers.as_mut_ptr().cast()),
                200_000_000,
                TimerWidth::Bits16,
            )
        };
        validate_width::<u32>(&timer);
    }
}
