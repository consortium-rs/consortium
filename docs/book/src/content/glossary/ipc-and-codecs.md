# IPC and Codecs

**Channel**  
A typed, directed endpoint. `Channel<Tx, ...>` sends one message type;
`Channel<Rx, ...>` receives one message type.

**`Chan`**  
A validated identifier in the signaling or framing namespace, such as an MU line, IPCC
channel, or logical UART channel.

**Codec**  
A serialization family that converts typed values to and from bytes. Consortium
supports postcard, prost, and rkyv families behind features.

**`CodecFor<T>`**  
The codec contract for a particular message type. Its `Decoded<'buf>` associated type
supports both owned decoding and values that borrow the receive buffer.

**`Connect`**  
The one-time asynchronous readiness rendezvous between peers. It runs after transport
construction, before traffic, and before splitting a full-duplex transport.

**Descriptor**  
Metadata at the start of a shared-memory structure. The `CIPC` descriptor identifies
the IPC protocol and records the layout the peer must validate.

**Doorbell**  
A signaling mechanism that wakes or notifies another processor without carrying the
message payload. Implementations include STM32 IPCC/HSEM, i.MX MU, and GPIO.

**`IpcSafe`**  
A marker trait and derive macro for types that may cross an IPC boundary. The derive
rejects address-space-local or word-size-dependent fields such as pointers, references,
function pointers, `usize`, and `isize`.

**MTU**  
Maximum transfer unit: the largest encoded payload a channel can carry. It also sizes
the channel's fixed scratch buffer.

**Primary / Secondary**  
Compile-time side markers used by shared-memory IPC. The primary publishes the
descriptor; the secondary validates and acknowledges it. They are protocol roles, not
processor priorities.

**`ReceivedMessage`**  
The value returned by `recv()`. It wraps a codec's decoded representation so the same
API supports owned and zero-copy codecs.

**Region**  
A shared-memory abstraction with an address, length, and explicit cache-maintenance
semantics. Current region types include SRAM, DDR, and a virtual serial region.

**Slot**  
A shared-memory area holding one directional message and its protocol metadata.

**Transceiver**  
A bidirectional typed endpoint formed by pairing one Tx channel with one Rx channel.

**Transport**  
The byte-moving layer beneath a channel. `SendTransport` and `RecvTransport` model its
directional halves; `Transport` marks a full-duplex implementation.

**Type-A / Type-B IPC**  
Informal project categories. Type-A uses a separate doorbell, as shared-memory IPC does.
Type-B is transport-coupled: UART byte arrival is the notification, so no doorbell or
processor side is required.
