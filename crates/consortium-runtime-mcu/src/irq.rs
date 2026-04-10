//! Interrupt enable helpers for hardware doorbells.

/// Enable the HSEM interrupt in the NVIC.
///
/// Must be called before any `HsemDoorbell::wait()` to ensure the IRQ handler is active.
///
/// # Safety
///
/// Caller must ensure:
/// - NVIC is initialized
/// - `HSEM_IRQHandler` is registered in the interrupt vector table
#[cfg(feature = "stm32mp2xx")]
pub unsafe fn enable_hsem_irq() {
    // STM32MP2xx uses Cortex-M NVIC. HSEM1 is typically IRQ 90.
    // Use cortex-m::peripheral::NVIC or the PAC to enable the interrupt.
    // This is platform-specific; for now we document the requirement.
    //
    // Example (using cortex-m crate):
    // unsafe { cortex_m::peripheral::NVIC::unmask(cortex_m::peripheral::Interrupt::HSEM1) };
    //
    // The actual HSEM IRQ number varies by STM32MP variant (STM32MP21/23/25).
    // Prefer using the PAC's generated interrupt definitions.
}

/// Enable MU1 and MU2 interrupts in the NVIC.
///
/// Must be called before any `MuDoorbell::wait()` to ensure the IRQ handlers are active.
///
/// # Safety
///
/// Caller must ensure:
/// - NVIC is initialized
/// - `MU1_IRQHandler` and `MU2_IRQHandler` are registered in the interrupt vector table
#[cfg(feature = "imx93")]
pub unsafe fn enable_mu_irq() {
    // i.MX93 MU1 is typically IRQ 216, MU2 is IRQ 217.
    // Use cortex-m::peripheral::NVIC or the PAC to enable both interrupts.
    //
    // Example (using cortex-m crate):
    // unsafe {
    //     cortex_m::peripheral::NVIC::unmask(cortex_m::peripheral::Interrupt::MUA);
    //     cortex_m::peripheral::NVIC::unmask(cortex_m::peripheral::Interrupt::MUB);
    // };
}

#[cfg(feature = "imx95")]
pub unsafe fn enable_mu_irq() {
    // i.MX95 MU interrupt numbers differ from i.MX93.
    // Consult the i.MX95 reference manual for exact IRQ numbers.
}
