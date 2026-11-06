# consortium-pmi-scmi

An SCMI agent for Consortium: the first backend of `consortium-pmi`.

SCMI — Arm's System Control and Management Interface (DEN0056) — is how an agent asks
a platform for clocks, pins, power, and resets it does not own. On i.MX 95 the
platform is the System Manager on the CM33 and the M7 is one of its agents. `Client`
speaks the agent side and implements `ClockProvider`, `PinctrlProvider`,
`PowerProvider`, and `ResetProvider` on top of it.

`no_std`, no allocation.

## Transport

The Shared Memory Transport (`smt`): a mailbox window the agent writes a request into,
plus a doorbell to tell the platform it is there. Both are `consortium-core`
primitives — the window is any `Region`, the doorbell any `Doorbell` — so a chip
binding is a choice of region and bell (an i.MX MU, an STM32 IPCC, an Arm MHU), not new
protocol code.

```rust,ignore
use consortium_pmi::{ClockId, ClockProvider};
use consortium_pmi_scmi::{Client, Completion, Smt};

let smt = Smt::new(region)?;                      // the SM's mailbox window
let mut scmi = Client::new(smt, mu, a2p, Completion::Doorbell(p2a));
scmi.enable(ClockId(LPUART3_CLK)).await?;
```

Completion is either a second doorbell line the platform rings back
(`Completion::Doorbell`) or polling the channel's free bit (`Completion::Poll`).

## What is implemented

| Protocol       | Messages                                                                    |
| -------------- | --------------------------------------------------------------------------- |
| BASE (0x10)    | `PROTOCOL_VERSION`, `DISCOVER_VENDOR`                                       |
| CLOCK (0x14)   | `CLOCK_CONFIG_SET` (v2 and v3+ layouts), `CLOCK_RATE_SET`, `CLOCK_RATE_GET` |
| PINCTRL (0x19) | `PINCTRL_SETTINGS_CONFIGURE` (function select; configs, chunked)            |
| POWER (0x11)   | `POWER_STATE_SET`, `POWER_STATE_GET`                                        |
| RESET (0x16)   | `RESET` (assert, release, autonomous pulse)                                 |

Message ids, payload layouts, and status codes follow the spec as Linux's
`drivers/firmware/arm_scmi` implements it. Not yet: the platform (server) role,
asynchronous commands and delayed responses, notifications, PERF/SENSOR/VOLTAGE/
SYSTEM, NXP's vendor protocols, and any chip binding.

## Testing

`tests/fake_server.rs` runs the agent against a scripted platform over a heap-backed
`UncachedRegion`: header and token sequencing, every provider's encoding, error
statuses, a busy channel, the channel-error bit, a mismatched token, short and
oversized messages, and polled and doorbell completion against a late platform.

```bash
just test pmi host
```

## Features

| Feature | Effect                                                  |
| ------- | ------------------------------------------------------- |
| `std`   | Provider futures are `Send`.                            |
| `defmt` | Derive `defmt::Format` on the protocol and error types. |

## License

Apache-2.0. See [LICENSE](../../LICENSE).
