//! Polling ADC driver for the i.MX 9 SAR ADC.
//!
//! Clock-root and pin-mux configuration are intentionally left to the caller.

use pac::peripherals::adc::{ADC, ADC_NCMR0, ADC_NCMR1};

use crate::pac;

#[cfg(feature = "dma")]
use crate::dma::{DmaChannel, ReadableRingBuffer, RingError, Transfer, request};
#[cfg(feature = "dma")]
use core::future::Future;
#[cfg(feature = "dma")]
use core::pin::Pin;
#[cfg(feature = "dma")]
use core::task::{Context, Poll};

/// ADC configuration shared by i.MX93 and i.MX95.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Config {
    /// Use the bus clock directly instead of bus clock divided by two.
    pub full_speed_clock: bool,
    /// Sampling-time selector (`0..=3`).
    pub sample_time: u8,
    /// Number-of-samples selector (`0..=3`).
    pub sample_count: u8,
    /// Enable hardware averaging.
    pub averaging: bool,
}

/// ADC operation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Only precision channels 0-7 and internal channels 32-39 are available.
    InvalidChannel,
    /// A conversion is already active.
    Busy,
    /// Hardware calibration failed.
    CalibrationFailed,
}

/// Normal-conversion trigger used by DMA sampling.
#[cfg(feature = "dma")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTrigger {
    /// Start immediately and scan continuously.
    SoftwareContinuous,
    /// Perform a conversion on each rising external trigger.
    ExternalRising,
    /// Perform a conversion on each falling external trigger.
    ExternalFalling,
}

/// Polling ADC driver.
pub struct Adc {
    adc: ADC,
    active_channel: Option<u8>,
}

impl Adc {
    /// Bind and power up an ADC instance.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid, exclusively owned i.MX 9 ADC register
    /// block whose clock root has been enabled.
    pub unsafe fn new(base: *mut (), config: Config) -> Self {
        assert!(config.sample_time <= 3);
        assert!(config.sample_count <= 3);
        // SAFETY: upheld by the caller.
        let adc = unsafe { ADC::from_ptr(base) };
        adc.MCR().modify(|w| {
            w.set_PWDN(false);
            w.set_ADCLKSE(config.full_speed_clock);
            w.set_TSAMP(config.sample_time);
            w.set_NRSMPL(config.sample_count);
            w.set_AVGEN(config.averaging);
            w.set_MODE(false);
        });
        Self {
            adc,
            active_channel: None,
        }
    }

    /// Run the ADC's hardware calibration and wait for completion.
    pub fn calibrate(&mut self) -> Result<(), Error> {
        self.adc.MCR().modify(|w| w.set_CALSTART(true));
        while self.adc.MSR().read().CALBUSY() {}
        if self.adc.MSR().read().CALFAIL() {
            Err(Error::CalibrationFailed)
        } else {
            Ok(())
        }
    }

    /// Start a single software-triggered conversion.
    pub fn start_conversion(&mut self, channel: u8) -> Result<(), Error> {
        if self.active_channel.is_some() || self.adc.MSR().read().NSTART() {
            return Err(Error::Busy);
        }
        match channel {
            0..=7 => {
                self.adc.NCMR0().write_value(ADC_NCMR0(1u32 << channel));
                self.adc.NCMR1().write_value(ADC_NCMR1(0));
                self.adc.CEOCFR0().write_value(1u32 << channel);
            }
            32..=39 => {
                self.adc.NCMR0().write_value(ADC_NCMR0(0));
                self.adc
                    .NCMR1()
                    .write_value(ADC_NCMR1(1u32 << (channel - 32)));
                self.adc.CEOCFR1().write_value(1u32 << (channel - 32));
            }
            _ => return Err(Error::InvalidChannel),
        }
        self.active_channel = Some(channel);
        self.adc.MCR().modify(|w| w.set_NSTART(true));
        Ok(())
    }

    /// Return the completed sample, or `None` while conversion is in progress.
    pub fn try_read(&mut self) -> Option<u16> {
        let channel = self.active_channel?;
        let value = if channel < 8 {
            let data = self.adc.PCDR(channel as usize).read();
            data.VALID().then(|| data.CDATA())
        } else {
            let data = self.adc.ICDR((channel - 32) as usize).read();
            data.VALID().then(|| data.CDATA())
        };
        let value = value?;
        self.active_channel = None;
        Some(value)
    }

    /// Start a conversion and busy-wait for its result.
    pub fn read_blocking(&mut self, channel: u8) -> Result<u16, Error> {
        self.start_conversion(channel)?;
        loop {
            if let Some(value) = self.try_read() {
                return Ok(value);
            }
        }
    }

    /// Start a finite DMA sample stream for one ADC channel.
    ///
    /// The SAR ADC stores each channel in a separate `PCDR`/`ICDR`; therefore
    /// this operation repeatedly samples one channel. A heterogeneous scan
    /// requires an eDMA scatter/gather sequence with one source register per
    /// channel.
    #[cfg(feature = "dma")]
    pub fn read_dma<'a>(
        &'a mut self,
        dma: &'a mut DmaChannel,
        channel: u8,
        samples: &'a mut [u16],
        trigger: DmaTrigger,
    ) -> Result<AdcDmaTransfer<'a>, Error> {
        self.prepare_dma_channel(channel, trigger)?;
        let register = self.channel_data_ptr(channel)?;
        // SAFETY: the selected PCDR/ICDR is the ADC1 request's data register.
        let transfer =
            unsafe { dma.read_peripheral_with_request(register, samples, request::ADC1) };
        self.enable_dma_channel(channel);
        self.start_dma_conversion(trigger);
        Ok(AdcDmaTransfer {
            adc: self,
            channel,
            transfer: Some(transfer),
        })
    }

    /// Construct a circular DMA sampler for one ADC channel.
    #[cfg(feature = "dma")]
    pub fn into_ring_buffered<'a>(
        &'a mut self,
        dma: &'a mut DmaChannel,
        channel: u8,
        samples: &'a mut [u16],
        trigger: DmaTrigger,
    ) -> Result<RingBufferedAdc<'a>, Error> {
        self.prepare_dma_channel(channel, trigger)?;
        let register = self.channel_data_ptr(channel)?;
        // SAFETY: the selected PCDR/ICDR is the ADC1 request's data register.
        let ring = unsafe { dma.read_circular(register, samples, request::ADC1) };
        Ok(RingBufferedAdc {
            adc: self,
            channel,
            trigger,
            ring: Some(ring),
            started: false,
        })
    }

    #[cfg(feature = "dma")]
    fn prepare_dma_channel(&mut self, channel: u8, trigger: DmaTrigger) -> Result<(), Error> {
        if self.active_channel.is_some() || self.adc.MSR().read().NSTART() {
            return Err(Error::Busy);
        }
        let (normal0, normal1) = channel_masks(channel)?;
        self.adc.NCMR0().write_value(ADC_NCMR0(normal0));
        self.adc.NCMR1().write_value(ADC_NCMR1(normal1));
        self.adc.CEOCFR0().write_value(normal0);
        self.adc.CEOCFR1().write_value(normal1);
        self.adc.DMAR0().write(|w| w.0 = 0);
        self.adc.DMAR1().write(|w| w.0 = 0);
        self.adc.DMAE().modify(|w| {
            w.set_DMAEN(false);
            // Clear the request by reading its data register. This matches a
            // circular stream where the eDMA request remains enabled.
            w.set_DCLR(true);
        });
        self.adc.MCR().modify(|w| {
            w.set_OWREN(true);
            w.set_TRGEN(!matches!(trigger, DmaTrigger::SoftwareContinuous));
            w.set_EDGE(matches!(trigger, DmaTrigger::ExternalRising));
            w.set_MODE(matches!(trigger, DmaTrigger::SoftwareContinuous));
        });
        Ok(())
    }

    #[cfg(feature = "dma")]
    fn channel_data_ptr(&self, channel: u8) -> Result<*const u16, Error> {
        match channel {
            0..=7 => Ok(self.adc.PCDR(channel as usize).as_ptr().cast::<u16>()),
            32..=39 => Ok(self
                .adc
                .ICDR((channel - 32) as usize)
                .as_ptr()
                .cast::<u16>()),
            _ => Err(Error::InvalidChannel),
        }
    }

    #[cfg(feature = "dma")]
    fn enable_dma_channel(&mut self, channel: u8) {
        if channel < 8 {
            self.adc.DMAR0().modify(|w| w.0 |= 1u32 << channel);
        } else {
            self.adc.DMAR1().modify(|w| w.0 |= 1u32 << (channel - 32));
        }
        self.adc.DMAE().modify(|w| w.set_DMAEN(true));
    }

    #[cfg(feature = "dma")]
    fn start_dma_conversion(&mut self, trigger: DmaTrigger) {
        if matches!(trigger, DmaTrigger::SoftwareContinuous) {
            self.adc.MCR().modify(|w| w.set_NSTART(true));
        }
    }

    #[cfg(feature = "dma")]
    fn stop_dma(&mut self) {
        self.adc.DMAE().modify(|w| w.set_DMAEN(false));
        self.adc.DMAR0().write(|w| w.0 = 0);
        self.adc.DMAR1().write(|w| w.0 = 0);
        self.adc.MCR().modify(|w| {
            w.set_NSTART(false);
            w.set_TRGEN(false);
            w.set_MODE(false);
        });
        self.active_channel = None;
    }
}

#[cfg(feature = "dma")]
fn channel_masks(channel: u8) -> Result<(u32, u32), Error> {
    match channel {
        0..=7 => Ok((1u32 << channel, 0)),
        32..=39 => Ok((0, 1u32 << (channel - 32))),
        _ => Err(Error::InvalidChannel),
    }
}

/// Finite ADC DMA sampling operation.
#[cfg(feature = "dma")]
pub struct AdcDmaTransfer<'a> {
    adc: &'a mut Adc,
    channel: u8,
    transfer: Option<Transfer<'a>>,
}

#[cfg(feature = "dma")]
impl AdcDmaTransfer<'_> {
    /// Busy-wait for every sample.
    pub fn wait_blocking(mut self) -> Result<(), crate::dma::Error> {
        let result = self
            .transfer
            .take()
            .expect("transfer already consumed")
            .wait_blocking();
        self.adc.stop_dma();
        result
    }

    /// Channel being sampled.
    pub const fn channel(&self) -> u8 {
        self.channel
    }
}

#[cfg(feature = "dma")]
impl Future for AdcDmaTransfer<'_> {
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
            this.adc.stop_dma();
        }
        result
    }
}

#[cfg(feature = "dma")]
impl Drop for AdcDmaTransfer<'_> {
    fn drop(&mut self) {
        drop(self.transfer.take());
        self.adc.stop_dma();
    }
}

/// Circular one-channel ADC sample consumer.
#[cfg(feature = "dma")]
pub struct RingBufferedAdc<'a> {
    adc: &'a mut Adc,
    channel: u8,
    trigger: DmaTrigger,
    ring: Option<ReadableRingBuffer<'a, u16>>,
    started: bool,
}

#[cfg(feature = "dma")]
impl<'a> RingBufferedAdc<'a> {
    /// Enable the ADC request and begin sampling.
    pub fn start(&mut self) {
        if !self.started {
            self.adc.enable_dma_channel(self.channel);
            self.ring_mut().start();
            self.adc.start_dma_conversion(self.trigger);
            self.started = true;
        }
    }

    /// Read immediately available samples.
    pub fn read(&mut self, output: &mut [u16]) -> Result<(usize, usize), RingError> {
        self.ring_mut().read(output)
    }

    /// Fill `output` asynchronously.
    pub async fn read_exact(&mut self, output: &mut [u16]) -> Result<usize, RingError> {
        self.ring_mut().read_exact(output).await
    }

    /// Return the newest samples.
    pub fn read_latest(&mut self, output: &mut [u16]) -> Result<usize, RingError> {
        self.ring_mut().read_latest(output)
    }

    /// Number of unread samples.
    pub fn len(&self) -> Result<usize, RingError> {
        self.ring.as_ref().expect("ring has been released").len()
    }

    /// Whether no samples are available.
    pub fn is_empty(&self) -> Result<bool, RingError> {
        self.ring
            .as_ref()
            .expect("ring has been released")
            .is_empty()
    }

    /// Pause eDMA requests.
    pub fn pause(&mut self) {
        self.ring_mut().pause();
    }

    /// Resume eDMA requests.
    pub fn resume(&mut self) {
        self.ring_mut().resume();
    }

    /// Stop eDMA before disabling the ADC request.
    pub async fn stop(&mut self) {
        self.ring_mut().stop().await;
        self.adc.stop_dma();
        self.started = false;
    }

    fn ring_mut(&mut self) -> &mut ReadableRingBuffer<'a, u16> {
        self.ring.as_mut().expect("ring has been released")
    }
}

#[cfg(feature = "dma")]
impl Drop for RingBufferedAdc<'_> {
    fn drop(&mut self) {
        drop(self.ring.take());
        self.adc.stop_dma();
    }
}

#[cfg(all(test, feature = "dma"))]
mod tests {
    use super::*;
    use crate::dma::DmaState;

    #[cfg(feature = "mimx93")]
    const ADC_DMA_CHANNEL: usize = 30;
    #[cfg(feature = "mimx95")]
    const ADC_DMA_CHANNEL: usize = 7;

    #[test]
    fn finite_dma_configures_and_cleans_adc_requests() {
        static STATE: DmaState = DmaState::new();
        let mut adc_registers = [0u32; 0x100];
        let words = ((ADC_DMA_CHANNEL + 2) * 0x1_0000) / 4;
        let mut dma_registers = std::vec![0u32; words];

        // SAFETY: both fake register blocks are exclusively owned by the test.
        let mut adc = unsafe { Adc::new(adc_registers.as_mut_ptr().cast(), Config::default()) };
        // SAFETY: fake eDMA management and channel pages fit the allocation.
        let mut dma =
            unsafe { DmaChannel::new(dma_registers.as_mut_ptr().cast(), ADC_DMA_CHANNEL, &STATE) };
        let mut samples = [0u16; 4];

        {
            let transfer = adc
                .read_dma(&mut dma, 3, &mut samples, DmaTrigger::SoftwareContinuous)
                .unwrap();
            assert_eq!(adc_registers[0x44 / 4], 1 << 3);
            assert_eq!(adc_registers[0x40 / 4] & 0x3, 0x3);
            assert_ne!(adc_registers[0] & (1 << 24), 0);
            drop(transfer);
        }

        assert_eq!(adc_registers[0x44 / 4], 0);
        assert_eq!(adc_registers[0x40 / 4] & 0x1, 0);
        assert_eq!(adc_registers[0] & (1 << 24), 0);
    }
}
