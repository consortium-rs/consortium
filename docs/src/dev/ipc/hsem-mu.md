# HSEM vs MU — Inconsistencies & Resolution

| Aspect | HSEM | MU (GI) | How Resolved |
|--------|------|---------|--------------|
| **Signal mechanism** | 1-step read-lock on semaphore (stateful) | Set `GCR[GIRn]` (idempotent flag) | `ring()` uses `set_gcr_gir()` instead of a data register write |
| **ring() guard** | LOCKID ownership check (MASTERID) | None — `GCR[GIRn]` set is always valid | No `AlreadyLocked` condition possible with GI registers |
| **AlreadyLocked** | Another master holds the semaphore | N/A — variant removed | `MuError::AlreadyLocked` deleted; GCR set while pending is idempotent |
| **Interrupt clear** | Dedicated ICR register (write-1-to-clear) | Write 1 to `GSR[GIPn]` (W1C) | `clear_gsr_gip()` — non-destructive, does not consume data |
| **Interrupt enable** | IER bit per semaphore (16 bits) | `GIER[GIEn]` bit per channel (4 bits) | `enable/disable_gi_interrupt()` via GIER instead of IER/ICR |
| **Interrupt status** | MISR (masked = IER & ISR) | `GSR[GIPn]` (non-destructive read) | `gsr_pending()` reads GSR directly without consuming the signal |
| **Const generic** | PROC: u8 (selects C1/C2/C3 interrupt bank) | None needed | Base address + std feature determines the side |
| **IRQ handlers** | Single `HSEM_IRQHandler` | `MU1_IRQHandler` + `MU2_IRQHandler` (+ more for imx95) | One `AtomicWaker` per MU instance; `waker_for(mu_id)` selects it |
| **Channel count** | 16 (flat semaphore space) | `MU_INSTANCE_COUNT × 4` GI channels | `MuChanInfo::mu_id()` / `gi_id()` split the channel ID |
