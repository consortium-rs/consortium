//! Panic handler that routes the panic message through the CDBG debug ring.
//!
//! When the `panic-handler` feature is enabled, this module installs the crate's
//! `#[panic_handler]`. On a panic it:
//!
//! 1. disables interrupts so no logging interrupt can race the encoder,
//! 2. emits the panic message as an ordinary defmt frame through the existing
//!    [`ConsortiumLogger`] pipeline (so the host decodes it with no new code),
//! 3. latches [`Flags::Panic`](crate::descriptor::Flags::Panic) in the
//!    descriptor so the host reader can tell a crash from a clean end-of-stream,
//! 4. halts the core.
//!
//! The message and flag are left in the retained debug SRAM for a post-mortem
//! read; the handler never blocks waiting for the A-core to drain the ring.
//!
//! Interrupt-disable and halt are architecture specific and selected by exactly
//! one of the mutually exclusive `arch-cortex-m` / `arch-riscv` features (the
//! `panic-handler` gate is checked in `lib.rs`).
#![cfg(feature = "panic-handler")]

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::config;
use crate::logger::ConsortiumLogger;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arch::disable_interrupts();

    // Guard against a double panic: if `defmt::error!` itself faults, the second
    // entry skips logging and goes straight to halt instead of recursing.
    static PANICKED: AtomicBool = AtomicBool::new(false);
    if !PANICKED.swap(true, Ordering::SeqCst) {
        defmt::error!("panic: {}", defmt::Display2Format(info));
        if config::BASE != 0 {
            // SAFETY: `config::BASE_PTR` describes the configured debug shared-memory
            // region (the same layout the logger writes into above).
            unsafe { ConsortiumLogger::mark_panic(config::BASE_PTR) };
        }
    }

    arch::halt()
}

#[cfg(feature = "arch-cortex-m")]
mod arch {
    /// Masks interrupts via `CPSID i` so no defmt-logging interrupt races the
    /// panic message encode.
    pub fn disable_interrupts() {
        cortex_m::interrupt::disable();
    }

    /// Halts the core with an undefined instruction (`UDF`), which traps to the
    /// HardFault handler / debugger.
    pub fn halt() -> ! {
        cortex_m::asm::udf()
    }
}

#[cfg(feature = "arch-riscv")]
mod arch {
    /// Clears `mstatus.MIE` so no interrupt races the panic message encode.
    pub fn disable_interrupts() {
        riscv::interrupt::disable();
    }

    /// Halts the core by parking in a low-power `WFI` loop with interrupts masked.
    pub fn halt() -> ! {
        loop {
            riscv::asm::wfi();
        }
    }
}
