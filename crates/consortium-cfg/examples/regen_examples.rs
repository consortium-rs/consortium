// Dev tool: regenerate the checked-in `consortium.gen.rs` for the melt-pot
// example app/mcu crates from their `Consortium.toml`, mirroring what
// `consortium-builder` does (`consortium_cfg::render(ir.initialize(core, ty))`).
//
// Run with: cargo run -p consortium-cfg --example regen_examples

use std::path::Path;

use consortium_cfg::Session;
use consortium_cfg_common::config::{Party, VisibleCore};

fn main() {
    for (manifest_dir, shared_ident) in [
        ("examples/melt-pot/stm32mp25", "melt_pot_shared"),
        ("examples/melt-pot/imx95", "melt_pot_shared"),
    ] {
        let dir = Path::new(manifest_dir);
        let toml = std::fs::read_to_string(dir.join("Consortium.toml")).expect("read manifest");
        let session = Session::from_toml(&toml).expect("parse/validate manifest");
        let ir = session.lower().expect("lower manifest");
        let ty_crate: syn::Path = syn::parse_str(&format!("::{shared_ident}")).unwrap();

        for (party, root) in &session.config.endpoints {
            // Only the app (non-secure A-core) and controller crates `include!`
            // the generated module; the secure/TA crate does not.
            let core = party.core();
            let is_app = matches!(party, Party::AcoreNonSecure);
            let is_mcu = matches!(core, VisibleCore::Mcore(_));
            if !(is_app || is_mcu) {
                continue;
            }
            let out = dir.join(root).join("src").join("consortium.gen.rs");
            if !out.parent().map(Path::exists).unwrap_or(false) {
                continue;
            }
            let rendered =
                consortium_cfg::render(ir.initialize(core, &ty_crate)).expect("render codegen");
            std::fs::write(&out, rendered).expect("write gen.rs");
            println!("wrote {}", out.display());
        }
    }
}
