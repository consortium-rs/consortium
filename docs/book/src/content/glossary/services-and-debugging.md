# Services and Debugging

**Bridge**  
The backend-neutral HMI command registry. It maps command names and string payloads to
synchronous or asynchronous response handlers.

**CDBG**  
Consortium's shared-memory debug descriptor and ring format for transporting firmware
`defmt` frames to the application processor.

**Cog / WPE WebKit**  
The browser-based HMI engine supported by `consortium-hmi-webkit`, suited to kiosk
applications that need web-platform compatibility.

**`defmt`**  
A compact logging format suited to firmware. Consortium can write encoded frames to a
CDBG ring and decode them on Linux.

**FFI**  
Foreign Function Interface. `consortium-ffi` contains integration contracts for code
outside native Rust boundaries.

**HMI**  
Human-Machine Interface. Consortium provides a common command bridge with Cog/WPE
WebKit and Pocket rendering adapters.

**ORT**  
ONNX Runtime. `consortium-ort` contains the project's model-inference integration.

**Pocket**  
The native retained HMI path. PocketJS produces a JavaScript bundle and resource pack
consumed by `consortium-hmi-pocket`.

**TEE**  
Trusted Execution Environment. Consortium's current integration targets OP-TEE and
generates typed client/trusted-application command glue.

**`TeeParam`**  
The conversion contract between a Rust value and OP-TEE parameter slots. Primitive
values use value slots; serialized values use memory-reference slots.

**`tracing`**  
The structured logging facade used on Linux. Automatic debug readers forward decoded
firmware logs into `tracing`.
