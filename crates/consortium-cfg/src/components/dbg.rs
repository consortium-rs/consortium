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

//! Debug-logging IR: one defmt-over-shared-memory ring per coprocessor.
//!
//! Unlike the IPC ([`ShuHan`](crate::components::ipc_shm::ShuHan)) and doorbell
//! ([`BellA`](crate::components::bella::BellA)) IR, this component generates no
//! Rust. The debug log ring is configured at *build* time, not in generated
//! source: the builder injects each logger's resolved layout as the
//! `CONSORTIUM_DBG_*` environment variables that `consortium_dbg::config` reads
//! with `option_env!`. This IR resolves those values per logging core so the
//! builder does not re-derive addresses from the manifest.

use std::collections::HashMap;

use consortium_cfg_common::config::{DbgMode, VisibleCore};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

/// One coprocessor's debug log ring, resolved to the address its firmware embeds
/// at compile time plus the host-side mapping the AP uses to read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebugRing {
    /// Physical base of the ring as seen from the logging core's bus, i.e. the
    /// manifest base plus the chip's SRAM alias for that core's world.
    pub base: u64,
    /// Ring length in bytes.
    pub length: u64,
    /// Maximum transfer unit for debug frames.
    pub mtu: u32,
    /// UIO device index the AP maps this ring through (the aggregate `dbg-shm`
    /// device).
    pub ap_uio_id: usize,
    /// mmap window index of this ring within that device.
    pub ap_mmap_id: usize,
    /// Path to the logging core's firmware artifact (its `[endpoints]` value),
    /// from which the host resolves the defmt symbol table.
    pub elf_path: String,
}

impl DebugRing {
    /// The `CONSORTIUM_DBG_*` build-time variables a build of this core's
    /// firmware reads through `consortium_dbg::config`.
    ///
    /// Only `BASE` and `LENGTH` are emitted, matching what `config.rs` parses;
    /// values are decimal to match the other `CONSORTIUM_*` injections.
    pub fn env(&self) -> Vec<(&'static str, String)> {
        vec![
            ("CONSORTIUM_DBG_BASE", self.base.to_string()),
            ("CONSORTIUM_DBG_LENGTH", self.length.to_string()),
        ]
    }
}

/// Every coprocessor that declares a `[dbg.<core>]` ring, keyed by logging core,
/// plus the host consumption [`DbgMode`] selected by `[dbg] mode`.
#[derive(Debug, Clone, Default)]
pub struct DebugLogs {
    rings: HashMap<VisibleCore, DebugRing>,
    mode: DbgMode,
}

impl DebugLogs {
    /// Builds the IR from a resolved per-core ring map and the host mode.
    pub fn new(rings: HashMap<VisibleCore, DebugRing>, mode: DbgMode) -> Self {
        Self { rings, mode }
    }

    /// Every resolved logger, keyed by the coprocessor it runs on.
    pub fn rings(&self) -> &HashMap<VisibleCore, DebugRing> {
        &self.rings
    }

    /// The host consumption mode for the rings.
    pub fn mode(&self) -> DbgMode {
        self.mode
    }

    /// The ring configured for `core`, or `None` when it has no debug logging.
    pub fn ring(&self, core: VisibleCore) -> Option<DebugRing> {
        self.rings.get(&core).cloned()
    }

    /// Generates the AP-side host console code, or `None` when no coprocessor
    /// declares a debug ring.
    ///
    /// The returned [`HostCodegen`] carries the `ConsortiumEndpoints` field
    /// (declared *before* `uio` so its threads/handles drop before the mappings
    /// they read are unmapped) and the `init()` statement that brings it up. In
    /// [`DbgMode::Auto`] this spawns a forwarding console; in
    /// [`DbgMode::Interface`] it returns reader handles for the integrator.
    pub fn initialize_host(&self, uio_ident: &Ident) -> Option<HostCodegen> {
        if self.rings.is_empty() {
            return None;
        }

        // Deterministic order: sort by core name so the generated specs and the
        // dbg-shm mmap windows agree run to run.
        let mut rings: Vec<(&VisibleCore, &DebugRing)> = self.rings.iter().collect();
        rings.sort_by_key(|(core, _)| core.to_string());

        let specs = rings.iter().map(|(core, ring)| {
            let uio_id = Literal::usize_unsuffixed(ring.ap_uio_id);
            let mmap_id = Literal::usize_unsuffixed(ring.ap_mmap_id);
            let len = Literal::usize_unsuffixed(ring.length as usize);
            let elf_path = &ring.elf_path;
            let label = core.to_string();
            quote! {
                ::consortium_runtime_app::dbg_console::DbgRingSpec {
                    uio_id: #uio_id,
                    mmap_id: #mmap_id,
                    len: #len,
                    elf_path: #elf_path,
                    label: #label,
                }
            }
        });
        let specs = quote! { &[ #(#specs),* ] };

        let field_name = format_ident!("_consortium_dbg");
        let (field, init) = match self.mode {
            DbgMode::Auto => (
                quote! {
                    #field_name: ::consortium_runtime_app::dbg_console::DbgConsole
                },
                quote! {
                    let #field_name =
                        ::consortium_runtime_app::dbg_console::spawn(&#uio_ident, #specs)?;
                },
            ),
            DbgMode::Interface => (
                quote! {
                    #field_name: ::std::vec::Vec<
                        ::consortium_runtime_app::dbg_console::DbgRingHandle
                    >
                },
                quote! {
                    let #field_name =
                        ::consortium_runtime_app::dbg_console::open(&#uio_ident, #specs)?;
                },
            ),
        };

        Some(HostCodegen {
            field,
            field_name,
            init,
        })
    }

    /// Per-core `CONSORTIUM_DBG_*` build-time variables, keyed by logging core.
    ///
    /// The builder injects these into the matching controller-core firmware
    /// image; a core absent from the map has no debug logging configured.
    pub fn env(&self) -> HashMap<VisibleCore, Vec<(&'static str, String)>> {
        self.rings
            .iter()
            .map(|(core, ring)| (*core, ring.env()))
            .collect()
    }
}

/// The fragments [`DebugLogs::initialize_host`] contributes to the AP's
/// generated `ConsortiumEndpoints` and `init()`.
pub struct HostCodegen {
    /// `ConsortiumEndpoints` field declaration (`name: Type`).
    pub field: TokenStream,
    /// The field's identifier, for the struct literal in `init()`.
    pub field_name: Ident,
    /// The `let <field_name> = …;` statement that brings the console up.
    pub init: TokenStream,
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::McoreName;

    fn cm33() -> VisibleCore {
        VisibleCore::Mcore(McoreName::Cm33)
    }

    fn ring() -> DebugRing {
        DebugRing {
            base: 0x2040_8000,
            length: 0x2000,
            mtu: 64,
            ap_uio_id: 3,
            ap_mmap_id: 0,
            elf_path: "./firmware".to_string(),
        }
    }

    #[test]
    fn ring_emits_decimal_base_and_length_env() {
        let env: HashMap<_, _> = ring().env().into_iter().collect();
        assert_eq!(env["CONSORTIUM_DBG_BASE"], "541097984");
        assert_eq!(env["CONSORTIUM_DBG_LENGTH"], "8192");
    }

    #[test]
    fn logs_key_env_by_core() {
        let logs = DebugLogs::new(HashMap::from([(cm33(), ring())]), DbgMode::Auto);

        assert!(logs.ring(cm33()).is_some());
        assert!(logs.ring(VisibleCore::Ap).is_none());

        let env = logs.env();
        let cm33_env: HashMap<_, _> = env[&cm33()].iter().cloned().collect();
        assert_eq!(cm33_env["CONSORTIUM_DBG_LENGTH"], "8192");
    }

    #[test]
    fn host_codegen_none_when_empty() {
        let logs = DebugLogs::new(HashMap::new(), DbgMode::Auto);
        let uio = format_ident!("uio");
        assert!(logs.initialize_host(&uio).is_none());
    }

    #[test]
    fn auto_mode_spawns_console_with_ring_spec() {
        let logs = DebugLogs::new(HashMap::from([(cm33(), ring())]), DbgMode::Auto);
        let uio = format_ident!("uio");
        let host = logs.initialize_host(&uio).expect("rings present");

        let init_str = host.init.to_string();
        assert!(init_str.contains(":: consortium_runtime_app :: dbg_console :: spawn"));
        assert!(init_str.contains("DbgRingSpec"));
        assert!(init_str.contains("\"cm33\""));
        assert!(init_str.contains("\"./firmware\""));
        assert!(host.field.to_string().contains("DbgConsole"));

        // The init statement parses as a valid Rust block.
        let init = &host.init;
        let wrapped = quote! { fn _f(uio: ()) -> ::core::result::Result<(), ()> { #init Ok(()) } };
        syn::parse_file(&wrapped.to_string()).expect("host init is valid Rust");
    }

    #[test]
    fn interface_mode_opens_reader_handles() {
        let logs = DebugLogs::new(HashMap::from([(cm33(), ring())]), DbgMode::Interface);
        let uio = format_ident!("uio");
        let host = logs.initialize_host(&uio).expect("rings present");

        assert!(
            host.init
                .to_string()
                .contains(":: consortium_runtime_app :: dbg_console :: open")
        );
        assert!(host.field.to_string().contains("DbgRingHandle"));
    }
}
