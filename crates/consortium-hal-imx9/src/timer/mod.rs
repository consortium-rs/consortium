//! i.MX 9 TPM timers and their operating modes.
//!
//! A [`Timer`] owns one TPM register block, taken from its peripheral
//! singleton. Consume it with [`Timer::into_pwm`] to select PWM mode,
//! mirroring Embassy's timer-driver ownership model:
//!
//! ```ignore
//! let p = consortium_hal_imx9::init();
//! let pwm = Timer::new(p.TPM1, 24_000_000).into_pwm(1_000);
//! ```

use embassy_hal_internal::{Peri, PeripheralType};
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

/// Which TPM a [`Timer`] drives.
///
/// Carried by [`Instance::ID`] so the DMA layer can select the instance's
/// request lines; it names no addresses of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerId {
    Tpm1,
    Tpm2,
    Tpm3,
    Tpm4,
    Tpm5,
    Tpm6,
}

/// Crate-internal half of [`Instance`]: visible to the generated bindings that
/// implement it, sealed against downstream crates.
pub(crate) trait SealedInstance {
    /// This instance's register block, at its database-resolved base.
    fn regs() -> TPM;
    /// The CCM gate the constructor opens before the first register access
    /// (see [`crate::clock`]); `None` where this core need not open one.
    const LPCG: Option<u32>;
}

/// A TPM the [`Timer`] driver can drive.
///
/// Implemented by the generated peripheral singletons; sealed so the base
/// address stays single-sourced from the chip database.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType {
    /// NVIC line this instance raises, when the chip routes one to this core.
    const IRQ: Option<u16>;
    /// Which TPM this is, for eDMA request selection.
    const ID: TimerId;
}

/// Bind a generated peripheral singleton to this driver. Invoked only by the
/// generated `peripherals` module.
macro_rules! impl_tpm {
    ($instance:ident, $base:expr, $id:ident, $irq:expr) => {
        impl crate::timer::SealedInstance for crate::peripherals::$instance {
            const LPCG: ::core::option::Option<u32> = crate::clock::lpcg(stringify!($instance));

            #[inline]
            fn regs() -> crate::pac::peripherals::tpm::TPM {
                // SAFETY: `$base` is this instance's TPM base as extracted from
                // the chip database, and the singleton this impl is keyed to
                // can be held by only one driver at a time.
                unsafe { crate::pac::peripherals::tpm::TPM::from_ptr($base as *mut ()) }
            }
        }

        impl crate::timer::Instance for crate::peripherals::$instance {
            const IRQ: ::core::option::Option<u16> = $irq;
            const ID: crate::timer::TimerId = crate::timer::TimerId::$id;
        }
    };
}
pub(crate) use impl_tpm;

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
    id: TimerId,
}

impl Timer {
    /// Bind a TPM from its peripheral singleton.
    ///
    /// The platform must leave the instance's clock root enabled; `clock_hz` is
    /// the frequency it runs at.
    pub fn new<T: Instance>(peri: Peri<'static, T>, clock_hz: u32) -> Self {
        assert!(clock_hz > 0, "timer clock must be non-zero");
        // Owning the singleton for the driver's lifetime is what makes exclusive
        // register access safe; it is consumed here and never reissued.
        let _ = peri;
        // On i.MX 93 Linux closes the gate of every peripheral no kernel
        // driver claims; the first register access would BusFault without this.
        crate::clock::ungate(T::LPCG);
        Self {
            tpm: T::regs(),
            clock_hz,
            #[cfg(feature = "dma")]
            id: T::ID,
        }
    }

    /// Build a timer over an arbitrary register block, for tests that stand a
    /// plain array in for the TPM.
    #[cfg(test)]
    pub(crate) fn over(tpm: TPM, clock_hz: u32, id: TimerId) -> Self {
        #[cfg(not(feature = "dma"))]
        let _ = id;
        Self {
            tpm,
            clock_hz,
            #[cfg(feature = "dma")]
            id,
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
    pub(crate) fn id(&self) -> TimerId {
        self.id
    }
}
