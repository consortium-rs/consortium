use optee_utee_build::{Error, TaConfig};

fn main() -> Result<(), Error> {
    let ta_config = TaConfig::new_default_with_cargo_env("197c710c-bdab-11eb-8f3f-17a5f698d23b")?;
    optee_utee_build::build(optee_utee_build::RustEdition::Edition2024, ta_config)
}
