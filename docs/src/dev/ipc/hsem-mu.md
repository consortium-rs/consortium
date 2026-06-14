# Doorbell Register Model Comparison

| Aspect | HSEM | IPCC | MU (GI) |
|--------|------|------|---------|
| Platform | STM32MP23/25 | STM32MP21/23/25 | NXP i.MX 9x |
| Signal mechanism | 1-step read-lock then unlock semaphore | Set `CxSCR.CHnS` to mark a channel occupied | Set `GCR[GIRn]` |
| Backpressure guard | LOCKID ownership check; can report `AlreadyLocked` | Outgoing `CxTOCySR.CHnF`; returns `ChannelOccupied` while occupied | No guard; setting `GIRn` while pending is idempotent |
| Receive status | `CnMISR` masked interrupt status | Opposite direction `CyTOCxSR.CHnF` | `GSR[GIPn]` |
| Interrupt enable | `CnIER` bit per semaphore | `CxCR.RXOIE` plus `CxMR.CHnOM` unmask | `GIER[GIEn]` |
| Interrupt clear | Dedicated `CnICR` W1C bit | `CxSCR.CHnC` W1C-style clear | `GSR[GIPn]` W1C |
| Const generic | `PROC: u8` selects C1/C2/C3 interrupt bank | `PROC: u8` selects processor 1 or 2 register bank | None; base array and channel mapping select instance |
| IRQ wakeup | Single `HSEM_IRQHandler` | Board/PAC calls `notify_rx_occupied_interrupt()` from the verified IPCC RX occupied line | `MU1_IRQHandler` ... `MU8_IRQHandler` by instance |
| Channel count | 16 flat semaphore IDs; 0-1 reserved | 16 channels per IPCC instance; 0-1 reserved | `MU_INSTANCE_COUNT * 4`; 0-1 reserved |
| Register style | `bitfield` register wrappers + volatile access | `bitfield` register wrappers + volatile access | `bitfield` register wrappers + volatile access |

IPCC address map:

| Family | IPCC1 | IPCC2 |
|--------|-------|-------|
| STM32MP21 | `0x4049_0000..=0x4049_03FF` | not present |
| STM32MP23/25 | `0x4049_0000..=0x4049_03FF` | SmartRun `0x4625_0000..=0x4625_03FF` |
