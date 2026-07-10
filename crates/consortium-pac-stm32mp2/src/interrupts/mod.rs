//! Interrupt line definitions, generated from the chip databases by
//! `scripts/pac/gen_interrupts.py`.
//!
//! Enabling the `rt` feature additionally exposes the `cortex_m_rt::interrupt`
//! attribute, an `interrupt` type alias, and the `__INTERRUPTS` vector table
//! consumed by `cortex-m-rt`'s `device` feature.

#[cfg(feature = "stm32mp257")]
mod stm32mp25;
#[cfg(feature = "stm32mp257")]
pub use stm32mp25::*;

#[cfg(feature = "stm32mp21")]
mod stm32mp21;
#[cfg(feature = "stm32mp21")]
pub use stm32mp21::*;
