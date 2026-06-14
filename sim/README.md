# Simulator Images

Heterogeneous Multiprocessors often lacks existed simulators; for our project, we need to simulate those processors before hardware-in-the-loop (HIL) testing.

Since its notoriously difficult to write a QEMU emulator, we choose to use `Renode`. All the simulation scripts are located in `sim/renode`.

Right now we only support [STM32MP257F](https://www.st.com/en/microcontrollers-microprocessors/stm32mp257f.html) SoC, which is a heterogeneous multiprocessor with two ARM Cortex-A35 cores, one ARM Cortex-M33 core, and one ARM Cortex-M0+ core. For simplicity, we don't include the Cortex-M0+ core in our simulation, since it's only used for power management and doesn't have much interaction with the other cores.
