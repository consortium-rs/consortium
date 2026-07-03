//! Minimal NVIC accessor for Armv8-M cores.
//!
//! Both supported chip families put a Cortex-M33 behind the same
//! architecturally defined NVIC register layout, so this module is shared.
//! Interrupt numbers are the device-specific external interrupt numbers
//! (exception number minus 16), as used by the chip reference manuals and
//! vector tables. All functions perform MMIO on the core-private peripheral
//! space and are only meaningful when executed on the target core.

const NVIC_ISER: usize = 0xe000_e100;
const NVIC_ICER: usize = 0xe000_e180;
const NVIC_ISPR: usize = 0xe000_e200;
const NVIC_ICPR: usize = 0xe000_e280;
const NVIC_IPR: usize = 0xe000_e400;

/// Highest external interrupt number addressable through this module.
pub const IRQ_MAX: u16 = 479;

const fn bank_and_bit(irq: u16) -> (usize, u32) {
    assert!(irq <= IRQ_MAX);
    ((irq / 32) as usize, 1 << (irq % 32))
}

fn write_bank(base: usize, irq: u16) {
    let (bank, bit) = bank_and_bit(irq);
    // SAFETY: The address is the architecturally defined NVIC register for
    // this interrupt bank on Armv8-M; the register is write-1-to-act, so
    // writing the single bit cannot disturb other interrupts.
    unsafe { ((base + bank * 4) as *mut u32).write_volatile(bit) }
}

fn read_bank(base: usize, irq: u16) -> bool {
    let (bank, bit) = bank_and_bit(irq);
    // SAFETY: The address is the architecturally defined NVIC register for
    // this interrupt bank on Armv8-M; reads have no side effects.
    unsafe { ((base + bank * 4) as *const u32).read_volatile() & bit != 0 }
}

/// Enable an interrupt.
///
/// # Safety
///
/// Enabling an interrupt may break mask-based critical sections and requires
/// that a handler for it is installed in the active vector table.
pub unsafe fn enable(irq: u16) {
    write_bank(NVIC_ISER, irq);
}

/// Disable an interrupt.
pub fn disable(irq: u16) {
    write_bank(NVIC_ICER, irq);
}

/// Check whether an interrupt is enabled.
pub fn is_enabled(irq: u16) -> bool {
    read_bank(NVIC_ISER, irq)
}

/// Set an interrupt pending in software.
pub fn pend(irq: u16) {
    write_bank(NVIC_ISPR, irq);
}

/// Clear an interrupt's pending state.
pub fn unpend(irq: u16) {
    write_bank(NVIC_ICPR, irq);
}

/// Check whether an interrupt is pending.
pub fn is_pending(irq: u16) -> bool {
    read_bank(NVIC_ISPR, irq)
}

/// Set an interrupt's priority byte.
///
/// The value is the raw 8-bit priority register contents: lower is more
/// urgent, and only the implemented high-order bits are held by hardware
/// (four bits on both supported chip families, so multiples of `0x10`).
pub fn set_priority(irq: u16, priority: u8) {
    assert!(irq <= IRQ_MAX);
    // SAFETY: The address is the architecturally defined byte-accessible
    // NVIC_IPRn entry for `irq` on Armv8-M.
    unsafe { ((NVIC_IPR + irq as usize) as *mut u8).write_volatile(priority) }
}

/// Read an interrupt's raw priority byte.
pub fn priority(irq: u16) -> u8 {
    assert!(irq <= IRQ_MAX);
    // SAFETY: The address is the architecturally defined byte-accessible
    // NVIC_IPRn entry for `irq` on Armv8-M; reads have no side effects.
    unsafe { ((NVIC_IPR + irq as usize) as *const u8).read_volatile() }
}

#[cfg(test)]
mod tests {
    use super::bank_and_bit;

    #[test]
    fn banks_split_every_32_interrupts() {
        assert_eq!(bank_and_bit(0), (0, 1));
        assert_eq!(bank_and_bit(31), (0, 1 << 31));
        assert_eq!(bank_and_bit(32), (1, 1));
        assert_eq!(bank_and_bit(479), (14, 1 << 31));
    }

    #[test]
    #[should_panic]
    fn rejects_out_of_range_interrupts() {
        bank_and_bit(480);
    }
}
