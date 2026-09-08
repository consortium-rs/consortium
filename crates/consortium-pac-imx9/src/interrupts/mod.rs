//! Interrupt line definitions, generated from the chip databases by
//! `scripts/pac/gen_interrupts.py`.
//!
//! Enabling the `rt` feature additionally exposes the `cortex_m_rt::interrupt`
//! attribute, an `interrupt` type alias, and the `__INTERRUPTS` vector table
//! consumed by `cortex-m-rt`'s `device` feature.

#[cfg(feature = "mimx95")]
mod imx95;
#[cfg(feature = "mimx95")]
pub use imx95::*;

// `not(mimx95)` suppresses a duplicate-definition cascade when both chip
// features are on. That combination is already fatal — `lib.rs` raises a
// `compile_error!` naming it — and this keeps that guard the only diagnostic.
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
mod imx93;
#[cfg(all(feature = "mimx93", not(feature = "mimx95")))]
pub use imx93::*;
