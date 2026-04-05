# Consortium IPC Implementation Status

**Date:** 2026-04-05
**Current Focus:** Hardware-specific implementations and runtime layers

---

## Executive Summary

The **core IPC abstractions** (traits, Channel, Transport, codec) are complete and tested. The **high-level architecture** (design documents, TEE proc macros with advanced type support) is well-defined.

What remains is **hardware-specific glue** and the **application runtime layer** to tie everything together into a deployable system.

---

## Completed ✅

### Core IPC Layer (`consortium-ipc`)
- **Traits:** `Doorbell`, `Region`, `Transport`, `Codec`, `Channel`
- **Channel API:** Full typed, allocation-free async sender/receiver
- **Error handling:** `ChannelError<C, Tr>` distinguishing codec vs transport failures
- **Features:** `std`, `alloc`, `futures` gates; `no_std` by default
- **Testing:** Unit tests for trait contracts

### Transport Implementations
- **`consortium-ipc-mem` (bare-metal):**
  - `MemTransport<D: Doorbell, R: Region>` generic over doorbell and region types
  - Slot protocol (len/flags/payload) with volatile I/O
  - SendFut (eager, synchronous doorbell.ring)
  - RecvFut (polling with async doorbell.wait)
  - ~225 LOC, production-ready

- **`consortium-ipc-uio` (Linux/Tokio):**
  - `UioDevice::open(uio_num)` parsing binary descriptor
  - Channel multiplexing via `uio.channel(id)`
  - Background Tokio task for M→A interrupt handling
  - Descriptor lifecycle state machine (`KernelInit` → `Ready`)
  - ~320 LOC, Linux-only, tested

### Memory Region Implementations (`consortium-ipc` sub-modules)
- **`SramRegion`:** Cache-coherent, no-op flush/invalidate
- **`DdrRegion`:** Non-coherent, AArch64 `dc cvac`/`dc ivac` + `dsb sy`
- **`SerialRegion`:** Virtual region for UART/SWD logging

### Codec
- Default: `PostcardCodec` via `postcard` crate (serde)
- Compact binary, `no_std` compatible
- GAT-based `Decoded<'buf, T>` for zero-copy future support

### TEE Support (`consortium-tee` + `consortium-tee-proc-macros`)
- **Error types:** `TeeError`, `TeeErrorOrigin`, `TeeResult` with `is_transient()`
- **`tee_service!` macro:** Generates `Command` enum + dispatcher
- **`#[tee_command]` macro:** Advanced type support including:
  - Input types: `bool`, `u32`, `&[u8]`, `&mut [u8]`, generics with `Serialize`/`Deserialize`
  - Return types: `u32`, `u64`, `bool`, `Vec<u8>`
  - Context passing (first `&mut` arg, excluded from TEE slots)
  - CA-side `call_*` stubs (`#[cfg(feature = "ca")]`)
  - TA-side `*_dispatched` handlers with full parameter unpacking
- ~929 LOC of sophisticated proc macros

### Configuration (`consortium-common`)
- **`Config`:** Top-level with `general` + `project` sections
- **`Chip` enum:** STM32MP2x, i.MX 9x families with traits (`has_rt_core()`, `remoteproc_name()`, etc.)
- **`CortexMcuCore` enum:** M0p, M33, M7, M4/M4f with Rust target mapping

### Application Runtime (`consortium-runtime-app`)
- **`RemoteProc` sysfs wrapper:**
  - Index-based constructor mapping to `/sys/class/remoteproc/remoteprocN`
  - State reader: `Offline`, `Suspended`, `Running`, `Crashed`, `Deleted`, `Attached`, `Detached`
  - Controls: `start()`, `stop()`, `set_firmware()`, `set_coredump()`, `set_recovery()`
  - `RemoteProcCoredump` enum: `Enabled`, `Disabled`, `Inline`

### Hardware Doorbell: HSEM (`consortium-ipc-driver-hsem`)
- **M-core (bare-metal, no_std):** `HsemDoorbell<PROC>` as zero-size type with `AtomicWaker`. HSEM register access and IRQ handler registration.
- **A-core (Linux, std):** `HsemDoorbell<PROC>` holds `Arc<tokio::sync::Notify>`. Exposes `.notifier()` for UIO IRQ loop integration.
- **Channel validation:** `HsemChanValidate` for max 64 channels per core.
- **Re-exports:** From `consortium-ipc-mem` and `consortium-ipc-uio` for convenience; from `consortium-runtime-rt` as `hsem` for backward compatibility.
- **Platform support:** STM32MP2x and STM32MP25x SoCs
- **Status:** Production-ready with test coverage

### Documentation
- **ipc-design.md:** Complete IPC architecture (traits, layers, slot protocol, descriptor lifecycle)
- **tee-sdk-design.md:** OP-TEE Trusted Application SDK design
- **tee-advanced-type-system.md:** Proc macro advanced type support

---

## Partial / In Progress 🟡

### Build System (`consortium-builder`)
**Status:** Minimal stub, ~19 LOC
**Planned:**
- `ApplicationBuilder` for cross-compilation (Cortex-A35 / OpenSTLinux SDK)
- `FirmwareBuilder` for firmware binary compilation (M-core targets)
- Environment variable handling (`SDKTARGETSYSROOT`, `OECORE_*`, etc.)
- ELF magic verification post-build
- Release mode support

**What's Missing:**
- SDK environment detection
- Cargo invocation with proper flags
- Binary name extraction from `Cargo.toml`
- Target selection based on `CortexMcuCore`

### CLI Tool (`consortium-cli`)
**Status:** Stub, no implementation
**Planned:** Debug tooling for IPC monitoring, state inspection

---

## Remaining Doorbell Implementations 🟡

### ✅ Completed: HSEM (Hardware Semaphore) for STM32MP2/STM32MP25

HSEM is now fully implemented in `consortium-ipc-driver-hsem` with:
- Bare-metal M-core support (no_std + AtomicWaker)
- Linux A-core support (std + tokio::sync::Notify)
- Channel validation and error handling
- Re-exports for convenience in `consortium-ipc-mem` and `consortium-ipc-uio`

### Required Additional Implementations

**i.MX 9x Series (NXP) targets (i.MX 93, i.MX 95, etc.):**
- **MU (Message Unit):** Primary signaling mechanism
  - Register layout: TX/RX mailboxes, interrupt enable/status
  - Interrupt routing via GIC (Generic Interrupt Controller)
  - Platform-specific register definitions for each variant
  - Interrupt handler registration
  - Tests on actual hardware
  - Implementation pattern: follow `consortium-ipc-driver-hsem` structure

**STM32MP2 / STM32MP25 (STMicroelectronics):**
- HSEM is the sole IPC signaling mechanism on these platforms; no additional doorbell implementations required

#### Implementation Pattern (for each Doorbell type)

1. **Trait impl `Doorbell` for `XyzDoorbell`:**
   ```rust
   pub struct HsemDoorbell {
       channel_id: u8,
       register_base: *mut u32
   }

   impl Doorbell for HsemDoorbell {
       fn ring(&self, ch: Chan) -> Result<(), Self::Error> { /* write register */ }
       fn wait<'a>(&self, ch: Chan) -> Self::WaitFut<'a> { /* poll or async irq */ }
   }
   ```

2. **Platform detection:** Gate behind feature flags or runtime chip detection
3. **Initialization:** Expose safe constructor that validates platform
4. **Testing:** Unit tests with mocked register access; integration tests on real hardware

### Application Runtime Layer (`consortium-runtime-rt`)

**Status:** Stub, just a dummy `add()` function
**Critical gaps:**

1. **Firmware execution harness (M-core):**
   - Bootstrap code (stack setup, relocation, initialization)
   - Async executor (cooperative or RTIC-based)
   - Interrupt handler registration
   - Integration with MemTransport for IPC
   - Reset/panic handlers
   - Example: minimal firmware that spins up a Channel and sends data
   - Current design: use `Embassy` for async runtime with `no_std` support. When the project is targeting wider market, try to support Zephyr RTOS as well, which is widely used in embedded space and has good support for STM32 and NXP platforms.

2. **Application harness (A-core Linux):**
   - RemoteProc lifecycle management
   - Firmware loading and start
   - Error recovery and logging
   - Integration with UioDevice for IPC
   - Graceful shutdown
   - Example: minimal app that opens UIO, creates Channel, receives firmware messages

3. **Glue code:**
   - SharedMemory descriptor initialization (kernel-side or M-core-side?)
   - MTU negotiation
   - Channel table setup

### Full Compilation & Deployment Pipeline

The user specified an 9-step pipeline:

```
1. Meta crate: Config → device tree + PAC
2. Compile TF-M (NS-M) + bundle into firmware ELF per remoteproc
3. Compile TF-A (OP-TEE) + optionally sign
4. Compile HMI web interface (Vite) + bundle into app
5. Quantize ONNX models + bundle into app
6. Compile application
7. Generate config files (systemd, supervisor, nginx, etc.)
8. Attach installer script
9. Bundle into .tar.gz
```

**Status:** Not started. Requires:
- Device tree generation tooling (from `Config`)
- Build orchestration (Cargo build scripts or separate build crate)
- Signing workflow (cargo-optee integration or custom)
- Web asset bundling (Vite integration)
- Systemd/supervisor config generation
- Installer script templates

### Distributed Testing

No current test harness for:
- Two-core IPC over actual shared memory
- Interrupt signaling between cores
- Cache coherency on real hardware
- Actual remoteproc firmware loading

---

## Crate Mapping (Current)

| Name | Status | Description |
|------|--------|-----------|
| `consortium-ipc` | ✅ Core traits | Doorbell, Region, Transport, Channel, Codec |
| `consortium-ipc-mem` | ✅ Bare-metal transport | MemTransport with generic doorbell/region |
| `consortium-ipc-uio` | ✅ Linux UIO transport | UioDevice and UioChannel via Tokio |
| `consortium-ipc-driver-hsem` | ✅ HSEM doorbell | Hardware Semaphore for STM32MP2/MP25 |
| `consortium-runtime-app` | 🟡 A-core runtime | RemoteProc sysfs wrapper (requires expansion) |
| `consortium-runtime-rt` | ❌ M-core runtime | Stub (requires bootstrap + executor) |
| `consortium-builder` | 🟡 Build system | Minimal (requires cross-compilation helpers) |
| `consortium-common` | ✅ Configuration | Config, Chip, CortexMcuCore types |
| `consortium-tee` | ✅ TEE support | TeeError, TeeResult types |
| `consortium-tee-proc-macros` | ✅ TEE macros | tee_service!, #[tee_command] with advanced types |
| `consortium-cli` | ❌ Debug tool | Stub (requires IPC monitoring) |

---

## Dependency Graph

```
Application (user code)
├── consortium-runtime-app
│   ├── consortium-ipc-uio
│   │   └── consortium-ipc (with std feature)
│   └── consortium-tee (for CA-side calls)
│
└── Firmware (user code)
    └── consortium-ipc-mem
        └── consortium-ipc (with no_std)
```

---

## Testing Status

| Component | Tests | Notes |
|-----------|-------|-------|
| `consortium-ipc` | ✅ Trait tests | Basic contract verification |
| `consortium-ipc-mem` | ✅ Transport tests | Slot protocol, doorbell mocking |
| `consortium-ipc-uio` | ✅ Descriptor parsing | Linux-only; no real UIO device tests |
| `consortium-tee-proc-macros` | ✅ Macro expansion | Compile-time correctness |
| `consortium-ipc-driver-hsem` | ✅ HSEM tests | Register access, async wakeup, channel validation |
| Doorbells (MU) | ❌ None | Not implemented |
| Runtime (M-core) | ❌ None | Not implemented |
| Runtime (A-core) | ⚠️ Partial | RemoteProc methods exist but not tested |
| E2E (firmware ↔ app) | ❌ None | Would require actual hardware or simulation |

---

## Recommended Implementation Order

### ✅ Phase 0: HSEM Doorbell (COMPLETED)
1. **HSEM Doorbell** (`consortium-ipc-driver-hsem`)
   - Register-based implementation with no_std and std variants
   - Tests for async wakeup and channel validation
   - Platform support for STM32MP2x and STM32MP25x

### Phase 1: M-Core & A-Core Runtime (using HSEM)
1. **M-core bootstrap** (`consortium-runtime-rt/src/lib.rs`)
   - Stack/BSS setup
   - Simple polling executor or RTIC integration
   - MemTransport initialization with HsemDoorbell
   - Example firmware that sends a value per IRQ

2. **A-core app skeleton** (`consortium-runtime-app/src/lib.rs` expansion)
   - RemoteProc state machine for lifecycle management
   - UioDevice integration with HSEM notifications
   - Firmware ELF loading
   - Simple recv loop

3. **Integration test (STM32MP2 with HSEM)**
   - Load firmware on M-core
   - Open UioDevice on A-core
   - Exchange messages via HSEM signaling
   - Verify correctness on real hardware

### Phase 2: NXP i.MX 9x Port (MU Doorbell)
1. Implement MU Doorbell for i.MX 93, i.MX 95, etc., following HSEM pattern
2. Extend M-core and A-core runtimes for NXP platform
3. Test cross-core coordination on NXP hardware

### Phase 3: Build Pipeline
1. Implement `consortium-builder` for firmware compilation
2. Wire up device tree generation
3. Add OP-TEE compilation
4. Package everything into `.tar.gz`

### Phase 4: Polish & Documentation
1. Add hardware-specific examples (HSEM, MU)
2. Write integration test suites
3. Document chip selection and features
4. Update CLAUDE.md ← **Done**

---

## CLAUDE.md Update Required

The CLAUDE.md file references outdated crate names and needs immediate updates:

**Changes:**
- `consortium-mem` → `consortium-ipc-mem`
- `consortium-uio` → `consortium-ipc-uio`
- `consortium-app` → `consortium-runtime-app`
- `consortium-rt` → `consortium-runtime-rt`
- Update test commands to match new names
- Add note: "Hardware-specific Doorbell implementations (HSEM, MU) must be provided by platform"
- Add note: "M-core and A-core runtimes are stubs; require implementation per platform"
- Update section on "what's left": doorbells, runtimes, build pipeline

---

## Summary: What's Left

| Layer | Status | Effort | Dependency |
|-------|--------|--------|-----------|
| Traits + Channel | ✅ Done | — | — |
| MemTransport (bare-metal) | ✅ Done | — | Doorbell trait |
| UioTransport (Linux) | ✅ Done | — | Kernel UIO driver |
| **Doorbell: HSEM (STM32MP2/MP25)** | ✅ Done | — | — |
| **Doorbell: MU (i.MX 9x series)** | ❌ TODO | Medium | Hardware knowledge |
| **M-core runtime + executor (STM32MP2)** | ❌ TODO | High | HSEM (now available) |
| **A-core runtime + lifecycle (STM32MP2)** | ❌ TODO | High | RemoteProc + UIO expansion |
| **Build pipeline (8-step)** | ❌ TODO | High | All prior phases |
| TEE support (macros) | ✅ Done | — | — |
| Configuration types | ✅ Done | — | — |

**Total estimated effort for M-core + A-core runtime (STM32MP2 with HSEM):** 2–3 weeks (HSEM now complete).

**Total estimated effort for NXP i.MX 9x port (MU + runtimes + build pipeline):** 4–6 weeks additional time after STM32MP2 is complete.

