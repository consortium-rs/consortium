# STM32MP2 Peripheral Access Crate

The `stm32mp2-pac` crate provides low-level access to the peripherals of the STM32MP2 family of microcontrollers. It is generated using `chiptool` and includes support for various peripherals such as I2C, SPI, and more.

## Procedures

We parse the STM32CubeMP2 header files to generate chiptool-compatible metadata, which is then used to generate the Rust code for the peripheral access API. The generated code includes structures and methods for accessing the registers of each peripheral.

## Supported Peripherals

- I3C
- I2C
- SPI
- USART
- RCC
- HSEM

We are actively working on adding support for more peripherals and improving the existing implementations. Contributions are welcome!
