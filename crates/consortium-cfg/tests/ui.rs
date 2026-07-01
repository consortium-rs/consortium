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

//! `Consortium.toml` -> generated init code -> `trybuild` compile check.
//!
//! Where [`conf.rs`](./conf.rs) snapshots the *text* of the generated code,
//! this test proves it actually **type-checks** against the real runtime crates.
//! For every `tests/configs/<name>/Consortium.toml`, each participating core's
//! [`LoweredIr::initialize`] output is written to a standalone `tests/ui/` crate
//! and compiled with `trybuild`:
//!
//! - `tests/ui/portable/<config>__<core>.rs`: controller cores (M/R). These name
//!   only `no_std`-friendly crates (`consortium-ipc`, `-transport-memory`,
//!   `-doorbell-*`, `-shared-memory`, `-codec`) and compile on any host.
//! - `tests/ui/linux/<config>__ap.rs`: the AP/Linux side, which additionally
//!   names `consortium-runtime-app` (`#![cfg(target_os = "linux")]`). These are
//!   emitted and compiled only on Linux.
//!
//! The generated `::shared::<Msg>` message types do not exist in any crate, so
//! each UI file embeds a `mod shared` with one stub per referenced type, deriving
//! exactly the traits its channel's codec requires (`serde` for `postcard`,
//! `rkyv` for `rkyv`). The codegen's `ty_crate` is pointed at `crate::shared`.
//!
//! The files are regenerated on every run, so they always track the codegen.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use consortium_cfg::Session;
use consortium_cfg::bridge::LoweredIr;
use consortium_cfg_common::config::{Codec, VisibleCore};
use proc_macro2::TokenStream;

/// The codegen `ty_crate`: generated message types resolve to the file-local
/// `mod shared` (see [`shared_module`]).
fn ty_crate() -> syn::Path {
    syn::parse_str("crate::shared").unwrap()
}

/// Pretty-prints generated *items* (struct/impl/enum/fn) as a Rust source
/// fragment, failing if the codegen stops being parseable.
fn render_items(tokens: TokenStream) -> String {
    consortium_cfg::render(tokens).expect("generated code should parse as Rust")
}

/// Every distinct core that participates in any transceiver, AP first then
/// controller cores sorted by name (mirrors `conf.rs`).
fn cores(ir: &LoweredIr) -> Vec<VisibleCore> {
    let mut seen: BTreeMap<String, VisibleCore> = BTreeMap::new();
    let mut has_ap = false;
    for t in ir.shuhan.transceivers() {
        for (core, _) in [&t.channel_primary, &t.channel_secondary] {
            if *core == VisibleCore::Ap {
                has_ap = true;
            } else {
                seen.insert(core.to_string(), *core);
            }
        }
    }

    let mut out = Vec::new();
    if has_ap {
        out.push(VisibleCore::Ap);
    }
    out.extend(seen.into_values());
    out
}

/// Maps each referenced message type name to the codec of the channel that
/// carries it, across every transceiver in the manifest.
fn message_types(ir: &LoweredIr) -> BTreeMap<String, Codec> {
    let mut types = BTreeMap::new();
    for t in ir.shuhan.transceivers() {
        types.insert(t.channel_primary.1.type_name.clone(), t.codec);
        types.insert(t.channel_secondary.1.type_name.clone(), t.codec);
    }
    types
}

/// A `mod shared` defining a stub for every referenced message type, each
/// deriving the traits its codec's `CodecFor` impl requires.
fn shared_module(types: &BTreeMap<String, Codec>) -> String {
    let mut out = String::from("mod shared {\n");
    for (name, codec) in types {
        let derives = match codec {
            // `PostcardCodec: CodecFor<T>` needs `T: Serialize + DeserializeOwned`.
            Codec::Postcard => "#[derive(::serde::Serialize, ::serde::Deserialize)]",
            // `RkyvCodec: CodecFor<T>` (alloc path) needs `T: Archive + Serialize`
            // with `T::Archived: CheckBytes`, all produced by the rkyv derives
            // (bytecheck emits `CheckBytes` for the archived type).
            Codec::Rkyv => "#[derive(::rkyv::Archive, ::rkyv::Serialize)]",
            Codec::Prost => panic!(
                "trybuild stub for the prost codec is not implemented (type `{name}`); \
                 add a `prost::Message` stub and the `prost` codec feature if a fixture needs it"
            ),
        };
        writeln!(out, "    {derives}").unwrap();
        writeln!(
            out,
            "    pub struct {name} {{\n        pub _v: u32,\n    }}"
        )
        .unwrap();
    }
    out.push_str("}\n");
    out
}

/// The full standalone UI crate source for one core of one manifest.
fn ui_file(ir: &LoweredIr, core: VisibleCore, types: &BTreeMap<String, Codec>) -> String {
    let mut file = String::new();
    // The generated code is never executed here; silence the resulting dead-code
    // and unused-unsafe noise so `trybuild` only fails on real type errors.
    file.push_str("#![allow(dead_code, unused, unused_unsafe, non_snake_case)]\n\n");
    file.push_str(&shared_module(types));
    file.push('\n');
    file.push_str(&render_items(ir.initialize(core, &ty_crate())));
    file.push_str("\nfn main() {}\n");
    file
}

/// The `tests` directory next to this file.
fn tests_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")
}

/// Removes any previously generated `*.rs` from `dir` (creating it if absent),
/// so stale cores/configs never linger.
fn reset_dir(dir: &Path) {
    fs::create_dir_all(dir).expect("create ui dir");
    for entry in fs::read_dir(dir).expect("read ui dir") {
        let path = entry.expect("dir entry").path();
        if path.extension().is_some_and(|e| e == "rs") {
            fs::remove_file(&path).expect("remove stale ui file");
        }
    }
}

#[test]
fn generated_init_code_compiles() {
    let tests = tests_dir();
    let configs = tests.join("configs");

    let portable = tests.join("ui").join("portable");
    reset_dir(&portable);
    #[cfg(target_os = "linux")]
    let linux = tests.join("ui").join("linux");
    #[cfg(target_os = "linux")]
    reset_dir(&linux);

    let mut entries: Vec<_> = fs::read_dir(&configs)
        .expect("configs dir exists")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.is_dir())
        .collect();
    entries.sort();
    assert!(!entries.is_empty(), "no config fixtures under {configs:?}");

    for config in entries {
        let name = config
            .file_name()
            .expect("config dir name")
            .to_str()
            .expect("utf-8 config name")
            .to_string();
        let manifest = config.join("Consortium.toml");
        let toml = fs::read_to_string(&manifest)
            .unwrap_or_else(|e| panic!("read {}: {e}", manifest.display()));

        let session = Session::from_toml(&toml)
            .unwrap_or_else(|e| panic!("parse {}: {e}", manifest.display()));
        let mut ir = session
            .lower()
            .unwrap_or_else(|e| panic!("lower {}: {e}", manifest.display()));

        // The AP codegen `include_bytes!`es each dbg ring's firmware ELF. The
        // real path is a Stage-1 build output the builder substitutes; here we
        // point every ring at a committed placeholder ELF so the generated
        // `include_bytes!` resolves against a file that exists relative to the
        // emitted `tests/ui/linux/*.rs` crate.
        let fixture = PathBuf::from("../../fixtures/placeholder.elf");
        let resolved: HashMap<VisibleCore, PathBuf> =
            ir.dbg.rings().keys().map(|core| (*core, fixture.clone())).collect();
        ir.dbg = std::mem::take(&mut ir.dbg).with_resolved_elf_paths(&resolved);

        let types = message_types(&ir);

        for core in cores(&ir) {
            let source = ui_file(&ir, core, &types);
            let file_name = format!("{name}__{core}.rs");
            if core == VisibleCore::Ap {
                // AP code names the Linux-only `consortium-runtime-app`; only
                // emit (and below, only compile) it on Linux.
                #[cfg(target_os = "linux")]
                fs::write(linux.join(&file_name), source).expect("write linux ui file");
            } else {
                fs::write(portable.join(&file_name), source).expect("write portable ui file");
            }
        }
    }

    let t = trybuild::TestCases::new();
    t.pass("tests/ui/portable/*.rs");
    #[cfg(target_os = "linux")]
    t.pass("tests/ui/linux/*.rs");
}
