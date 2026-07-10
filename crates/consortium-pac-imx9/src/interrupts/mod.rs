//! Interrupt line definitions, generated from the chip databases by
//! `scripts/pac/gen_interrupts.py`.
//!
//! Enabling the `rt` feature additionally exposes the `cortex_m_rt::interrupt`
//! attribute, an `interrupt` type alias, and the `__INTERRUPTS` vector table
//! consumed by `cortex-m-rt`'s `device` feature.

#[cfg(feature = "imx95")]
mod imx95;
#[cfg(feature = "imx95")]
pub use imx95::*;

#[cfg(feature = "imx93")]
mod imx93;
#[cfg(feature = "imx93")]
pub use imx93::*;
