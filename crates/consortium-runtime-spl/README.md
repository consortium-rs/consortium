# consortium-runtime-spl

Type-level markers for partitioning: which peripheral, which security domain, which
privilege role.

> **Early stage.** This crate is a placeholder for partitioning-oriented code and
> currently exposes only three marker traits. Nothing in the workspace consumes it yet;
> the shape is expected to change as the partitioning work lands.

## What it is for

On an STM32MP2 or i.MX 9, a peripheral is not simply "present". It is assigned to a
security domain and a privilege level by a resource-isolation controller — RIFSC on
STM32MP2, TRDC on i.MX 9 — and getting that assignment wrong produces a **bus error**,
not a compile error.

These traits exist so an assignment can be carried in a _type_ rather than a runtime
number. That is the same reasoning behind `Side` in `consortium-ipc` and the peripheral
singletons in `consortium-hal`: push the mismatch to build time, where it is cheap,
instead of discovering it on hardware, where it is not.

| Trait        | Carries                                                         |
| ------------ | --------------------------------------------------------------- |
| `Peripheral` | The peripheral's index, as the isolation controller numbers it. |
| `Domain`     | The security/isolation domain index it is assigned to.          |
| `Role`       | Whether that assignment is secure, and whether privileged.      |

All three are const-only, so an implementor is a zero-sized marker and the information
is available in `const` contexts — usable for a compile-time assertion about an
assignment rather than a runtime check.

Note that `Peripheral::INDEX` is the _isolation controller's_ index. That is not the
same as a position in the chip HAL's singleton list or an NVIC line, and conflating the
three is the mistake this crate is meant to prevent.

## Relationship to the rest of the workspace

Today the authoritative statement of peripheral ownership is `[peripheral.<name>]` in
`Consortium.toml`, validated by `consortium-cfg` against the per-core chip map and
lowered into generated bring-up code.

`consortium-cfg` currently **rejects `secure = true` outright**, because the chip HALs'
peripheral singletons carry only the non-secure alias of each window. This crate is
where a type-level encoding of the same facts would live if that restriction is lifted.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
