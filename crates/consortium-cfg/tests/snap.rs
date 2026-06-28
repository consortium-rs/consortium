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

//! Snapshot coverage for the `components` code generators. Generated token
//! streams are reparsed and pretty-printed so the snapshots stay readable and
//! the test fails if the emitted code stops being valid Rust.

use std::collections::HashMap;

use consortium_cfg::components::address::{Address, ApAddress};
use consortium_cfg::components::bella::{Bella, HardwareDoorbell};
use consortium_cfg::components::ipc_shm::{
    BellaMeta, BellaRingerSide, Channel, Direction, Transceiver,
};
use consortium_cfg::components::region::Region;
use consortium_cfg_common::config::{Codec, McoreName, VisibleCore};
use consortium_cfg_common::database::{HardwareDoorbellPeripheralType, MemoryRegionType};
use proc_macro2::TokenStream;
use quote::format_ident;

/// Pretty-print a fragment of statements by wrapping it in a function body, so
/// `prettyplease` can format it and we assert the output is valid Rust.
fn stmts(tokens: TokenStream) -> String {
    let wrapped: TokenStream = quote::quote! { fn _generated() { #tokens } };
    let file = syn::parse_file(&wrapped.to_string()).expect("generated code should parse as Rust");
    prettyplease::unparse(&file)
}

/// Pretty-print a fragment that is a single expression.
fn expr(tokens: TokenStream) -> String {
    let wrapped: TokenStream = quote::quote! { fn _generated() { let _ = #tokens; } };
    let file = syn::parse_file(&wrapped.to_string()).expect("generated code should parse as Rust");
    prettyplease::unparse(&file)
}

fn cm33() -> VisibleCore {
    VisibleCore::Mcore(McoreName::Cm33)
}

/// An address mapped both through an AP UIO window and a controller-core
/// physical pointer, so both `memory_lookup` arms can be exercised.
fn shared_address(mmap_id: usize, cp_addr: u32) -> Address {
    Address {
        ap: ApAddress { uio_id: 0, mmap_id },
        cp: HashMap::from([(cm33(), cp_addr)]),
    }
}

fn transceiver() -> Transceiver {
    Transceiver {
        name: "ping".to_string(),
        doorbell_address: shared_address(2, 0x4400_0000),
        mtu: 256,
        codec: Codec::Postcard,
        region: Region {
            r#type: MemoryRegionType::Sram,
            address: shared_address(1, 0x3400_0000),
            length: 0x8000,
        },
        channel_primary: (
            VisibleCore::Ap,
            Channel {
                doorbell_chan: 0,
                type_name: "Ping".to_string(),
                buffer_name: "ping_tx_buf".to_string(),
            },
        ),
        channel_secondary: (
            cm33(),
            Channel {
                doorbell_chan: 1,
                type_name: "Pong".to_string(),
                buffer_name: "ping_rx_buf".to_string(),
            },
        ),
        bella: BellaMeta {
            r#type: HardwareDoorbellPeripheralType::Hsem,
            id: None,
        },
    }
}

#[test]
fn transceiver_primary_side_binds_uio_notifier() {
    let uio = format_ident!("uio");
    let tokens = transceiver().initialize(&VisibleCore::Ap, &uio, BellaRingerSide::Primary);
    insta::assert_snapshot!(stmts(tokens));
}

#[test]
fn transceiver_secondary_side_uses_controller_addresses() {
    let uio = format_ident!("uio");
    let tokens = transceiver().initialize(&cm33(), &uio, BellaRingerSide::Secondary);
    insta::assert_snapshot!(stmts(tokens));
}

#[test]
fn region_initialize_sram_ap() {
    let uio = format_ident!("uio");
    let tokens = transceiver().region.initialize(&VisibleCore::Ap, &uio);
    insta::assert_snapshot!(expr(tokens));
}

#[test]
fn channel_initialize_tx() {
    let chan = format_ident!("chan_tx");
    let transport = format_ident!("transport_tx");
    let tokens = transceiver().channel_primary.1.initialize(
        Direction::Tx,
        &chan,
        &transport,
        Codec::Postcard,
    );
    insta::assert_snapshot!(expr(tokens));
}

#[test]
fn bella_nvic_enable_for_controller_core() {
    let bella = Bella {
        bells: vec![
            HardwareDoorbell {
                name: "hsem".to_string(),
                address: shared_address(0, 0),
                r#type: HardwareDoorbellPeripheralType::Hsem,
                channels: vec![0],
                irq: HashMap::from([(cm33(), 199)]),
            },
            HardwareDoorbell {
                name: "ipcc2".to_string(),
                address: shared_address(0, 0),
                r#type: HardwareDoorbellPeripheralType::Ipcc,
                channels: vec![1],
                irq: HashMap::from([(cm33(), 13)]),
            },
        ],
    };

    let tokens = bella
        .nvic_enable(&cm33())
        .expect("controller core has irqs");
    insta::assert_snapshot!(stmts(tokens));
}
