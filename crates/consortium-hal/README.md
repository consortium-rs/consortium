# consortium-hal

Shared HAL vocabulary for Consortium chip support crates: the peripheral singleton
machinery, a handful of narrow traits, and NVIC helpers.

`no_std`, no features, no hardware access of its own.

## Scope

This crate is deliberately small. It holds only what the chip HALs
([`consortium-hal-imx9`](../consortium-hal-imx9),
[`consortium-hal-stm32mp2`](../consortium-hal-stm32mp2)) and their generated code
need to **agree** on.

Every register layout, clock tree, and driver lives in a chip HAL, because that is
where the differences are — and a shared trait that tried to abstract an i.MX 9 clock
tree and an STM32MP2 one behind one interface would either leak the difference or
lie about it. So this crate abstracts the *shape* of a chip HAL, not the chip.

## Peripherals are owned, not addressed

The load-bearing idea: a driver takes ownership of a peripheral, not an address.

`peripherals!` declares a chip's zero-sized singleton markers and the `Peripherals`
struct that hands them out **once**. Each driver constructor consumes the one it
drives:

```rust,ignore
let p = consortium_hal_imx9::init();
let i2c = consortium_hal_imx9::i2c::I2c::new(p.LPI2C1, 24_000_000, 400_000);
let heartbeat = consortium_hal_imx9::gpio::Gpio::new(p.GPIO2).pin(0);
```

Because the base address and NVIC line travel with the singleton:

- **no driver takes an MMIO base**, and
- **no constructor is `unsafe`** — the aliasing question is settled by the type
  system rather than by a comment asking the caller to be careful.

`peripherals::<NAME>::steal()` is the escape hatch for bring-up outside `init()` (a
second-stage loader, a test harness). It carries the exclusivity obligation the safe
path discharges.

Per-instance driver state hangs off the marker rather than a named `static`, so an
application forwards an interrupt as
`<hal>::i2c::on_interrupt::<peripherals::LPI2C1>()` instead of naming a global.

### Two deliberate divergences from Embassy

`peripherals!` is `embassy_hal_internal::peripherals!` minus two things, and both
removals are load-bearing:

| Removed                             | Why                                                       |
| ----------------------------------- | --------------------------------------------------------- |
| Per-type `defmt::Format` derive      | Consortium chips have hundreds of peripherals, and the derive emits a `.defmt.*` link section per type. On a `defmt`-enabled host build that overruns the Mach-O section limit and the HAL stops compiling on macOS. The markers are zero-sized and never formatted, so nothing is lost. |
| `#[unsafe(no_mangle)]` on the once-guard | Two chip HALs can land in one binary — the `consortium-cfg` UI tests link both — and an unmangled guard symbol would collide. |

## Traits

| Trait             | Answers                                              |
| ----------------- | ---------------------------------------------------- |
| `Peripheral`      | What is this instance called in generated metadata?    |
| `ClockControl`    | Enable and reset the peripheral clock.                 |
| `InterruptSource` | Which interrupt line does it raise?                    |
| `MonotonicTimer`  | Read ticks and program the next alarm.                 |

`MonotonicTimer` is what a chip HAL's Embassy time driver is built on (STM32MP2 TIM2,
i.MX9 LPIT1). `consortium-cfg` emits its bring-up into the generated `init()` unless
`[profile] time_driver = false`.

The `nvic` module adds masking, pending, and enable for lines identified by **raw
external interrupt number** (exception number minus 16, as the reference manuals and
chip databases use them). That exists because a typed PAC `Interrupt` enum cannot name
a line chosen at runtime, which generated bring-up code and runtime-configured drivers
both need.

## `Peri` and `PeripheralType`

Both are re-exported from `embassy_hal_internal`, so chip HALs and their dependents —
the doorbell crates, the UART transport glue — can name a singleton handle without
taking an Embassy dependency of their own.

## Implementing a chip HAL

1. Depend on this crate and on your generated PAC (with `default-features = false`, so
   the PAC's default chip is not pulled in alongside yours).
2. Invoke `consortium_hal::peripherals!` with the chip's singletons. Don't hand-write
   this: `scripts/pac/gen_peripherals.py` emits it from the `consortium-data` chip
   dumps into `src/peripherals/<chip>.rs`, which `lib.rs` mounts as a private `chip`
   module.
3. Each driver module owns an `impl_<driver>!` macro that the generated file invokes
   with an instance's base and NVIC line. **The generated file stays data only** — the
   driver keeps the logic. Re-run the generator rather than editing it after a chip
   dump changes.
4. Expose `init() -> Peripherals` that calls `Peripherals::take()` once.
5. Keep chip-specific detail in the chip HAL; add to this crate only what two chip
   HALs must agree on.

## License

Apache-2.0. See [LICENSE](../../LICENSE).
