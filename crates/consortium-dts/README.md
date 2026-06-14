# consortium-dts

Device Tree Source AST construction and deterministic code generation for
Consortium-generated DTS, DTSI, and DTSO fragments.

This crate is the DTS language-tooling layer. It owns syntax-shaped AST values,
ergonomic builders, generated-fragment parsing, and stable text output. Higher-level
crates such as `consortium-cfg` should continue to own resource policy: peripheral
ownership, secure/non-secure assignment, RIFSC/TRDC mapping, and pin validation. Those
crates can use `consortium-dts` to construct, parse, and print the resulting DTS
fragments.

## Current Status

The current implementation supports builder-driven AST construction, deterministic
code generation, and parsing for generated DTS/DTSI/DTSO fragments. It does not
resolve include files, labels, phandles, or overlays.

Unknown or not-yet-modeled DTS expressions can be emitted with raw values while the
parser, resolver, and bundler mature.

## Example

```rust
use consortium_dts::{File, NodeOverlay, Property, ToDts, parse_dts};

let file = File::dtsi().item(
    NodeOverlay::label("uart4")
        .property(Property::string("status", "okay")),
);

assert_eq!(file.to_dts(), "/dts-v1/;\n\n&uart4 {\n    status = \"okay\";\n};\n");

let parsed = parse_dts(&file.to_dts()).unwrap();
assert_eq!(parsed.to_dts(), file.to_dts());
```

## Future Work

- Broader DTS parser coverage for hand-written board files.
- Richer source spans and diagnostics.
- Resolver:
  - resolve include paths relative to a source root.
  - parse included files.
  - build label and path side tables.
  - report duplicate labels and unresolved references.
  - keep parsed AST values immutable.
- Bundler:
  - flatten a `File` plus resolved includes into a single DTS output.
  - preserve deterministic preamble and tree ordering.
  - optionally support bundled DTSO output.
- DTSO operation list and diff printer.
- Optional `dtc` validation in `consortium-builder` or `consortium-cli`.
