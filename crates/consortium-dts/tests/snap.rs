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

use consortium_dts::{
    Cell, File, Include, MemReserve, Node, NodeOverlay, PropValue, Property, Reference, ToDts,
    parse_dts,
};

// ── builder output ────────────────────────────────────────────────────────────

#[test]
fn dtsi_overlay_with_child_node() {
    insta::assert_snapshot!(
        File::dtsi()
            .item(
                NodeOverlay::label("uart4")
                    .property(Property::string("status", "okay"))
                    .child(
                        Node::new("pins")
                            .property(Property::cells(
                                "pinmux",
                                [Cell::macro_call("STM32_PINMUX", ["'A'", "5", "AF1"])],
                            ))
                            .property(Property::boolean("bias-disable")),
                    ),
            )
            .to_dts()
    );
}

#[test]
fn dtso_path_overlay() {
    insta::assert_snapshot!(
        File::dtso()
            .item(
                NodeOverlay::path("/soc/serial@40010000")
                    .property(Property::string("status", "okay")),
            )
            .to_dts()
    );
}

#[test]
fn root_node_with_property_variants() {
    insta::assert_snapshot!(
        Node::root()
            .property(Property::strings(
                "compatible",
                ["test,board", "simple-bus"],
            ))
            .property(Property::cells(
                "reg",
                [Cell::number(0x4000_0000), Cell::number(0x1000)],
            ))
            .property(Property::bytes(
                "local-mac-address",
                [0xde, 0xad, 0xbe, 0xef, 0x00, 0x01],
            ))
            .property(Property::reference("clocks", Reference::label("rcc"),))
            .property(Property::compound(
                "ranges",
                [
                    PropValue::Cells(vec![Cell::number(0)]),
                    PropValue::Cells(vec![Cell::number(0x4000_0000)]),
                    PropValue::Cells(vec![Cell::number(0x1000)]),
                ],
            ))
            .property(Property::raw("interrupt-parent", "<&gic>"))
            .child(
                Node::with_unit_address("serial", "0x40010000")
                    .label("uart4")
                    .status_okay(),
            )
            .to_dts()
    );
}

#[test]
fn file_with_mem_reserve_and_includes() {
    insta::assert_snapshot!(
        File::dtsi()
            .mem_reserve(MemReserve::new("0x80000000", "0x1000"))
            .include(Include::quoted("board.dtsi"))
            .include(Include::angled("dt-bindings/gpio/gpio.h"))
            .item(NodeOverlay::label("uart4").status_okay())
            .to_dts()
    );
}

#[test]
fn file_with_delete_node() {
    insta::assert_snapshot!(
        File::dtsi()
            .delete_node(Reference::label("uart4"))
            .delete_node(Reference::path("/soc/serial@40010000"))
            .to_dts()
    );
}

#[test]
fn multiple_overlays_in_dtsi() {
    insta::assert_snapshot!(
        File::dtsi()
            .item(NodeOverlay::label("uart4").property(Property::string("status", "okay")))
            .item(NodeOverlay::label("i2c5").property(Property::string("secure-status", "okay")))
            .to_dts()
    );
}

// ── parse error display ───────────────────────────────────────────────────────

#[test]
fn parse_error_missing_semicolon() {
    let err = parse_dts("node { status = \"okay\" }").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}

#[test]
fn parse_error_invalid_byte() {
    let err = parse_dts("node { bytes = [zz]; };").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}

#[test]
fn parse_error_unexpected_eof_in_cell_list() {
    let err = parse_dts("node { reg = <1 2; };").unwrap_err();
    insta::assert_snapshot!(err.to_string());
}
