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

//! Hardware doorbell -> UIO device.
//!
//! A doorbell peripheral exposes its register block as the UIO device's single
//! `reg` window. Its interrupt is taken from the Linux (non-secure A-core)
//! party, since the UIO device is consumed by the Linux application; a doorbell
//! with no Linux-facing interrupt becomes a memory-only UIO device.

use consortium_cfg_common::database::{HardwareDoorbellPeripheral, Party};
use consortium_dts::utils::irq_method::IrqMethod;

use super::ToUio;
use crate::components::uio::UioDevice;
use crate::error::CfgError;

impl ToUio for HardwareDoorbellPeripheral {
    fn to_uio(&self) -> Result<Option<UioDevice>, CfgError> {
        // The UIO device is driven from Linux, so use that party's GIC SPI.
        // GIC SPI doorbell lines are level-high triggered.
        let irq = self
            .parties
            .iter()
            .find(|party| party.core == Party::AcoreNonSecure)
            .map(|party| (party.irq, IrqMethod::LevelHigh));

        // A doorbell always exposes its single register window as `map0`, so
        // there is no `mmap` ordering to violate.
        Ok(Some(UioDevice {
            mmap: vec![(self.memory.base, self.memory.length)],
            irq,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::database::{
        HardwareDoorbellParty, HardwareDoorbellPeripheralType, HardwareMemory,
    };

    fn party(core: &str, irq: u32) -> HardwareDoorbellParty {
        HardwareDoorbellParty {
            core: core.parse().unwrap(),
            irq,
            page_memory: 0,
            page_irq: 0,
        }
    }

    fn doorbell(parties: Vec<HardwareDoorbellParty>) -> HardwareDoorbellPeripheral {
        HardwareDoorbellPeripheral {
            name: "HSEM".to_string(),
            page: 1306,
            r#type: HardwareDoorbellPeripheralType::Hsem,
            memory: HardwareMemory {
                base: 0x0624_0000,
                length: 0x400,
            },
            parties,
        }
    }

    #[test]
    fn uses_linux_party_interrupt() {
        let uio = doorbell(vec![
            party("optee", 200),
            party("linux", 199),
            party("cm33", 199),
        ])
        .to_uio()
        .unwrap()
        .unwrap();

        assert_eq!(uio.mmap, vec![(0x0624_0000, 0x400)]);
        assert_eq!(uio.irq, Some((199, IrqMethod::LevelHigh)));
    }

    #[test]
    fn memory_only_without_linux_party() {
        let uio = doorbell(vec![party("cm33", 199), party("cm0p", 21)])
            .to_uio()
            .unwrap()
            .unwrap();

        assert_eq!(uio.mmap, vec![(0x0624_0000, 0x400)]);
        assert!(uio.irq.is_none());
    }
}
