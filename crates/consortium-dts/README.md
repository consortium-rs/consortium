# consortium-dts

Device Tree Source AST construction, parsing, and deterministic code generation for
Consortium-generated DTS, DTSI, and DTSO fragments.

## Scope: language tooling, not policy

This crate owns syntax-shaped AST values, ergonomic builders, a lexer and parser, and
stable text output.

It owns no _policy_. Which peripheral belongs to which core, what is secure, how RIFSC
and TRDC map, whether a pin assignment is legal — all of that stays in
[`consortium-cfg`](../consortium-cfg), which uses this crate to construct, parse, and
print the resulting fragments. Keeping the split means a device-tree question and a
resource-ownership question never get answered in the same place.

## Modules

```text
  builder ──► ast ──► codegen ──► DTS text
               ▲                     │
               └──── parser ◄── lexer ┘
```

| Module    | Role                                                                    |
| --------- | ----------------------------------------------------------------------- |
| `ast`     | Owned, syntax-shaped values: `File`, `Node`, `Property`, `PropValue`, … |
| `builder` | Chainable constructors so codegen reads like the output.                |
| `codegen` | `ToDts` and `CodegenOptions` — deterministic text output.               |
| `lexer`   | Tokenizer, with spans for diagnostics.                                  |
| `parser`  | `parse_dts` — token stream to `File`.                                   |
| `error`   | `DtsError`, `LexError`, `ParseError`, `Span`.                           |
| `utils`   | Read-only analysis over a parsed file, plus `IrqMethod`.                |

## Current status

Builder-driven AST construction, deterministic code generation, and parsing of
generated DTS/DTSI/DTSO fragments and the vendor board/SoC trees `consortium-cfg`
merges onto.

It does **not** resolve include files, labels, phandles, or overlays. Unknown or
not-yet-modeled DTS expressions can be emitted with `PropValue::Raw` while the parser,
resolver, and bundler mature — so `consortium-cfg` is never blocked on parser coverage.

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

## Determinism is a requirement, not a nicety

Generated device trees are committed and reviewed as diffs, so output ordering is
fixed: properties and children are emitted in declaration order, and nothing is sorted
or deduplicated behind the caller's back. A codegen change should produce a reviewable
diff rather than a reshuffle.

That is also what makes the round trip above hold — `parse_dts(x.to_dts())?.to_dts() ==
x.to_dts()` for fragments this crate emits. `consortium-cfg`'s snapshot tests depend on
both properties.

## Mutation is expressed by appending, not editing

`utils::edit` is read-**only**: it locates a node by label, reads its `compatible`
strings, checks whether a property is present.

To change a vendor base tree you _append_ `NodeOverlay` items — a label override
carrying `/delete-property/`, `status = "disabled"`, and so on — rather than rewriting
nodes in place. Parsed AST values stay immutable, and the delta stays visible in the
output, which is what a reviewer of a merged tree needs to see.

This is also why `consortium-cfg`'s overlay mode cannot strip a remoteproc node's
`mboxes`/`mbox-names`: a `.dtso` fragment can add and disable but not delete, so that
fixup needs merge mode.

## `IrqMethod`

`utils::irq_method::IrqMethod` is the one place a device-tree value and an MCU register
value must agree. Its discriminants match the Linux
`dt-bindings/interrupt-controller/irq.h` constants, so the same enum can be printed
into a DTS `interrupts` cell and used to configure an NVIC line.

## Limitations

- **Not a `dtc` replacement.** No preprocessor, so `#include` and macros are not
  expanded (an `Include` is carried through as a path). Hand-written board files using
  constructs outside this subset may not parse.
- **No resolution.** A `Reference` stays the label it names; phandles are not assigned.
- **Coarse spans.** Diagnostics point into the source, but not yet precisely.

## Future work

- Broader DTS parser coverage for hand-written board files.
- Richer source spans and diagnostics.
- **Resolver**: resolve include paths relative to a source root, parse included files,
  build label and path side tables, report duplicate labels and unresolved references —
  keeping parsed AST values immutable.
- **Bundler**: flatten a `File` plus resolved includes into a single DTS output,
  preserving the deterministic preamble and tree ordering; optionally bundled DTSO.
- DTSO operation list and diff printer.
- Optional `dtc` validation, in `consortium-builder` or `csti` rather than here.

## Testing

```bash
cargo test -p consortium-dts
cargo test -p consortium-cfg   # snapshot tests of the rendered device trees
```

## License

Apache-2.0. See [LICENSE](../../LICENSE).
