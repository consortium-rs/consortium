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
//!
#[cfg(feature = "stm32mp2xx")]
use cortex_m::interrupt::InterruptNumber;
use cortex_m::peripheral::NVIC;

#[derive(Copy, Clone)]
pub struct DoorbellInterrupt;

// SAFETY: TRM verified.
unsafe impl InterruptNumber for DoorbellInterrupt {
    fn number(self) -> u16 {
        #[cfg(feature = "stm32mp2xx")]
        {
            199 // HSEM_IT1 (non-secure) = IRQ 199 in NVIC
        }
        #[cfg(feature = "imx95")]
        {
            207 // MU7 (MU-B) = IRQ 207 in NVIC for i.MX95
        }
        #[cfg(feature = "imx93")]
        {
            22 // MU1 (MU-B) = IRQ 22 in NVIC for i.MX93
        }
    }
}

/// Enables the hardware doorbell interrupt.
///
/// # Safety
///
/// This function is unsafe because it can break mask-based critical sections. It should only be
/// called once during initialization, and the caller must ensure that it is not called while critical
/// sections are active.
#[inline]
pub unsafe fn enable_doorbell_interrupt() {
    unsafe { NVIC::unmask(DoorbellInterrupt) }
}
