# Consortium Architecture

The Consortium is a asymmetric multiprocessing framework that cooperates non-secure Linux, secure OP-TEE, non-secure RTOS, and secure RTOS. The architecture of the Consortium is designed to provide a secure and efficient communication channel between the different components while maintaining the security and isolation of each component.

Traditionally, developing heterogeneous and asymmetric multiprocessing systems, e.g., NXP's i.MX 8 series and ST's STM32MP1 series, requires significant engineering effort to design and implement the communication channel between the different components. The Consortium aims to simplify this process by providing a unified framework that abstracts away the complexities of inter-component communication.

The Consortium architecture consists of the following components:

- **Consortium IPC**: A communication channel that allows the different components to communicate with each other securely and efficiently. It provides a unified API for sending and receiving messages between the components, abstracting away the underlying communication mechanisms. Asynchronous and type-safe is the default choice. We implement hardware-specific IPC mechanisms, e.g., HSEM in STM32MP series and MU in i.MX series, as IPC backends. You only need to write `.send().await?` and `.recv().await?` to send and receive messages, and the IPC layer will take care of the rest, including serialization, deserialization, and synchronization.
- **Consortium TEE**: A secure execution environment that runs on the OP-TEE OS. It provides a secure environment for executing sensitive code and handling sensitive data. The Consortium TEE is designed to be compatible with the GlobalPlatform TEE specifications, allowing developers to leverage existing TEE applications and libraries. It uses macro-based code generation to simplify the development of TEE applications and ensure type safety. You only need to write it like a normal Rust library, and the macro will generate the necessary boilerplate code for you.
- **Consortium Runtime**: A runtime library that runs on the non-secure Linux and non-secure RTOS. It provides a unified API for interacting with the Consortium IPC and Consortium TEE, allowing developers to easily integrate the Consortium into their applications. The Consortium Runtime is designed to be lightweight and efficient, minimizing the overhead of using the Consortium in your applications.
- **Consortium HMI**: A human-machine interface (HMI) library that provides a unified API for creating user interfaces on the non-secure Linux side. It abstracts away the complexities of different GUI frameworks and allows developers to create user interfaces using a simple and consistent API. The Consortium HMI is designed to be flexible and extensible, allowing developers to easily integrate it with their existing applications and frameworks. It allows whatever GUI framework you want, especially web-based GUI frameworks. We can bundle a WebGTK automatically, and you can also view the page via a browser on another device, e.g., your phone, to interact with the Consortium system. You only need to write the UI logic, and the HMI layer will take care of the rest, including rendering and event handling.
- **Consortium FFI**: A foreign function interface (FFI) library that provides a unified API for calling functions across the different components. It allows developers to call functions in Python to run OpenCV for computer vision tasks, call functions in C to run TensorRT for AI inference tasks, etc. Though we recommend using Rust for the best experience, you can still use other languages if you want. The Consortium FFI is designed to be flexible and extensible, allowing developers to easily integrate it with their existing applications and frameworks.
- **Consortium ONNX Runtime**: A library that provides a wrapper of `ort` for running ONNX models in the Consortium system. It allows developers to easily run ONNX models on the different components, leveraging the hardware acceleration capabilities of each component. The Consortium ONNX Runtime is designed to be efficient and easy to use, allowing developers to quickly integrate ONNX model inference into their applications. We also automatically bundle the runtime library and the model together, so you can just drop the model file into a specific directory and start using it without worrying about the details of loading the model and running inference.
- **Consortium Developer Tools**: A set of tools that assist developers in building, testing, and debugging their applications using the Consortium framework. These tools include a build system that simplifies the process of building and deploying applications to the different components, a testing framework that allows developers to easily write and run tests for their applications, and a debugging tool that provides insights into the communication between the different components.

The Consortium architecture is designed to be modular and extensible, allowing developers to easily add new components and features as needed. By providing a unified framework for inter-component communication and secure execution, the Consortium aims to simplify the development of heterogeneous and asymmetric multiprocessing systems, enabling developers to focus on building innovative applications rather than dealing with the complexities of inter-component communication.

## Inter-process communication (IPC)

Consortium IPC provides a unified API for sending and receiving messages between the different components of CPUs. It abstracts away the underlying communication mechanisms, allowing developers to focus on the logic of their applications rather than the details of inter-process communication.

The abstraction of the Consortium IPC is as follows:

```
doorbell -> transport -> codec -> channel
```

- **Doorbell**: The doorbell is the hardware mechanism that triggers the communication between the different components. It can be implemented using various hardware mechanisms, such as semaphores, message units, or shared memory. The Consortium IPC provides a unified API for interacting with the doorbell, allowing developers to easily trigger communication between the components.
- **Transport**: The transport layer is responsible for managing the communication channel between the different components. It handles the synchronization and coordination of messages, ensuring that messages are delivered reliably and in order. The Consortium IPC provides a unified API for interacting with the transport layer, allowing developers to easily manage the communication channel between the components.
- **Codec**: The codec layer is responsible for serializing and deserializing messages between the different components. It ensures that messages are properly formatted and can be correctly interpreted by the receiving component. The Consortium IPC provides a unified API for interacting with the codec layer, allowing developers to easily serialize and deserialize messages between the components.
- **Channel**: The channel layer is the abstraction that represents the communication channel between the different components. It provides a simple and consistent API for sending and receiving messages, abstracting away the complexities of the underlying communication mechanisms. The Consortium IPC provides a unified API for interacting with the channel layer, allowing developers to easily send and receive messages between the different components.

The final effect of the Consortium IPC is that developers can send and receive messages between the different components using a simple and consistent API, without worrying about the details of inter-process communication. This allows developers to focus on building innovative applications rather than dealing with the complexities of inter-component communication.

### IPC Layer Stack

The Consortium IPC is organized as a series of abstraction layers:

```
┌──────────────────────────────────────────────────────────────┐
│  Application Layer                                           │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Channel<Tx/Rx, MessageType, Transport, Codec>          │ │
│  │  .send(msg).await? / .recv().await?                     │ │
│  │  Type-safe, allocation-free, async                      │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│  Codec Layer                                                 │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Codec Trait: T <─→ [u8]                                │ │
│  │  Default: PostcardCodec (serde-based, compact binary)   │ │
│  │  Future: Zero-copy codecs (rkyv)                        │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│  Transport Layer                                             │
│  ├─ MemTransport (bare-metal M-core)                        │
│  │  Generic over Doorbell + Region                          │
│  │  Manages slot protocol, async polling                    │
│  └─ UioChannel (Linux A-core)                               │
│     Tokio-based, interrupt-driven                           │
└──────────────────────────────────────────────────────────────┘
                           │
┌────────────┬─────────────┴──────────────┬───────────┐
│            │                            │           │
▼            ▼                            ▼           ▼
Doorbell   Slot Protocol              Region        Channel
 │           │                          │            ID
 │           ▼                          ▼            │
 │   [len:u32|flags:u32|data]   Cache coherency   Chan(u8)
 │                              Management
 ▼
Hardware Signaling
├─ HSEM (STM32MP)
├─ MU (i.MX)
└─ GPIO/Notify (testing)
```

### Developer Experience

#### M-Core (Bare-Metal Firmware)

```rust
// Static buffer for IPC (sized to max message MTU)
static mut TX_BUF: [u8; 256] = [0u8; 256];
static mut HSEM_DOORBELL: HsemDoorbell = /* ... */;
static mut REGION: DdrRegion = /* ... */;

#[main]
async fn main() {
    // 1. Create transport (owning doorbell, managing slots)
    let mut transport = MemTransport::new(
        &mut HSEM_DOORBELL,
        &mut REGION,
        unsafe { TX_SLOT_PTR },  // physical address in shared SRAM
        unsafe { RX_SLOT_PTR },
    );

    // 2. Create typed channel for a specific message
    let mut tx: Channel<Tx, SensorData, _, PostcardCodec> =
        Channel::new(Chan(0), transport, unsafe { &mut TX_BUF });

    // 3. Send data (async, awaitable)
    loop {
        let data = SensorData {
            temperature: 26.9,
            timestamp_ms: system_time_ms(),
        };
        tx.send(&data).await?;  // blocks until A-core reads
        delay_ms(1000);
    }
}
```

#### A-Core (Linux + Tokio)

```rust
use consortium_runtime_app::{RemoteProc, ipc_uio::UioDevice};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load and start firmware
    let remoteproc = RemoteProc::new(0);
    remoteproc.set_firmware("firmware.elf")?;
    remoteproc.start()?;

    // 2. Open UIO device (waits for descriptor ready)
    let uio = UioDevice::open(0).await?;

    // 3. Create typed channel matching M-core
    let mut rx: Channel<Rx, SensorData, _, PostcardCodec> =
        Channel::new(Chan(0), uio.channel(0).await?, unsafe { &mut RX_BUF });

    // 4. Receive messages in loop (async, awaitable)
    loop {
        let msg = rx.recv().await?;
        println!("Temp: {}°C at {}ms", msg.temperature, msg.timestamp_ms);
    }

    // Cleanup
    remoteproc.stop()?;
    Ok(())
}
```

## Trusted Execution Environment (TEE)

The Consortium TEE provides a secure execution environment for running sensitive code and handling sensitive data. It is designed to be compatible with the GlobalPlatform TEE specifications, allowing developers to leverage existing TEE applications and libraries. The Consortium TEE uses macro-based code generation to simplify the development of TEE applications and ensure type safety. Developers can define their TEE applications using simple Rust functions, and the `#[tee_command]` macro will generate the necessary boilerplate code for both the Trusted Application (TA) running in the secure world and the Client Application (CA) running in the normal world. This allows developers to focus on writing their business logic without worrying about the complexities of TEE development.

### Developer Experience

```rust
#[derive(serde::Serialize, serde::Deserialize, TeeParam)]
pub struct HmacKey {
    key: [u8; 32],
}

// Define a TA with shared context and commands
tee_service! {
    context TaContext { counter: u32, hmac_key: [u8; 32] }
    commands { authenticate, encrypt_aes_cbc, derive_key_hmac }
}

#[tee_command]
fn authenticate(ctx: &mut TaContext, secret: &[u8]) -> Result<bool> {
    ctx.counter += 1;
    Ok(constant_time_compare(secret, &ctx.hmac_key))
}

#[tee_command]
fn encrypt_aes_cbc(
    ctx: &mut TaContext,
    plaintext: &[u8],
    ciphertext: &mut [u8],
) -> Result<u32> {
    // ciphertext is pre-allocated by CA
    let iv = &ctx.hmac_key[0..16];
    let key = &ctx.hmac_key[16..32];

    // Use optee_utee::crypto (secure, never copied to normal world)
    optee_utee::crypto::aes_cbc_encrypt(key, iv, plaintext, ciphertext)?;
    Ok(ciphertext.len() as u32)
}

#[tee_command]
fn derive_key_hmac(ctx: &mut TaContext, seed: &[u8]) -> Result<HmacKey> {
    // It allows serializable return types, and the macro will handle the packing/unpacking of parameters and return values.
    let mut key = [0u8; 32];
    optee_utee::crypto::hmac_sha256(seed, &ctx.hmac_key, &mut key)?;
    Ok(HmacKey { key })
}

// In the normal world, you can call these commands like this:

fn main() -> Result<()> {
    let session = ...;  // Initialize TEE client and open session

    let result = session.call_authenticate(b"my secret key")?;
    println!("Authenticated: {}", result);
}
```

## System Topology

### Typical Deployment: STM32MP2 / i.MX 95

```
┌─────────────────────────────────────────────────────────────┐
│  SoC (ARM-based HMP)                                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────────────┐      ┌──────────────────────┐   │
│  │  Cortex-A (A-core)   │      │  Cortex-M (M-core)   │   │
│  │  (Linux + OP-TEE)    │      │  (RTOS / Firmware)   │   │
│  │                      │      │                      │   │
│  │ ┌──────────────────┐ │      │ ┌──────────────────┐ │   │
│  │ │ Application      │ │      │ │ Firmware         │ │   │
│  │ │ ┌──────────────┐ │ │      │ │ ┌──────────────┐ │ │   │
│  │ │ │ UioDevice    │ │ │      │ │ │ MemTransport │ │ │   │
│  │ │ │ (Tokio)      │ │ │      │ │ │ (executor)   │ │ │   │
│  │ │ └──────────────┘ │ │      │ │ └──────────────┘ │ │   │
│  │ └────────┬─────────┘ │      │ └────────┬────────┘ │   │
│  │          │           │      │          │          │   │
│  │ OP-TEE   │           │      │          │          │   │
│  │ ┌─────┐  │           │      │          │          │   │
│  │ │ TAs │  │           │      │          │          │   │
│  │ └─────┘  │           │      │          │          │   │
│  └──────────┼───────────┘      └──────────┼──────────┘   │
│             │ remoteproc                  │               │
│             └──────────────┬───────────────┘               │
│                            │ shared memory                │
│  ┌────────────────────────────────────────────────────┐   │
│  │  Shared Memory Region (DDR / SRAM)                │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │   │
│  │  │ TX Slot  │  │ RX Slot  │  │ IPC Descriptor   │ │   │
│  │  │ (A→M)    │  │ (M→A)    │  │ (state machine)  │ │   │
│  │  └──────────┘  └──────────┘  └──────────────────┘ │   │
│  └────────────────────────────────────────────────────┘   │
│             ▲                            ▲                │
│             │ HSEM / MU IRQ             │ HSEM / MU IRQ  │
└─────────────┼────────────────────────────┼────────────────┘
              │                            │
          ┌───┘                            └───┐
          │ Doorbell signaling                 │
          └────────────────────┬───────────────┘
```

### Logical Component Interaction

```
┌──────────────────┐
│   Consortium     │
│   Application    │
│  (M-core FW)     │
└────────┬─────────┘
         │
         │ Channel<Tx, ...>
         │ .send().await
         ▼
    ┌─────────┐
    │  IPC    │
    │ Layer   │
    └────┬────┘
         │ slot write, doorbell ring
         ▼
┌─────────────────────────────┐
│  Shared Memory + Signaling  │
│  (SRAM or DDR)              │
└────┬────────────────────────┘
     │ interrupt notification
     ▼
┌──────────────────────────────┐
│   Consortium                 │
│   Application                │
│   (A-core Linux/Tokio)       │
│                              │
│  Channel<Rx, ...>            │
│  .recv().await               │
└──────────────────────────────┘
```

---

## Security Architecture

The Consortium is designed with a multi-layer security model:

### Layer 1: Isolation via TEE

- **Secure World (OP-TEE):** Runs Trusted Applications (TAs) that handle cryptographic operations, key management, and sensitive data processing
- **Normal World:** Runs Linux application on A-core, untrusted by default
- **M-Core Firmware:** Real-time, safety-critical code; isolated from Linux via hardware compartmentalization

### Layer 2: Channel-Level Isolation

- Each IPC channel is independently addressed (Chan ID)
- Permission model: Each component can only send/receive on authorized channels
- Message format: Type-safe via Rust generics—decoder enforces contract

### Layer 3: Memory Safety

- All IPC code is written in Rust, eliminating buffer overflows and use-after-free
- Shared memory accessed via volatile reads/writes (no compiler optimizations)
- Region trait enforces cache coherency rules

### Layer 4: Physical Isolation

- M-core and A-core execute in separate processor domains
- Hardware doorbell mechanisms prevent unauthorized signaling
- Shared memory region is mapped into both cores' address spaces with appropriate MPU/MMU restrictions

---

## Performance Characteristics

### Latency (single message roundtrip)

- **Bare-metal M-core → A-core (via HSEM):** ~1–5 µs (depends on interrupt handling)
- **A-core (Linux) → M-core (via MU):** ~10–100 µs (Linux scheduling + kernel)
- **Serialization (PostcardCodec):** < 1 µs for typical messages

### Throughput

- **Maximum message size:** Determined by descriptor MTU (typically 256–4096 bytes)
- **Slot capacity:** 1 message per direction (tx/rx slots are buffers, not queues)
- **Back-pressure:** Sender blocks on `send().await` until receiver reads; automatic flow control

### Memory Footprint

- **MemTransport:** ~128 bytes (for doorbell + region references)
- **Channel:** ~32 bytes (codec handle + buffer reference)
- **Shared memory:** Minimal—just slot headers (8 bytes each) + payload buffer

### Zero-Cost Abstractions

- Channel API compiles to direct memory writes + doorbell ring instruction
- Codec with monomorphization: no vtables, fully inlined
- Futures/async: zero-cost via Rust's async/await desugaring

---

## Supported Platforms

### STM32MP2 / STM32MP25 (ST Microelectronics)

- **A-core:** Cortex-A35 running Linux (OpenSTLinux SDK)
- **M-core:** Cortex-M33 / Cortex-M7
- **Doorbell mechanisms:**
  - HSEM (Hardware Semaphore) for M→A signaling
  - MU (Message Unit) for A→M signaling
- **Status:** Planned PoC (HSEM implementation in progress)

### i.MX 95 (NXP)

- **A-core:** Cortex-A35 running Linux
- **M-core:** Cortex-M33 (M-core) + Cortex-A71 (secondary real-time core)
- **Doorbell mechanisms:**
  - MU (Message Unit) for both directions
- **Status:** Planned support after STM32MP2

### Testing / Simulation

- **Mock Doorbell:** Tokio `Notify` for unit tests
- **In-memory Transport:** Virtual channels, no hardware required
- **Status:** All core traits tested; E2E tests require hardware

---

## Build & Deployment Pipeline

The Consortium system requires a 9-step compilation and deployment pipeline:

```
1. Config → Device Tree + PAC
   Input: YAML/TOML config (chip, cores, frequencies)
   Output: device tree .dtb, peripheral access crate

2. Model Quantization & Optimization
   Input: ONNX models, calibration datasets
   Process:
     → Quantize to INT8/FP16 (if specified)
     → Calibrate on representative data
     → Fuse operators, fold constants
     → Select optimal kernels per hardware
   Output: Optimized .ort files with metadata

3. TF-M/NS-M Compilation
   Input: M-core firmware sources
   Output: ELF binaries (secure + non-secure variants)

4. TF-A / OP-TEE Compilation
   Input: Trusted Firmware-A, OP-TEE sources
   Output: u-boot, TEE OS (optionally signed)

5. HMI Web UI Compilation
   Input: React/Vue/Svelte sources (if applicable)
   Output: Static assets, bundled into A-core app

6. Application Binary Compilation
   Input: A-core Rust application + quantized models
   Process:
     → Embed quantized models in binary (or separate package)
     → Link TEE libraries
     → Include FFI worker stubs
   Output: Linux ELF binary

7. Configuration Generation
   Input: Config + build artifacts
   Output: systemd units, supervisor configs, nginx configs

8. Installer Script
   Input: All artifacts from steps 1–7
   Output: setup.sh with pre/post-install hooks

9. Package
   Input: All artifacts
   Output: consortium-system-<version>.tar.gz
```

**Key addition (Step 2):** Model quantization is now a dedicated pipeline step, ensuring:
- Optimal model size/latency tradeoffs
- Hardware-specific optimizations
- Automatic calibration for INT8
- Reproducible builds with metadata tracking

---

## Development Workflow

### Typical Development Cycle

1. **Define message types** (Rust structs with `Serialize`/`Deserialize`)
2. **Write M-core firmware** using `MemTransport` API
3. **Write A-core application** using `RemoteProc` + `UioDevice` API
4. **Configure IPC channels** (address map, MTU, permissions)
5. **Build & package** using `consortium-builder`
6. **Deploy to hardware** (or qemu simulation)
7. **Debug** via sysfs state inspection, logging, trace tools

### Testing Strategy

- **Unit tests:** Each crate tests in isolation (mocked Doorbell, virtual Transport)
- **Integration tests:** M-core firmware + A-core app on real/emulated hardware
- **Stress tests:** High-frequency messages, large payloads, error injection
- **Security tests:** Unauthorized channel access, malformed messages, buffer overflow attempts

---

## Future Roadmap

### Short Term (Q2 2026)

- Complete HSEM / MU Doorbell implementations for STM32MP2
- Finish M-core runtime bootstrap and executor
- Expand A-core runtime with full lifecycle management
- Implement build pipeline orchestration

### Medium Term (Q3–Q4 2026)

- Multi-platform support (i.MX 95, additional STM32 variants)
- HMI framework integration (web-based dashboards)
- ONNX Runtime wrapper for AI inference
- Comprehensive test suite and CI/CD integration

### Long Term (2027+)

- FFI layer for Python/C function calls
- Distributed multi-SoC support (mesh networking)
- Over-the-air firmware updates
- Advanced diagnostics and observability tools

---

## Consortium TEE: Secure Execution Environment

The Consortium TEE provides a type-safe, macro-driven interface for building OP-TEE Trusted Applications (TAs) and calling them securely from the normal world (A-core Linux application).

### TEE Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  A-Core (Normal World - Linux)                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Consortium Application                                │ │
│  │  ┌──────────────────────────────────────────────────┐  │ │
│  │  │  CA (Client Application)                         │  │ │
│  │  │  call_authenticate(secret)                       │  │ │
│  │  │  call_encrypt(plaintext) → ciphertext            │  │ │
│  │  │  call_derive_key(seed)                           │  │ │
│  │  │                                                  │  │ │
│  │  │  (generated by #[tee_command] macro)             │  │ │
│  │  └──────────────────────────────────────────────────┘  │ │
│  └──────────────┬───────────────────────────────────────────┘ │
│                 │ optee_teec (TEE Client API)                │
│                 │ UUID, session, invoke_command              │
│                 ▼                                             │
│  OP-TEE Kernel Driver                                        │
│  └─────────────────────────────────────────────────────────┘ │
└────────────────────┬──────────────────────────────────────────┘
                     │ SMC (Secure Monitor Call)
┌────────────────────▼──────────────────────────────────────────┐
│  Secure World (OP-TEE OS)                                    │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  Trusted Application (TA)                              │  │
│  │  ┌──────────────────────────────────────────────────┐  │  │
│  │  │  TA Command Handlers (generated)                 │  │  │
│  │  │  fn authenticate_dispatched(ctx, params)         │  │  │
│  │  │  fn encrypt_dispatched(ctx, params)              │  │  │
│  │  │  fn derive_key_dispatched(ctx, params)           │  │  │
│  │  │                                                  │  │  │
│  │  │  Unpacks parameters, calls user logic            │  │  │
│  │  └──────────────────────────────────────────────────┘  │  │
│  │  ┌──────────────────────────────────────────────────┐  │  │
│  │  │  User-Defined Business Logic                     │  │  │
│  │  │  fn authenticate(ctx, secret) → bool             │  │  │
│  │  │  fn encrypt(ctx, plaintext) → Vec<u8>            │  │  │
│  │  │  fn derive_key(ctx, seed) → [u8; 32]             │  │  │
│  │  └──────────────────────────────────────────────────┘  │  │
│  └────────────────────────────────────────────────────────┘  │
│                                                               │
│  Hardware Crypto Engine (AES, SHA, RSA, etc.)               │
│  (optee_utee provides safe wrappers)                        │
└───────────────────────────────────────────────────────────────┘
```

### Developer Experience: Define Once, Generate Twice

Developers write a single Rust function and the `#[tee_command]` macro generates both TA and CA code automatically.

#### Step 1: Define the Trusted Application (Shared Library)

```rust
// my-ta/src/lib.rs
use consortium_tee_proc_macros::{tee_command, tee_service};

// Define context (shared state)
pub struct TaContext {
    counter: u32,
    hmac_key: [u8; 32],
}

// Define service with commands
tee_service! {
    context TaContext
    commands {
        authenticate,
        encrypt_aes_cbc,
        derive_key_hmac,
    }
}

// Business logic: just plain Rust
#[tee_command]
fn authenticate(ctx: &mut TaContext, secret: &[u8]) -> Result<bool> {
    ctx.counter += 1;
    Ok(constant_time_compare(secret, &ctx.hmac_key))
}

#[tee_command]
fn encrypt_aes_cbc(
    ctx: &mut TaContext,
    plaintext: &[u8],
    ciphertext: &mut [u8],
) -> Result<u32> {
    // ciphertext is pre-allocated by CA
    let iv = &ctx.hmac_key[0..16];
    let key = &ctx.hmac_key[16..32];

    // Use optee_utee::crypto (secure, never copied to normal world)
    optee_utee::crypto::aes_cbc_encrypt(key, iv, plaintext, ciphertext)?;
    Ok(ciphertext.len() as u32)
}

#[tee_command]
fn derive_key_hmac(ctx: &mut TaContext, seed: &[u8]) -> Result<Vec<u8>> {
    // Vec<u8> return: CA pre-allocates buffer, TA writes into it
    let mut key = vec![0u8; 32];
    optee_utee::crypto::hmac_sha256(seed, &ctx.hmac_key, &mut key)?;
    Ok(key)
}
```

#### Step 2: TA Binary (with Generated Dispatcher)

```rust
// my-ta/src/main.rs
#![no_main]
#![no_std]

use my_ta::*;

// Generated by macro: invoke_command(ctx, cmd_id, params)
#[no_mangle]
extern "C" fn TA_InvokeCommand(
    sess_ctx: *mut c_void,
    cmd_id: u32,
    param_types: u32,
    params: *mut TEE_Param,
) -> TEE_Result {
    // Dispatcher generated by tee_service! macro
    invoke_command(
        &mut TA_CONTEXT,
        cmd_id,
        param_types,
        unsafe { &mut *params.cast() },
    )
}

// Generated TA header by build.rs
include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
```

**What the macro generates:**
- `invoke_command()` dispatcher that matches `cmd_id` to function handlers
- For each `#[tee_command]` function, a `function_name_dispatched()` handler that:
  - Unpacks parameters from TEE slots (ParamValue, ParamMemref)
  - Validates parameter types
  - Calls the user's function
  - Packs return values back into output slots

#### Step 3: CA Client Application (with Generated Stubs)

```rust
// my-ca/src/main.rs
use my_ta::call_authenticate;
use my_ta::call_encrypt_aes_cbc;
use my_ta::call_derive_key_hmac;

#[tokio::main]
async fn main() -> Result<()> {
    // Generated stubs handle all TEE protocol machinery

    // Call 1: authenticate
    let secret = b"super-secret-password";
    let is_valid: bool = call_authenticate(secret).await?;
    println!("Authentication: {}", is_valid);

    // Call 2: encrypt
    let plaintext = b"Hello, Secure World!";
    let mut ciphertext = vec![0u8; plaintext.len() + 16];  // AES padding
    let encrypted_len = call_encrypt_aes_cbc(plaintext, &mut ciphertext).await?;
    ciphertext.truncate(encrypted_len as usize);
    println!("Encrypted: {:x?}", ciphertext);

    // Call 3: derive key
    let seed = b"random-seed-12345";
    let derived_key: Vec<u8> = call_derive_key_hmac(seed).await?;
    println!("Derived key: {:x?}", derived_key);

    Ok(())
}
```

**What the macro generates:**
- For each `#[tee_command]` function, a `call_function_name()` stub that:
  - Creates a TEE session (if not already open)
  - Packs parameters into ParamValue / ParamMemref structures
  - Invokes the TA via `optee_teec::Session::invoke_command()`
  - Unpacks return values
  - Returns as a native Rust type

### TEE Type System

The `#[tee_command]` macro supports advanced type conversion:

```rust
#[tee_command]
fn process(
    ctx: &mut TaContext,
    // Input types (TEE slots → Rust types)
    flag: bool,                          // ParamValue.a()
    count: u32,                          // ParamValue.a()
    data: &[u8],                         // ParamMemref.buffer()
    config: &mut [u8],                  // ParamMemref (mutable)
    custom: MyStruct,                    // serde::Deserialize

    // Output types (Rust types → TEE slots)
) -> Result<(u32, Vec<u8>)> {
    // User logic
    Ok((count * 2, data.to_vec()))
}
```

**Mapping:**
| Rust Type | TEE Slot | Direction |
|-----------|----------|-----------|
| `bool` | ParamValue.a (u32) | in/out |
| `u32` | ParamValue.a | in/out |
| `u64` | ParamValue (a + b) | in/out |
| `&[u8]` | ParamMemref (read-only) | in |
| `&mut [u8]` | ParamMemref (read-write) | in/out |
| `T: Serialize` | ParamMemref (postcard) | in |
| `Vec<u8>` | ParamMemref (output buffer) | out |
| `T: Serialize` | ParamMemref (postcard) | out |

### TEE Performance & Security

**Latency:**
- CA → TA call: ~100–500 µs (SMC + OP-TEE context switch)
- Crypto operation (SHA256): ~1–10 ms
- Total roundtrip: dominated by crypto, not IPC overhead

**Memory:**
- TA binary: ~100–500 KB
- CA shared memory buffers: 4 KB per call (for large payloads)

**Security Guarantees:**
- Data in TA never leaves secure world in plaintext
- Crypto keys isolated from normal world OS
- TA isolation enforced by hardware (TrustZone)
- Type safety prevents buffer overflow exploits in CA ↔ TA boundary

### Build Integration

```toml
# my-ta/Cargo.toml
[lib]
crate-type = ["rlib"]

[[bin]]
name = "ta"
crate-type = ["cdylib"]

[dependencies]
optee_utee = "0.4"
consortium-tee = { path = "../../consortium-tee" }
consortium-tee-proc-macros = { path = "../../consortium-tee-proc-macros" }
serde = { version = "1.0", features = ["derive"] }
postcard = "1.0"

[build-dependencies]
optee_utee_build = "0.4"

[features]
ca = []  # CA-side code generation (enabled by CA's build)
```

**Build process:**
1. `build.rs` runs `optee_utee_build::build()` → generates `user_ta_header.rs`
2. Macro generates `invoke_command()` dispatcher
3. `cdylib` compiles to TA binary
4. CA depends on `rlib` with `features = ["ca"]` → generates `call_*` stubs

---

## Consortium HMI: Human-Machine Interface

The Consortium HMI provides a unified framework for building responsive, real-time UIs on the A-core Linux side, with automatic backend integration and multi-protocol support (web, embedded GUI, mobile).

### HMI Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application (A-Core Linux)                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Tokio Runtime                                        │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  HMI Event Loop                                 │  │  │
│  │  │  ws_listen → recv_events → update_state         │  │  │
│  │  │              ↓                                   │  │  │
│  │  │  Render Template (Tera/Askama)                  │  │  │
│  │  │              ↓                                   │  │  │
│  │  │  Broadcast to WebSocket Clients                 │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  │  ┌─────────────────────────────────────────────────┐  │  │
│  │  │  State Management                               │  │  │
│  │  │  • System status (temps, voltages, frequencies) │  │  │
│  │  │  • Device state (on/off, mode, settings)        │  │  │
│  │  │  • User preferences (theme, language, etc.)     │  │  │
│  │  └─────────────────────────────────────────────────┘  │  │
│  └────────┬────────────────────────────────────────────────┘  │
│           │                                                    │
│  ┌────────┴────────────────────────────────────────────────┐  │
│  │  IPC / TEE / ORT Integration                           │  │
│  │  • Fetch sensor data via IPC (M-core)                  │  │
│  │  • Get auth status via TEE (A-core)                    │  │
│  │  • Run inference via ORT (AI model)                    │  │
│  └──────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
        │              │              │
        ▼              ▼              ▼
    ┌────────┐   ┌─────────┐   ┌───────────┐
    │ WebGTK │   │ Browser │   │ Mobile    │
    │ (local)│   │ (remote)│   │ (Tauri)   │
    └────────┘   └─────────┘   └───────────┘
```

### Developer Experience: Declarative UI + Reactive State

#### Step 1: Define State & Events

State in the HMI is the single source of truth. All UI updates derive from state changes.

```rust
// my-app/src/hmi/state.rs
use serde::{Deserialize, Serialize};
use consortium_hmi::State;

/// Application state (serializable for persistence/sync)
#[derive(State, Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    // Sensor data (from M-core via IPC)
    pub temperature: f32,
    pub humidity: f32,
    pub pressure_hpa: f32,

    // Device control
    pub device_mode: DeviceMode,
    pub target_temperature: f32,
    pub heating_enabled: bool,

    // User session
    pub is_authenticated: bool,
    pub username: String,
    pub user_role: UserRole,

    // UI state
    pub selected_tab: TabId,
    pub show_advanced_settings: bool,
    pub notification_queue: Vec<Notification>,

    // Metrics
    pub uptime_seconds: u64,
    pub last_update_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeviceMode {
    Idle,
    Heating { target: f32 },
    Cooling { target: f32 },
    Auto,
    Error { reason: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Guest,
    User,
    Administrator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabId {
    Overview,
    Control,
    Settings,
    Analytics,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub timestamp_ms: u64,
    pub dismissible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            temperature: 20.0,
            humidity: 50.0,
            pressure_hpa: 1013.25,
            device_mode: DeviceMode::Idle,
            target_temperature: 22.0,
            heating_enabled: false,
            is_authenticated: false,
            username: String::new(),
            user_role: UserRole::Guest,
            selected_tab: TabId::Overview,
            show_advanced_settings: false,
            notification_queue: Vec::new(),
            uptime_seconds: 0,
            last_update_ms: 0,
        }
    }
}
```

#### Step 2: Define Events (User Interactions)

Events are actions triggered by user interaction or external systems. They mutate the state.

```rust
// my-app/src/hmi/events.rs
use consortium_hmi::Event;

/// All possible user interactions and external triggers
#[derive(Event, Clone, Debug)]
pub enum AppEvent {
    // Sensor data updates (from M-core)
    SensorData {
        temperature: f32,
        humidity: f32,
        pressure_hpa: f32,
    },

    // User interactions
    SetMode(DeviceMode),
    SetTargetTemperature(f32),
    ToggleHeating,
    SelectTab(TabId),
    ShowAdvancedSettings(bool),

    // Authentication
    Login { username: String, password: String },
    Logout,

    // System events
    SystemHealthCheck,
    ClearNotification(usize),
    AddNotification(Notification),

    // Periodic updates
    Tick { elapsed_ms: u64 },
}

impl AppEvent {
    pub fn describe(&self) -> String {
        match self {
            Self::SensorData { temperature, .. } => {
                format!("Sensor update: temp={:.1}°C", temperature)
            }
            Self::SetMode(mode) => format!("Device mode: {:?}", mode),
            Self::Login { username, .. } => format!("Login: {}", username),
            _ => format!("{:?}", self),
        }
    }
}
```

#### Step 3: Define HMI Handler with Full State Management

```rust
// my-app/src/hmi/handler.rs
use consortium_hmi::hmi;
use crate::hmi::{AppState, AppEvent, Notification, NotificationLevel};

#[hmi]
pub struct AppHmi {
    template_cache: std::sync::Arc<tokio::sync::Mutex<TemplateCache>>,
}

impl AppHmi {
    pub fn new() -> Self {
        Self {
            template_cache: std::sync::Arc::new(tokio::sync::Mutex::new(TemplateCache::new())),
        }
    }

    /// Render current state to HTML (< 50ms typical)
    pub async fn render(&self, state: &AppState) -> Result<String> {
        use tera::{Tera, Context};

        let mut tera = Tera::new("templates/**/*.html")?;
        let mut context = Context::new();
        context.insert("state", &state);

        let template_name = match state.selected_tab {
            TabId::Overview => "overview.html",
            TabId::Control => "control.html",
            TabId::Settings => "settings.html",
            TabId::Analytics => "analytics.html",
        };

        tera.render(template_name, &context)
            .map_err(|e| format!("Render error: {}", e).into())
    }

    /// Process event and update state atomically
    pub async fn on_event(
        &self,
        event: AppEvent,
        state: &mut AppState,
        ipc: &mut IpcChannels,
        tee: &TeeSession,
        ort: &ModelPool,
    ) -> Result<bool> {
        let old_state = state.clone();

        match event {
            // Sensor data with optional ML
            AppEvent::SensorData { temperature, humidity, pressure_hpa } => {
                state.temperature = temperature;
                state.humidity = humidity;
                state.pressure_hpa = pressure_hpa;

                if state.is_authenticated && temperature > 30.0 {
                    let prediction = ort.predict(&[temperature, humidity, pressure_hpa]).await?;
                    if prediction.anomaly_score > 0.8 {
                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Warning,
                            message: "Temperature anomaly detected".to_string(),
                            timestamp_ms: state.last_update_ms,
                            dismissible: true,
                        });
                    }
                }
            }

            // Device control with authentication check
            AppEvent::SetMode(mode) => {
                if !state.is_authenticated && !matches!(mode, DeviceMode::Idle) {
                    state.notification_queue.push(Notification {
                        level: NotificationLevel::Error,
                        message: "Authentication required".to_string(),
                        timestamp_ms: state.last_update_ms,
                        dismissible: true,
                    });
                    return Ok(false);
                }

                state.device_mode = mode.clone();
                let cmd = match mode {
                    DeviceMode::Heating { target } => {
                        Command::SetHeating { enabled: true, target_temp: target }
                    }
                    DeviceMode::Cooling { target } => {
                        Command::SetCooling { enabled: true, target_temp: target }
                    }
                    DeviceMode::Idle => Command::SetIdle,
                    _ => return Ok(false),
                };

                ipc.cmd_channel.send(&cmd).await?;
            }

            // Temperature validation
            AppEvent::SetTargetTemperature(target) => {
                if target < 15.0 || target > 35.0 {
                    state.notification_queue.push(Notification {
                        level: NotificationLevel::Error,
                        message: "Temperature must be 15–35°C".to_string(),
                        timestamp_ms: state.last_update_ms,
                        dismissible: true,
                    });
                    return Ok(false);
                }
                state.target_temperature = target;
                ipc.cmd_channel.send(&Command::SetTargetTemp(target)).await?;
            }

            // TEE-based authentication
            AppEvent::Login { username, password } => {
                match tee.call_authenticate(&username, password.as_bytes()).await {
                    Ok(auth_result) if auth_result.valid => {
                        state.is_authenticated = true;
                        state.username = username.clone();
                        state.user_role = auth_result.role;

                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Info,
                            message: format!("Welcome, {}!", username),
                            timestamp_ms: state.last_update_ms,
                            dismissible: false,
                        });

                        if !matches!(state.user_role, UserRole::Administrator) {
                            state.show_advanced_settings = false;
                        }
                    }
                    _ => {
                        state.notification_queue.push(Notification {
                            level: NotificationLevel::Error,
                            message: "Invalid credentials".to_string(),
                            timestamp_ms: state.last_update_ms,
                            dismissible: true,
                        });
                    }
                }
            }

            AppEvent::SelectTab(tab) => {
                state.selected_tab = tab;
            }

            AppEvent::Tick { elapsed_ms } => {
                state.uptime_seconds += 1;
                state.last_update_ms = elapsed_ms;
                state.notification_queue.retain(|n| {
                    elapsed_ms - n.timestamp_ms < 10_000 || !n.dismissible
                });
            }

            _ => {}
        }

        Ok(old_state != *state)
    }
}
```

#### Step 4: Complete Integration in Main App

```rust
// my-app/src/main.rs
use my_app::hmi::{AppHmi, AppState, AppEvent};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    // 1. Setup runtime
    let remoteproc = RemoteProc::new(0);
    remoteproc.set_firmware("firmware.elf")?;
    remoteproc.start()?;

    let uio = UioDevice::open(0).await?;
    let tee = TeeSession::open().await?;

    // 2. Initialize state
    let mut state = AppState::default();
    let hmi = AppHmi::new();
    let (state_tx, _) = broadcast::channel(100);
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(1000);

    // 3. Task A: Listen for sensor updates from M-core
    let event_tx_clone = event_tx.clone();
    tokio::spawn(async move {
        let mut sensor_ch = uio.channel(1).await.unwrap();
        loop {
            match sensor_ch.recv::<SensorReading>().await {
                Ok(reading) => {
                    let event = AppEvent::SensorData {
                        temperature: reading.temperature,
                        humidity: reading.humidity,
                        pressure_hpa: reading.pressure,
                    };
                    let _ = event_tx_clone.send(event).await;
                }
                Err(e) => {
                    tracing::error!("Sensor error: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    // 4. Task B: Periodic tick
    let event_tx_clone = event_tx.clone();
    tokio::spawn(async move {
        let start = std::time::Instant::now();
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let _ = event_tx_clone.send(AppEvent::Tick { elapsed_ms }).await;
        }
    });

    // 5. Task C: WebSocket server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let state_tx_clone = state_tx.clone();
    let event_tx_clone = event_tx.clone();

    tokio::spawn(async move {
        loop {
            if let Ok((stream, peer_addr)) = listener.accept().await {
                tracing::info!("Client: {}", peer_addr);
                let state_tx = state_tx_clone.clone();
                let event_tx = event_tx_clone.clone();

                tokio::spawn(async move {
                    let _ = handle_websocket_client(stream, peer_addr, state_tx, event_tx).await;
                });
            }
        }
    });

    // 6. Main event loop
    while let Some(event) = event_rx.recv().await {
        tracing::debug!("Event: {}", event.describe());

        match hmi.on_event(event, &mut state, &mut IpcChannels {…}, &tee, &ort_pool).await {
            Ok(state_changed) if state_changed => {
                match hmi.render(&state).await {
                    Ok(html) => { let _ = state_tx.send(html); }
                    Err(e) => tracing::error!("Render: {}", e),
                }
            }
            Err(e) => {
                tracing::error!("Error: {}", e);
                state.notification_queue.push(Notification {
                    level: NotificationLevel::Error,
                    message: format!("Error: {}", e),
                    timestamp_ms: 0,
                    dismissible: true,
                });
            }
            _ => {}
        }
    }

    remoteproc.stop()?;
    Ok(())
}

async fn handle_websocket_client(
    stream: tokio::net::TcpStream,
    peer_addr: std::net::SocketAddr,
    state_tx: broadcast::Sender<String>,
    event_tx: tokio::sync::mpsc::Sender<AppEvent>,
) -> Result<()> {
    let ws = tokio_tungstenite::accept_async(stream).await?;
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut state_rx = state_tx.subscribe();

    loop {
        select! {
            msg = state_rx.recv() => {
                match msg {
                    Ok(html) => ws_tx.send(Message::Text(html)).await?,
                    Err(broadcast::error::RecvError::Lagged(_)) => {},
                    Err(e) => break,
                }
            }

            msg = ws_rx.next() => {
                match msg {
                    Some(Ok(Message::Text(json))) => {
                        if let Ok(event) = serde_json::from_str::<AppEvent>(&json) {
                            let _ = event_tx.send(event).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(_)) => break,
                    None => break,
                }
            }
        }
    }

    tracing::info!("Client closed: {}", peer_addr);
    Ok(())
}
```

#### Step 5: Full HTML Template with Styling & Interactivity

```html
<!-- templates/overview.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Consortium Control</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container { max-width: 1200px; margin: 0 auto; }
        .header { color: white; margin-bottom: 30px; }
        .header h1 { font-size: 2.5em; margin-bottom: 10px; }

        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .card {
            background: white;
            border-radius: 12px;
            padding: 24px;
            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
            transition: transform 0.2s;
        }
        .card:hover { transform: translateY(-5px); }

        .sensor-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px; }
        .sensor { text-align: center; padding: 15px; background: #f8f9fa; border-radius: 8px; }
        .sensor-value { font-size: 2.5em; font-weight: bold; color: #667eea; }

        button {
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
            background: #667eea;
            color: white;
            transition: all 0.2s;
        }
        button:hover { background: #5568d3; }

        .notification {
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 15px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🏠 Environmental Control</h1>
        </div>

        <div class="grid">
            <div class="card">
                <h2>📊 Sensors</h2>
                <div class="sensor-grid">
                    <div class="sensor">
                        <div>Temperature</div>
                        <div class="sensor-value">{{ state.temperature | round(precision=1) }}°C</div>
                    </div>
                    <div class="sensor">
                        <div>Humidity</div>
                        <div class="sensor-value">{{ state.humidity | round }}%</div>
                    </div>
                </div>
            </div>

            <div class="card">
                <h2>🎛️ Device</h2>
                <p>Mode: <strong>{% match state.device_mode %}
                    {% when DeviceMode::Idle %}Idle
                    {% when DeviceMode::Heating {target} %}Heating to {{ target }}°C
                {% endmatch %}</strong></p>
                <button onclick="send('SetMode', 'Heating')">Heat</button>
                <button onclick="send('SetMode', 'Idle')">Off</button>
            </div>
        </div>
    </div>

    <script>
        const ws = new WebSocket("ws://" + location.host + "/ws");
        ws.onmessage = (e) => document.documentElement.innerHTML = e.data;

        function send(action, value) {
            const event = {};
            event[action] = value;
            ws.send(JSON.stringify(event));
        }
    </script>
</body>
</html>
```

### HMI Key Insights

1. **State is immutable from render perspective:** Templates are pure functions of state
2. **Event-driven:** All mutations go through the event processor
3. **Broadcast pattern:** State changes sent to all connected clients instantly
4. **Error handling:** User-friendly notifications for all error conditions
5. **Integration:** Seamlessly connects IPC + TEE + ORT

### HMI Features

| Feature | Implementation |
|---------|-----------------|
| **Template Engine** | Tera/Askama for HTML generation |
| **State Management** | Tokio channels + broadcast |
| **WebSocket Transport** | tokio-tungstenite for real-time updates |
| **Multi-Protocol** | HTTP, WebSocket, optional Tauri for desktop |
| **Responsive Design** | CSS Grid/Flexbox, mobile-first |
| **Accessibility** | WCAG 2.1 AA semantic HTML |
| **Customization** | CSS variables, dark mode, internationalization |

### HMI Performance

- **State update latency:** < 100 ms (WebSocket broadcast)
- **WebSocket message size:** ~1–10 KB per update
- **Concurrent clients:** 100+ (typical)
- **Memory overhead:** ~1 MB per connected client

---

## Consortium ORT: ONNX Runtime Integration

The Consortium ORT (ONNX Runtime Wrapper) provides seamless AI model inference across all components (M-core, A-core, TEE) with automatic model bundling, hardware acceleration selection, and type-safe inference.

### ORT Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application                                     │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Inference Request                                   │  │
│  │  let output = model.predict(&input).await?           │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Consortium ORT Layer                               │  │
│  │  ├─ Model loader (ONNX .onnx file)                 │  │
│  │  ├─ Provider selector (CPU, GPU, NPU, etc.)        │  │
│  │  ├─ Session manager (pooling, caching)             │  │
│  │  └─ Type-safe I/O (ndarray, glam, etc.)            │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  ONNX Runtime (ort crate)                           │  │
│  │  ├─ Model parsing (protobuf)                        │  │
│  │  ├─ Graph execution (CPU threading)                 │  │
│  │  └─ Memory management (arena allocators)            │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Hardware Providers (Optional)                      │  │
│  │  ├─ NVIDIA CUDA (GPU inference)                     │  │
│  │  ├─ OpenVINO (Intel NPU/Movidius)                   │  │
│  │  ├─ QNN (Qualcomm Hexagon DSP)                      │  │
│  │  └─ CoreML (Apple Neural Engine)                    │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Developer Experience: Model-First API

#### Step 1: Define Inference Schema

```rust
// my-app/src/models.rs
use consortium_ort::Model;
use ndarray::Array2;

// 1. Define input/output types
pub struct ObjectDetectionInput {
    pub image: Array3<f32>,  // [height, width, 3] (RGB)
}

pub struct ObjectDetectionOutput {
    pub boxes: Vec<[f32; 4]>,      // [x1, y1, x2, y2]
    pub confidences: Vec<f32>,     // [0.0, 1.0]
    pub class_ids: Vec<u32>,       // class index
}

// 2. Define model type
pub struct ObjectDetectionModel {
    session: ort::Session,
}

impl Model for ObjectDetectionModel {
    type Input = ObjectDetectionInput;
    type Output = ObjectDetectionOutput;

    async fn load(path: &str) -> Result<Self> {
        let session = ort::Session::builder()
            .with_model_from_file(path)?
            .build()?;
        Ok(Self { session })
    }

    async fn predict(&self, input: &Self::Input) -> Result<Self::Output> {
        // 1. Prepare input tensor
        let input_tensor = input.image.view();

        // 2. Run inference
        let outputs = self.session.run(
            ort::inputs![input_tensor.into()]?
        )?;

        // 3. Parse output tensors
        let boxes_raw = outputs[0].try_extract_tensor::<f32>()?;
        let confs_raw = outputs[1].try_extract_tensor::<f32>()?;
        let classes_raw = outputs[2].try_extract_tensor::<i32>()?;

        // 4. Convert to user types
        let boxes = boxes_raw
            .iter()
            .chunks(4)
            .into_iter()
            .map(|mut chunk| [
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
                chunk.next().unwrap().clone(),
            ])
            .collect();

        let confidences = confs_raw.to_vec();
        let class_ids = classes_raw
            .iter()
            .map(|&id| id as u32)
            .collect();

        Ok(ObjectDetectionOutput {
            boxes,
            confidences,
            class_ids,
        })
    }
}
```

#### Step 2: Load & Predict with Full Workflow

```rust
// my-app/src/models/detection.rs
use ndarray::Array3;
use std::sync::Arc;
use std::path::Path;

/// Load and preprocess image
pub fn load_and_preprocess(path: &Path) -> Result<ObjectDetectionInput> {
    use image::io::Reader as ImageReader;

    // 1. Load image
    let img = ImageReader::open(path)?
        .decode()?
        .to_rgb8();

    // 2. Resize to model input size (e.g., 640x640)
    let resized = image::imageops::resize(&img, 640, 640, image::imageops::FilterType::Lanczos3);

    // 3. Convert to ndarray and normalize
    let arr = ndarray::Array3::from_shape_fn((640, 640, 3), |(y, x, c)| {
        resized.get_pixel(x as u32, y as u32)[c] as f32 / 255.0
    });

    Ok(ObjectDetectionInput { image: arr })
}

/// Post-process model outputs
pub fn postprocess(output: &ObjectDetectionOutput, conf_threshold: f32) -> Vec<Detection> {
    let mut detections = Vec::new();

    for (i, conf) in output.confidences.iter().enumerate() {
        if *conf >= conf_threshold {
            detections.push(Detection {
                bbox: output.boxes[i],
                confidence: *conf,
                class_id: output.class_ids[i],
                class_name: get_class_name(output.class_ids[i]),
            });
        }
    }

    // Sort by confidence descending
    detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    detections
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub bbox: [f32; 4],
    pub confidence: f32,
    pub class_id: u32,
    pub class_name: String,
}

fn get_class_name(id: u32) -> String {
    match id {
        0 => "person".to_string(),
        1 => "bicycle".to_string(),
        2 => "car".to_string(),
        // ... etc
        _ => format!("class_{}", id),
    }
}
```

#### Step 3: Complete Application with Model Management

```rust
// my-app/src/main.rs
use my_app::models::{
    ObjectDetectionModel,
    ObjectDetectionInput,
    load_and_preprocess,
    postprocess,
};
use consortium_ort::ModelPool;
use std::time::Instant;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // 1. Load model from bundled assets
    tracing::info!("Loading YOLO model...");
    let model = ObjectDetectionModel::load(
        "/usr/share/consortium/models/yolov8n.onnx"
    ).await?;

    // 2. Create inference session pool (one per CPU core)
    let pool_size = num_cpus::get();
    tracing::info!("Creating session pool (size: {})", pool_size);
    let pool = ModelPool::new(model, pool_size)?;

    // 3. Batch inference on multiple images
    let image_paths = vec![
        "test1.jpg",
        "test2.jpg",
        "test3.jpg",
    ];

    tracing::info!("Processing {} images", image_paths.len());
    let batch_start = Instant::now();

    let mut tasks = Vec::new();
    for path in &image_paths {
        let pool_clone = pool.clone();
        let path = path.to_string();

        tasks.push(tokio::spawn(async move {
            process_image(&pool_clone, &path).await
        }));
    }

    // Await all tasks
    let results = futures::future::join_all(tasks).await;

    // 4. Aggregate results
    for (path, result) in image_paths.iter().zip(results.iter()) {
        match result {
            Ok(Ok(detections)) => {
                tracing::info!("✓ {}: {} objects detected", path, detections.len());
                for det in detections.iter().take(3) {
                    tracing::info!(
                        "  - {} ({:.2}): {:?}",
                        det.class_name, det.confidence, det.bbox
                    );
                }
            }
            Ok(Err(e)) => {
                tracing::error!("✗ {}: {}", path, e);
            }
            Err(e) => {
                tracing::error!("✗ {}: Task error: {}", path, e);
            }
        }
    }

    let elapsed = batch_start.elapsed();
    tracing::info!("Batch processing completed in {:.2}s", elapsed.as_secs_f32());

    // 5. Real-time stream example (from video or camera)
    process_video_stream(&pool).await?;

    Ok(())
}

/// Process a single image
async fn process_image(
    pool: &ModelPool<ObjectDetectionModel>,
    image_path: &str,
) -> Result<Vec<Detection>> {
    let start = Instant::now();

    // 1. Load and preprocess
    let input = load_and_preprocess(Path::new(image_path))?;

    // 2. Run inference
    let output = pool.predict(&input).await?;

    // 3. Post-process with confidence threshold
    let detections = postprocess(&output, 0.5);

    let elapsed = start.elapsed();
    tracing::debug!("{}: inference took {:.2}ms", image_path, elapsed.as_secs_f32() * 1000.0);

    Ok(detections)
}

/// Process video stream (e.g., from USB camera or RTSP stream)
async fn process_video_stream(
    pool: &ModelPool<ObjectDetectionModel>,
) -> Result<()> {
    use opencv::videoio::VideoCapture;
    use opencv::core::Mat;

    tracing::info!("Starting video stream processing...");

    let mut cap = VideoCapture::new_default(0)?; // Camera index 0
    let mut frame = Mat::default();

    let mut frame_count = 0;
    let stream_start = Instant::now();

    loop {
        if !cap.read(&mut frame)? {
            break;
        }

        frame_count += 1;

        // Skip frames for real-time performance (process every 3rd frame)
        if frame_count % 3 != 0 {
            continue;
        }

        // Convert OpenCV Mat to ndarray
        // (simplified; actual implementation depends on opencv crate version)
        let height = frame.rows() as usize;
        let width = frame.cols() as usize;

        // Prepare input
        let mut input_data = Vec::with_capacity(height * width * 3);
        for pixel in frame.data_mut().iter() {
            input_data.push(*pixel as f32 / 255.0);
        }

        let input = ObjectDetectionInput {
            image: ndarray::Array3::from_shape_vec(
                (height, width, 3),
                input_data,
            )?,
        };

        // Run inference
        match pool.predict(&input).await {
            Ok(output) => {
                let detections = postprocess(&output, 0.5);
                if !detections.is_empty() {
                    tracing::info!("Frame {}: {} objects", frame_count, detections.len());
                }
            }
            Err(e) => {
                tracing::error!("Inference error: {}", e);
            }
        }

        // Print stats every 100 frames
        if frame_count % 100 == 0 {
            let elapsed = stream_start.elapsed().as_secs_f32();
            let fps = frame_count as f32 / elapsed;
            tracing::info!("Processing: {:.1} FPS", fps);
        }
    }

    let total_elapsed = stream_start.elapsed();
    tracing::info!(
        "Video stream complete: {} frames in {:.2}s ({:.1} avg FPS)",
        frame_count,
        total_elapsed.as_secs_f32(),
        frame_count as f32 / total_elapsed.as_secs_f32()
    );

    Ok(())
}
```

#### Step 4: Model Performance Monitoring

```rust
// my-app/src/models/monitoring.rs
use std::time::Instant;
use std::sync::{Arc, Mutex};

pub struct InferenceMetrics {
    pub total_inferences: u64,
    pub total_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub errors: u64,
}

impl InferenceMetrics {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            total_inferences: 0,
            total_latency_ms: 0.0,
            min_latency_ms: f64::INFINITY,
            max_latency_ms: 0.0,
            errors: 0,
        }))
    }

    pub fn record(&mut self, latency_ms: f64) {
        self.total_inferences += 1;
        self.total_latency_ms += latency_ms;
        self.min_latency_ms = self.min_latency_ms.min(latency_ms);
        self.max_latency_ms = self.max_latency_ms.max(latency_ms);
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }

    pub fn average_latency_ms(&self) -> f64 {
        if self.total_inferences == 0 {
            0.0
        } else {
            self.total_latency_ms / self.total_inferences as f64
        }
    }

    pub fn print_summary(&self) {
        tracing::info!("=== Inference Metrics ===");
        tracing::info!("Total inferences: {}", self.total_inferences);
        tracing::info!("Errors: {}", self.errors);
        tracing::info!("Average latency: {:.2}ms", self.average_latency_ms());
        tracing::info!("Min latency: {:.2}ms", self.min_latency_ms);
        tracing::info!("Max latency: {:.2}ms", self.max_latency_ms);
        if self.total_inferences > 0 {
            let throughput = 1000.0 / self.average_latency_ms();
            tracing::info!("Throughput: {:.1} inferences/sec", throughput);
        }
    }
}

// Usage in inference loop
pub async fn monitored_inference(
    pool: &ModelPool<ObjectDetectionModel>,
    input: &ObjectDetectionInput,
    metrics: Arc<Mutex<InferenceMetrics>>,
) -> Result<ObjectDetectionOutput> {
    let start = Instant::now();

    match pool.predict(input).await {
        Ok(output) => {
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            metrics.lock().unwrap().record(elapsed);
            Ok(output)
        }
        Err(e) => {
            metrics.lock().unwrap().record_error();
            Err(e)
        }
    }
}
```

### ORT Model Quantization & Bundling

The build system provides a complete pipeline for model optimization and bundling:

#### Step 1: Define Models in Cargo.toml

```toml
# my-app/Cargo.toml
[package]
name = "my-app"

# Models to include
[package.metadata.consortium-models]
yolov8n = { path = "models/yolov8n.onnx", quantize = "int8", providers = ["cpu"] }
resnet50 = { path = "models/resnet50.onnx", quantize = "fp16", providers = ["cpu", "gpu"] }
pose = { path = "models/pose.onnx", quantize = false, providers = ["cpu"] }

[build-dependencies]
consortium-builder = { path = "../../consortium-builder" }
```

#### Step 2: Build Script with Quantization

```rust
// build.rs
use consortium_builder::{ModelOptimizer, ModelBundle};

fn main() {
    let models = vec![
        ModelConfig {
            name: "yolov8n",
            path: "models/yolov8n.onnx",
            quantization: Quantization::Int8,
            hardware_providers: vec!["cpu"],
            calibration_dataset: Some("data/calibration_images/"),
        },
        ModelConfig {
            name: "resnet50",
            path: "models/resnet50.onnx",
            quantization: Quantization::FP16,
            hardware_providers: vec!["cpu", "gpu"],
            calibration_dataset: None,  // FP16 doesn't need calibration
        },
        ModelConfig {
            name: "pose",
            path: "models/pose.onnx",
            quantization: Quantization::None,
            hardware_providers: vec!["cpu"],
            calibration_dataset: None,
        },
    ];

    for model in models {
        println!("Processing model: {}", model.name);

        // 1. Load original ONNX
        let mut onnx_model = ort::Model::load(&model.path)?;

        // 2. Quantize (if specified)
        let quantized_model = match model.quantization {
            Quantization::Int8 => {
                println!("  → Quantizing to INT8...");
                let calibrator = ModelOptimizer::create_calibrator(
                    &onnx_model,
                    model.calibration_dataset.unwrap(),
                )?;
                ort::quantization::quantize_dynamic(
                    &onnx_model,
                    Some(calibrator),
                    ort::quantization::QuantType::INT8,
                )?
            }
            Quantization::FP16 => {
                println!("  → Converting to FP16...");
                ort::quantization::convert_float_to_float16(&onnx_model)?
            }
            Quantization::None => onnx_model,
        };

        // 3. Optimize graph (operator fusion, constant folding, etc.)
        println!("  → Optimizing graph...");
        let optimized_model = ModelOptimizer::optimize(
            &quantized_model,
            &model.hardware_providers,
        )?;

        // 4. Serialize to binary format with metadata
        let out_dir = env::var("OUT_DIR").unwrap();
        let model_path = format!("{}/models/{}.ort", out_dir, model.name);

        let bundle = ModelBundle {
            onnx_bytes: optimized_model.to_bytes()?,
            input_shapes: optimized_model.input_shapes().clone(),
            input_types: optimized_model.input_types().clone(),
            output_shapes: optimized_model.output_shapes().clone(),
            output_types: optimized_model.output_types().clone(),
            quantization: model.quantization,
            hardware_providers: model.hardware_providers,
            size_original: std::fs::metadata(&model.path)?.len(),
            size_quantized: optimized_model.to_bytes()?.len(),
        };

        bundle.write_to_file(&model_path)?;

        println!(
            "  ✓ {} → {} ({:.1}% size reduction)",
            model.name,
            model_path,
            (1.0 - (bundle.size_quantized as f64 / bundle.size_original as f64)) * 100.0
        );
    }
}
```

#### Step 3: Load Models with Metadata

```rust
// my-app/src/models.rs
use consortium_ort::{ModelBundle, Model};

#[tokio::main]
async fn main() -> Result<()> {
    // Models are pre-compiled and bundled at build time
    // Read from embedded binary or file system
    let yolo_bundle = ModelBundle::load_embedded("yolov8n")?;
    let resnet_bundle = ModelBundle::load_embedded("resnet50")?;

    println!("YOLOv8n:");
    println!("  Quantization: {:?}", yolo_bundle.quantization);
    println!("  Original size: {} MB", yolo_bundle.size_original / 1_000_000);
    println!("  Quantized size: {} MB", yolo_bundle.size_quantized / 1_000_000);
    println!("  Compression: {:.1}%",
        (1.0 - yolo_bundle.size_quantized as f64 / yolo_bundle.size_original as f64) * 100.0);

    // Create session with auto-selected hardware provider
    let session = yolo_bundle.create_session()?;

    Ok(())
}
```

### Quantization Impact

| Model | Original | INT8 | FP16 | Speed (INT8) | Accuracy Loss |
|-------|----------|------|------|------|------|
| YOLOv8n (24 MB) | 24 MB | 6 MB | 12 MB | ~1.5x faster | < 0.5% mAP |
| ResNet50 (98 MB) | 98 MB | 25 MB | 49 MB | ~2x faster | < 1% top-1 acc |
| MobileNetV2 (14 MB) | 14 MB | 3.5 MB | 7 MB | ~1.2x faster | < 0.3% top-1 acc |

**Process:**
1. Original model → load via ONNX Runtime
2. Calibration → if INT8, run representative dataset to collect activation statistics
3. Quantization → convert weights/activations to lower precision
4. Optimization → fuse operations, fold constants, select best kernels
5. Serialization → write as `.ort` file with metadata
6. Bundling → embed in binary or package separately

### ORT Performance & Hardware Acceleration

| Provider | Hardware | Latency | Throughput |
|----------|----------|---------|-----------|
| **CPU (ONNX Runtime)** | ARM Cortex-A | ~100–500 ms (YOLOv8) | 2–10 inferences/sec |
| **GPU (CUDA)** | NVIDIA | ~10–50 ms | 20–100 inferences/sec |
| **NPU (OpenVINO)** | Intel Movidius / VPU | ~20–100 ms | 10–50 inferences/sec |
| **DSP (QNN)** | Qualcomm Hexagon | ~50–200 ms | 5–20 inferences/sec |

**Selection logic:** Auto-detect available hardware; fall back to CPU if necessary.

### ORT Features

- **Automatic quantization:** Int8, FP16 support with accuracy preservation
- **Model optimization:** Operator fusion, constant folding, pruning
- **Batching:** Automatic batching for throughput optimization
- **Caching:** Session / intermediate result caching
- **Monitoring:** Inference time, memory usage, cache hit rates

---

## Consortium FFI: Foreign Function Interface

The Consortium FFI provides a unified interface for calling functions in Python, C, C++, and other languages from Rust code, with automatic serialization, process pooling, and type safety.

### FFI Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Consortium Application (Rust)                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Rust Code                                            │  │
│  │  let result = python_fn("process", input).await?      │  │
│  │  let cv_result = cpp_fn("detect_edges", image).await? │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Consortium FFI Layer                               │  │
│  │  ├─ Function registry (Python/C/C++)                │  │
│  │  ├─ RPC marshaler (pickle, protobuf, JSON)          │  │
│  │  ├─ Process pool (worker management)                │  │
│  │  └─ Error translation                               │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  IPC Transport (stdio, socket, shared memory)        │  │
│  └────────┬────────────────────────────────────────────┘  │
│           │                                                 │
│  ┌────────▼────────────────────────────────────────────┐  │
│  │  Worker Processes                                   │  │
│  │  ├─ Python subprocess (with OpenCV, TensorFlow)     │  │
│  │  ├─ C subprocess (with libfoo)                       │  │
│  │  └─ C++ subprocess (with custom algorithms)         │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Developer Experience: Function Proxies

#### Step 1: Define Worker Services (Python Side)

Worker processes expose functions via JSON-RPC or binary protocols. They run independently and communicate via sockets/stdio.

```python
# ffi_workers/python_cv/main.py
"""
Computer Vision worker: provides OpenCV-based functions.
Each function runs in response to RPC calls from Rust.
"""
import cv2
import numpy as np
import json
import sys
from typing import Dict, List, Any
from dataclasses import dataclass, asdict

# Utilities
@dataclass
class EdgeDetectionResult:
    edges: bytes  # encoded as base64 in JSON
    width: int
    height: int

@dataclass
class PoseKeypoint:
    x: float
    y: float
    confidence: float

@dataclass
class PoseEstimateResult:
    keypoints: List[PoseKeypoint]
    average_confidence: float

# Worker functions
def detect_edges(image_bytes: bytes, threshold1: int = 100, threshold2: int = 200) -> Dict:
    """
    Detect edges in image using Canny algorithm.

    Args:
        image_bytes: JPEG-encoded image as bytes
        threshold1: Lower Canny threshold
        threshold2: Upper Canny threshold

    Returns:
        Dict with edges (base64-encoded PNG), width, height
    """
    try:
        # 1. Decode image
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        if img is None:
            raise ValueError("Failed to decode image")

        # 2. Convert to grayscale and detect edges
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        edges = cv2.Canny(gray, threshold1, threshold2)

        # 3. Encode result
        success, buffer = cv2.imencode('.png', edges)
        if not success:
            raise ValueError("Failed to encode result")

        # 4. Return as base64 for JSON serialization
        import base64
        edges_b64 = base64.b64encode(buffer).decode('utf-8')

        return {
            "status": "ok",
            "result": {
                "edges": edges_b64,
                "width": int(edges.shape[1]),
                "height": int(edges.shape[0]),
            }
        }
    except Exception as e:
        return {"status": "error", "error": str(e)}

def estimate_pose(image_bytes: bytes, model_path: str) -> Dict:
    """
    Estimate human pose using MediaPipe.

    Args:
        image_bytes: JPEG-encoded image
        model_path: Path to pose estimation model

    Returns:
        Dict with keypoints and confidence
    """
    try:
        import mediapipe as mp

        # 1. Decode image
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
        img_rgb = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

        # 2. Run pose detection
        mp_pose = mp.solutions.pose
        with mp_pose.Pose() as pose:
            results = pose.process(img_rgb)

        if not results.pose_landmarks:
            return {"status": "ok", "result": {"keypoints": [], "average_confidence": 0.0}}

        # 3. Extract keypoints
        keypoints = []
        confidences = []
        for landmark in results.pose_landmarks.landmark:
            keypoints.append({
                "x": float(landmark.x),
                "y": float(landmark.y),
                "confidence": float(landmark.visibility),
            })
            confidences.append(float(landmark.visibility))

        avg_conf = np.mean(confidences) if confidences else 0.0

        return {
            "status": "ok",
            "result": {
                "keypoints": keypoints,
                "average_confidence": float(avg_conf),
            }
        }
    except Exception as e:
        return {"status": "error", "error": str(e)}

# Worker server loop
if __name__ == "__main__":
    import json

    # Register available functions
    FUNCTIONS = {
        "detect_edges": detect_edges,
        "estimate_pose": estimate_pose,
    }

    while True:
        try:
            # Read JSON-RPC request from stdin
            line = input()
            request = json.loads(line)

            method = request.get("method")
            params = request.get("params", {})
            request_id = request.get("id")

            if method not in FUNCTIONS:
                response = {
                    "id": request_id,
                    "error": f"Unknown method: {method}"
                }
            else:
                # Call function
                result = FUNCTIONS[method](**params)
                response = {
                    "id": request_id,
                    "result": result
                }

            # Send response
            print(json.dumps(response))
            sys.stdout.flush()

        except EOFError:
            break
        except Exception as e:
            sys.stderr.write(f"Worker error: {e}\n")
            sys.stderr.flush()
```

#### Step 2: Define Rust Interface & RPC Proxies

```rust
// my-app/src/ffi/mod.rs
use serde::{Deserialize, Serialize};
use consortium_ffi::{FfiPool, RemoteCallError};

// Define return types that match Python functions
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EdgeDetectionResult {
    pub edges: Vec<u8>,  // decoded from base64
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoseKeypoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoseEstimateResult {
    pub keypoints: Vec<PoseKeypoint>,
    pub average_confidence: f32,
}

// FFI request/response wrapper
#[derive(Serialize, Deserialize)]
struct WorkerResponse<T> {
    status: String,
    result: Option<T>,
    error: Option<String>,
}

impl<T> WorkerResponse<T> {
    fn into_result(self) -> Result<T, RemoteCallError> {
        match self.status.as_str() {
            "ok" => self.result.ok_or_else(||
                RemoteCallError::InvalidResponse("Missing result".to_string())
            ),
            _ => Err(RemoteCallError::RemoteError(
                self.error.unwrap_or_else(|| "Unknown error".to_string())
            )),
        }
    }
}

/// Python CV worker functions
pub struct PythonCvWorker {
    pool: FfiPool,
}

impl PythonCvWorker {
    pub async fn new(num_workers: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = FfiPool::new()
            .add_python_worker("ffi_workers/python_cv", num_workers)?
            .start()
            .await?;

        Ok(Self { pool })
    }

    /// Detect edges in image
    pub async fn detect_edges(
        &self,
        image_bytes: Vec<u8>,
        threshold1: u32,
        threshold2: u32,
    ) -> Result<EdgeDetectionResult, RemoteCallError> {
        let params = serde_json::json!({
            "image_bytes": image_bytes,
            "threshold1": threshold1,
            "threshold2": threshold2,
        });

        let response: WorkerResponse<EdgeDetectionResult> =
            self.pool.call_python("detect_edges", params).await?;

        response.into_result()
    }

    /// Estimate pose in image
    pub async fn estimate_pose(
        &self,
        image_bytes: Vec<u8>,
        model_path: String,
    ) -> Result<PoseEstimateResult, RemoteCallError> {
        let params = serde_json::json!({
            "image_bytes": image_bytes,
            "model_path": model_path,
        });

        let response: WorkerResponse<PoseEstimateResult> =
            self.pool.call_python("estimate_pose", params).await?;

        response.into_result()
    }
}

// Error types
#[derive(Debug)]
pub enum RemoteCallError {
    Serialization(serde_json::Error),
    Transport(String),
    RemoteError(String),
    InvalidResponse(String),
    Timeout,
}

impl std::fmt::Display for RemoteCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization(e) => write!(f, "Serialization: {}", e),
            Self::Transport(e) => write!(f, "Transport: {}", e),
            Self::RemoteError(e) => write!(f, "Remote error: {}", e),
            Self::InvalidResponse(e) => write!(f, "Invalid response: {}", e),
            Self::Timeout => write!(f, "RPC timeout"),
        }
    }
}

impl std::error::Error for RemoteCallError {}
```

#### Step 3: Comprehensive Application Integration

```rust
// my-app/src/main.rs
use my_app::ffi::PythonCvWorker;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    // 1. Initialize FFI worker pool
    tracing::info!("Starting Python CV worker pool...");
    let cv_workers = PythonCvWorker::new(num_cpus::get()).await?;
    tracing::info!("Worker pool started");

    // 2. Load image
    let image_path = "test_image.jpg";
    let image_bytes = std::fs::read(image_path)?;
    tracing::info!("Loaded image: {} bytes", image_bytes.len());

    // 3. Single call example
    {
        tracing::info!("Calling detect_edges...");
        let start = Instant::now();

        match cv_workers.detect_edges(image_bytes.clone(), 100, 200).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                tracing::info!("✓ Edge detection: {}x{} (took {:?})", result.width, result.height, elapsed);

                // Save edges
                let edges_bytes = base64::decode(&result.edges)?;
                std::fs::write("edges.png", edges_bytes)?;
            }
            Err(e) => {
                tracing::error!("✗ Edge detection failed: {}", e);
            }
        }
    }

    // 4. Concurrent calls example
    {
        tracing::info!("Running concurrent calls...");
        let start = Instant::now();

        let (edges_result, pose_result) = tokio::join!(
            cv_workers.detect_edges(image_bytes.clone(), 100, 200),
            cv_workers.estimate_pose(image_bytes.clone(), "models/pose.pb".to_string()),
        );

        let elapsed = start.elapsed();

        match (edges_result, pose_result) {
            (Ok(edges), Ok(pose)) => {
                tracing::info!("✓ Concurrent calls completed in {:?}", elapsed);
                tracing::info!("  - Edges: {}x{}", edges.width, edges.height);
                tracing::info!("  - Pose: {} keypoints (avg conf: {:.2})", pose.keypoints.len(), pose.average_confidence);
            }
            (e1, e2) => {
                tracing::error!("✗ One or more calls failed: {:?}, {:?}", e1, e2);
            }
        }
    }

    // 5. Batch processing example
    {
        tracing::info!("Batch processing...");
        let image_paths = vec!["img1.jpg", "img2.jpg", "img3.jpg"];

        let mut tasks = Vec::new();
        for path in image_paths {
            let image = std::fs::read(path)?;
            let cv_clone = &cv_workers;

            tasks.push(async move {
                cv_clone.detect_edges(image, 100, 200).await
            });
        }

        let results = futures::future::join_all(tasks).await;
        let successful = results.iter().filter(|r| r.is_ok()).count();
        tracing::info!("Batch: {}/{} images processed", successful, results.len());
    }

    Ok(())
}
```

#### Step 4: Error Handling & Retry Logic

```rust
// my-app/src/ffi/retry.rs
use async_trait::async_trait;
use tokio::time::{sleep, Duration};

#[async_trait]
pub trait Retryable<T>: Sized {
    async fn with_retry(self, max_attempts: u32) -> Result<T, Box<dyn std::error::Error>>;
}

pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(5),
        }
    }
}

impl<T, E: 'static + std::error::Error> Retryable<T> for Result<T, E> {
    async fn with_retry(self, max_attempts: u32) -> Result<T, Box<dyn std::error::Error>> {
        let mut attempt = 0;
        let mut backoff = Duration::from_millis(100);

        loop {
            attempt += 1;

            match self {
                Ok(value) => return Ok(value),
                Err(e) if attempt >= max_attempts => {
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }
                Err(e) => {
                    tracing::warn!(
                        "Attempt {} failed: {}. Retrying in {:?}...",
                        attempt, e, backoff
                    );
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(5));
                }
            }
        }
    }
}

// Usage in application
async fn detect_edges_with_retry(
    cv: &PythonCvWorker,
    image: Vec<u8>,
) -> Result<EdgeDetectionResult, Box<dyn std::error::Error>> {
    let mut attempt = 0;
    let max_attempts = 3;

    loop {
        attempt += 1;
        match cv.detect_edges(image.clone(), 100, 200).await {
            Ok(result) => return Ok(result),
            Err(e) if attempt >= max_attempts => {
                return Err(format!("Failed after {} attempts: {}", max_attempts, e).into());
            }
            Err(e) => {
                tracing::warn!("Attempt {}: {}, retrying...", attempt, e);
                sleep(Duration::from_millis(100 * attempt as u64)).await;
            }
        }
    }
}
```

### FFI Best Practices

**1. Function Design:**
- Keep functions pure (no side effects)
- Serialize/deserialize at boundaries only
- Batch operations where possible

**2. Performance:**
- Use binary serialization (postcard) for large data
- Pool workers to amortize process creation cost
- Cache expensive initialization (models, connections)

**3. Error Handling:**
- Implement retry logic for transient failures
- Log detailed errors for debugging
- Set timeouts for long-running calls

**4. Resource Management:**
- Limit worker pool size to available CPU cores
- Set memory limits per worker
- Clean up resources on shutdown

### FFI Features

| Feature | Implementation |
|---------|-----------------|
| **Multi-language** | Python, C, C++, via subprocess |
| **Serialization** | postcard, JSON, protobuf (pluggable) |
| **Process pooling** | Work-stealing queue, load balancing |
| **Error handling** | Automatic translation, stack traces |
| **Type safety** | Compile-time type checking via macros |
| **Async/await** | Full Tokio integration |
| **Resource management** | Automatic cleanup, memory limits |
| **Monitoring** | Execution time, memory usage, error rates |

### FFI Performance

**Latency breakdown (Python function call):**
| Component | Time |
|-----------|------|
| Serialization | ~0.1–1 ms |
| IPC (socket) | ~0.5–2 ms |
| Python deserialization | ~0.1–0.5 ms |
| Function execution | *variable* |
| Python serialization | ~0.1–0.5 ms |
| IPC response | ~0.5–2 ms |
| Rust deserialization | ~0.1–1 ms |
| **Total overhead** | **~2–7 ms** |

**Optimization strategies:**
- Keep functions long-running (amortize overhead)
- Batch multiple calls where possible
- Use binary serialization (postcard) over JSON
- Pool workers to avoid process spawn overhead

---

## Integration: Complete Example

Here's how TEE, HMI, ORT, and FFI work together:

```rust
// my-app/src/main.rs
use consortium_runtime_app::RemoteProc;
use consortium_tee::TeeSession;
use consortium_ort::ModelPool;
use consortium_ffi::FfiPool;
use my_app::models::ObjectDetectionModel;
use my_app::hmi::AppHmi;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize all subsystems
    let remoteproc = RemoteProc::new(0);
    remoteproc.start()?;

    let tee = TeeSession::open().await?;
    let od_model = ObjectDetectionModel::load("yolo.onnx").await?;
    let ffi_pool = FfiPool::new()
        .add_python_worker("cv_workers", 4)?
        .start()
        .await?;

    let hmi = AppHmi;

    // 2. Incoming image stream (from sensor via IPC or camera)
    let mut image_stream = create_image_stream().await?;

    while let Some(image) = image_stream.next().await {
        // Step A: Run ML inference
        let detections = od_model.predict(&image).await?;

        // Step B: Call Python for post-processing
        let edges = ffi_pool.detect_edges(image.to_bytes()).await?;

        // Step C: Verify authenticity (in TEE)
        let is_legit = tee.call_verify_image_signature(&edges).await?;

        // Step D: Update HMI with results
        let event = AppEvent::ObjectsDetected {
            detections,
            verified: is_legit,
        };
        broadcast_event(event).await?;
    }

    Ok(())
}
```

This demonstrates the power of the Consortium framework: developers can seamlessly combine:
- **IPC** for low-latency sensor data
- **TEE** for security-critical operations
- **ORT** for AI inference
- **FFI** for specialized libraries
- **HMI** for user interaction

All with a clean, ergonomic Rust API.
