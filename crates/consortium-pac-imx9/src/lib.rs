#![no_std]

pub mod common;
pub mod interrupts;
pub mod metadata;
#[cfg(feature = "imx95")]
pub mod peripherals;
