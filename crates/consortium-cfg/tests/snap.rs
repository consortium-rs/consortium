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
use consortium_cfg::components::bella::{BellA, HardwareDoorbell};
use consortium_cfg::components::ipc_shm::{
    Channel, DoorbellMeta, DoorbellSource, ShuHan, Transceiver, initialize_instances,
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
    consortium_cfg::render(wrapped).expect("generated code should parse as Rust")
}

/// Pretty-print a fragment that is a single expression.
fn expr(tokens: TokenStream) -> String {
    let wrapped: TokenStream = quote::quote! { fn _generated() { let _ = #tokens; } };
    consortium_cfg::render(wrapped).expect("generated code should parse as Rust")
}

/// Pretty-print a fragment that is a sequence of items (e.g. struct + impl).
fn items(tokens: TokenStream) -> String {
    consortium_cfg::render(tokens).expect("generated code should parse as Rust")
}

fn cm33() -> VisibleCore {
    VisibleCore::Mcore(McoreName::Cm33)
}

fn cm7() -> VisibleCore {
    VisibleCore::Mcore(McoreName::Cm7)
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
            cache: false,
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
        doorbell: DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Hsem,
            id: None,
        },
    }
}

#[test]
fn transceiver_primary_side_binds_uio_notifier() {
    let uio = format_ident!("uio");
    // The build phase resolves the AP region/doorbell and binds the UIO notifier;
    // it evaluates to the unsplit transport (handshake deferred to `connect`).
    let tokens = transceiver().build_field_init(
        VisibleCore::Ap,
        &uio,
        DoorbellSource::Uio,
        "::consortium_hal_stm32mp2",
    );
    insta::assert_snapshot!(expr(tokens));
}

#[test]
fn transceiver_secondary_side_uses_controller_addresses() {
    let uio = format_ident!("uio");
    // A controller core takes the doorbell's peripheral singleton rather than a
    // mapped base; `__db_hsem` is the parameter the endpoint installer binds.
    let tokens = transceiver().build_field_init(
        cm33(),
        &uio,
        DoorbellSource::Owned(format_ident!("__db_hsem")),
        "::consortium_hal_stm32mp2",
    );
    insta::assert_snapshot!(expr(tokens));
}

#[test]
fn region_initialize_uncached_ap() {
    let uio = format_ident!("uio");
    let tokens = transceiver().region.initialize(VisibleCore::Ap, &uio);
    insta::assert_snapshot!(expr(tokens));
}

#[test]
fn bella_nvic_enable_for_controller_core() {
    let bella = BellA {
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
        .nvic_enable(&cm33(), "::consortium_hal_stm32mp2")
        .expect("controller core has irqs");
    insta::assert_snapshot!(stmts(tokens));
}

/// A second transceiver on a distinct doorbell, core, and codec, so a
/// multi-endpoint `ShuHan` exercises more than one of every generated arm.
fn pong() -> Transceiver {
    Transceiver {
        name: "pong".to_string(),
        doorbell_address: Address {
            ap: ApAddress {
                uio_id: 2,
                mmap_id: 0,
            },
            cp: HashMap::from([(cm7(), 0x4500_0000)]),
        },
        mtu: 512,
        codec: Codec::Rkyv,
        region: Region {
            r#type: MemoryRegionType::Sram,
            cache: false,
            address: Address {
                ap: ApAddress {
                    uio_id: 0,
                    mmap_id: 1,
                },
                cp: HashMap::from([(cm7(), 0x3404_0000)]),
            },
            length: 0x4000,
        },
        channel_primary: (
            VisibleCore::Ap,
            Channel {
                doorbell_chan: 2,
                type_name: "Telemetry".to_string(),
                buffer_name: "pong_tx_buf".to_string(),
            },
        ),
        channel_secondary: (
            cm7(),
            Channel {
                doorbell_chan: 3,
                type_name: "Command".to_string(),
                buffer_name: "pong_rx_buf".to_string(),
            },
        ),
        doorbell: DoorbellMeta {
            r#type: HardwareDoorbellPeripheralType::Ipcc,
            id: Some(2),
        },
    }
}

/// A multi-transceiver node spanning two doorbell families, two codecs, and two
/// controller cores. The full channel definitions + endpoint installers are the
/// most representative "complicated IR -> code" surface.
fn shuhan() -> ShuHan {
    ShuHan::new(vec![transceiver(), pong()])
}

#[test]
fn shuhan_ap_side_emits_full_endpoint_struct() {
    let uio = format_ident!("uio");
    let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
    let tokens = initialize_instances(
        &shuhan(),
        VisibleCore::Ap,
        &uio,
        &ty_crate,
        "::consortium_hal_stm32mp2",
        &[],
    );
    insta::assert_snapshot!(items(tokens));
}

#[test]
fn shuhan_controller_side_emits_single_endpoint_struct() {
    // A controller core only participates in the transceivers it is a party to,
    // but `ShuHan::initialize` still emits the whole node; the snapshot shows the
    // cm33 view (it is only a party to `ping`, so `pong`'s cm7 addresses are not
    // resolvable for this core).
    let uio = format_ident!("uio");
    let ty_crate: syn::Path = syn::parse_str("::shared").unwrap();
    let tokens = initialize_instances(
        &ShuHan::new(vec![transceiver()]),
        cm33(),
        &uio,
        &ty_crate,
        "::consortium_hal_stm32mp2",
        // The cm33 chip map reaches HSEM, so its doorbell takes the singleton.
        &["HSEM".to_string()],
    );
    insta::assert_snapshot!(items(tokens));
}
