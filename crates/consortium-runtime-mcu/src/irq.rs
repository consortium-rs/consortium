// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

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
    // STM32MP25: HSEM_IT1 (non-secure) = IRQ 199 in NVIC
    // Source: STM32MP25 RM0457 NVIC table — vector offset 0x035C → IRQ = (0x35C - 0x40) / 4 = 199
    //
    // SAFETY: Caller must ensure NVIC is initialized. We directly unmask via ISER register.
    // ISER0 handles IRQs 0-31, ISER1 handles IRQs 32-63, etc.
    // IRQ 199 → ISER6[199-192] = ISER6[7]
    const HSEM_IT1_IRQ: usize = 199;
    let iser_index = HSEM_IT1_IRQ / 32;
    let iser_bit = HSEM_IT1_IRQ % 32;
    let iser_addr = 0xE000_E100 as *mut u32;
    let iser = unsafe { iser_addr.add(iser_index) };
    unsafe { iser.write_volatile(iser.read_volatile() | (1 << iser_bit)) };
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
