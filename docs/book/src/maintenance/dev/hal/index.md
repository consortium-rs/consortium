# On-Chip Peripheral Drivers

Alongside the cross-core IPC contracts, Consortium ships firmware-side drivers for the
on-chip peripherals a controller core drives directly. They live in the chip HAL crates —
`consortium-hal-imx9` for the NXP i.MX 9x family and `consortium-hal-stm32mp2` for the
STMicroelectronics STM32MP2 family — layered over the generated PAC crates and the small
shared `consortium-hal` trait crate.

Each chip HAL exposes a module per peripheral class, and each module is behind a Cargo
feature so a firmware image only compiles the drivers it uses. The current set is `adc`,
`can`, `dma`, `gpio`, `i2c`, `i3c`, `spi`, `timer`, `uart`, and `timer_driver`; all are
enabled by default alongside the crate's default chip selection. STM32MP2 also exposes
`lptim`. The crates are `no_std`, keep their unsafe blocks narrow, and hold chip-specific
detail inside the chip HAL rather than in the shared traits.

## A common shape

The drivers follow one construction and execution pattern, so moving between peripherals —
and between the two chip families — stays predictable.

- **Construction consumes a peripheral singleton.** `<hal>::init()` hands out the chip's
  singletons once as a `Peripherals` struct, and a driver's `new` constructor takes the
  one it drives (`I2c::new(p.LPI2C1, …)`). The singleton type carries the instance's
  register base, NVIC line, and — through the sealed `Instance::state()` — the
  per-instance interrupt bookkeeping (an `embassy_sync::waitqueue::AtomicWaker` and any
  flags), so no constructor takes an address or a `static` and none is `unsafe`. The one
  exception is STM32MP2 CAN, whose shared message-RAM window has no singleton yet.
  On i.MX 93 each constructor also opens the peripheral's CCM gate first (see
  `clock::lpcg`), because Linux closes every gate no kernel driver claims.
- **The blocking path needs no interrupts.** Every driver implements the relevant
  `embedded-hal` (or `embedded-can`) blocking trait, and those methods work purely by
  polling hardware registers. This is the simplest way to bring a peripheral up. Each
  poll loop is bounded by a `consortium_hal::timeout::Timeout` poll budget (set with the
  driver's `set_timeout`), so a stuck bus surfaces as `Error::Timeout` instead of a hang.
- **The async path is opt-in and IRQ-driven.** The same drivers implement the
  `embedded-hal-async` traits. Async methods park on the driver's waker and expect the
  peripheral's interrupt to be forwarded to the module's `on_interrupt::<T>()`, keyed by
  the same singleton type the driver consumed (`can::on_interrupt::<peripherals::CAN1>()`
  and so on), which wakes the pending future.
- **The application owns the vector table.** Per the workspace convention shared with the
  doorbell crates, no HAL crate installs `#[interrupt]` handlers. Firmware enables the
  peripheral's interrupt line and, from its own ISR, calls the driver's `on_interrupt`.
  Enable the chip HAL's `rt` feature to pull in the PAC vector table and the
  `#[interrupt]` attribute macro for that application code.

The sections below introduce the two bus drivers that most benefit from a conceptual
overview. GPIO, I2C, UART, DMA, and the Embassy time driver follow the same construction
and blocking/async split.

## CAN

The CAN drivers present a classic (non-FD) 2.0A/2.0B controller through the
`embedded-can` traits: blocking transfers via `embedded_can::nb::Can`, and async
`Can::transmit` / `Can::receive` once the instance's interrupt reaches
`can::on_interrupt::<T>()`. Both drivers deliberately implement a minimal, fixed mailbox
layout as a first cut; FD frames, extra mailboxes, and hardware acceptance filtering are
left as room to grow, and the receive filter accepts every identifier.

`Can::split` hands out independent `CanTx` and `CanRx` halves for code that transmits and
receives concurrently — `consortium-ipc-transport-fdcan` is the motivating caller. The
halves drive disjoint registers and storage and park on separate wakers in the shared
per-instance state; a single waker would let whichever half registered last clobber the other's
registration and strand it.

The two families differ in how message storage is arranged, reflecting the underlying IP:

- **i.MX 9x (FlexCAN).** `Can::new` takes the `CANn` singleton, its clock, and a
  `Config`.
  The driver uses a fixed two-mailbox layout: message buffer 0 transmits and message
  buffer 1 receives every frame. With only one receive mailbox, a frame arriving before
  the previous one is read is reported as `Error::Overrun` and lost, so a burst needs a
  task already parked in `receive`. FlexCAN's RX FIFO would remove the constraint.
- **STM32MP2 (FDCAN / Bosch M_CAN).** The `unsafe` `Can::new` additionally takes the
  shared CAN message-RAM base and this instance's byte offset within it.
  The SoC gives all FDCAN instances one shared `CAN_SRAM` window, so the caller
  partitions that window by handing each instance a distinct message-RAM offset, leaving
  at least `can::REGION_BYTES` between them. Within its slice, the driver lays out an
  eight-element receive FIFO 0 followed by a single dedicated transmit buffer, using
  8-byte classic elements. The FIFO is deep enough that a back-to-back burst — what a
  segmenting transport emits for one large message — survives a receiver that is not
  already parked. The async path enables interrupt line 0 (`FDCANx_IT0`).

## SPI

The SPI drivers are byte-oriented, full-duplex masters exposed through the
`embedded-hal` `SpiBus` trait (blocking) and its `embedded-hal-async` counterpart (async,
once the instance's interrupt reaches `spi::on_interrupt::<T>()`). `Spi::new` takes the
instance's singleton, its kernel clock, and a `Config`. Frames are eight bits, and chip-select handling
reflects each IP's model:

- **i.MX 9x (LPSPI).** The driver drives one hardware chip-select (`PCS`) and holds it
  asserted for the duration of each `SpiBus` call via `TCR.CONT`, releasing it on the
  final word of the call.
- **STM32MP2 (SPI2S v2).** The driver uses software slave management (`CFG2.SSM` +
  `CR1.SSI`) so the peripheral stays a master without driving a hardware `NSS` line —
  chip-select is the caller's to manage with a GPIO. Each `SpiBus` method programs
  `CR2.TSIZE` with the frame count and issues a single `CR1.CSTART`, so it reads as one
  hardware transaction; buffers longer than the 16-bit `TSIZE` field are split into
  back-to-back transactions.

Both drivers are structured so a DMA-backed data path can be layered on later — the byte
primitives push through the transmit data register and drain the receive data register —
but the interrupt-and-polling path is the default and only path today.

## I3C

Both chip HALs expose controller-mode I3C through their `i3c` feature. The common public
shape supports I3C SDR and legacy-I2C private transfers, repeated-start transactions,
broadcast CCCs, RSTDAA, and ENTDAA. Blocking operations poll the peripheral; async
operations require the application to forward the selected I3C interrupt to
`i3c::on_interrupt::<T>()`.

The NXP implementation follows Embassy MCXA's controller engine, which uses the same NXP
I3C IP as i.MX 9. The STM32MP2 implementation follows the STM32CubeMP2 LL/HAL controller
sequence and uses the same Embassy-style waker contract as the other Consortium drivers.
Target mode, HDR transfers, IBI handling, and DMA are not part of this first controller
implementation.

## Where drivers fit

These drivers cover the mechanics of talking to a peripheral. Deciding _which_ core owns a
peripheral, and enforcing that ownership across cores and secure domains, is the job of the
[Resource Partition Isolation subsystem](../rpi/index.md), which drives peripheral
ownership from `Consortium.toml` and the generated device tree. A controller core reaches
the peripherals its manifest assigns to it through `context.peripherals`, populated by the
generated `init()`.
