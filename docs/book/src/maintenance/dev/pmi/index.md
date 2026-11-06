# Platform Management Subsystem

Platform management (PMI) is how a core obtains resources it does not own: clocks,
pins, power domains, and resets. It is the subsystem behind `consortium-pmi` and its
backends, of which `consortium-pmi-scmi` is the first. (PMI is not HMI: one is the
platform's control plane, the other the user interface.)

## Why it exists

Who owns a resource differs per chip, and so does how a core gets it:

| Chip              | Owner of clocks / pins / power / reset      | How a controller core gets one |
| ----------------- | ------------------------------------------- | ------------------------------ |
| STM32MP2, i.MX 93 | Nobody in particular (Linux, by convention) | Writes RCC/CCM/IOMUXC directly |
| i.MX 95, i.MX 952 | The System Manager on the CM33              | Asks it over SCMI              |
| RISC-V platforms  | A platform microcontroller                  | Asks it over RPMI              |

Before PMI, the i.MX 95 path did not exist: `consortium-hal-imx9`'s `clock` module is
deliberately inert there, and [pin assignment](../hal/pin-assignment.md) step 4 had no
way to mux a pad from the M7. Drivers and generated `init()` should not have to know
which row they are on.

## Layering

```text
HAL drivers, generated init()
        │  ClockProvider / PinctrlProvider / PowerProvider / ResetProvider
        ▼
consortium-pmi            provider traits, opaque u32 ids, async (RPITIT + MaybeSend)
        │
        ├── consortium-pmi-scmi   SCMI agent: header, status, SMT, BASE/CLOCK/PINCTRL/POWER/RESET
        ├── (chip HAL)            direct register backend — future
        └── (consortium-pmi-rpmi) RISC-V — future
        │
        ▼
consortium-core           Region (the SMT window) + Doorbell (MU, IPCC, …)
```

This mirrors IPC — `consortium-ipc` traits over `consortium-ipc-transport-*` — and,
like IPC, it sits on `consortium-core`'s primitives rather than on IPC itself. An SCMI
channel is a shared window plus a doorbell, exactly what a CIPC link is made of, but its
contract is a fixed request/response protocol with vendor firmware, not typed messages
between two endpoints the user writes. That is why the bells were moved out of the
shared-memory IPC transport into `consortium-core` first.

## Design decisions

- **Async provider traits.** Every operation returns `impl Future + MaybeSend`. A
  message backend waits on the platform; a register backend completes on first poll.
  Methods take `&mut self`, so one handle carries one request at a time — for SCMI,
  one outstanding command per channel, with no token table.
- **Opaque identifiers.** `ClockId`, `PinId`, … are `u32` newtypes numbered by the
  backend. SCMI passes them through as domain ids; mapping a manifest name to a number
  is `consortium-cfg`'s job, not the trait's.
- **Completion is a choice.** `Completion::Doorbell(line)` parks on a line the platform
  rings and sets the SMT completion-interrupt flag; `Completion::Poll` spins on the
  free bit, yielding between polls.
- **Version-dependent layouts are queried, not assumed.** `CLOCK_CONFIG_SET` gained an
  OEM word in clock protocol 3.0; the client asks the clock version once and caches it,
  as Linux does.
- **The spec, as Linux implements it, is the reference.** Message ids, payload layouts,
  SMT offsets, and status codes were checked against `drivers/firmware/arm_scmi`.

## What is not built

1. **The i.MX 95 binding.** Which MU, which lines, and which SMT window the System
   Manager gives the M7 come from the SM configuration (`imx-sm/configs/mx95evk.cfg`),
   not from the reference manual. That belongs in the chip database as an SCMI record
   (transport bell, a2p/p2a lines, shmem window, agent), with `Session` reserving the MU
   so no `[[ipc.channel]]` can claim it.
2. **Lowering.** `consortium-cfg` should turn `[peripheral.*] clock`/`frequency` and
   `pin.*` into provider calls for SM chips. Generated MCU `init()` is synchronous and
   infallible today, so this needs either an async bring-up step or a polled one.
3. **More of SCMI.** The platform (server) role, delayed responses and notifications,
   PERF/SENSOR/VOLTAGE/SYSTEM, and NXP's vendor protocols (LMM, BBM, misc).
4. **A register backend**, implementing the same traits from today's direct-access
   paths, after which `consortium-hal`'s unused `ClockControl` can go.
5. **RPMI** for RISC-V platforms.
