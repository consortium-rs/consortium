//! NVIC helpers built on the `cortex-m` crate.
//!
//! Both supported chip families put a Cortex-M behind the standard NVIC. Every
//! helper here takes an [`InterruptNumber`], so the caller normally passes the
//! chip PAC's own `Interrupt` variant — that enum names the same line the
//! vector table and the reference manual do, and it is what generated bring-up
//! and driver code should spell. [`Irq`] wraps a bare external interrupt number
//! (exception number minus 16) for the cases where the line is only known as a
//! value at runtime, such as a driver `Instance`'s associated constant.

use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::NVIC;

/// Interrupt line identified by its raw external interrupt number.
///
/// Prefer the chip PAC's `Interrupt` enum where the line is known statically;
/// this is the escape hatch for numbers carried as data.
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
pub unsafe fn enable(irq: impl InterruptNumber) {
    // SAFETY: Propagates the caller's critical-section/handler contract.
    unsafe { NVIC::unmask(irq) }
}

/// Disable (mask) an interrupt.
pub fn disable(irq: impl InterruptNumber) {
    NVIC::mask(irq);
}

/// Check whether an interrupt is enabled.
pub fn is_enabled(irq: impl InterruptNumber) -> bool {
    NVIC::is_enabled(irq)
}

/// Set an interrupt pending in software.
pub fn pend(irq: impl InterruptNumber) {
    NVIC::pend(irq);
}

/// Clear an interrupt's pending state.
pub fn unpend(irq: impl InterruptNumber) {
    NVIC::unpend(irq);
}

/// Check whether an interrupt is pending.
pub fn is_pending(irq: impl InterruptNumber) -> bool {
    NVIC::is_pending(irq)
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
pub unsafe fn set_priority(irq: impl InterruptNumber, priority: u8) {
    // SAFETY: Register access is scoped to the NVIC priority byte; the
    // caller upholds the priority-based critical-section contract.
    #[allow(clippy::multiple_unsafe_ops_per_block)]
    unsafe {
        cortex_m::Peripherals::steal()
            .NVIC
            .set_priority(irq, priority);
    }
}

/// Read an interrupt's raw priority byte.
pub fn priority(irq: impl InterruptNumber) -> u8 {
    NVIC::get_priority(irq)
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
