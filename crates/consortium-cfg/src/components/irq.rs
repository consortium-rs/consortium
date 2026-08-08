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

//! How generated bring-up names an interrupt line.

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// One interrupt line as a single visible core sees it.
///
/// The chip HAL re-exports its PAC's `Interrupt` enum, so generated code should
/// name a line the way the vector table and the reference manual do rather than
/// declaring an enum of its own or passing a bare number. That is only sound
/// where the PAC's shipped table describes the core being generated for — a PAC
/// carries one table per chip, and a shared line name such as `IPCC2_RX` is
/// numbered differently on STM32MP2's CM33 and CM0+. [`variant`](Self::variant)
/// therefore holds the name only when
/// [`ChipMap::has_pac_interrupts`](consortium_data::chipmap::ChipMap::has_pac_interrupts)
/// allows it, and the remaining cases fall back to the HAL's raw-number carrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterruptLine {
    /// Raw interrupt number: NVIC position on a controller core, GIC SPI on the
    /// AP.
    pub number: u32,
    /// The chip PAC's `Interrupt` variant for this line, when one applies.
    pub variant: Option<String>,
}

impl InterruptLine {
    /// A line known only by its number.
    pub fn new(number: u32) -> Self {
        Self {
            number,
            variant: None,
        }
    }

    /// The expression naming this line to the HAL's `nvic` helpers, and the
    /// label the generated trace prints for it.
    ///
    /// Returns `None` when the number does not fit an external interrupt
    /// number, which is the AP's GIC-only case rather than anything a
    /// controller core enables.
    pub fn tokens(&self, hal: &TokenStream) -> Option<(TokenStream, String)> {
        let number = u16::try_from(self.number).ok()?;
        Some(match &self.variant {
            Some(variant) => {
                let variant = Ident::new(variant, Span::call_site());
                (quote! { #hal::Interrupt::#variant }, variant.to_string())
            }
            None => (quote! { #hal::hal::nvic::Irq(#number) }, number.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hal() -> TokenStream {
        quote! { ::consortium_hal_stm32mp2 }
    }

    #[test]
    fn names_the_pac_variant_when_one_applies() {
        let line = InterruptLine {
            number: 171,
            variant: Some("IPCC1_RX".to_string()),
        };
        let (tokens, label) = line.tokens(&hal()).unwrap();

        assert_eq!(
            tokens.to_string(),
            ":: consortium_hal_stm32mp2 :: Interrupt :: IPCC1_RX"
        );
        assert_eq!(label, "IPCC1_RX");
    }

    #[test]
    fn falls_back_to_the_raw_number_carrier() {
        let (tokens, label) = InterruptLine::new(13).tokens(&hal()).unwrap();

        assert_eq!(
            tokens.to_string(),
            ":: consortium_hal_stm32mp2 :: hal :: nvic :: Irq (13u16)"
        );
        assert_eq!(label, "13");
    }

    #[test]
    fn rejects_a_number_wider_than_an_interrupt_line() {
        assert!(InterruptLine::new(0x1_0000).tokens(&hal()).is_none());
    }
}
