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

//! Loads a `Consortium.toml` manifest and binds it to its chip-family database.

use std::collections::HashMap;

use consortium_cfg_common::config::{Config, MemoryRegion, Party, VisibleCore};
use consortium_cfg_common::database::{ChipDatabase, HardwareDoorbellPeripheral};

use crate::{database, error::CfgError};

/// A parsed manifest resolved against the database for its target chip.
#[derive(Debug, Clone)]
pub struct Session {
    /// The parsed `Consortium.toml`.
    pub config: Config,
    /// The chip-family database selected by `[profile] chip`.
    pub database: ChipDatabase,
}

impl Session {
    /// Parses a `Consortium.toml` string, resolves its chip database, and
    /// validates that the manifest only references resources the chip provides.
    ///
    /// # Errors
    ///
    /// - [`CfgError::Manifest`] if the TOML is malformed.
    /// - [`CfgError::UnknownChip`] / [`CfgError::DatabaseParse`] if the chip
    ///   cannot be resolved to a bundled database.
    /// - [`CfgError::UnknownDoorbell`] / [`CfgError::OwnerNotVisible`] from
    ///   [`Self::validate`].
    pub fn from_toml(content: &str) -> Result<Self, CfgError> {
        let config: Config =
            toml::from_str(content).map_err(|err| CfgError::Manifest(err.to_string()))?;
        let database = database::load(&config.profile.chip)?;

        let session = Self { config, database };
        session.validate()?;
        Ok(session)
    }

    /// Checks that every doorbell and peripheral owner named in the manifest
    /// exists on the target chip, and that the shared-memory regions fit within
    /// the chip's SRAM without overlapping.
    pub fn validate(&self) -> Result<(), CfgError> {
        let chip = &self.config.profile.chip;

        self.validate_memory()?;
        self.validate_ipc()?;

        for (name, peripheral) in &self.config.peripheral {
            if !self.is_core(peripheral.owner) {
                return Err(CfgError::OwnerNotVisible(
                    name.clone(),
                    peripheral.owner.to_string(),
                    chip.clone(),
                ));
            }
        }

        Ok(())
    }

    /// Validates IPC doorbell references, party support, and channel allocation.
    fn validate_ipc(&self) -> Result<(), CfgError> {
        let chip = &self.config.profile.chip;
        let mut used_channels: HashMap<(String, u32), Party> = HashMap::new();

        for channel in &self.config.ipc.channels {
            let doorbell = self
                .doorbell(&channel.doorbell)
                .ok_or_else(|| self.unknown_doorbell(&channel.doorbell))?;

            for (party, endpoint) in &channel.endpoints {
                if !self.is_party(*party) {
                    return Err(CfgError::IpcEndpointNotVisible(
                        channel.doorbell.clone(),
                        party.to_string(),
                        chip.clone(),
                    ));
                }

                if !doorbell.parties.iter().any(|entry| entry.core == *party) {
                    return Err(self.doorbell_party_unavailable(doorbell, *party));
                }

                if endpoint.chan >= doorbell.channels {
                    return Err(CfgError::DoorbellChannelUnavailable {
                        doorbell: doorbell.name.clone(),
                        channel: endpoint.chan,
                        channels: doorbell.channels,
                        chip: chip.clone(),
                        trm: self.database.trm.clone(),
                        page: doorbell.channel_page,
                    });
                }

                let key = (doorbell.name.to_ascii_lowercase(), endpoint.chan);
                if let Some(first_party) = used_channels.insert(key, *party) {
                    return Err(CfgError::DoorbellChannelDuplicate {
                        doorbell: doorbell.name.clone(),
                        channel: endpoint.chan,
                        first_party: first_party.to_string(),
                        second_party: party.to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validates shared-memory regions: each must have a non-zero length, lie
    /// entirely within one of the chip's SRAM regions, and not overlap another.
    fn validate_memory(&self) -> Result<(), CfgError> {
        let mut regions = Vec::new();

        if let Some(defmt) = self.config.dbg.defmt {
            regions.push(("dbg.defmt".to_string(), defmt.memory));
        }

        for (index, channel) in self.config.ipc.channels.iter().enumerate() {
            if let Some(memory) = channel.memory() {
                regions.push((format!("ipc.channel[{index}].config.memory"), memory));
            }
        }

        for (name, region) in &regions {
            self.validate_memory_region(name, *region)?;
        }

        for (left_index, (left_name, left)) in regions.iter().enumerate() {
            let left_end = left.base + left.length;
            for (right_name, right) in regions.iter().skip(left_index + 1) {
                let right_end = right.base + right.length;
                if left.base < right_end && right.base < left_end {
                    return Err(CfgError::RegionOverlap {
                        a: left_name.clone(),
                        a_base: left.base,
                        a_end: left_end,
                        b: right_name.clone(),
                        b_base: right.base,
                        b_end: right_end,
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_memory_region(&self, name: &str, region: MemoryRegion) -> Result<(), CfgError> {
        // Exclusive end address; a region that wraps past u64::MAX cannot
        // fit in any SRAM window, so treat the overflow as out-of-bounds.
        let Some(end) = region.base.checked_add(region.length) else {
            return Err(self.region_out_of_bounds(name, region.base, region.base));
        };

        if region.length == 0 {
            return Err(CfgError::RegionEmpty {
                region: name.to_string(),
            });
        }

        let contained = self.database.memory.regions.iter().any(|sram| {
            let sram_base = sram.base;
            let sram_end = sram_base + sram.length;
            region.base >= sram_base && end <= sram_end
        });
        if !contained {
            return Err(self.region_out_of_bounds(name, region.base, end));
        }

        Ok(())
    }

    /// Builds a [`CfgError::RegionOutOfBounds`] annotated with the chip's TRM
    /// URL and the memory-map page(s), so the user knows where to look.
    fn region_out_of_bounds(&self, region: &str, base: u32, end: u32) -> CfgError {
        let pages = self
            .database
            .memory
            .trm_page
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        CfgError::RegionOutOfBounds {
            region: region.to_string(),
            base,
            end,
            chip: self.config.profile.chip.clone(),
            trm: self.database.trm.clone(),
            pages,
        }
    }

    /// Builds a [`CfgError::UnknownDoorbell`] listing the doorbells the chip
    /// actually exposes, each with its TRM page.
    fn unknown_doorbell(&self, name: &str) -> CfgError {
        let available = self
            .database
            .peripheral
            .doorbells
            .iter()
            .map(|entry| format!("{} (TRM p. {})", entry.name, entry.page))
            .collect::<Vec<_>>()
            .join(", ");

        CfgError::UnknownDoorbell {
            name: name.to_string(),
            chip: self.config.profile.chip.clone(),
            trm: self.database.trm.clone(),
            available,
        }
    }

    /// Finds a database doorbell by manifest name, ignoring ASCII case.
    fn doorbell(&self, name: &str) -> Option<&HardwareDoorbellPeripheral> {
        self.database
            .peripheral
            .doorbells
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
    }

    /// Builds a [`CfgError::DoorbellPartyUnavailable`] with the configured
    /// parties and the TRM page documenting party support.
    fn doorbell_party_unavailable(
        &self,
        doorbell: &HardwareDoorbellPeripheral,
        party: Party,
    ) -> CfgError {
        let available = doorbell
            .parties
            .iter()
            .map(|entry| {
                format!(
                    "{} (TRM memory p. {}, IRQ p. {})",
                    entry.core, entry.page_memory, entry.page_irq
                )
            })
            .collect::<Vec<_>>()
            .join(", ");

        CfgError::DoorbellPartyUnavailable {
            doorbell: doorbell.name.clone(),
            party: party.to_string(),
            chip: self.config.profile.chip.clone(),
            available,
            trm: self.database.trm.clone(),
            page: doorbell.party_page,
        }
    }

    /// Whether `core` is the application processor or one of the chip's
    /// declared coprocessors.
    fn is_core(&self, core: VisibleCore) -> bool {
        match core {
            VisibleCore::Ap => true,
            VisibleCore::Mcore(name) => self
                .database
                .cp
                .iter()
                .any(|cp| cp.name.eq_ignore_ascii_case(name.as_str())),
            VisibleCore::Rcore(name) => self
                .database
                .cp
                .iter()
                .any(|cp| cp.name.eq_ignore_ascii_case(name.as_str())),
            VisibleCore::Rvcore(name) => self
                .database
                .cp
                .iter()
                .any(|cp| cp.name.eq_ignore_ascii_case(&name.to_string())),
        }
    }

    /// Whether `party` names one of the chip's visible runtime endpoints.
    fn is_party(&self, party: Party) -> bool {
        self.is_core(party.core())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = include_str!("../Consortium.example.stm32mp257.toml");

    #[test]
    fn loads_example_manifest() {
        let session = Session::from_toml(MANIFEST).expect("example manifest should load");

        assert_eq!(session.config.profile.chip, "stm32mp257fak3");
        assert_eq!(session.database.name, "STM32MP25");
        assert_eq!(session.config.ipc.channels.len(), 2);
        assert_eq!(session.config.ipc.channels[0].doorbell, "hsem");
        assert_eq!(
            session.config.ipc.channels[0].endpoints[&Party::AcoreNonSecure].chan,
            0
        );
        assert_eq!(
            session.config.ipc.channels[0].endpoints[&"cm33".parse().unwrap()].r#type,
            "MotionControl"
        );
        assert_eq!(session.config.endpoints[&Party::AcoreNonSecure], "./app");
        assert_eq!(
            session
                .config
                .tee
                .as_ref()
                .unwrap()
                .uuid
                .to_string()
                .to_uppercase(),
            "48551DD5-74B2-4D24-BF9D-A09F02F5C56F"
        );

        let i2c1 = &session.config.peripheral["i2c1"];
        assert_eq!(i2c1.owner, VisibleCore::Ap);
        assert_eq!(i2c1.pin["sda"], "PB0");

        let spi1 = &session.config.peripheral["spi1"];
        assert_eq!(spi1.owner, "cm33".parse().unwrap());
    }

    #[test]
    fn rejects_unknown_doorbell() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [[ipc.channel]]
            name = "probe"
            doorbell = "nope"
            pattern = "memory"
            party.linux = { chan = 0, type = "Ping" }
            party.cm33 = { chan = 1, type = "Pong" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        let err = Session::from_toml(manifest).unwrap_err();
        assert!(matches!(err, CfgError::UnknownDoorbell { .. }));
        // The hint lists real doorbells with their TRM pages.
        assert!(err.to_string().contains("HSEM (TRM p. 1306)"));
    }

    #[test]
    fn rejects_doorbell_party_unavailable() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [[ipc.channel]]
            name = "probe"
            doorbell = "ipcc1"
            pattern = "memory"
            party.cm0p = { chan = 0, type = "PowerMode" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        let err = Session::from_toml(manifest).unwrap_err();
        assert!(matches!(err, CfgError::DoorbellPartyUnavailable { .. }));
        assert!(err.to_string().contains("Configured parties: linux"));
        assert!(err.to_string().contains("TRM"));
    }

    #[test]
    fn rejects_unavailable_doorbell_channel() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [[ipc.channel]]
            name = "probe"
            doorbell = "hsem"
            pattern = "memory"
            party.linux = { chan = 16, type = "Ping" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        assert!(matches!(
            Session::from_toml(manifest),
            Err(CfgError::DoorbellChannelUnavailable { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_doorbell_channel() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [[ipc.channel]]
            name = "probe"
            doorbell = "hsem"
            pattern = "memory"
            party.linux = { chan = 0, type = "Ping" }
            party.cm33 = { chan = 0, type = "Pong" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        assert!(matches!(
            Session::from_toml(manifest),
            Err(CfgError::DoorbellChannelDuplicate { .. })
        ));
    }

    /// IPC region `0x40_0000..0x41_0000` sits outside every SRAM window on
    /// STM32MP25 (SRAM1 starts at `0x400000`), so it must be rejected.
    #[test]
    fn rejects_region_outside_sram() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [[ipc.channel]]
            name = "probe"
            doorbell = "hsem"
            pattern = "memory"
            party.linux = { chan = 0, type = "Ping" }
            config.memory = { base = 0x00400000, length = 0x00080000 }
            mtu = 128
        "#;

        assert!(matches!(
            Session::from_toml(manifest),
            Err(CfgError::RegionOutOfBounds { .. })
        ));
    }

    #[test]
    fn rejects_overlapping_regions() {
        let manifest = r#"
            [profile]
            chip = "stm32mp257fak3"

            [dbg.defmt]
            memory = { base = 0x00404000, length = 0x00002000 }
            mtu = 64

            [[ipc.channel]]
            name = "probe"
            doorbell = "hsem"
            pattern = "memory"
            party.linux = { chan = 0, type = "Ping" }
            config.memory = { base = 0x00400000, length = 0x00008000 }
            mtu = 128
        "#;

        assert!(matches!(
            Session::from_toml(manifest),
            Err(CfgError::RegionOverlap { .. })
        ));
    }
}
