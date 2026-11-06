# consortium-pmi

Platform management provider traits for Consortium: how a core obtains clocks,
pins, power domains, and resets it does not own.

`no_std`, no allocation, no backend.

On STM32MP2 and i.MX 93 a controller core writes its clock, reset, and pin-mux
registers itself. On i.MX 95 a System Manager owns all of them and has to be asked
over SCMI. A RISC-V platform may answer RPMI instead. Drivers and generated
`init()` should not have to care which, and this crate is the seam that keeps them
from caring:

| Trait             | Operations                              |
| ----------------- | --------------------------------------- |
| `ClockProvider`   | `enable`, `disable`, `set_rate`, `rate` |
| `PinctrlProvider` | `set_function`, `set_config`            |
| `PowerProvider`   | `set_state`, `state`                    |
| `ResetProvider`   | `assert`, `deassert`, `reset`           |

Every operation is async (`impl Future + MaybeSend`), and the identifiers
(`ClockId`, `PinId`, …) are opaque `u32` newtypes numbered by the backend.

## Backends

| Backend                | Crate                 | Chips                      |
| ---------------------- | --------------------- | -------------------------- |
| SCMI client            | `consortium-pmi-scmi` | i.MX 95/952 System Manager |
| Direct register access | chip HAL (future)     | STM32MP2, i.MX 93          |

## Features

| Feature | Effect                                                            |
| ------- | ----------------------------------------------------------------- |
| `std`   | `MaybeSend` requires `Send`.                                      |
| `defmt` | Derive `defmt::Format` on the identifier and configuration types. |

## License

Apache-2.0. See [LICENSE](../../LICENSE).
