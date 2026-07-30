#![no_std]

//! Peripheral Access Crate for the NXP i.MX 9 family: register blocks, interrupt
//! definitions, and the `cortex-m-rt` vector table.
//!
//! **This crate is generated.** Register accessors come from `chiptool` fed by
//! metadata parsed out of NXP's CMSIS headers; interrupt tables and the linker
//! `PROVIDE` fragments come from `scripts/pac/gen_interrupts.py` reading the
//! `consortium-data` chip dumps. Do not hand-edit the output — change the parser
//! scripts or the vendor patch workflow and re-run the `just pac-*` recipes.
//!
//! Most code should depend on [`consortium-hal-imx9`] rather than on this crate
//! directly: it re-exports everything here as `pac` and adds owned peripheral
//! singletons, so drivers never touch an MMIO base.
//!
//! [`consortium-hal-imx9`]: https://docs.rs/consortium-hal-imx9
//!
//! # Modules
//!
//! | Module          | Contents                                                     |
//! | --------------- | ------------------------------------------------------------ |
//! | [`peripherals`] | Register block definitions, one module per peripheral kind.    |
//! | [`interrupts`]  | The chip's `Interrupt` enum; with `rt`, the vector table too.  |
//! | [`common`]      | Register/field access primitives the generated code builds on.  |
//! | [`metadata`]    | Machine-readable register description, ported from `stm32-data`. |
//!
//! # Features
//!
//! | Feature  | Effect                                                             |
//! | -------- | ------------------------------------------------------------------ |
//! | `mimx95` | Chip selection (default).                                           |
//! | `mimx93` | Chip selection.                                                     |
//! | `rt`     | `cortex-m-rt` integration: the `#[interrupt]` macro and `__INTERRUPTS`. |
//! | `defmt`  | Derive `defmt::Format` on generated types where applicable.           |
//!
//! Choose **exactly one** chip: the feature selects which generated interrupt and
//! peripheral map is mounted, and enabling both is a duplicate-definition error. A
//! dependent selecting `mimx93` must set `default-features = false` so `mimx95` is not
//! also pulled in — two chips mean an ambiguous `Interrupt` enum.
//!
//! `rt` should be enabled by the *application*, since it is what owns the vector table.

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
#[cfg(any(feature = "mimx93", feature = "mimx95"))]
pub mod peripherals;
