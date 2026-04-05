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

## Not Started / Blocked ❌

### Hardware-Specific Doorbells

This is **the critical blocker** for real-world deployment.

#### Required Implementations

**STM32MP2 / STM32MP25 targets:**
- **HSEM (Hardware Semaphore):** For M-core wakeup from A-core
  - Driver interaction: read/write `/dev/semaphore*` or mmap HSEM registers
  - Interrupt line configuration
  - Tests on actual hardware

- **MU (Message Unit):** For M→A signaling
  - Register layout: TX/RX mailboxes, interrupt enable/status
  - Interrupt handler registration
  - Tests on actual hardware

**i.MX 95 targets:**
- **MU (Message Unit):** Primary signaling
  - Register definitions for i.MX 95 variant
  - Potentially different register layout than STM32
  - Interrupt routing via GIC
  - Tests on actual hardware

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

The user specified an 8-step pipeline:

```
1. Meta crate: Config → device tree + PAC
2. Compile TF-M (NS-M) + bundle into firmware ELF per remoteproc
3. Compile TF-A (OP-TEE) + optionally sign
4. Compile HMI web interface (Vite) + bundle into app
5. Compile application
6. Generate config files (systemd, supervisor, nginx, etc.)
7. Attach installer script
8. Bundle into .tar.gz
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

## Crate Mapping (Current vs. CLAUDE.md)

| CLAUDE.md (Outdated) | Current | Status |
|------|---------|--------|
| `consortium-ipc` | `consortium-ipc` | ✅ Core traits |
| `consortium-mem` | `consortium-ipc-mem` | ✅ Bare-metal transport |
| `consortium-uio` | `consortium-ipc-uio` | ✅ Linux UIO transport |
| `consortium-app` | `consortium-runtime-app` | 🟡 RemoteProc sysfs only |
| `consortium-rt` | `consortium-runtime-rt` | ❌ Stub |
| `consortium-builder` | `consortium-builder` | 🟡 Minimal |
| `consortium-common` | `consortium-common` | ✅ Config types |
| `consortium-tee` | `consortium-tee` | ✅ Error types + re-exports |
| `consortium-tee-proc-macros` | `consortium-tee-proc-macros` | ✅ Advanced macros |
| `consortium-cli` | `consortium-cli` | ❌ Stub |

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
| Doorbells (HSEM, MU) | ❌ None | Not implemented |
| Runtime (M-core) | ❌ None | Not implemented |
| Runtime (A-core) | ⚠️ Partial | RemoteProc methods exist but not tested |
| E2E (firmware ↔ app) | ❌ None | Would require actual hardware or simulation |

---

## Recommended Implementation Order

### Phase 1: Proof of Concept (HSEM on STM32MP2)
1. **HSEM Doorbell** (`consortium-ipc-*.rs`)
   - Minimal register-based implementation
   - Mocked tests first
   - Real hardware tests with devboard

2. **M-core bootstrap** (`consortium-runtime-rt/src/lib.rs`)
   - Stack/BSS setup
   - Simple polling executor
   - MemTransport initialization
   - Example firmware that sends a value per IRQ

3. **A-core app skeleton** (`consortium-runtime-app/src/lib.rs` expansion)
   - RemoteProc state machine
   - UioDevice integration
   - Firmware ELF loading
   - Simple recv loop

4. **Integration test**
   - Load firmware on M-core
   - Open UioDevice on A-core
   - Exchange messages
   - Verify correctness

### Phase 2: Multi-core & MU
1. Implement MU Doorbell for same target
2. Test cross-core coordination
3. Extend to i.MX 95

### Phase 3: Build Pipeline
1. Implement `consortium-builder` for firmware compilation
2. Wire up device tree generation
3. Add OP-TEE compilation
4. Package everything into `.tar.gz`

### Phase 4: Polish & Documentation
1. Add hardware-specific examples
2. Write integration test suites
3. Document chip selection and features
4. Update CLAUDE.md ← **Do this first**

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
| **Doorbell: HSEM (STM32MP2)** | ❌ TODO | Medium | Hardware knowledge |
| **Doorbell: MU (STM32MP2/i.MX95)** | ❌ TODO | Medium | Hardware knowledge |
| **M-core runtime + executor** | ❌ TODO | High | Doorbell impl |
| **A-core runtime + lifecycle** | ❌ TODO | High | RemoteProc expanded |
| **Build pipeline (8-step)** | ❌ TODO | High | All prior phases |
| TEE support (macros) | ✅ Done | — | — |
| Configuration types | ✅ Done | — | — |

**Total estimated effort for full system:** 4–6 weeks for a single platform (STM32MP2 + HSEM), plus additional time for multi-platform support and production hardening.

