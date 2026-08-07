# Configuration and Runtime

**Chip selection**
The full part number in `[profile].chip`. It selects processor topology, memory maps,
doorbells, peripherals, and compilation targets from the chip database.

**`Consortium.toml`**
The whole-system manifest declaring endpoints, shared-memory IPC, debug rings,
peripheral ownership, and optional TEE and HMI configuration.

**`consortium.gen.rs`**
The endpoint-specific Rust module defining `Config`, `Peripherals`, `Channels`, errors,
and `init(config)`.

**`Config`**
The generated, non-exhaustive local bring-up policy. Its public boolean fields default
to `true` and exist for conditionally enabled initialization steps.

**`Peripherals`**
The generated singleton allowlist returned as `p`. On firmware it contains exactly the
manifest-owned hardware singletons; on Linux it contains platform resources such as UIO
owners and the optional debug console. It has no `Drop` implementation, allowing partial
moves such as `I2c::new(p.LPI2C1, ...)`.

**`Channels`**
The generated take-once IPC singleton bundle returned as `c`. Each uppercase field such
as `c.SENSOR` is consumed to construct or connect only that endpoint. It has no `Drop`
implementation so fields can be moved independently.

**Generated `init(config)`**
The synchronous local bring-up function. MCU initialization is infallible and returns
`(Peripherals, Channels)`; Linux initialization may fail while opening mappings or a
debug console and returns `Result<(Peripherals, Channels), ConsortiumInitError>`.

**Endpoint**
A constructed but unconnected channel owner. `Endpoint::new(c.NAME)` consumes the
singleton; `connect().await` negotiates, splits, and returns a typed transceiver.

**Lowering**
Conversion from validated user configuration into the model used by Rust, device-tree,
UIO, and build generation.

**Party**
One endpoint's participation in an IPC channel, including its outgoing message type and
doorbell channel.

**Session**
The parsed manifest plus resolved chip data and cross-section validation.

**Visible core**
A processor identity exposed by the selected chip database and eligible to own an
endpoint, peripheral, debug ring, or IPC side.
