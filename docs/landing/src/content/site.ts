/**
 * Everything the landing page says lives here (copy, cards, chips, targets)
 * plus the code samples in `src/snippets/`. Edit this file, run `vp dev`, and
 * the page re-renders. Keep claims in step with the crate sources: when an API
 * changes, update the matching snippet in `src/snippets/` too.
 *
 * Rich-text fields accept `inline code` in backticks.
 */

import type { Site } from './types.ts'

const github = 'https://github.com/consortium-rs/consortium'
const docs = 'https://book.csti.dev'

export const site: Site = {
  meta: {
    title: 'Consortium — One workspace for every core in your SoC',
    description:
      'A Rust workspace for heterogeneous SoCs: typed IPC between Linux and real-time cores, one manifest for the whole build, OP-TEE commands as plain functions.',
    url: 'https://csti.dev',
  },

  links: { github, docs },

  nav: [
    { label: 'Architecture', href: '#architecture' },
    { label: 'Manifest', href: '#manifest' },
    { label: 'Transports', href: '#transports' },
    { label: 'Crates', href: '#crates' },
    { label: 'Targets', href: '#targets' },
    { label: 'Docs', href: docs },
  ],

  hero: {
    eyebrow: 'Rust for HAMPU systems',
    title: 'One workspace for every component in your SoC.',
    lede: 'Consortium pairs a Linux-capable A-core with Cortex-M real-time cores, the heterogeneous and asymmetric multiprocessing (HAMPU) pattern behind SoCs such as i.MX 9 and STM32MP2, with typed IPC, explicit cross-core contracts, and code generated from one manifest.',
    primary: { label: 'Read the docs', href: docs },
    secondary: { label: 'View source', href: github },
    code: { title: 'mcu/src/main.rs · CM33', snippet: 'hero-mcu.rs', lang: 'rust' },
  },

  architecture: {
    id: 'architecture',
    title: 'Explicit contracts across the core boundary',
    lede: 'The Linux host sits at the center: talking down to the real-time controller, up into the secure world, left to the kiosk interface, and right into the broader ecosystem.',
    nodes: {
      secure: {
        tag: 'TrustZone',
        title: 'OP-TEE',
        sub: 'Secure World via Teaclave',
        accent: 'accent2',
        badge: 'unverified',
      },
      interface: {
        tag: 'Interface',
        title: 'Kiosk HMI',
        sub: 'Cog / PocketJS',
        accent: 'neutral',
        badge: 'unverified',
      },
      host: {
        tag: 'Host',
        title: 'Linux',
        sub: 'Tokio, UIO, aarch64 / armv7',
        accent: 'accent',
      },
      ecosystem: {
        tag: 'Ecosystem',
        title: 'Integration Layers',
        sub: 'ONNX & ROS & Zenoh',
        accent: 'neutral',
        badge: 'in development',
      },
      controller: {
        tag: 'Controller',
        title: 'Bare-metal',
        sub: 'Embassy and defmt',
        accent: 'accent2',
      },
      wired: {
        tag: 'Wired',
        title: 'Remote Controller',
        sub: 'External components like ESC',
        accent: 'neutral',
      },
    },
    links: {
      secure: '#[tee_command]',
      controller: 'Transceiver + doorbell',
      wired: 'FDCAN / UART',
    },
    footnote:
      'Badged parts are not production-ready: TEE code is generated to compile against Teaclave but is not yet formally tested, the HMI is untested, and the integration layers are in development.',
  },

  manifest: {
    id: 'manifest',
    title: 'One manifest drives the whole build.',
    body: [
      '`csti` reads `Consortium.toml`, validates it against the embedded chip database, and generates a `consortium.gen.rs` for every endpoint, including memory map, doorbell vectors, time driver, and a synchronous `init()` that hands back typed `Peripherals` and `Channels`.',
      'Each channel then runs its own async `Transceiver::connect` handshake, so bring-up is one line per side with no `Primary`/`Secondary` branch.',
    ],
    commands: [
      { cmd: 'csti check', desc: 'Validate the manifest and `cargo check` every endpoint.' },
      {
        cmd: 'csti build --linux <DIR>',
        desc: 'Build app, firmware, TA and device tree into a deployable tree.',
      },
      { cmd: 'csti fetch', desc: 'Prefetch dependencies so `build` can run `--offline`.' },
    ],
    tabs: [
      { title: 'Consortium.toml', snippet: 'manifest.toml', lang: 'toml' },
      { title: 'shared/ipc.rs', snippet: 'shared-ipc.rs', lang: 'rust' },
      { title: 'app/main.rs', snippet: 'app-main.rs', lang: 'rust' },
      { title: 'ta/lib.rs', snippet: 'ta-lib.rs', lang: 'rust' },
    ],
  },

  transports: {
    id: 'transports',
    title: 'Pick the wire, keep the channel',
    lede: 'Every transport implements the same `SendTransport`, `RecvTransport` and `Connect` traits, so `Channel` and `Transceiver` code is unchanged whichever one carries it.',
    items: [
      {
        name: 'Shared memory',
        crate: 'consortium-ipc-transport-memory',
        signal: 'IPCC · MU · HSEM · GPIO doorbells',
        desc: 'A descriptor plus 64-byte-aligned slots in a shared window. Splits into Tx/Rx halves; `config.ack` adds acknowledged sends on their own doorbell lines.',
        status: 'Generated from `pattern = "memory"`',
        accent: 'accent',
      },
      {
        name: 'UART',
        crate: 'consortium-ipc-transport-uart',
        signal: 'byte arrival',
        desc: '`COBS(chan | payload | CRC32)` framing. Tokio serial on Linux, an IRQ-buffered `BufferedUart` on firmware.',
        status: 'Crate API',
        accent: 'accent2',
      },
      {
        name: 'FDCAN',
        crate: 'consortium-ipc-transport-fdcan',
        signal: 'frame arrival',
        desc: 'One header byte segments messages across classic or FD frames. Each direction binds its own CAN identifier.',
        status: 'Crate API',
        accent: 'accent2',
      },
    ],
    footnote:
      'Message types derive `IpcSafe`, which rejects pointers, references, `usize` and anything else that means something different on the other core.',
  },

  logs: {
    title: 'One log stream, both cores',
    lede: '`consortium-dbg` carries `defmt` frames off the M-core through a shared-memory ring; the generated app decodes them into `tracing` events beside the host’s own logs.',
    code: { title: 'melt-pot-stm32mp25-app · CA35', snippet: 'host.log', lang: 'log' },
  },

  crates: {
    id: 'crates',
    title: 'A crate for every layer',
    lede: 'From wire protocol to human interface, each concern is its own crate.',
    items: [
      {
        category: 'Core IPC',
        name: 'consortium-ipc',
        desc: 'Typed `Channel` and `Transceiver`, transport and doorbell traits, and `IpcSafe`.',
        accent: 'accent',
      },
      {
        category: 'Transport',
        name: 'consortium-ipc-transport-memory',
        desc: 'Descriptor/slot shared memory with IPCC, MU and HSEM doorbells.',
        accent: 'accent2',
      },
      {
        category: 'Transport',
        name: 'consortium-ipc-transport-uart',
        desc: 'COBS/CRC-framed IPC over a serial line, for Linux and firmware.',
        accent: 'accent',
      },
      {
        category: 'Transport',
        name: 'consortium-ipc-transport-fdcan',
        desc: 'Segmented IPC over classic and FD CAN frames.',
        accent: 'accent2',
      },
      {
        category: 'Codecs',
        name: 'consortium-codec',
        desc: 'postcard, prost and rkyv codec families for IPC and TEE.',
        accent: 'accent',
      },
      {
        category: 'Configuration',
        name: 'consortium-cfg',
        desc: '`Consortium.toml` validation, code generation, and DTS/UIO transpilers.',
        accent: 'accent2',
      },
      {
        category: 'HAL / PAC',
        name: 'consortium-hal',
        desc: 'Singleton-owned drivers and generated PACs for i.MX 9 and STM32MP2.',
        accent: 'accent',
      },
      {
        category: 'Build',
        name: 'csti',
        desc: 'The CLI: check, fetch and build every endpoint from one manifest.',
        accent: 'accent2',
      },
      {
        category: 'Debug',
        name: 'consortium-dbg',
        desc: 'Shared-memory `defmt` logging with an optional panic handler.',
        accent: 'accent',
      },
      {
        category: 'Logging',
        name: 'consortium-log',
        desc: 'One `defmt`/`tracing` facade so generated and cross-platform code share a call site.',
        accent: 'accent2',
      },
      {
        category: 'TEE',
        name: 'consortium-tee',
        desc: 'Write OP-TEE commands as plain Rust functions; `#[tee_command]` generates code that compiles against Teaclave.',
        accent: 'accent',
        badge: 'untested',
      },
      {
        category: 'HMI',
        name: 'consortium-hmi',
        desc: 'Backend-neutral command bridge for WPE WebKit and PocketJS kiosks.',
        accent: 'accent2',
        badge: 'untested',
      },
    ],
  },

  targets: {
    id: 'targets',
    title: 'Supported chips & targets',
    lede: 'STM32MP25 and i.MX 93 are officially supported and tested in hardware. STM32MP23 and STM32MP21 reuse the STM32MP25 support without formal verification, i.MX 95 is still being brought up, and i.MX 94 is planned.',
    chips: [
      {
        name: 'STM32MP25',
        cores: 'A35 + M33 + M0+ · NPU/GPU',
        controller: 'Cortex-M33',
        doorbells: 'IPCC',
        support: 'official',
        supportLabel: 'Verified',
        note: 'Advanced industrial HMI, AI and multimedia.',
      },
      {
        name: 'i.MX 93',
        cores: 'A55 + M33 · NPU/GPU',
        controller: 'Cortex-M33',
        doorbells: 'MU',
        support: 'official',
        supportLabel: 'Verified',
        note: 'Low-power intelligent edge devices.',
      },
      {
        name: 'STM32MP23',
        cores: 'A35 + M33',
        controller: 'Cortex-M33',
        doorbells: 'IPCC',
        support: 'official',
        supportLabel: 'Derived from MP25',
        note: 'Mid-range HMI and lightweight edge AI.',
      },
      {
        name: 'STM32MP21',
        cores: 'A35 + M33',
        controller: 'Cortex-M33',
        doorbells: 'IPCC',
        support: 'in-progress',
        supportLabel: 'In progress',
        note: 'Cost-sensitive industrial control and gateways.',
      },
      {
        name: 'i.MX 95',
        cores: 'A55 + M7 + M33 · NPU/GPU/VPU',
        controller: 'Cortex-M7',
        doorbells: 'MU',
        support: 'in-progress',
        supportLabel: 'In progress',
        note: 'High-performance edge AI and multi-camera vision.',
      },
      {
        name: 'i.MX 94',
        cores: 'A55 + M7 + M33 · NPU',
        controller: '2 Cortex-M7 & 1 Cortex-M33',
        doorbells: 'MU',
        support: 'planned',
        supportLabel: 'Planned',
        note: 'Industrial and automotive networking with TSN.',
      },
    ],
    arch: {
      title: 'Architecture targets',
      lede: '`aarch64-unknown-linux-gnu` and `thumbv8m.main-none-eabihf` run on real hardware. The other listed targets compile the code, but have not run on a board yet. Dashed targets are not built yet; support is coming.',
    },
    groups: [
      {
        title: 'A-core Linux',
        accent: 'accent',
        targets: [
          { triple: 'aarch64-unknown-linux-gnu', status: 'verified' },
          { triple: 'armv7-unknown-linux-gnueabihf', status: 'compiles' },
        ],
      },
      {
        title: 'M-core bare-metal',
        accent: 'accent2',
        targets: [
          { triple: 'thumbv8m.main-none-eabihf', status: 'verified' },
          { triple: 'thumbv7em-none-eabihf', status: 'compiles' },
          { triple: 'thumbv6m-none-eabi', status: 'planned' },
        ],
      },
      {
        title: 'RISC-V',
        accent: 'neutral',
        targets: [
          { triple: 'riscv32imc-unknown-none-elf', status: 'planned' },
          { triple: 'riscv32imac-unknown-none-elf', status: 'planned' },
          { triple: 'riscv32imafc-unknown-none-elf', status: 'planned' },
        ],
      },
    ],
  },

  footer: {
    copyright: '© 2026 Consortium project · Apache-2.0',
    links: [
      { label: 'Docs', href: docs },
      { label: 'GitHub', href: github },
      { label: 'Crates.io', href: 'https://crates.io/crates/consortium-ipc' },
    ],
  },
}
