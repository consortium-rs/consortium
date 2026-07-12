//! NVIC helpers built on the `cortex-m` crate.
//!
//! Both supported chip families put a Cortex-M33 behind the standard NVIC,
//! and consortium-cfg's generated bring-up code already drives it through
//! `cortex_m::peripheral::NVIC` with [`InterruptNumber`] wrappers; this
//! module gives chip HAL drivers the same mechanism for interrupt lines
//! known only at runtime. Interrupt lines are identified by the
//! device-specific external interrupt number (exception number minus 16), as
//! used by the chip reference manuals, vector tables, and the consortium-cfg
//! chip databases.

use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::NVIC;

/// Interrupt line identified by its raw external interrupt number.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Irq(pub u16);

// SAFETY: `Irq` transparently forwards the external interrupt number, which
// the caller sources from validated chip metadata; the NVIC ignores lines the
// device does not implement.
unsafe impl InterruptNumber for Irq {
    fn number(self) -> u16 {
        self.0
    }
}

/// Enable (unmask) an interrupt.
///
/// # Safety
///
/// Enabling an interrupt may break mask-based critical sections and requires
/// that a handler for it is installed in the active vector table.
pub unsafe fn enable(irq: u16) {
    // SAFETY: Propagates the caller's critical-section/handler contract.
    unsafe { NVIC::unmask(Irq(irq)) }
}

/// Disable (mask) an interrupt.
pub fn disable(irq: u16) {
    NVIC::mask(Irq(irq));
}

/// Check whether an interrupt is enabled.
pub fn is_enabled(irq: u16) -> bool {
    NVIC::is_enabled(Irq(irq))
}

/// Set an interrupt pending in software.
pub fn pend(irq: u16) {
    NVIC::pend(Irq(irq));
}

/// Clear an interrupt's pending state.
pub fn unpend(irq: u16) {
    NVIC::unpend(Irq(irq));
}

/// Check whether an interrupt is pending.
pub fn is_pending(irq: u16) -> bool {
    NVIC::is_pending(Irq(irq))
}

/// Set an interrupt's priority byte.
///
/// The value is the raw 8-bit priority register contents: lower is more
/// urgent, and only the implemented high-order bits are held by hardware
/// (four bits on both supported chip families, so multiples of `0x10`).
///
/// # Safety
///
/// Changing priorities may break priority-based critical sections.
pub unsafe fn set_priority(irq: u16, priority: u8) {
    // SAFETY: Register access is scoped to the NVIC priority byte; the
    // caller upholds the priority-based critical-section contract.
    #[allow(clippy::multiple_unsafe_ops_per_block)]
    unsafe {
        cortex_m::Peripherals::steal()
            .NVIC
            .set_priority(Irq(irq), priority);
    }
}

/// Read an interrupt's raw priority byte.
pub fn priority(irq: u16) -> u8 {
    NVIC::get_priority(Irq(irq))
}

#[cfg(test)]
mod tests {
    use super::Irq;
    use cortex_m::interrupt::InterruptNumber;

    #[test]
    fn irq_forwards_its_number() {
        assert_eq!(Irq(0).number(), 0);
        assert_eq!(Irq(225).number(), 225);
    }
}
