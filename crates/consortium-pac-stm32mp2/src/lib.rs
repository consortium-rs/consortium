#![no_std]

pub mod common;
pub mod interrupts;
pub mod metadata;
#[cfg(feature = "stm32mp257")]
pub mod peripherals;
