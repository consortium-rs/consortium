use consortium_dts::{Cell, File, Node, NodeOverlay, Property, ToDts, parse_dts};

#[test]
fn builds_linux_and_optee_style_fragments() {
    let file = File::dtsi()
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
        .item(NodeOverlay::label("i2c5").property(Property::string("secure-status", "okay")));

    assert_eq!(
        file.to_dts(),
        "/dts-v1/;\n\n&uart4 {\n    status = \"okay\";\n\n    pins {\n        pinmux = <STM32_PINMUX('A', 5, AF1)>;\n        bias-disable;\n    };\n};\n\n&i2c5 {\n    secure-status = \"okay\";\n};\n"
    );
}

#[test]
fn generated_fragments_round_trip_through_parser() {
    let dts = File::dtsi()
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
        .to_dts();

    assert_eq!(parse_dts(&dts).unwrap().to_dts(), dts);
}

#[test]
fn dtso_fragments_round_trip_through_parser() {
    let dts = File::dtso()
        .item(NodeOverlay::label("uart4").property(Property::string("status", "okay")))
        .to_dts();

    assert_eq!(parse_dts(&dts).unwrap().to_dts(), dts);
}
