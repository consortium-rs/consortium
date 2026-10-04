# HAL Gap Roadmap (Against Embassy)

**Date:** 2026-10-04
**Scope:** `consortium-pac-{stm32mp2,imx9}`, `consortium-hal`, `consortium-hal-{stm32mp2,imx9}`
**Baselines:** Embassy `f357dad2` (`submodules/embassy`), STM32CubeMP2 v1.3.2, mcux-devices-imx `d0112c67`, imx-sm `rel_imx_sm_2026q3`

This note lists what the Consortium PACs and chip HALs lack compared with Embassy, ordered
so the gaps can be closed one step at a time. Each item names where the gap is, what
Embassy does instead, and when the item counts as done. Treat it as a working checklist:
tick items as they land, and re-verify anything older than a few weeks against the
source, because both trees move quickly.

## Reference map

Embassy supports neither STM32MP nor i.MX 9, so every comparison is against the nearest
family that shares the IP block:

| Our target            | Embassy reference                                   | Shared IP                                                     |
| --------------------- | --------------------------------------------------- | ------------------------------------------------------------- |
| STM32MP2 (CM33)       | `embassy-stm32` + `stm32-metapac`                   | USART v4, SPI v4, I2C v2, FDCAN, LPTIM v2, TIM, GPDMA (HPDMA) |
| STM32MP2 RIF          | `embassy-stm32/src/rif.rs` (STM32N6)                | RIFSC                                                         |
| i.MX 9 (CM33/CM7)     | `embassy-mcxa` + `nxp-pac`                          | LPUART, LPI2C, LPSPI, NXP I3C, eDMA                           |
| i.MX 9 GPIO / IOMUXC  | `embassy-nxp` RT1xxx (`gpio/rt1xxx.rs`, `build.rs`) | RGPIO-style ports, IOMUXC pad/mux                             |
| Shared infrastructure | `embassy-hal-internal`, `embassy-embedded-hal`      | `Peri`, typelevel interrupts, `OnDrop`, `SetConfig`           |

## Phase 0 — Fixed on 2026-10-04

These were defects found during the comparison; they are fixed, with the residue noted.

- [x] **MP2 HPDMA `LPTIM*` request IDs swapped between chips** (`hal-stm32mp2/src/dma.rs`).
      The MP25 and MP21 rows now match `stm32mp2xx_hal_dma.h` (`#if defined(LPDMA)` is
      MP23/25, `#else` is MP21). A test checks that MP25 request numbers are unique.
      _Residue:_ ST's MP21 table gives `LPTIM2_IC1` and `LPTIM2_IC2` both 115. The gap at
      114 suggests a vendor typo, but there is no second source; confirm against RM0506
      before relying on MP21 LPTIM2 capture DMA.
- [x] **MP2 `LPUART1` programmed with the USART baud formula** (`hal-stm32mp2/src/uart.rs`).
      `Instance::LPUART` selects `BRR = 256·f/baud` (20-bit, ≥ `0x300`), and both kinds
      now search `PRESC` like Embassy's `find_and_set_brr`. _Residue:_ 8× oversampling is
      not used, so the top USART baud rate is `f/16`.
- [x] **I2C repeated START between same-direction operations** (both `i2c.rs`). Runs of
      same-direction operations are one segment, as `embedded-hal` requires. On MP2 a
      repeated START now waits for `TC`, and the first START waits for `!BUSY`.
- [x] **Async drivers left hardware mid-transfer when dropped or failed.** Drop guards on
      both chips: I2C (STOP, then a `PE` toggle on MP2; FIFO reset and STOP on LPI2C), SPI
      (`CSUSP`/`SPE` on MP2; module soft-reset that keeps the configuration on LPSPI,
      which releases `PCS`), CAN transmit (`TXBCR` on FDCAN; mailbox inactivation on
      FlexCAN), and I3C (FIFO flush plus STOP on i.MX 9; flush on MP2). UART was already
      cancel-safe; its module docs now say so.
      _Residue:_ MP2 I3C cannot yet end a repeated-start sequence abandoned between
      messages (no bare-STOP path). The recovery spins are counted loops (`RECOVERY_SPINS`,
      `ABORT_SPINS`), not timed.

## Phase 1 — Remaining correctness

Small, independent fixes. Do these before building on the drivers.

- [ ] **P1.1 MP2 time driver accepts 16-bit timers.** `impl_tim_driver!` is emitted for every
      TIM, but `GpTimerTimeDriver` assumes `ARR = u32::MAX` and `HALF_PERIOD = 0x8000_0000`.
      Either restrict `timer_driver::Instance` to TIM2/TIM5 in `gen_peripherals.py`, or follow
      Embassy's `time_driver/tim.rs`, which selects a `u16` or `u32` counter per timer and
      derives `HALF_COUNTER = 1 << (BITS - 1)`, so any timer works. The `TimerWidth` that
      `impl_tim!` already records is the natural input.
      _Done when_ `init_timer(p.TIM3, …)` either fails to compile or keeps correct time
      across wraps, with a unit test for the chosen path.
- [ ] **P1.2 Blocking loops have no timeout.** I2C, SPI, LPI2C, LPSPI, I3C and the DMA
      `halt` spin forever on a stuck bus. Add a `Timeout` the way `embassy-stm32` does in
      `i2c/mod.rs` (with or without `embassy-time`), surfaced as `Error::Timeout`.
- [ ] **P1.3 MP2 I2C DMA hangs on NACK.** `write_dma`/`read_dma` block in
      `Transfer::wait_blocking` while the I2C is NACKed and never raises a DMA request.
      Poll the I2C error flags alongside DMA completion, or make the DMA path async.
- [ ] **P1.4 i.MX 93 LPCG coverage.** `clock::lpcg` lists only LPIT1/2 and the MU
      windows. Add a gate for every driver-bearing singleton, from Linux's `clk-imx93.c`,
      and call `clock::ungate(T::LPCG)` in each constructor (LPUART, LPI2C, LPSPI, FlexCAN,
      TPM, LPTMR, GPIO, ADC, eDMA, I3C, WDOG). _Done when_ each `Instance` carries `LPCG`.
- [ ] **P1.5 GPIO pin aliasing.** `Gpio::pin(&self, n)` can hand out the same pin twice,
      and `Pin` is `Copy`. Emit one singleton per pin (as Embassy does from
      `METADATA.pins`) or make `Gpio` hand out pins by value, once each.
- [ ] **P1.6 i.MX 9 I2C zero-length read.** A `Read(&mut [])` sends a read address with no
      `RECEIVE` command. Reject it (as Embassy does with `ZeroLengthTransfer`) or define
      the behaviour.
- [x] **P1.7 Existing clippy failure.** `hal-imx9/src/clock.rs` tripped
      `multiple_unsafe_ops_per_block` on the i.MX 93 firmware target. The block is
      now split into one access per `unsafe`.
- [ ] **P1.8 Stale docs.** Several module headers and `hal/index.md` still say
      constructors take "an MMIO base and `&'static` state". Rewrite them for the
      singleton API.

## Phase 2 — Metadata and code generation

This is the foundation that most later phases depend on. Embassy's PACs ship a populated
`METADATA` table, and the HAL's `build.rs` generates the chip glue from it. Ours carry the
`metadata.rs` types ported from stm32-data but **no data**, and nothing reads them;
`scripts/pac/gen_peripherals.py` emits only a base address and one optional IRQ per
instance.

- [ ] **P2.1 Populate PAC metadata.** For each chip, emit `METADATA` with `peripherals`
      (registers kind/version, clock enable/reset fields, pins with AF or IOMUXC mux,
      DMA request signals, every interrupt signal) plus `pins` and `dma_channels`.
      Sources: STM32CubeMX DB / CubeMP2 headers; `mcux-devices-imx` `periph` headers and
      pin tools; `consortium-data` chip dumps. Reference: `embassy-stm32/build.rs` and
      `embassy-mcxa/build.rs` show what each field is used for.
- [ ] **P2.2 Clock enable/reset per peripheral.** Generate the equivalent of Embassy's
      `RccPeripheral` (`RCC_INFO` with refcounts for shared enable bits) for MP2 `RCC_xxxCFGR`
      and i.MX 9 LPCG/`CLOCK_ROOT`, and call it from every constructor. Respect ownership:
      on MP2 the A35 may own RCC, so make it an opt-in `Config` knob, not a silent write.
- [ ] **P2.3 Kernel-clock frequency.** Replace the `clock_hz` constructor argument with a
      generated `frequency()` read from the live mux/divider registers (Embassy's
      `ClockGen::gen_mux`). Keep an override for clocks the core cannot read.
- [ ] **P2.4 Typed pins.** Generate `TxPin<T>`, `SckPin<T>`, etc. with AF (MP2) or IOMUXC
      mux + input-select (i.MX 9), and make constructors take pins. Reference: Embassy's
      `pin_trait_impl!` and `embassy-nxp`'s `generate_iomuxc`.
- [ ] **P2.5 Typed DMA requests.** Generate `TxDma<T>`/`RxDma<T>` per peripheral signal, and
      replace the hand tables in both `dma.rs` (MP2 has no USART/SPI/I2C/I3C/SAI entries;
      i.MX 9 has only LPTMR/TPM/ADC). Removes the raw `tx_request: u8` from MP2 I2C DMA.
- [ ] **P2.6 Typed interrupts.** Adopt `embassy_hal_internal::interrupt_mod!` and
      `bind_interrupts!` so async constructors require a `Binding` proof and unpend/enable
      their own NVIC line. Today a missing forwarder makes `.await` hang silently.
- [ ] **P2.7 Every interrupt signal.** `impl_*!(…, Some(irq))` carries one line. MP2
      TIM1/8/20 bind only `_UP` (BRK/TRG_COM/CC are lost) and FDCAN only IT0.
- [ ] **P2.8 `consortium-cfg` vectors for every driver.** `components/peripheral.rs`
      generates `#[interrupt]` forwarders and NVIC enables only for I2C. Extend to UART
      (plain vs buffered), SPI, CAN, I3C, DMA channels, LPTIM, ADC. Much of this falls
      away once P2.6 lands.

## Phase 3 — Cross-cutting HAL infrastructure

- [ ] **P3.1 `Peri<'d, T>` lifetimes and reconfiguration.** Constructors take
      `Peri<'static, T>`, so a driver cannot be rebuilt with a different configuration.
      Take `Peri<'d, T>` (allowing `reborrow()`), and implement
      `embassy_embedded_hal::SetConfig` for UART, SPI and I2C so shared-bus helpers work.
- [ ] **P3.2 EXTI / GPIO edge interrupts.** Both `gpio.rs` have an unwired `State`. MP2
      needs an EXTI driver (no PAC block yet) and `ExtiInput`; i.MX 9 needs the RGPIO
      `ICR`/`ISFR` path on `GPIOn_0/1`. Implement `embedded_hal_async::digital::Wait`.
      Reference: `embassy-stm32/src/exti`, `embassy-nxp/src/gpio/rt1xxx.rs`.
- [ ] **P3.3 GPIO modes.** MP2 lacks pull, speed, open-drain, alternate-function and analog;
      i.MX 9 lacks IOMUXC pad control (mux, pull, drive, `SION`, input select). Model them
      as Embassy's `Flex`/`Input`/`Output`/`OutputOpenDrain`, plus `AfType`.
- [ ] **P3.4 Clock model.** A read-only view of the clock tree (MP2 flexgen
      `XBAR`/`FINDIV`/`PREDIV`; i.MX 9 `CLOCK_ROOT`) with a `clocks()` accessor, following
      `embassy-mcxa/src/clocks`.
- [ ] **P3.5 Low power.** No stop-mode refcounts, RTC wakeup, or `low-power` executor
      (`embassy-stm32/src/low_power.rs`, `embassy-mcxa/src/clocks/sleep.rs`).
- [ ] **P3.6 Executors and utilities.** `InterruptExecutor` support, `uid`, a `fmt.rs`
      shim, and `defmt::Format` on public config/error types.

## Phase 4 — Per-driver feature parity

| ID   | Driver | Missing versus Embassy                                                                                                                                                                                                           |
| ---- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P4.1 | UART   | Data bits, parity, stop bits; RTS/CTS; RS-485 DE timing; half-duplex; swap/invert; 8× oversampling; `read_until_idle`; break; DMA and `RingBufferedUartRx`                                                                       |
| P4.2 | SPI    | 16/32-bit words; DMA; slave mode; TX-only/RX-only/half-duplex; hardware NSS; CRC; I2S (MP2)                                                                                                                                      |
| P4.3 | I2C    | Target (slave) mode; 10-bit addressing; async DMA; rise/fall-aware timing and Fast-mode Plus (MP2 `TIMINGR`, LPI2C `MCCR0`)                                                                                                      |
| P4.4 | DMA    | MP2 LPDMA channels (IRQs exist, no bindings); general linked-list/scatter-gather (`embassy-mcxa` `ScatterGatherBuilder`); priority; RIF/CID config (MP2)                                                                         |
| P4.5 | CAN    | FD frames; hardware acceptance filters; more TX buffers; i.MX 9 RX FIFO instead of one RX mailbox                                                                                                                                |
| P4.6 | Timers | MP2 modes are polling-only (make input capture / PWM input async); MP2 TIM PAC is hand-written with most registers as bare `u32`; i.MX 9 TPM has PWM only (no input capture, QEI, one-pulse); no TPM/LPTMR time driver on i.MX 9 |
| P4.7 | HSEM   | Embassy's `hsem` offers mutex lock/unlock; ours is doorbell-only                                                                                                                                                                 |

## Phase 5 — Missing peripherals

**STM32MP2.** ST's CM33 header defines 86 register-block types; our PAC covers 14 (`adc`,
`dma`, `dma_channel`, `fdcan`, `gpio`, `hsem`, `i2c`, `i3c`, `iwdg`, `lptim`, `rcc`, `spi`,
`timer`, `usart`). Missing, in rough priority for a coprocessor:

- [ ] EXTI, PWR, SYSCFG, IPCC (currently hand-rolled in `consortium-ipc-doorbell-ipcc`)
- [ ] RTC, TAMP, WWDG
- [ ] RIF: RIFSC, RISAF, RISAB, IAC (see P6.2)
- [ ] CRC, RNG, HASH, PKA
- [ ] SAI, SDMMC, OCTOSPI, MDF/ADF, SPDIFRX
- [ ] VREFBUF, DTS

**i.MX 9.**

- [ ] SAI, FlexSPI and FlexIO drivers (the PAC blocks exist for the latter two)
- [ ] TRDC model (the i.MX counterpart of RIF)

## Phase 6 — Platform plumbing Embassy never needs

- [ ] **P6.1 i.MX 95 SCMI client.** Clocks, power domains, pinctrl and resets belong to the
      System Manager and are requested over an MU with SCMI. Nothing in the HAL speaks
      SCMI yet (`submodules/imx-sm` is unused). Until it does, `clock.rs` is inert on
      i.MX 95 and depends on the SM board config pre-opening everything.
- [ ] **P6.2 STM32MP2 RIF ownership checks.** Verify at construction that the peripheral's
      (and DMA channel's) RIFSC CID belongs to this core, and return an error instead of a
      fault or a silent no-op. `embassy-stm32/src/rif.rs` is the register model to follow.

## Suggested order

1. Phase 1, which is small and independent.
2. P2.1 → P2.6: populate metadata, then generate clock, pin, DMA-request and interrupt glue.
   Most of Phase 4 becomes mechanical afterwards.
3. P3.2 + P3.3 (EXTI and pin modes), then P3.1.
4. P6.1 and P1.4 before any new i.MX 9 driver work, since without them drivers fault on
   real hardware.
5. Phase 4 and Phase 5 by demand.
