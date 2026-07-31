# Configuration and Runtime

**Chip selection**  
The full part number in `[profile].chip`. It chooses the processor topology, memory map,
doorbells, peripherals, and compilation targets from the chip database.

**`Consortium.toml`**  
The whole-system manifest. It declares the target chip, endpoints, shared-memory IPC,
debug rings, peripheral ownership, and optional TEE and HMI configuration.

**`consortium.gen.rs`**  
The endpoint-specific Rust module produced by the build pipeline. It defines `Context`,
initialization errors, and the generated `init()` function.

**`Context`**  
The aggregate returned by generated initialization. It owns ready IPC transceivers and,
where configured, UIO devices, debug services, and controller-owned peripherals.

**Generated `init()`**  
The asynchronous bring-up function emitted for an endpoint. It constructs local
hardware, connects IPC peers, starts configured services, and returns `Context`.

**Lowering**  
The conversion from validated, user-facing configuration into a platform-neutral model
used by code, device-tree, UIO, and build generation.

**Party**  
One endpoint's participation in an IPC channel. Its manifest entry names the message
type and doorbell channel used for that party's outgoing direction.

**Session**  
The parsed manifest plus resolved chip data. Session validation checks cross-section
constraints that TOML parsing alone cannot prove.

**Visible core**  
A processor identity exposed by the selected chip database and therefore eligible to
own an endpoint, peripheral, debug ring, or IPC side.
