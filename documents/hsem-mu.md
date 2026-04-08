# HSEM vs MU — Inconsistencies & Resolution

| Aspect | HSEM | MU (v1) | How Resolved |
|--------|------|----|--------------|
| **Signal mechanism** | 1-step read-lock on semaphore (stateful) | Write 1 to TRn (data transfer) | `ring()` uses `write_volatile(1)` instead of `read_lock()` |
| **ring() guard** | LOCKID ownership check (MASTERID) | TSR bit set (TRn empty) | `tx_empty()` replaces `read_lock()` + LOCKID comparison |
| **AlreadyLocked semantics** | Another master holds the semaphore | Remote hasn't consumed last signal | Same error, different root cause — documented in `ring()` doc |
| **Interrupt clear** | Dedicated ICR register (write-1-to-clear) | Reading RRn clears RSR (destructive) | `consume_rr()` instead of `clear_interrupt()`. No pre-clear on first poll |
| **Interrupt enable** | IER bit per semaphore (16 bits) | RCR bit per RX register (4 bits) | `enable/disable_rx_interrupt()` via RCR instead of IER/ICR |
| **Interrupt status** | MISR (masked = IER & ISR) | RSR (unmasked RX status) | `rx_full()` reads RSR directly |
| **Const generic** | PROC: u8 (selects C1/C2/C3 interrupt bank) | None needed | Base address + std feature determines the side |
| **IRQ handlers** | Single HSEM_IRQHandler | MU1_IRQHandler + MU2_IRQHandler | One AtomicWaker per MU instance; `waker_for(mu_id)` selects it |
| **Channel count** | 16 (flat semaphore space) | 8 virtual (2 MU × 4 pairs) | `MuChanInfo::mu_id()` / `tx_rx_id()` split the channel ID |
