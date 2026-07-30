# consortium-cfg-common

The `Consortium.toml` schema and the chip-database schema it is validated against —
types and `serde` derives only, no logic.

## Why it is a separate crate

This is the vocabulary two otherwise-unrelated consumers must share, and that is the
whole reason it is not folded into `consortium-cfg`:

- [`consortium-cfg`](../consortium-cfg) parses, validates, lowers, and generates code
  from a manifest.
- [`consortium-data`](../consortium-data) embeds the chip-family YAML databases and
  resolves a part number to one, so it needs the `database` types — but must not
  depend on the codegen crate, which depends on *it*.

Splitting the schema out breaks that cycle:

```text
consortium-cfg  ──depends on──►  consortium-data  ──depends on──►  consortium-cfg-common
      └────────────────────── also depends on ──────────────────────────────┘
```

The rule for where something goes: **anything with behaviour belongs in
`consortium-cfg`; anything a manifest or database file has to deserialize into
belongs here.**

## `config` — the manifest schema

`Config` is the root of `Consortium.toml`. **This is the authoritative definition of
the current schema.** Prefer it over the older melt-pot example manifests, which
drift; `crates/consortium-cfg/Consortium.example.*.toml` and
`consortium-cfg/src/session.rs`'s tests are the best worked references.

| Section               | Type                              |
| --------------------- | --------------------------------- |
| `[profile]`           | `Profile`                          |
| `[dts]`               | `Dts`, `DtsMode`                   |
| `[endpoints]`         | `Endpoints`                        |
| `[dbg]`, `[dbg.<core>]` | `Dbg`, `DbgCore`, `DbgMode`       |
| `[[ipc.channel]]`     | `Ipc`, `IpcChannel`, `IpcPattern`, `IpcChannelConfig` |
| `[tee]`               | `Tee`                              |
| `[hmi]`, `[hmi.logical]` | `Hmi`, `HmiEngine`, `HmiBackend`, `HmiLogical` |
| `[peripheral.<name>]` | `Peripheral`                       |

Two schema habits worth knowing:

- **Core names are newtypes, not strings** — `AcoreName`, `McoreName`, `RcoreName`,
  `RvCoreName` — so a manifest cannot quietly name a core kind the chip does not
  have. `VisibleCore` is the resolved, validated form the rest of the pipeline uses.
- **Most structs carry `#[serde(deny_unknown_fields)]`**, so a typo or a leftover
  field from an older schema is an error rather than a silently ignored key. That
  matters for a manifest whose whole job is to be precise about hardware.

## `database` — the chip-database schema

What a chip *offers*, against which a manifest's requests are checked:

| Type                          | Describes                                        |
| ----------------------------- | ------------------------------------------------ |
| `ChipDatabase`                | A chip family: everything below.                  |
| `Coprocessor`, `CoprocessorVariant` | Which non-AP cores exist.                   |
| `MemoryDatabase`, `MemoryRegion`, `MemoryAlias`, `MemoryRegionType` | Address windows and their aliases. |
| `PeripheralDatabase`, `PeripheralAlias` | Peripherals and their alternate names.  |
| `HardwareDoorbellPeripheral`, `HardwareDoorbellPeripheralType`, `HardwareDoorbellParty`, `DoorbellClock` | Doorbell peripherals — their channel counts and parties bound what `[[ipc.channel]]` may ask for. |

The YAML and JSON that populate these types live in `consortium-data`.

## Changing the schema

Work outward, because each step's tests depend on the previous:

1. **types here**,
2. validation in `consortium-cfg`'s `Session::from_toml`,
3. lowering into the `components` IR (`consortium-cfg/src/bridge.rs`),
4. generated code and build artifacts — then update the snapshot tests under
   `crates/consortium-cfg/tests/`.

Skipping step 2 is the common mistake: a field that parses but is never validated
becomes a runtime bus error instead of a manifest error, which is exactly what this
schema exists to prevent.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
