# consortium-shared-memory

Shared-memory building blocks for Consortium: memory regions with explicit cache
semantics, and the descriptor header every Consortium shared-memory structure
begins with.

`no_std`, no allocation, no protocol.

## Scope

This crate answers two narrow questions that several subsystems need answered the
same way:

1. **How is this window mapped, and what does it cost to make a write visible to
   another core?** — the `Region` trait and its implementations.
2. **Whose structure is this, and what version?** — the `descriptor` header.

Protocol lives one level up. `consortium-ipc-transport-memory` owns the `CIPC`
descriptor, channel table, and slot protocol; `consortium-dbg` owns `CDBG` and its
log rings. Both share what is defined here.

## Regions and coherency

On a heterogeneous SoC, cache behavior follows the mapping attributes, not the
kind of RAM backing the window. A non-cacheable mapping cannot retain dirty or
stale lines. A cacheable mapping needs the writer to clean dirty lines and the
reader to clean-invalidate stale lines. Getting this wrong silently produces stale
or lost data.

`Region` makes the difference explicit rather than assumed:

| Type             | Mapping       | `flush` / `invalidate`        | `is_coherent` |
| ---------------- | ------------- | ----------------------------- | ------------- |
| `UncachedRegion` | non-cacheable | no-ops                        | `true`        |
| `CachedRegion`   | cacheable     | AArch64 / ARMv7-M maintenance | `false`       |

Transports are generic over the region, so `is_coherent` and the no-op cache
methods resolve at monomorphization: an uncached link compiles to code with no
cache maintenance in it at all, rather than branching per operation at runtime.

### Adaptations and limitations

- **AArch64** uses `dc cvac` for `flush` and EL0-legal `dc civac` for
  `invalidate`, sweeping each 64-byte line and ending with `dsb sy`. Plain `dc
ivac` is EL1-only and would trap from an AP userspace mapping.
- **ARMv7-M** uses the Cortex-M `SCB` clean and clean-invalidate-by-address
  helpers, which derive the Cortex-M7 line size from `CTR.DminLine`.
- **ARMv8-M is intentionally a no-op.** Although the API compiles there,
  `CTR.DminLine` is architecturally meaningless on cacheless Cortex-M33/M23
  parts. No supported Cortex-M33 has a data cache. Host, RISC-V, and ARM Linux
  builds are also no-ops.
- **The 64-byte line size is a constant,** correct for the Cortex-A35/A55 parts in
  the supported families, not queried from `CTR_EL0`.
- **Cache maintenance is whole-region.** There is no partial flush; a transport
  that wants finer granularity has to construct narrower regions.

## Safety model

Every region constructor is `unsafe` and takes a raw pointer and a length. This is
the one place the obligation is stated:

> The caller guarantees the window is valid for reads and writes for `len` bytes
> for as long as the region lives, and that no conflicting Rust alias to it exists.

In exchange, `UncachedRegion` and `CachedRegion` are `Send + Sync` — two processors
genuinely do touch these bytes, so there is no single-owner story to tell — and
everything built on top of them can be safe.

In practice the pointer comes from a `mmap` of `/dev/uioN` on Linux or from a
linker-placed symbol on firmware, and `consortium-cfg` has already validated the
window's bounds, overlap, and either MPU or cache-line alignment against
`Consortium.toml`.

## Descriptor header

Two cores agree on a memory window but not on when either of them boots. A
secondary that maps its window and finds arbitrary bytes must distinguish several
situations that all look like "not what I expected": the primary has not published
yet; this is a `CDBG` ring where a `CIPC` transport was expected (a plausible
outcome of a wrong base address); this is the right structure but a version this
build cannot parse; this is not Consortium memory at all.

`DescriptorHeader` makes that cheap — a 4-byte `DescriptorMagic` plus a version
byte at a fixed offset, readable before anything else about the layout is assumed.
`DescriptorMagic::check` returns `BadMagic` for a recognized-but-wrong structure
and `UnknownMagic` for unrecognized bytes, so a bring-up failure points at the
actual mistake.

| Magic  | Structure                        | Owner                             |
| ------ | -------------------------------- | --------------------------------- |
| `CIPC` | descriptor, channel table, slots | `consortium-ipc-transport-memory` |
| `CDBG` | defmt log ring descriptor        | `consortium-dbg`                  |

The header is `#[repr(C, packed)]` and the magic is stored big-endian so the bytes
read as ASCII in a memory dump (`43 49 50 43` = `CIPC`). Field access goes through
`from_raw_ptr` / `write_to_raw_ptr` because a packed struct's fields cannot be
borrowed.

## Example

```rust,ignore
use consortium_shared_memory::{CachedRegion, Region};

// SAFETY: `base` is a 64 KiB controller-side window mapped cacheably, aligned
// to cache lines, and nothing else aliases it for the lifetime of this region.
let region = unsafe { CachedRegion::new(base, 64 * 1024) };

// Writer side: publish, then make the bytes visible to the peer.
write_payload(&region);
region.flush();

// Reader side: drop stale lines before reading what the peer published.
region.invalidate();
read_payload(&region);
```

## Features

| Feature | Effect                                                                                             |
| ------- | -------------------------------------------------------------------------------------------------- |
| `defmt` | Derive `defmt::Format` on the descriptor error and magic types, for firmware builds that log them. |

## License

Apache-2.0. See [LICENSE](../../LICENSE).
