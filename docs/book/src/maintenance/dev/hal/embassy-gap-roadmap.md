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

## Phase 1 — Fixed on 2026-10-05

Small, independent fixes made before building on the drivers.

- [x] **P1.1 MP2 time driver accepts 16-bit timers.** `GpTimerTimeDriver` now follows
      Embassy's `time_driver/tim.rs`: `timer_driver::Instance::BITS` (16 or 32, from the
      same width table as `impl_tim!`) sets `ARR`, the half-period compare and the
      `period · 2^(BITS-1)` time base, so TIM3/4/12/15 keep time across wraps (replayed
      through several 16-bit wraps, with late interrupts, in unit tests). `gen_peripherals.py`
      binds the time driver only to TIM2/3/4/5/12/15, so `init_timer(p.TIM6, …)` and the
      single-channel timers fail to compile.
      _Residue:_ TIM1/8/20 are also excluded, because their capture/compare flags raise a
      separate `_CC` vector the driver is not bound to (P2.7).
- [x] **P1.2 Blocking loops have no timeout.** `consortium_hal::timeout::Timeout` bounds
      every blocking status poll in I2C, SPI, LPI2C, LPSPI and I3C (both chips), surfaced
      as `Error::Timeout`, configurable per driver with `set_timeout`. DMA `halt` (MP2) and
      the eDMA channel-inactive waits (i.MX 9) are bounded too. The MP2 I2C recovery's
      `PE` read-back loop is bounded.
      _Residue:_ the budget counts polls, not time, because the waits also run in `Drop`
      and before any time driver exists; its wall-clock bound scales with the core clock.
      A `Duration`-based deadline can follow P3.4. `Transfer::wait_blocking` itself stays
      unbounded, since a peripheral-paced transfer has no natural deadline.
- [x] **P1.3 MP2 I2C DMA hangs on NACK.** `write_dma`/`read_dma` now poll the I2C error
      flags alongside the channel's new `Transfer::status()` (also on i.MX 9), under the
      driver's `Timeout`; a NACK returns `Error::Nack`, halts the channel and recovers the
      bus. DMA failures are reported as `Error::Dma` instead of `Error::Bus`.
- [x] **P1.4 i.MX 93 LPCG coverage.** `clock::lpcg` carries every driver-bearing gate from
      Linux's `clk-imx93.c` (LPUART, LPI2C, LPSPI, FlexCAN, TPM, LPTMR, GPIO, ADC, eDMA
      channels by controller, I3C, WDOG, LPIT, MU), every chip HAL `SealedInstance` carries
      `LPCG`, and every constructor calls `clock::ungate(T::LPCG)` first. A test parses the
      generated `imx93.rs` and fails if any bound singleton lacks a gate.
      _Residue:_ the gate is opened but the `CLOCK_ROOT` behind it is not (P2.2/P2.3).
- [x] **P1.5 GPIO pin aliasing.** `Pin` is no longer `Copy`/`Clone`, and `Gpio` tracks
      handed-out pins: `pin(&mut self, n)` panics and `try_pin` returns `None` for a pin
      already taken. Per-pin singletons remain P2.4.
- [x] **P1.6 i.MX 9 I2C zero-length read.** A `Read(&mut [])` anywhere in a transaction is
      rejected with `Error::ZeroLengthTransfer` before anything goes on the bus, as
      `embassy-mcxa` does. Zero-length writes still make an address probe.
- [x] **P1.7 Existing clippy failure.** `hal-imx9/src/clock.rs` tripped
      `multiple_unsafe_ops_per_block` on the i.MX 93 firmware target. The block is
      now split into one access per `unsafe`.
- [x] **P1.8 Stale docs.** `hal/index.md`, the MP2 CAN/DMA/ring-buffer headers and the
      i.MX 9 I3C header now describe the singleton API and `on_interrupt::<T>()`
      forwarding instead of "an MMIO base and `&'static` state".

## Phase 2 — Metadata and code generation

This is the foundation that most later phases depend on. Embassy's PACs ship a populated
`METADATA` table, and the HAL's `build.rs` generates the chip glue from it. Ours carry the
`metadata.rs` types ported from stm32-data but **no data**, and nothing reads them;
`scripts/pac/gen_peripherals.py` emits only a base address and one optional IRQ per
instance.

- [ ] **P2.1 Populate PAC metadata.** For each chip, emit `METADATA` with `peripherals`
      (registers kind/version, clock enable/reset fields, pins with AF or IOMUXC mux,
      DMA request signals, every interrupt signal) plus `pins` and `dma_channels`.
      Sources are surveyed field by field in [Phase 2 data sources](#phase-2-data-sources).
      Reference: `embassy-stm32/build.rs` and `embassy-mcxa/build.rs` show what each field
      is used for.
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

### Phase 2 data sources

Surveyed on 2026-10-05. Each row says where a field can be generated from today, and what
still has to be read out of the reference manual (RM/TRM) or datasheet (DS). The sources
outside the repository are STM32CubeMX 6.18 (`STM32CubeMX.app/Contents/Resources/db`:
`mcu/*.xml`, `mcu/IP/*_Modes.xml`, `plugins/clock/STM32MP2{,1}.xml`) and the MCUXpresso
Config Tools 26.09 processor data for `MIMX9352xxxxM`, `MIMX9596xxxxN` and
`MIMX9596xxxxQ` (abbreviated CT 93 / CT 95 below; CT 95 figures are the VZ package unless
noted). Both are vendor-licensed, so a generator may read them but the repository should
not vendor them.

Corrections to earlier assumptions: CubeMX lists only the IPs it can configure, so it
omits HSEM, EXTI1/2, IPCC2 and the whole CM0+ context; the CMSIS headers and HAL RCC
macros (`__HAL_RCC_HSEM_CLK_ENABLE`, `__HAL_RCC_IPCC2_CLK_ENABLE`) still cover them.
The i.MX 93 SDK `fsl_clock.{h,c}` already carries the LPCG gate per IP and every root's
four mux sources; only PLL arithmetic and limits are missing. On i.MX 95 the SDK
`CLOCK_EnableClock` is an empty stub, because the System Manager owns clocks (P6.1).

Chip-dump fixes made during the survey:

- [x] **`stm32mp25_cm0p.json` had no peripherals.** `parse_stm32.py` matched only the
      TrustZone `NAME_BASE_NS` / `NAME_NS` spellings, and the CM0+ header (no TrustZone)
      uses `NAME_BASE` / `NAME`. The dump now lists the 25 instances the header declares
      (GPIOZ, LPDMA, EXTI2, HSEM, IPCC2, LPUART1, SPI8, I2C8, I3C4, LPTIM3–5, ADF1, RTC, TAMP,
      IWDG5, WWDG2, PWR, RISC, …), each at the same base as the CM33's non-secure view.
      Because the CM0+ now has a chip map, `consortium-cfg` builds its IPCC doorbell from
      the `IPCC2` singleton instead of `new_mapped(0x4625_0000)` (snapshots updated).
      The IPCC/HSEM/GPIO doorbells' `atomic-waker` needed compare-and-swap, which ARMv6-M
      lacks; on targets without pointer-width CAS they now enable its `portable-atomic`
      feature, and `consortium-runtime-mcu` already selects portable-atomic's
      `critical-section` fallback. The melt-pot firmware dependency set now type-checks for
      `thumbv6m-none-eabi`.
      The sealed time driver is now emitted only for a core whose chip map lists its timer
      (`components/time.rs`), so the CM0+ no longer brings up TIM2, which it cannot reach.
      _Residue:_ nothing links or runs that path, and no example has a CM0+ endpoint. The
      CM0+ has no time driver: an LPTIM3–5 driver (Embassy's `time_driver` has an LPTIM
      variant) would give it `embassy-time`.
- [x] **STM32MP23 is a supported subset target.** MP23 is the MP25 die (DIE505) and
      shares its reference manual (RM0457, memory map included), so
      `stm32mp235fxx_cm33.h` declares the MP25 superset; the MP23 datasheet (DS14634 Rev 3)
      defines what is actually there. - Chip dump: IR patch `scripts/pac/patches/ir/0002-stm32mp23-cm33-absent-instances.diff`
      drops GPIOJ/K (no MP23 package bonds them), LPDMA ("3 DMA controllers, 48 channels"
      = 3 HPDMA), IWDG5/WWDG2 ("4 independent + 1 window" watchdogs) and IPCC2 (no
      CM0+), with their IRQs. EXTI2 stays: the SmartRun domain remains. - HAL: `stm32mp23x` keeps the PAC's `stm32mp257` interrupt table (identical numbering)
      but mounts its own generated singleton map, `peripherals/stm32mp23.rs` (228 vs 253),
      so I2C3–6, SPI6/7, FDCAN3, TIM20, UART8/9, I3C3, ADF1, LPDMA, IPCC2, … are compile
      errors instead of bus faults. The IPCC doorbell binds IPCC2 on `stm32mp25x` only. - Memory database: SRAM1 base fixed (`0x000400000` → `0x00040000`), LPSRAM1–3
      removed (they are the 32 KB between 776 KB and 808 KB), IPCC2 removed, and the MP25
      `remoteproc` section added (its windows are all in DDR). - Tests: `Consortium.example.stm32mp235.toml`, a `tests/configs/stm32mp235` snapshot,
      and session tests rejecting IPCC2, a `cm0p` endpoint, I2C3 and an LPSRAM window.
      _Residue:_ no MP23 hardware is exercised; CM33 firmware that stays within the MP23
      set runs unchanged on an STM32MP257F-DK, since addresses and vectors match.

**STM32MP2** (MP21 / MP23 / MP25; CM33 and CM0+)

| Item      | Data                                    | From existing data                                                                                     | From RM / DS                                                                                                          |
| --------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------- |
| P2.1      | Instances, base addresses               | CMSIS headers → `consortium-data` chip dumps (done)                                                    | —                                                                                                                     |
| P2.1      | Register kind/version                   | CubeMX MCU XML `IP Version` (e.g. `sci3_v2_3`, `i2c2_v1_3`)                                            | —                                                                                                                     |
| P2.1/P2.4 | Pins and AF                             | CubeMX `GPIO-STM32MP2{,1,3}_gpio_v1_0_Modes.xml` (AF per pin and signal); balls per package in MCU XML | **RM:** confirm GPIOZ is the CM0+'s only port (the header declares no other)                                          |
| P2.1/P2.5 | DMA request signals                     | Numbers: `stm32mp2xx_hal_dma.h` (327 defines); names: CubeMX `HPDMA-*_Modes.xml`                       | **DS/errata:** identify cut 1 vs cut 2 (HPDMA 166–183 absent on cut 1); **RM:** check each name                       |
| P2.1      | DMA channels                            | CMSIS / CubeMX: HPDMA1–3 × 16, LPDMA × 4                                                               | **RM:** per-channel FIFO size and 2D/linked-list support; which HPDMA the M33 should use. MP23 has no LPDMA (DS14634) |
| P2.1/P2.6 | Interrupts                              | CMSIS `_cm33.h` / `_cm0.h`; CubeMX `NVIC2-*_Modes.xml` maps IRQ → instance                             | **RM:** fan-out of CM0+ aggregated lines (`EXTI2_L/H`)                                                                |
| P2.1      | NVIC priority bits                      | CMSIS `__NVIC_PRIO_BITS` (M33 = 4, CM0+ = 2)                                                           | —                                                                                                                     |
| P2.1      | IPs CubeMX omits (HSEM, EXTI1/2, IPCC2) | CMSIS headers (base, IRQs); HAL RCC macros                                                             | **RM:** which core may reach them; required RIF settings                                                              |
| P2.2      | Enable/reset bits                       | HAL `__HAL_RCC_*_CLK_ENABLE` / `_FORCE_RESET` (`xxxCFGR.EN/RST`)                                       | —                                                                                                                     |
| P2.2      | Shared enable bits (refcount)           | CubeMX `ClockEnableMode` (e.g. `ADC12`)                                                                | —                                                                                                                     |
| P2.2/P2.3 | RCC ownership and readability           | —                                                                                                      | **RM (RCC/RIF):** what a non-owner CID may read/write under `RCC_RxCIDCFGR`                                           |
| P2.3      | Peripheral → flexgen (`XBAR`) channel   | CubeMX `plugins/clock/STM32MP2{,1}.xml` (e.g. USART1 = XBAR19); HAL `RCC_PERIPHCLK_*`                  | **DS/errata:** `RCC_PERIPHCLK_*` indices shift between cuts                                                           |
| P2.3      | Mux sources and dividers                | HAL `stm32mp2xx_hal_rcc_ex.h` (XBAR sources, divider ranges); CubeMX clock XML                         | **RM:** exact `FINDIV`/`PREDIV` and PLL formulas                                                                      |
| P2.3      | Max kernel frequency                    | CubeMX clock XML (not yet verified)                                                                    | **DS:** frequency limits table                                                                                        |
| P2.7      | Every signal (TIM BRK/UP/TRG_COM/CC, …) | Encoded in CMSIS IRQ names, cross-checked with `NVIC2` XML                                             | —                                                                                                                     |
| P2.8      | `consortium-cfg` vectors                | Derived from P2.6/P2.7                                                                                 | —                                                                                                                     |
| P3.5      | Stop-mode behaviour                     | —                                                                                                      | **RM:** per-peripheral Stop1/Stop2/LP-Stop/LPLV-Stop table                                                            |

**i.MX 93** (CM33) **/ i.MX 95** (CM7)

| Item      | Data                                                        | From existing data                                                                                                                                                                                                | From TRM / DS                                                                                                                                                     |
| --------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P2.1      | Instances, base addresses                                   | SDK `*_COMMON.h` → chip dumps (done)                                                                                                                                                                              | —                                                                                                                                                                 |
| P2.1      | Register kind/version                                       | —                                                                                                                                                                                                                 | **TRM:** `VERID`/`PARAM` reset values per IP                                                                                                                      |
| P2.1/P2.4 | Pins, mux, daisy                                            | SDK `fsl_iomuxc.h`; CT 95: 758 pads, 907 connections (269 with daisy); CT 93: 306 balls, 601 connections (141 with daisy). CT adds ball, power group and signal direction. Per-package sets: see below            | **TRM (IOMUXC):** which functions require `SION` (CT never forces it)                                                                                             |
| P2.1/P2.5 | DMA request signals                                         | 95: `fsl_edma_soc.h`; 93: `periph/PERI_DMA.h` (CT 93 `DMA3.js`/`DMA4.js` duplicate it)                                                                                                                            | —                                                                                                                                                                 |
| P2.1      | DMA channels                                                | `FSL_FEATURE_EDMA_*`; channel IRQ names (DMA5 lines serve channel pairs)                                                                                                                                          | —                                                                                                                                                                 |
| P2.1/P2.6 | Interrupts                                                  | SDK IRQ enum; CT 95 `resource_tables/interrupts.xml` maps vector → device/function per core, but only 93 of 819 CM7 vectors                                                                                       | **95 TRM:** CM7 vectors that reach the NVIC directly vs through IRQSTEER (channel/master/slice), and the `Reserved*` entries. **93:** no table; interrupt chapter |
| P2.1      | NVIC priority bits                                          | CT 95 (CM7 = 4, CM33 = 3); CMSIS header on 93                                                                                                                                                                     | —                                                                                                                                                                 |
| P2.2      | Clock gate (93)                                             | `fsl_clock.h` `clock_ip_name_t`: LPCG index per IP (127 entries; shared gates appear as `kCLOCK_Mu_A`/`_B`); P1.4's `clock::lpcg`                                                                                 | **TRM (CCM):** confirm the shared-gate list only                                                                                                                  |
| P2.2      | Clock gate (95)                                             | SM-owned: `imx-sm` `DEV_SM_CLK_*` IDs and per-LM ownership in `configs/*.cfg`; gating goes over SCMI (P6.1)                                                                                                       | **SM docs:** whether assigning a resource to an LM ungates its LPCG                                                                                               |
| P2.2      | Reset                                                       | —                                                                                                                                                                                                                 | **TRM (SRC):** confirm per-domain resets only, so `reset` stays `None`                                                                                            |
| P2.3      | Root mux sources and divider (93)                           | `fsl_clock.c` `s_clockSourceName[root][0..3]` (every root's four sources); `CLOCK_Get/SetRootClockMux`/`Div` give the `CLOCK_ROOTn_CONTROL` semantics                                                             | —                                                                                                                                                                 |
| P2.3      | Root mux sources and divider (95)                           | CT 95 `clocks/CCM.xml`, `TOP.xml`: mux table with control values, `DIV+1`, `OFF` bit                                                                                                                              | —                                                                                                                                                                 |
| P2.3      | PLL / DFS frequency                                         | CT 95 `clocks/*_PLL*.xml`: VCO from `RDIV`, `MFI + MFN/MFD`, `ODIV`; DFS = VCO / (`MFI + MFN/5`), optional ÷2. The 93 SDK only writes these fields; its `CLOCK_GetIpFreq` uses a hard-coded `g_clockSourceFreq[]` | **93 TRM:** confirm the PLL is the same IP as on 95, and the `RDIV`/`ODIV` encoding                                                                               |
| P2.3      | Peripheral → bus and functional root                        | CT 95 `module_clocks.xml` (e.g. LPUART1 → `BUS_AON` + `LPUART1_CLK_ROOT`); functional root on 93 by name (`kCLOCK_Root_*`)                                                                                        | **93 TRM (CCM):** bus root per IP                                                                                                                                 |
| P2.3      | Max frequency per root                                      | 95: CT limits per voltage mode (OD/NM); `imx-sm` `ES_MAX_HZ_*`/`ES_MIN_HZ_*`                                                                                                                                      | **93 DS:** clock-root limits table (the SDK states only CM33 = 250 MHz at OD)                                                                                     |
| P2.3      | Runtime frequency (95)                                      | SCMI clock-rate query (P6.1)                                                                                                                                                                                      | **SM docs / TRM:** whether the CM7 may read CCM directly                                                                                                          |
| P2.7      | Every signal (CAN `IRQ`/`ERROR`, GPIO `_0`/`_1`, DMA pairs) | Encoded in IRQ names; CT 95 covers CAN                                                                                                                                                                            | **TRM:** what each GPIO `_0`/`_1` line carries; DMA channel-pair → line mapping                                                                                   |
| P2.8      | `consortium-cfg` vectors                                    | Derived from P2.6/P2.7                                                                                                                                                                                            | —                                                                                                                                                                 |
| P3.5      | Stop-mode behaviour                                         | 95: `imx-sm` `fsl_cpu.c` LPI table (GPIO1–5, CAN1–5, LPUART1–8, WDOG3–5)                                                                                                                                          | **TRM (GPC):** the rest of 95, all of 93                                                                                                                          |

The SM model shipped with CT 95 (`sm/sm_model`) covers TRDC security (MDAC/MBC/MRC) only,
and CT `POWER_MODES.xml` names RUN/WAIT/STOP without per-peripheral detail; neither feeds
Phase 2.

#### Package and revision variants

The _line_ (MP257, MP215, 9352, 9596) fixes registers, interrupts, DMA requests and which
GPIO ports exist; the _package_ fixes which pins exist and, on the smallest STM32MP21
package, which peripherals. P2.4 therefore needs the package as well as the line, as
Embassy's metapac chip names do (`stm32h743zi`). Today the cargo features (`stm32mp25x`,
`mimx95`) and `[profile] chip` name only the line.

| Line              | Package                                                       | GPIO pins             | Difference                                                                                                                                                                                                                                             |
| ----------------- | ------------------------------------------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| STM32MP25x        | `AJ` TFBGA361 16×16, `AK` VFBGA424 14×14, `AL` VFBGA361 10×10 | 144 (A–I, Z)          | Identical pins, signals per pin and IP list                                                                                                                                                                                                            |
| STM32MP25x        | `AI` TFBGA436 18×18                                           | 172                   | Adds PI12–15, PJ0–15, PK0–7 (the only package with ports J/K); same IP list                                                                                                                                                                            |
| STM32MP23x        | `AJ`, `AK`, `AL`                                              | 144 (A–I, Z)          | Identical; no package bonds J/K                                                                                                                                                                                                                        |
| STM32MP21x        | `AL` VFBGA361 10×10, `AM` TFBGA289 14×14, `AN` VFBGA273 11×11 | 123 (A–I, Z)          | Identical                                                                                                                                                                                                                                              |
| STM32MP21x        | `AO` VFBGA225 8×8                                             | 98                    | Loses PA12, PB1–3/5–7/9/10, PE10/14, PG8–10/13–15, PH7/8, PI5/6/8, PZ0/1/3, and the IPs LPUART1, LPTIM5, TIM16, TIM17 (plus CSI, LPTIM3, LPTIM4 on MP215); five surviving pins (e.g. PG4, PH4, PA9) lose those IPs' signals                            |
| i.MX 93 (9352)    | VT FCBGA306 14×14, VV FCBGA306 11×11                          | —                     | One CT pin model for every 9352 part (AVTXM, CVVXM, DVVXM, XVVXM)                                                                                                                                                                                      |
| i.MX 93 (93x1/x2) | VX FCBGA208 9×9                                               | —                     | Sold only as MIMX9311/9312/9321/9322 (no 9352). The SDK reuses the 9352 drivers, IOMUXC table and IRQ list for them, so nothing in hand says which pads are unbonded                                                                                   |
| i.MX 95 (9596)    | VZ FCBGA716 19×19                                             | 128 GPIO-capable pads | 313 muxable pads                                                                                                                                                                                                                                       |
| i.MX 95 (9596)    | VT **FCBGA548** 15×15                                         | 125                   | Loses GPIO_IO35–37 (GPIO5 bits 15–17: LPSPI4 SIN/SOUT/SCK, LPUART7 TX/RX, LPUART6_RTS_B, PCIE2_CLKREQ_B routes), the ETH SerDes lane and PCIE2. Shared pads keep identical functions; the ball map differs. Registers, clocks and interrupts identical |

Also: the per-pin AF tables are per line (`GPIO-STM32MP2{,1,3}_gpio_v1_0_Modes.xml`), so a
package only removes pins and never moves an AF. Same-package line changes matter more
than package changes: MP257 → MP235 in `AK` keeps all 144 pins but drops 18 IPs (FDCAN3,
I2C3–6, SPI6/7, UART8/9, TIM20, I3C3, LPDMA1, ETHSW, PCIE, …). i.MX 95 VY (19×19, lidded)
is sold only as MIMX95N6CVYXN.

Silicon revision: the i.MX 95 rev N and rev Q packs are identical in pins (pads, functions,
ball coordinates), interrupts, module clocks, clock tree, SM model and resource tables.
Rev N adds the register blocks `AON_CMUA2`, `NOC_CMUN1` and `ENETC0_GLOBAL`, which the SDK
header already declares, so the generator needs no revision parameter on i.MX 95. The
STM32MP2 silicon cut does change DMA request numbers (reading list, item 2).

**Reading list, by impact.**

1. i.MX 95 CM7 IRQSTEER routing — blocks P2.6/P2.7.
2. STM32MP2 silicon cut identification — decides whether the generator takes a cut
   parameter for DMA requests and `RCC_PERIPHCLK_*`.
3. STM32MP2 RCC/RIF read access from the M33 — decides whether P2.3 reads live registers.
4. STM32MP2 CM0+ reach (GPIOZ only?) and `EXTI2` fan-out.
5. HPDMA per-channel capabilities.
6. i.MX 93 PLL confirmation and clock-root limits (DS).
7. i.MX 93 9×9 (VX) ball map — only if the 93x1/93x2 parts are to be supported.
8. `SION` requirements, `VERID` versions, SRC resets, stop-mode tables.

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

- [ ] EXTI, PWR, SYSCFG, IPCC (IPCC currently hand-rolled in `consortium-ipc-transport-memory::bell::ipcc`)
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

1. ~~Phase 1, which is small and independent.~~ Done.
2. P2.1 → P2.6: populate metadata, then generate clock, pin, DMA-request and interrupt glue.
   Most of Phase 4 becomes mechanical afterwards.
3. P3.2 + P3.3 (EXTI and pin modes), then P3.1.
4. P6.1 before any new i.MX 9 driver work, since without it drivers fault on real
   hardware (P1.4 covered i.MX 93).
5. Phase 4 and Phase 5 by demand.
