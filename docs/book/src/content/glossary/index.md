# Glossary

Consortium spans hardware, firmware, Linux, build tooling, and application services.
Terms are grouped by subsystem so related concepts can be read together:

- [Platforms and Hardware](platforms-and-hardware.md)
- [IPC and Codecs](ipc-and-codecs.md)
- [Configuration and Runtime](configuration-and-runtime.md)
- [Build and Deployment](build-and-deployment.md)
- [Services and Debugging](services-and-debugging.md)

Definitions describe the current implementation. Historical maintenance notes may use
older terminology.

## General terms

**Boundary contract**  
A rule shared by two execution domains, such as a message layout, memory region, command
identifier, or ownership assignment. Consortium aims to make these contracts typed and
validated.

**Endpoint**  
One runtime participant and its crate root, such as a Linux application, CM33 firmware,
CM7 firmware, or OP-TEE trusted application.

**Subsystem**  
A cohesive part of Consortium—IPC, TEE, HMI, configuration, logging, or hardware
support—with a portable contract and one or more platform-specific implementations.
