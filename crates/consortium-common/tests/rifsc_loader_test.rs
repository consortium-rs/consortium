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

#![cfg(all(feature = "serde", feature = "std"))]

use consortium_common::config::{load_all_rifsc_chips, load_rifsc_peripherals};
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

    println!("✓ Loaded {} peripherals from YAML", peripherals.len());
    println!("  Peripherals: {:?}", peripherals);

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

    println!("✓ Loaded {} chips with peripherals", chips.len());
    for (chip, peripherals) in &chips {
        println!("  {}: {} peripherals", chip, peripherals.len());
    }

    Ok(())
}

#[test]
fn test_load_from_real_rif_directory() -> Result<(), Box<dyn std::error::Error>> {
    // Try multiple possible paths
    let possible_paths = vec![
        PathBuf::from("data/rif/rifsc"),
        PathBuf::from("../../data/rif/rifsc"),
        PathBuf::from("../../../data/rif/rifsc"),
    ];

    let mut rif_dir = None;
    for path in possible_paths {
        if path.exists() {
            rif_dir = Some(path);
            break;
        }
    }

    let rif_dir = match rif_dir {
        Some(dir) => dir,
        None => {
            println!("⊘ Skipping real file test - RIF directory not found");
            return Ok(());
        }
    };

    let chips = load_all_rifsc_chips(&rif_dir)?;

    assert!(
        !chips.is_empty(),
        "Should find at least one RIFSC YAML file"
    );

    println!("✓ Loaded {} chips from real RIF directory", chips.len());
    for (chip, peripherals) in &chips {
        println!("  {}: {} peripherals", chip, peripherals.len());
        // Print first 5 peripherals as sample
        for (i, p) in peripherals.iter().take(5).enumerate() {
            println!("    {}. {}", i + 1, p);
        }
        if peripherals.len() > 5 {
            println!("    ... and {} more", peripherals.len() - 5);
        }
    }

    Ok(())
}
