# consortium-cfg

Turns a `Consortium.toml` manifest into validated configuration, generated Rust, and
Linux integration artifacts.

This is the crate that makes Consortium's "explicit cross-core contracts" idea pay
off. A manifest declares what the system _is_ — which cores exist, which memory
windows they share, which doorbell rings which — and this crate checks that against
the chip's real capabilities, then emits the bring-up code and device-tree nodes that
make it so. Firmware and application authors never write the address arithmetic, the
UIO ids, or the doorbell wiring by hand.

## Pipeline

```text
Consortium.toml
  └─ session   parse + resolve the chip database + validate       → Session
       └─ bridge    lower into the components IR                  → ShuHan, BellA, …
            └─ components   each IR node generates its own output
                 ├─ Rust tokens ─ render ─→ src/consortium.gen.rs
                 ├─ DTS nodes   ────────→ merged tree or .dtso
                 └─ Kconfig / boot / systemd fragments
```

| Stage           | Module       | Produces                                            |
| --------------- | ------------ | --------------------------------------------------- |
| Parse, validate | `session`    | `Session` — manifest bound to its chip database     |
| Lower           | `bridge`     | The `components` IR, addresses and UIO ids resolved |
| Generate        | `components` | Rust tokens, DTS nodes, UIO/Kconfig fragments       |
| Format          | `render`     | Formatted Rust source                               |

Schema types come from [`consortium-cfg-common`](../consortium-cfg-common); chip
databases, per-core chip maps, and vendor device trees come from
[`consortium-data`](../consortium-data) (re-exported here as `database`, `chipmap`,
`dts` for historical paths). [`consortium-builder`](../consortium-builder) drives this
and adds compilation, staging, and packaging.

## What validation catches

`Session::from_toml` resolves `[profile] chip` through the embedded databases and then
rejects manifests asking for things the chip cannot do. The `CfgError` variants are
the interesting part of this crate's contract:

- a doorbell the chip does not have, or a doorbell channel outside its range;
- a channel whose party is not a visible core, or has no declared endpoint;
- two channels claiming the same doorbell channel;
- memory regions out of bounds, overlapping, or misaligned for the MPU;
- debug regions colliding with IPC regions;
- a peripheral owned by a core that cannot reach it — checked against the _per-core_
  chip map, so the filter is the peripherals that core's CMSIS view actually contains;
- `[peripheral.*]` with `secure = true`, since the chip HAL's singletons carry only the
  non-secure alias.

Catching these at manifest-parse time is the whole point: each would otherwise surface
as a silent bus error or a link that never wakes, on hardware, at 3am.

## Where the fixed decisions live

`bridge` pins the AP-side address layout so the firmware, NVIC, and Linux DTS outputs
cannot disagree:

- **every device gets its `uioN` id by ascending mapped base** — the aggregate IPC
  shared-memory device, one device per distinct doorbell, and the aggregate debug-ring
  device — so ids follow the memory map the kernel enumerates;
- **within the IPC device, a region's `mmap_id` is its index in channel-declaration
  order**, and a doorbell always exposes its register window as `mmap_id` 0.

These are conventions rather than discoveries, which is why they are written down in
one place: three separate outputs have to agree on them.

## The components IR

Codegen is deliberately **not** centralised. Each IR type emits its own output
(`ShuHan::initialize`, `BellA::nvic_enable`, `ToUio::to_uio`), so a new component adds
a module rather than a branch in a shared emitter.

| Module          | Emits                                                                   |
| --------------- | ----------------------------------------------------------------------- |
| `ipc_shm`       | Transceiver construction, scratch-buffer statics, `IpcMemoryEndpoints`. |
| `bella`         | Type-A doorbell interrupt enablement.                                   |
| `peripheral`    | HAL driver handles, interrupt handlers, NVIC enablement.                |
| `time`          | The sealed Embassy time-driver vector and `defmt::timestamp!`.          |
| `dbg`           | Nothing — resolves ring layout for the builder's env vars.              |
| `uio`           | `generic-uio` DTS nodes and their `reserved-memory` carveouts.          |
| `memory_to_uio` | Groups shared-memory regions onto one UIO device.                       |
| `dts_render`    | The Linux device tree, merged onto a vendor base or as an overlay.      |
| `address`       | AP `(uio_id, mmap_id)` vs a controller's physical base.                 |
| `region`        | One shared-memory window's construction expression.                     |

Two conventions to know before adding a component:

- **Emitted `unsafe` blocks carry their justification as a `///` doc comment,** not a
  `//` comment. `proc_macro2` discards `//` comments because they are not tokens (a
  `///` comment survives as a `#[doc]` attribute), so `render` rewrites the doc lines
  back into ordinary comments after formatting. That is what keeps the rationale
  attached and satisfies `clippy::undocumented_unsafe_blocks` in the generated crate.
- **Items referencing a chip HAL are gated to `target_arch = "arm"`,** because the host
  trybuild harness never links one.

### Two design details worth understanding

**The AP zeroes the shared window; controller cores do not.** A carveout is `no-map`
reserved memory that survives a warm reboot, so a previous run's descriptor and slot
bytes could read as valid `CIPC` state. The AP is the primary in the descriptor
handshake, so it takes the first touch (`mmap_zeroed`).

**A doorbell singleton moves only once.** Two transceivers may share one doorbell block
(disjoint channels of the same MU or IPCC), so `ipc_shm`'s `DoorbellSource` gives the
first user the constructor parameter and has later ones borrow from it. The AP has no
singletons at all — it reaches doorbells through UIO `mmap`.

## Generated code

The output is `src/consortium.gen.rs` in each endpoint crate, which the crate
`include!`s. It defines `Context` and an `async fn init()` that:

1. resets the CDBG ring when the core has a `[dbg.<core>]` logger;
2. brings up the chip's Embassy time driver (unless `[profile] time_driver = false`);
3. constructs, `connect`s, and splits each IPC transport into a transceiver at
   `context.ipc_shm.<name>`;
4. hands out `context.peripherals` on cores that own `[peripheral.*]`.

Because a generated crate cannot be `cargo check`ed before the pipeline has run, the
melt-pot endpoint crates are excluded from the workspace.

## Device-tree output

`[dts] mode` selects between two forms sharing one analysis pass:

| Mode              | Output      | Can delete properties? |
| ----------------- | ----------- | ---------------------- |
| `merge` (default) | a full tree | **yes**                |
| `overlay`         | a `.dtso`   | no                     |

That difference is not cosmetic: stripping a remoteproc node's `mboxes`/`mbox-names`
requires deletion, so a manifest needing that fixup must use merge mode. Overlay mode
returns a warning when it has to skip one.

## Adding to the schema

Work outward, because each step's tests depend on the previous: schema types in
`consortium-cfg-common` → validation in `session` → lowering in `bridge` → codegen in
`components`. Then update the snapshot tests under `tests/`.

## Testing

```bash
cargo test -p consortium-cfg
just test cfg host
```

`tests/` holds `insta` snapshots of generated output plus a `trybuild` UI harness.
Update snapshots (`cargo insta review`) when generated output changes on purpose — the
snapshot diff is how a codegen change becomes reviewable.

`Consortium.example.mimx95.toml`, `Consortium.example.stm32mp257.toml`, and
`src/session.rs`'s tests are the best references for current manifest syntax.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
