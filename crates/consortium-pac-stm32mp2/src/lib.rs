#![no_std]
// Generated register code carries a single SAFETY contract per accessor and is
// not hand-edited; exempt it from the per-op unsafe lint discipline and from
// doc/style lints that the generator does not emit.
#![allow(
    clippy::derivable_impls,
    clippy::empty_docs,
    clippy::erasing_op,
    clippy::identity_op,
    clippy::missing_safety_doc,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_cast
)]

pub mod common;
pub mod interrupts;
pub mod metadata;
#[cfg(any(feature = "stm32mp21", feature = "stm32mp257"))]
pub mod peripherals;
