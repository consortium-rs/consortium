# Configuration in RPI Subsystem

The `RPI` regards `Consortium.toml` as the source of truth for configuration. The `Consortium.toml` is a configuration file that contains information about the resources in the system, such as the partitions, resources, and access control policies.

## What does it look like

The `RPI` configuration in `Consortium.toml` is configured in `peripheral` field. Its convention is `[peripheral.<name>]`, where `<name>` is the name of the peripheral. For example, for peripheral SPI1, the configuration would be under `[peripheral.spi1]`.

The name of the peripheral **must** match the name used in the device tree source, the hardware-specific RPI configuration (namely RIF in STM32MP series and TRDC in i.MX series), and the PAC crate. This is to ensure consistency across the different components of the system and to avoid confusion.

In the `[peripheral.<name>]` section, we can configure the `owner` field, which specifies the owner of the peripheral. The owner is the partition that has access to the peripheral. For example, if SPI1 is owned by the `cm33` core, the configuration would be `owner = "cm33"`. If it should be running in the secure domain, the configuration should also be `owner = "cm33"`, but you need to specify `secure = true` to indicate that it is in the secure domain.

In application core, we accept `linux` and `optee`. `linux` is always non-secure, while `optee` is secure by default. In the future, we will support `u-boot` and `tf-a` as well, and we will also support more flexible configuration for secure/non-secure in application core.

We currently only support single owner for each peripheral. In the future, we may support multiple owners for a peripheral, but this will require additional configuration to specify the access control policies for each owner, and we will introduce the concept of "shared peripherals" to handle this scenario, including semaphores, mutexes, etc. to ensure safe access to the shared peripherals.

We must ensure names in `Consortium.toml` are consistent with the names in:

- Device Tree Source (DTS): The names used in `Consortium.toml` should match the names used in the DTS files, as these are used to generate the device tree blobs that are consumed by the operating system and the RPI subsystem.
- Hardware-specific RPI Configuration: The names should also match the names used in the hardware-specific RPI configuration, such as RIF for STM32MP series and TRDC for i.MX series, to ensure that the correct resources are configured and accessed.
- PAC Crate: The names should also be consistent with the names used in the Peripheral Access Crate (PAC) to ensure that the correct registers and fields are accessed when configuring the peripherals in the code. This consistency is crucial for the correct functioning of the system and to avoid confusion during development and debugging.

```toml
[peripheral.spi1]
owner = "cm33"

[peripheral.spi2]
owner = "linux"
secure = false

[peripheral.spi3]
owner = "optee"
secure = true

[peripheral.i2c1]
owner = "cm33"
secure = true
```
