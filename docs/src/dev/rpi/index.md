# Resource Partition Isolation (RPI) Subsystem

The Resource Partition Isolation (RPI) subsystem is responsible for managing the isolation of resources in the system. It provides a mechanism for isolating resources such as memory, peripherals, and interrupts, and ensures that only authorized entities can access these resources.

In ST and NXP, they have respective names towards this subsystem, but we have decided to use the more generic name "RPI" to avoid confusion and to better reflect the purpose of the subsystem.

- STM32MP series: RIF (Resource Isolation Framework)
- i.MX series: RD (Resource Domain)

RPI provides compile-time peripheral ownership control, automatic DTS generation, and PAC customization to enforce strict, cross-core, cross-secure-domain isolation.

## Partition ID (PartID)

The Partition ID (PartID) is a unique identifier for each partition in the system. It is used to identify the owner of a resource and to enforce access control policies. The PartID is assigned to each partition at compile time and is used by the RPI subsystem to determine which resources a partition can access.

- STM32MP series: CID (Context ID)
- i.MX series: RD (Resource Domain)

# Resource ID (ResID)

The Resource ID (ResID) is a unique identifier for each resource in the system. It is used to identify the resource and to enforce access control policies. The ResID is assigned to each resource at compile time and is used by the RPI subsystem to determine which partitions can access the resource.

- STM32MP series: `rifsc_idx` (RIF Secure Context Index)
- i.MX series: `trdc_periph_id` (Trust Resource Domain Controller Peripheral ID)
