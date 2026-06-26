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

//! RIFSC YAML loader utility for reading hardware-specific peripheral configurations.
//!
//! Requires both `serde` and `std` features to be enabled.

use std::fs;
use std::path::Path;

use crate::config::rif::RifConfig;
use serde_yaml;

/// A chip family paired with the list of its peripheral names.
pub type ChipPeripherals = (String, Vec<String>);

/// Load a RIFSC YAML file and extract all peripheral names.
///
/// Returns a Vec<String> containing all peripheral names (e.g., "tim1", "spi1", "uart1", etc.)
/// that have RIFSC IDs defined in the YAML file.
pub fn load_rifsc_peripherals(yaml_path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(yaml_path)?;
    let config: RifConfig = serde_yaml::from_str(&content)?;
    let peripherals = config.peripherals.iter().map(|p| p.name.clone()).collect();
    Ok(peripherals)
}

/// Load all RIFSC YAML files from a directory and return peripheral lists for each chip.
///
/// # Arguments
/// * `rif_dir` - Path to the directory containing RIFSC YAML files (e.g., `data/rif/rifsc/`)
///
/// # Returns
/// A HashMap-like Vec where each entry contains the chip family and its peripheral list.
pub fn load_all_rifsc_chips(
    rif_dir: &Path,
) -> Result<Vec<ChipPeripherals>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    if !rif_dir.is_dir() {
        return Err(format!("Directory not found: {}", rif_dir.display()).into());
    }

    for entry in fs::read_dir(rif_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "yaml") {
            let content = fs::read_to_string(&path)?;
            let config: RifConfig = serde_yaml::from_str(&content)?;
            let peripherals = config.peripherals.iter().map(|p| p.name.clone()).collect();
            results.push((config.chip_family, peripherals));
        }
    }

    results.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_load_rifsc_peripherals_from_yaml() -> Result<(), Box<dyn std::error::Error>> {
        let test_yaml = r#"chip_family: stm32mp25xxxx
rifsc_id_max: 127
all_variants:
  - STM32MP251Axx
  - STM32MP251Cxx
peripherals:
  - name: tim1
    rifsc_id: 0
  - name: tim2
    rifsc_id: 1
  - name: spi1
    rifsc_id: 22
  - name: uart1
    rifsc_id: 50
"#;

        let temp_dir = TempDir::new()?;
        let yaml_path = temp_dir.path().join("test.yaml");
        fs::write(&yaml_path, test_yaml)?;

        let peripherals = load_rifsc_peripherals(&yaml_path)?;

        assert_eq!(peripherals.len(), 4);
        assert!(peripherals.contains(&"tim1".to_string()));
        assert!(peripherals.contains(&"tim2".to_string()));
        assert!(peripherals.contains(&"spi1".to_string()));
        assert!(peripherals.contains(&"uart1".to_string()));

        Ok(())
    }

    #[test]
    fn test_load_all_rifsc_chips_from_directory() -> Result<(), Box<dyn std::error::Error>> {
        let yaml1 = r#"chip_family: stm32mp25xxxx
rifsc_id_max: 127
all_variants:
  - STM32MP251Axx
peripherals:
  - name: tim1
    rifsc_id: 0
  - name: spi1
    rifsc_id: 22
"#;

        let yaml2 = r#"chip_family: stm32mp21xxxx
rifsc_id_max: 127
all_variants:
  - STM32MP211Axx
peripherals:
  - name: tim1
    rifsc_id: 0
  - name: uart1
    rifsc_id: 50
"#;

        let temp_dir = TempDir::new()?;
        fs::write(temp_dir.path().join("stm32mp25xxxx.yaml"), yaml1)?;
        fs::write(temp_dir.path().join("stm32mp21xxxx.yaml"), yaml2)?;

        let chips = load_all_rifsc_chips(temp_dir.path())?;

        assert_eq!(chips.len(), 2);

        // Check sorted order
        assert_eq!(chips[0].0, "stm32mp21xxxx");
        assert_eq!(chips[1].0, "stm32mp25xxxx");

        // Check stm32mp21xxxx peripherals
        assert_eq!(chips[0].1.len(), 2);
        assert!(chips[0].1.contains(&"tim1".to_string()));
        assert!(chips[0].1.contains(&"uart1".to_string()));

        // Check stm32mp25xxxx peripherals
        assert_eq!(chips[1].1.len(), 2);
        assert!(chips[1].1.contains(&"tim1".to_string()));
        assert!(chips[1].1.contains(&"spi1".to_string()));

        Ok(())
    }

    #[test]
    fn test_load_from_real_rif_directory() -> Result<(), Box<dyn std::error::Error>> {
        let rif_dir = PathBuf::from("../../data/rif/rifsc");

        // Only run if the directory exists (relative to test execution)
        if !rif_dir.exists() {
            println!(
                "Skipping real file test - RIF directory not found at {}",
                rif_dir.display()
            );
            return Ok(());
        }

        let chips = load_all_rifsc_chips(&rif_dir)?;

        assert!(
            !chips.is_empty(),
            "Should find at least one RIFSC YAML file"
        );

        // Check that we have peripherals for each chip
        for (chip, peripherals) in chips {
            assert!(
                !peripherals.is_empty(),
                "Chip {} should have peripherals",
                chip
            );
            println!("{}: {} peripherals", chip, peripherals.len());
        }

        Ok(())
    }
}
