//! i.MX 9 TPM timers and their operating modes.
//!
//! A [`Timer`] owns one TPM register block. Consume it with
//! [`Timer::into_pwm`] to select PWM mode, mirroring Embassy's timer-driver
//! ownership model.

use pac::peripherals::tpm::TPM;

use crate::pac;

pub mod pwm;
#[cfg(feature = "dma")]
pub mod ringbuffered;

pub use pwm::{Pwm, PwmChannel};
#[cfg(feature = "dma")]
pub use ringbuffered::{
    CaptureEdge, CaptureTransfer, InputCapture, PwmTransfer, RingBufferedCapture, RingBufferedPwm,
};

/// A TPM channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Channel 0.
    Ch0,
    /// Channel 1.
    Ch1,
    /// Channel 2.
    Ch2,
    /// Channel 3.
    Ch3,
}

impl Channel {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Ch0 => 0,
            Self::Ch1 => 1,
            Self::Ch2 => 2,
            Self::Ch3 => 3,
        }
    }
}

/// TPM peripheral instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instance {
    Tpm1,
    Tpm2,
    Tpm3,
    Tpm4,
    Tpm5,
    Tpm6,
}

/// Output polarity of a timer channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Active-high output.
    ActiveHigh,
    /// Active-low output.
    ActiveLow,
}

/// Exclusive ownership of one i.MX 9 TPM timer.
pub struct Timer {
    tpm: TPM,
    clock_hz: u32,
    #[cfg(feature = "dma")]
    instance: Option<Instance>,
}

impl Timer {
    /// Bind a TPM register block.
    ///
    /// # Safety
    ///
    /// `base` must point to a valid, exclusively owned i.MX 9 TPM register
    /// block. The caller must enable its clock root.
    pub unsafe fn from_raw(base: *mut (), clock_hz: u32) -> Self {
        assert!(clock_hz > 0, "timer clock must be non-zero");
        // SAFETY: upheld by the caller.
        let tpm = unsafe { TPM::from_ptr(base) };
        Self {
            tpm,
            clock_hz,
            #[cfg(feature = "dma")]
            instance: instance_from_address(base as usize),
        }
    }

    /// Bind a TPM register block while explicitly identifying its instance.
    ///
    /// Use this for remapped/secure aliases whose address is not recognized by
    /// [`Timer::from_raw`].
    ///
    /// # Safety
    ///
    /// The same ownership and clocking requirements as [`Timer::from_raw`]
    /// apply, and `instance` must identify the supplied register block.
    pub unsafe fn from_raw_instance(base: *mut (), clock_hz: u32, instance: Instance) -> Self {
        assert!(clock_hz > 0, "timer clock must be non-zero");
        #[cfg(not(feature = "dma"))]
        let _ = instance;
        // SAFETY: upheld by the caller.
        let tpm = unsafe { TPM::from_ptr(base) };
        Self {
            tpm,
            clock_hz,
            #[cfg(feature = "dma")]
            instance: Some(instance),
        }
    }

    /// Consume this timer and configure it for PWM output.
    pub fn into_pwm(self, freq_hz: u32) -> Pwm {
        Pwm::new(self, freq_hz)
    }

    /// Timer input-clock frequency.
    pub const fn clock_hz(&self) -> u32 {
        self.clock_hz
    }

    pub(crate) const fn regs(&self) -> TPM {
        self.tpm
    }

    #[cfg(feature = "dma")]
    pub(crate) fn instance(&self) -> Instance {
        self.instance
            .expect("TPM instance is required for DMA request selection")
    }
}

#[cfg(feature = "dma")]
const fn instance_from_address(address: usize) -> Option<Instance> {
    // Secure aliases add 0x1000_0000 to these non-secure CM addresses.
    match address & !0x1000_0000 {
        0x4431_0000 => Some(Instance::Tpm1),
        0x4432_0000 => Some(Instance::Tpm2),
        0x424e_0000 => Some(Instance::Tpm3),
        0x424f_0000 => Some(Instance::Tpm4),
        0x4250_0000 => Some(Instance::Tpm5),
        0x4251_0000 => Some(Instance::Tpm6),
        _ => None,
    }
}
