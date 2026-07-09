# What HAMPU Is

Heterogeneous and Asymmetric Multiprocessing Units (HAMPU) refers to a computing system that integrates multiple processing units with different purposes and capabilities. A typical HAMPU system includes at least one application core and one real-time core, commonly Cortex-A and Cortex-M cores respectively.

The application core handles general-purpose computing tasks, often with a standard OS such as Linux. The real-time core is optimized for deterministic, time-sensitive workloads. Modern HAMPU systems may also include NPUs, GPUs, DSPs, VPUs, or other accelerators to support AI, graphics, multimedia, and specialized processing.

This architecture enables a single system to efficiently support general-purpose applications, real-time control, and hardware-accelerated workloads.

## Comparison with Other Architectures

Compared with a traditional MCU that only includes Cortex-M or RISC-V cores, a HAMPU system can run advanced applications with standard OS support. This reduces the overhead of porting and maintaining applications while providing a more comfortable deployment environment. For example, the application core can run Linux-based HMI logic through WPE WebKit and execute image recognition workloads through an NPU, while the real-time core simultaneously handles peripheral data and actuator control, such as CAN bus communication.

Compared with application processors that only include Cortex-A or x86 cores, HAMPU systems can offload time-sensitive tasks to the real-time core. This improves system responsiveness and avoids the need to patch Linux for pseudo real-time behavior. Instead, bare-metal or RTOS programming on the real-time core can provide deterministic execution for tasks with strict timing constraints.

Compared with serial-wired upper-lower architectures, HAMPU systems integrate the application core and real-time core on the same chip. This reduces communication latency and improves system performance. The two cores can exchange data through shared memory or inter-core communication mechanisms, enabling tighter coordination than systems where the application processor and microcontroller are separate devices.

## Typical HAMPU Chips

### NXP i.MX 95

The NXP i.MX 95 is a flagship HAMPU chip that combines Cortex-A55 application cores, Cortex-M7 real-time cores, Cortex-M33 system manager cores, and integrated NPUs, GPUs, and VPUs. It is suitable for high-performance applications requiring AI, machine learning, multimedia processing, and real-time control.

Development can be done with the [FRDM-IMX95 board](https://www.nxp.com/design/design-center/development-boards-and-designs/FRDM-IMX95). Consortium provides official examples and tutorials for setting up the development environment, programming both application and real-time cores, and leveraging integrated accelerators.

### STM32MP257

The STM32MP257 combines Cortex-A35 application cores, Cortex-M33 real-time cores, and integrated NPUs and GPUs. It is designed for industrial automation, consumer electronics, and IoT applications. It is also suitable for critical-mission scenarios, with SIL 3 certification and support for TrustZone-based resource partition isolation.

Development can be done with the [STM32MP257F-DK board](https://www.st.com/en/evaluation-tools/stm32mp257f-dk.html). Consortium provides examples and tutorials for using both application and real-time cores, as well as the integrated NPU and GPU.

### NXP i.MX 93

The NXP i.MX 93 is a more cost-effective HAMPU chip compared with the i.MX 95. It includes Cortex-A55 application cores, Cortex-M33 real-time cores, and integrated NPUs and GPUs. It is suitable for applications that need a balance between performance, cost, and acceleration, such as smart home devices, wearables, and consumer electronics.

Development can be done with the [FRDM-IMX93 board](https://www.nxp.com/design/design-center/development-boards-and-designs/FRDM-IMX93). Consortium provides examples and tutorials covering environment setup, multi-core programming, and accelerator usage.

### STM32MP215

The STM32MP215 is a more cost-effective option compared with the STM32MP257. It combines Cortex-A35 application cores and Cortex-M33 real-time cores, without integrated NPUs or GPUs. It targets applications that require a practical balance of performance, security, and real-time capability, such as industrial automation, consumer electronics, and IoT devices.

Development can be done with the [STM32MP215F-DK board](https://www.st.com/en/evaluation-tools/stm32mp215f-dk.html). Consortium provides tutorials and examples for developing applications across the application and real-time cores.

## Chip Generations

Common HAMPU chips usually combine Cortex-A and Cortex-M cores. Earlier examples include the STM32MP1 series, with Cortex-A7 application cores and a Cortex-M4 real-time core, and the NXP i.MX 8M series, with Cortex-A53 application cores and a Cortex-M4 real-time core.

In Consortium, the focus is on newer HAMPU generations, especially the NXP i.MX 9x series and STM32MP2x series. Compared with previous generations, these chips provide stronger application cores, such as Cortex-A55, and newer real-time cores, such as Cortex-M33.

A key improvement is resource partition isolation through Cortex-M33-based designs. This helps reduce security risks caused by different cores competing for shared resources such as memory and peripherals. In older designs, similar functionality often required combining an application processor, such as the NXP i.MX 8M Plus, with a separate MCU, such as an STM32F4.

## Benefits and Challenges

The main cost of HAMPU is increased software and system complexity. Developers must coordinate multiple processing units, manage inter-core communication, and handle resource partitioning. Integration on a single chip may also increase power consumption and thermal requirements, depending on workload and system design.

Despite these challenges, HAMPU systems provide strong benefits in performance, flexibility, and real-time capability. They are well suited for applications ranging from consumer electronics to industrial automation.

The Consortium framework is designed to address HAMPU development complexity by providing a unified programming model and toolchain. It abstracts coordination between different processing units, allowing developers to focus on application logic rather than low-level hardware details. With Consortium, developers can write code that runs across both application and real-time cores while taking full advantage of the HAMPU architecture.
