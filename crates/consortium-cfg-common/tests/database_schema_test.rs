use consortium_cfg_common::config::VisibleCore;
use consortium_cfg_common::database::{ChipDatabase, HardwareDoorbellPeripheralType};
use std::fs;
use std::path::PathBuf;

#[test]
fn loads_all_database_yaml_files() {
    for path in database_yaml_files() {
        let yaml = fs::read_to_string(&path).expect("database yaml should be readable");
        let _: ChipDatabase = serde_yaml::from_str(&yaml)
            .unwrap_or_else(|err| panic!("{} should match Rust schema: {err}", path.display()));
    }
}

#[test]
fn loads_stm32mp21_database_yaml() {
    let database: ChipDatabase = serde_yaml::from_str(include_str!(
        "../../consortium-data/database/memory/stm32mp21.yaml"
    ))
    .expect("stm32mp21 database should match Rust schema");

    assert_eq!(database.name, "STM32MP21");
    assert_eq!(
        database.memory.regions[0].visible,
        vec![VisibleCore::Ap, "cm33".parse().unwrap()]
    );
    assert_eq!(
        database.peripheral.doorbells[0].r#type,
        HardwareDoorbellPeripheralType::Ipcc
    );
}

#[test]
fn loads_stm32mp23_database_yaml() {
    let database: ChipDatabase = serde_yaml::from_str(include_str!(
        "../../consortium-data/database/memory/stm32mp23.yaml"
    ))
    .expect("stm32mp23 database should match Rust schema");

    assert_eq!(database.name, "STM32MP23");
    assert_eq!(database.peripheral.doorbells.len(), 3);
    assert_eq!(database.memory.trm_page, vec![229]);
    assert_eq!(database.peripheral.doorbells[0].memory.base, 0x0624_0000);
    assert_eq!(database.peripheral.doorbells[0].page, 1306);
    assert_eq!(database.peripheral.doorbells[0].party_page, 1306);
    assert_eq!(database.peripheral.doorbells[0].channels, 16);
    assert_eq!(database.peripheral.doorbells[0].channel_page, 1306);
}

#[test]
fn loads_stm32mp25_database_yaml() {
    let database: ChipDatabase = serde_yaml::from_str(include_str!(
        "../../consortium-data/database/memory/stm32mp25.yaml"
    ))
    .expect("stm32mp25 database should match Rust schema");

    assert_eq!(database.name, "STM32MP25");
    assert_eq!(database.cp.len(), 2);
    assert!(
        database
            .memory
            .regions
            .iter()
            .any(|region| region.visible.contains(&"cm0p".parse().unwrap()))
    );
}

fn database_yaml_files() -> Vec<PathBuf> {
    let database_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cfg-common crate should live under crates/")
        .join("consortium-data/database/memory");

    let mut paths = fs::read_dir(database_dir)
        .expect("database directory should be readable")
        .map(|entry| entry.expect("database entry should be readable").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yaml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}
