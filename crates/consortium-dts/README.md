# consortium-dts

Device Tree Source AST construction and deterministic code generation for
Consortium-generated DTS, DTSI, and DTSO fragments.

This crate is the DTS language-tooling layer. It owns syntax-shaped AST values,
ergonomic builders, and stable text output. Higher-level crates such as
`consortium-cfg` should continue to own resource policy: peripheral ownership,
secure/non-secure assignment, RIFSC/TRDC mapping, and pin validation. Those crates
can use `consortium-dts` to construct and print the resulting DTS fragments.

## Current Status

The first implementation milestone supports builder-driven AST construction and
code generation only. It does not parse existing DTS source and does not resolve
includes, labels, phandles, or overlays.

Unknown or not-yet-modeled DTS expressions can be emitted with raw values while
the parser and resolver mature.

## Example

```rust
use consortium_dts::{File, NodeOverlay, Property, ToDts};

let file = File::dtsi().item(
    NodeOverlay::label("uart4")
        .property(Property::string("status", "okay")),
);

assert_eq!(file.to_dts(), "/dts-v1/;\n\n&uart4 {\n    status = \"okay\";\n};\n");
```

## Future Work

- Parser and lexer for DTS/DTSI/DTSO source files.
- Source spans and diagnostics.
- Include and label resolution.
- DTSO operation list and diff printer.
- Optional `dtc` validation in `consortium-builder` or `consortium-cli`.
