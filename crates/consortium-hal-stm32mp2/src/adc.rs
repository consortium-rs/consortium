//! Polling and DMA-streaming ADC driver for STM32MP2 ADC instances.
//!
//! RCC clock selection and GPIO analog-mode setup remain the caller's
//! responsibility.
//!
//! A useful DMA smoke test configures a TIM trigger output as the ADC regular
//! external trigger, captures a slow ramp into a circular buffer, and checks
//! that every scan contains the configured channel count without overruns.

use pac::peripherals::adc::{ADC, ADC_CFGR1, ADC_PCSEL, ADC_SMPR1, ADC_SMPR2};

use crate::pac;

/// Conversion resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Resolution {
    /// 12-bit conversion.
    #[default]
    Bits12,
    /// 10-bit conversion.
    Bits10,
    /// 8-bit conversion.
    Bits8,
    /// 6-bit conversion.
    Bits6,
}

impl Resolution {
    const fn bits(self) -> u8 {
        match self {
            Self::Bits12 => 0,
            Self::Bits10 => 1,
            Self::Bits8 => 2,
            Self::Bits6 => 3,
        }
    }
}

/// ADC setup for single regular conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Output resolution.
    pub resolution: Resolution,
    /// Sampling-time selector applied to every channel (`0..=7`).
    pub sample_time: u8,
    /// Calibrate the ADC before enabling it.
    pub calibrate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            resolution: Resolution::Bits12,
            sample_time: 7,
            calibrate: true,
        }
    }
}

/// ADC operation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The STM32MP2 regular sequencer accepts channels 0-19.
    InvalidChannel,
    /// A regular conversion is already active.
    Busy,
    /// A regular sequence must contain between 1 and 16 valid channels.
    InvalidSequence,
    /// A DMA buffer does not contain a whole number of ADC scans.
    InvalidBufferLength,
}

/// Polling ADC driver.
pub struct Adc {
    adc: ADC,
    active: bool,
}

impl Adc {
    /// Bind, optionally calibrate, and enable an ADC instance.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid, exclusively owned STM32MP2 ADC register
    /// block. Its RCC kernel clock must already be enabled.
    pub unsafe fn new(base: *mut (), config: Config) -> Self {
        assert!(config.sample_time <= 7);
        // SAFETY: upheld by the caller.
        let adc = unsafe { ADC::from_ptr(base) };

        adc.CR().modify(|w| w.set_DEEPPWD(false));
        if config.calibrate {
            adc.CR().modify(|w| {
                w.set_ADCALDIF(false);
                w.set_ADCAL(true);
            });
            while adc.CR().read().ADCAL() {}
        }

        adc.CFGR1()
            .write_value(ADC_CFGR1((config.resolution.bits() as u32) << 2));
        adc.SMPR1()
            .write_value(ADC_SMPR1(repeat_sample_time(config.sample_time, 10)));
        adc.SMPR2()
            .write_value(ADC_SMPR2(repeat_sample_time(config.sample_time, 10)));
        adc.CR().modify(|w| w.set_ADEN(true));
        while !adc.ISR().read().ADRDY() {}
        // ADRDY is write-one-to-clear.
        adc.ISR().write(|w| w.set_ADRDY(true));

        Self { adc, active: false }
    }

    /// Start one software-triggered regular conversion.
    pub fn start_conversion(&mut self, channel: u8) -> Result<(), Error> {
        if channel > 19 {
            return Err(Error::InvalidChannel);
        }
        if self.active || self.adc.CR().read().ADSTART() {
            return Err(Error::Busy);
        }

        self.adc.PCSEL().write_value(ADC_PCSEL(1u32 << channel));
        self.adc.SQR1().write(|w| {
            w.set_L(0);
            w.set_SQ1(channel);
        });
        // Clear stale completion/overrun flags before starting.
        self.adc.ISR().write(|w| {
            w.set_EOC(true);
            w.set_EOS(true);
            w.set_OVR(true);
        });
        self.active = true;
        self.adc.CR().modify(|w| w.set_ADSTART(true));
        Ok(())
    }

    /// Return the completed sample, or `None` while conversion is in progress.
    pub fn try_read(&mut self) -> Option<u16> {
        if !self.active || !self.adc.ISR().read().EOC() {
            return None;
        }
        self.active = false;
        Some(self.adc.DR().read().RDATA() as u16)
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

    /// Configure the regular conversion sequence.
    pub fn configure_sequence(&mut self, channels: &[u8]) -> Result<(), Error> {
        if channels.is_empty() || channels.len() > 16 || channels.iter().any(|&ch| ch > 19) {
            return Err(Error::InvalidSequence);
        }
        let mut selected = 0u32;
        for &channel in channels {
            selected |= 1 << channel;
        }
        self.adc.PCSEL().write_value(ADC_PCSEL(selected));
        self.adc.SQR1().write(|w| {
            w.set_L((channels.len() - 1) as u8);
            if let Some(&value) = channels.first() {
                w.set_SQ1(value);
            }
            if let Some(&value) = channels.get(1) {
                w.set_SQ2(value);
            }
            if let Some(&value) = channels.get(2) {
                w.set_SQ3(value);
            }
            if let Some(&value) = channels.get(3) {
                w.set_SQ4(value);
            }
        });
        self.adc.SQR2().write(|w| {
            if let Some(&value) = channels.get(4) {
                w.set_SQ5(value);
            }
            if let Some(&value) = channels.get(5) {
                w.set_SQ6(value);
            }
            if let Some(&value) = channels.get(6) {
                w.set_SQ7(value);
            }
            if let Some(&value) = channels.get(7) {
                w.set_SQ8(value);
            }
            if let Some(&value) = channels.get(8) {
                w.set_SQ9(value);
            }
        });
        self.adc.SQR3().write(|w| {
            if let Some(&value) = channels.get(9) {
                w.set_SQ10(value);
            }
            if let Some(&value) = channels.get(10) {
                w.set_SQ11(value);
            }
            if let Some(&value) = channels.get(11) {
                w.set_SQ12(value);
            }
            if let Some(&value) = channels.get(12) {
                w.set_SQ13(value);
            }
            if let Some(&value) = channels.get(13) {
                w.set_SQ14(value);
            }
        });
        self.adc.SQR4().write(|w| {
            if let Some(&value) = channels.get(14) {
                w.set_SQ15(value);
            }
            if let Some(&value) = channels.get(15) {
                w.set_SQ16(value);
            }
        });
        Ok(())
    }
}

#[cfg(feature = "dma")]
mod dma_support {
    use crate::dma::{
        DmaChannel, DmaNode, DmaRequest, Error as TransferError, ReadableRingBuffer, RingError,
    };

    use super::{Adc, Error};

    /// ADC regular trigger configuration.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Trigger {
        /// Run scans continuously after a software start.
        SoftwareContinuous,
        /// Start a scan on an ADC external trigger selector.
        External {
            /// Raw `EXTSEL` value from the reference manual.
            source: u8,
            /// Active edge.
            edge: TriggerEdge,
        },
    }

    /// External trigger edge.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u8)]
    pub enum TriggerEdge {
        Rising = 1,
        Falling = 2,
        Both = 3,
    }

    /// Finite ADC DMA error.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DmaError {
        Adc(Error),
        Transfer(TransferError),
    }

    impl From<Error> for DmaError {
        fn from(value: Error) -> Self {
            Self::Adc(value)
        }
    }

    impl From<TransferError> for DmaError {
        fn from(value: TransferError) -> Self {
            Self::Transfer(value)
        }
    }

    struct AdcDmaGuard<'a> {
        adc: &'a mut Adc,
    }

    impl Drop for AdcDmaGuard<'_> {
        fn drop(&mut self) {
            self.adc.stop_dma();
        }
    }

    impl Adc {
        fn prepare_dma(
            &mut self,
            channels: &[u8],
            samples: usize,
            circular: bool,
            trigger: Trigger,
        ) -> Result<(), Error> {
            if self.active || self.adc.CR().read().ADSTART() {
                return Err(Error::Busy);
            }
            self.configure_sequence(channels)?;
            if samples == 0 || !samples.is_multiple_of(channels.len()) {
                return Err(Error::InvalidBufferLength);
            }
            self.adc.CFGR1().modify(|w| {
                w.set_DMNGT(if circular { 3 } else { 1 });
                w.set_OVRMOD(circular);
                match trigger {
                    Trigger::SoftwareContinuous => {
                        w.set_CONT(circular || samples > channels.len());
                        w.set_EXTEN(0);
                    }
                    Trigger::External { source, edge } => {
                        assert!(source < 32);
                        w.set_CONT(false);
                        w.set_EXTSEL(source);
                        w.set_EXTEN(edge as u8);
                    }
                }
            });
            self.adc.ISR().write(|w| {
                w.set_EOC(true);
                w.set_EOS(true);
                w.set_OVR(true);
            });
            self.active = true;
            Ok(())
        }

        fn stop_dma(&mut self) {
            if self.adc.CR().read().ADSTART() {
                self.adc.CR().modify(|w| w.set_ADSTP(true));
            }
            self.adc.CFGR1().modify(|w| {
                w.set_DMNGT(0);
                w.set_CONT(false);
                w.set_EXTEN(0);
            });
            self.active = false;
        }

        fn data_pointer(&self) -> *const u16 {
            self.adc.DR().as_ptr().cast()
        }

        /// Read one or more complete regular scans with HPDMA.
        pub async fn read_dma(
            &mut self,
            dma: &mut DmaChannel,
            request: impl Into<DmaRequest>,
            channels: &[u8],
            samples: &mut [u16],
        ) -> Result<(), DmaError> {
            self.prepare_dma(channels, samples.len(), false, Trigger::SoftwareContinuous)?;
            let pointer = self.data_pointer();
            let guard = AdcDmaGuard { adc: self };
            // SAFETY: DR is the regular ADC data register associated with the
            // selected ADC request.
            let transfer = unsafe { dma.read_peripheral(pointer, samples, request) };
            guard.adc.adc.CR().modify(|w| w.set_ADSTART(true));
            let result = transfer.await;
            guard.adc.active = false;
            result.map_err(Into::into)
        }

        /// Blocking form of [`Adc::read_dma`].
        pub fn read_dma_blocking(
            &mut self,
            dma: &mut DmaChannel,
            request: impl Into<DmaRequest>,
            channels: &[u8],
            samples: &mut [u16],
        ) -> Result<(), DmaError> {
            self.prepare_dma(channels, samples.len(), false, Trigger::SoftwareContinuous)?;
            let pointer = self.data_pointer();
            let guard = AdcDmaGuard { adc: self };
            // SAFETY: DR is the regular ADC DMA source.
            let transfer = unsafe { dma.read_peripheral(pointer, samples, request) };
            guard.adc.adc.CR().modify(|w| w.set_ADSTART(true));
            let result = transfer.wait_blocking();
            guard.adc.active = false;
            result.map_err(Into::into)
        }

        /// Consume this ADC into a continuous DMA sampling stream.
        pub fn into_ring_buffered<'a>(
            mut self,
            dma: &'a mut DmaChannel,
            request: impl Into<DmaRequest>,
            node: &'a mut DmaNode,
            buffer: &'a mut [u16],
            channels: &[u8],
            trigger: Trigger,
        ) -> Result<RingBufferedAdc<'a>, Error> {
            if buffer.len() < 2
                || !buffer.len().is_multiple_of(2)
                || buffer.len() > crate::dma::MAX_TRANSFER / core::mem::size_of::<u16>()
            {
                return Err(Error::InvalidBufferLength);
            }
            self.prepare_dma(channels, buffer.len(), true, trigger)?;
            let pointer = self.data_pointer();
            // SAFETY: DR and the selected ADC request form a valid DMA source.
            let ring = unsafe { dma.read_circular(pointer, buffer, request, node) };
            Ok(RingBufferedAdc {
                adc: self,
                ring,
                channels: channels.len() as u8,
            })
        }
    }

    /// Continuous ADC scan ring.
    pub struct RingBufferedAdc<'a> {
        adc: Adc,
        ring: ReadableRingBuffer<'a, u16>,
        channels: u8,
    }

    impl RingBufferedAdc<'_> {
        pub fn start(&mut self) {
            self.ring.start();
            self.adc.adc.CR().modify(|w| w.set_ADSTART(true));
        }

        pub fn read(&mut self, samples: &mut [u16]) -> Result<(usize, usize), RingError> {
            self.ring.read(samples)
        }

        pub async fn read_exact(&mut self, samples: &mut [u16]) -> Result<usize, RingError> {
            self.ring.read_exact(samples).await
        }

        pub fn read_latest(&mut self, samples: &mut [u16]) -> Result<usize, RingError> {
            self.ring.read_latest(samples)
        }

        pub const fn channels_per_scan(&self) -> u8 {
            self.channels
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
            self.adc.stop_dma();
        }
    }

    impl Drop for RingBufferedAdc<'_> {
        fn drop(&mut self) {
            self.adc.stop_dma();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Adc, Trigger, TriggerEdge};
        use crate::pac::peripherals::adc::ADC;

        fn adc(registers: &mut [u32; 128]) -> Adc {
            Adc {
                // SAFETY: `registers` is a 512-byte, 4-byte-aligned array standing in
                // for the ADC register block; it covers every offset the driver
                // touches, and the `&mut` borrow keeps it alive and unaliased for the
                // returned driver's lifetime.
                adc: unsafe { ADC::from_ptr(registers.as_mut_ptr().cast()) },
                active: false,
            }
        }

        #[test]
        fn circular_external_trigger_configures_data_management() {
            let mut registers = [0u32; 128];
            let mut adc = adc(&mut registers);
            adc.prepare_dma(
                &[2, 3],
                8,
                true,
                Trigger::External {
                    source: 7,
                    edge: TriggerEdge::Falling,
                },
            )
            .unwrap();
            let cfgr = registers[0x0c / 4];
            assert_eq!(cfgr & 0x3, 3);
            assert_eq!((cfgr >> 5) & 0x1f, 7);
            assert_eq!((cfgr >> 10) & 0x3, 2);
            assert_ne!(cfgr & (1 << 12), 0);
            assert_eq!(cfgr & (1 << 13), 0);
        }

        #[test]
        fn rejects_partial_scan_buffer() {
            let mut registers = [0u32; 128];
            let mut adc = adc(&mut registers);
            assert!(
                adc.prepare_dma(&[1, 2, 3], 8, true, Trigger::SoftwareContinuous)
                    .is_err()
            );
        }
    }
}

#[cfg(feature = "dma")]
pub use dma_support::{DmaError, RingBufferedAdc, Trigger, TriggerEdge};

const fn repeat_sample_time(sample_time: u8, channels: u8) -> u32 {
    let mut value = 0u32;
    let mut channel = 0u8;
    while channel < channels {
        value |= (sample_time as u32) << (channel * 3);
        channel += 1;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::{Adc, repeat_sample_time};
    use crate::pac::peripherals::adc::ADC;

    #[test]
    fn sample_time_is_repeated_for_all_channels() {
        assert_eq!(repeat_sample_time(0b101, 3), 0b101_101_101);
        assert_eq!(repeat_sample_time(7, 10), 0x3fff_ffff);
    }

    #[test]
    fn sequence_spans_all_four_registers() {
        let mut registers = [0u32; 128];
        // SAFETY: `registers` is a 512-byte, 4-byte-aligned array standing in for the
        // ADC register block; it covers every offset `configure_sequence` writes, and
        // it outlives `driver`, which is the only holder of the pointer.
        let adc = unsafe { ADC::from_ptr(registers.as_mut_ptr().cast()) };
        let mut driver = Adc { adc, active: false };
        let channels = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        driver.configure_sequence(&channels).unwrap();
        assert_eq!(registers[0x30 / 4] & 0xf, 15);
        assert_eq!((registers[0x30 / 4] >> 6) & 0x1f, 0);
        assert_eq!((registers[0x34 / 4] >> 24) & 0x1f, 8);
        assert_eq!((registers[0x38 / 4] >> 24) & 0x1f, 13);
        assert_eq!(registers[0x3c / 4] & 0x1f, 14);
        assert_eq!((registers[0x3c / 4] >> 6) & 0x1f, 15);
    }
}
