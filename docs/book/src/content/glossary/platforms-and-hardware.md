# Platforms and Hardware

**A-core**  
An application-class processor, typically Arm Cortex-A, that runs Linux. Consortium
normally assigns it the shared-memory `Primary` role.

**Application processor (AP)**  
The Linux-capable processor in a Consortium system. In manifests, its endpoint is
conventionally named `linux`; peripheral ownership uses `ap`.

**Chip database**  
Generated hardware metadata used to resolve a part number and validate visible cores,
memory regions, doorbells, peripherals, pins, and interrupts.

**Controller core**  
A real-time or supervisory processor running firmware, such as Cortex-M, Cortex-R, or
RISC-V. Current examples use CM33 and CM7 cores.

**HAL**  
Hardware Abstraction Layer. Consortium's chip HALs wrap PAC definitions with peripheral
singletons and early firmware drivers.

**HAMPU**  
Heterogeneous and Asymmetric Multiprocessing Unit: a system-on-chip containing
processors with different architectures, capabilities, or runtime responsibilities.

**Interrupt forwarding**  
The application-owned handler that passes a hardware interrupt to a Consortium driver's
wake function. Doorbell and HAL crates do not install application vectors themselves.

**PAC**  
Peripheral Access Crate: generated low-level register and interrupt definitions for a
chip family.

**Peripheral singleton**  
A unique value proving ownership of one hardware peripheral. A chip HAL's `init()`
hands each singleton out once; a driver constructor consumes it.

**UIO**  
Linux Userspace I/O. Consortium uses `/dev/uioN` devices to map shared memory and
doorbell registers and to receive hardware interrupts in the Linux application.
