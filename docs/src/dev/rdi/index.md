# Resource Domain Isolation (RDI) Subsystem

The Resource Domain Isolation (RDI) subsystem is responsible for managing the isolation of resources in the system. It provides a mechanism for isolating resources such as memory, peripherals, and interrupts, and ensures that only authorized entities can access these resources.

In ST and NXP, they have respective names towards this subsystem, but we have decided to use the more generic name "RDI" to avoid confusion and to better reflect the purpose of the subsystem.

- STM32MP series: RIF (Resource Isolation Framework)
- i.MX series: RD (Resource Domain)

RDI provides compile-time peripheral ownership control, automatic DTS generation, and PAC customization to enforce strict, cross-core, cross-secure-domain isolation.

## Domain ID (DID)

The Domain ID (DID) is a unique identifier for each domain in the system. It is used to identify the owner of a resource and to enforce access control policies. The DID is assigned to each domain at compile time and is used by the RDI subsystem to determine which resources a domain can access.

- STM32MP series: CID (Context ID)
- i.MX series: RD (Resource Domain)

# Resource ID (RID)

The Resource ID (RID) is a unique identifier for each resource in the system. It is used to identify a specific resource and to enforce access control policies. The RID is assigned to each resource at compile time and is used by the RDI subsystem to determine which domains can access a resource.

- STM32MP series: `rifsc_idx` (RIF Secure Context Index)
- i.MX series: `trdc_periph_id` (Trust Resource Domain Controller Peripheral ID)

