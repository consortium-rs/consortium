# NVIC, July 11th, 2026 (Let M33 awake)

In Cortex-M, NVIC serves the task of handling interrupts.

The Consortium IPC handshake graph is as follows:

```
┌──────────┐                                 ┌──────────┐
│          │    step 1. init descriptor      │          │
│          │————————————————————————————————>│          │
│ Cortex-A │       fires an interrupt        │ Cortex-M │
│          │                                 │          │
│          │    step 2. ack descriptor       │          │
│ (AP)     │<————————————————————————————————│ (CM7)    │
│          │      fires an interrupt         │ OR       │
│          │                                 │ (CM33)   │
└──────────┘                                 └──────────┘
```

The interrupt handling mechanism:

- **Cortex-A** running on Rich OS (Linux/OP-TEE). We mainly use [`uio`](https://www.kernel.org/doc/html/v4.18/driver-api/uio-howto.html) for Linux I/O on `BellA` (Type-A Doorbell). That's said:

  ```
  GIC -> Kernel --uio_pdrv_genirq--> /dev/uio2 -> Userspace Code (Consortium) running root
  ```

- **Cortex-M** running on bare-metal (Embassy/RTIC). This is where `nvic` comes into effect.

  ```
  NVIC -> vector table (.isr_vectors) -> interrupt handler.
  ```

In Cortex-M, the vector tables stores at `.isr_vectors` (according to yesterday log).

## The Problem

When debugging with the Cortex-M, we stuck at this section:

```
root@stm32mp2-e3-cb-5f:~# ./consortium-example-defmt-host 0
CDBG uio0: mapped_len=0x2000 capacity=0x2000 read_ptr=0x10 write_ptr=0x71 pending~=0x61 flags=Flags(0x0)
drained 10 CDBG packet(s), 57 payload byte(s) total
packet 0: 4 byte(s) [00 11 7e 00]
consortium controller init: time driver bring-up
packet 1: 8 byte(s) [0a 01 05 3a 20 44 67 00]
consortium controller init: RCC register block at 0x44200000
packet 2: 4 byte(s) [0d 02 7a 00]
consortium controller init: TIM2 clock enabled
packet 3: 4 byte(s) [0e 03 7a 00]
consortium controller init: TIM2 timer driver initialized
packet 4: 4 byte(s) [0c 04 7a 00]
consortium controller init: TIM2 IRQ priority set to 0x80
packet 5: 4 byte(s) [0b 05 7a 00]
consortium controller init: TIM2 IRQ enabled/unmasked
packet 6: 8 byte(s) [2f 06 7a 06 10 80 78 00]
vtor value=2148533760
packet 7: 4 byte(s) [10 07 7a 00]
consortium controller init: mpu/nvic bring-up
packet 8: 4 byte(s) [12 08 7a 00]
peripherals stolen
packet 9: 13 byte(s) [08 09 06 3a 05 40 06 1b 20 05 40 6c 00]
consortium controller init: MPU carveout 0 base=0x20064000 size=0x00004000
drained 10 CDBG packet(s): decoded 10 defmt frame(s), 0 decode error(s)
```

This is the code in `consortium.gen.rs`, `fn init() -> IpcMemoryEndpoints` signature. The function snippet is:

```rust
#[cfg(target_arch = "arm")]
fn init() -> IpcMemoryEndpoints {
    ::consortium_runtime_mcu::log::trace!(
        "consortium controller init: mpu/nvic bring-up"
    );
    // SAFETY: `init` runs once during controller bring-up, in
    // privileged mode, before any other code touches the MPU.
    let mut __core = unsafe { ::cortex_m::Peripherals::steal() };
    ::consortium_runtime_mcu::log::trace!("peripherals stolen");
    // SAFETY: `init` holds exclusive privileged MPU access during
    // bring-up; `base`/`size` are this core's validated shared
    // region from the Consortium hardware layout.
    unsafe {
        ::consortium_runtime_mcu::mpu::disable_cache(
            &mut __core.MPU,
            0u8,
            537280512u32,
            16384u32,
        );
        ::consortium_runtime_mcu::log::trace!(
            "consortium controller init: MPU carveout {} base={:#010x} size={:#010x}",
            0u8, 537280512u32, 16384u32
        );
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u16)]
    pub enum BellaInterrupt {
        IPCC1 = 171u16,
    }
    unsafe impl ::cortex_m::interrupt::InterruptNumber for BellaInterrupt {
        fn number(self) -> u16 {
            self as u16
        }
    }
    // SAFETY: interrupt numbers are emitted from the validated
    // Consortium hardware database for this visible core.
    unsafe {
        ::consortium_hal_stm32mp2::hal::nvic::set_priority(
            BellaInterrupt::IPCC1 as u16,
            0x80,
        );
    }
    // SAFETY: interrupt numbers are emitted from the validated
    // Consortium hardware database for this visible core.
    unsafe {
        ::cortex_m::peripheral::NVIC::unmask(BellaInterrupt::IPCC1);
    }
    ::consortium_runtime_mcu::log::trace!(
        "consortium controller init: NVIC unmask {} (id={})", stringify!(IPCC1),
        BellaInterrupt::IPCC1 as u32
    );
    IpcMemoryEndpoints::new()
}
```

## Progress 1: Stuck at Unmasking NVIC

Then we can assume the code got stuck at the unmask procedure:

```rust
unsafe {
    ::cortex_m::peripheral::NVIC::unmask(BellaInterrupt::IPCC1);
}
```

But looking at the `cortex-m` [source code](https://docs.rs/cortex-m/latest/src/cortex_m/peripheral/nvic.rs.html#115-122), it's just a MMIO:

```rust
    /// Enables `interrupt`
    ///
    /// This function is `unsafe` because it can break mask-based critical sections
    #[inline]
    pub unsafe fn unmask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        // NOTE(ptr) this is a write to a stateless register
        (*Self::PTR).iser[usize::from(nr / 32)].write(1 << (nr % 32))
    }
```

## Skeptism 0: Four Possible Reasons

When the `write` stuck, there might have the following likelihoods:

1. `HardFault`. For example, aligned write or in user mode. This fails because the LED wasn't off, according to [yesterday's log](jul-10-rproc.md)
2. `panic`. For example, `usize::from(nr / 32)` exceeds the iser length. This fails because the LED wasn't off, according to [yesterday's log](jul-10-rproc.md)
3. Ran into an infinite loop.
4. MPU disabling cache made the region unreadable — disputed: Write log after flagging non-cacheable; read log after them all.

## Contradition: `TIM2` goes normally

Looking at the log, we can find `TIM2` was unmasked correctly. Let's look further on `TIM2` code:

```rust
fn init_time_driver() {
    consortium_runtime_mcu::log::trace!("consortium controller init: time driver bring-up");
    // STM32MP257F CM33 non-secure CMSIS names this TIM2_BASE_NS and TIM2_IRQn.
    // The boot/runtime handoff must leave TIM2's kernel clock at this rate.
    // SAFETY: RCC_BASE_NS is the STM32MP257F non-secure RCC register block.
    let rcc = unsafe { consortium_hal_stm32mp2::pac::peripherals::rcc::RCC::from_ptr(RCC_BASE_NS) };
    consortium_runtime_mcu::log::trace!(
        "consortium controller init: RCC register block at {:#010x}",
        RCC_BASE_NS as u32
    );
    rcc.TIM2CFGR().modify(|w| {
        w.set_TIM2RST(false);
        w.set_TIM2EN(true);
        w.set_TIM2LPEN(true);
    });
    consortium_runtime_mcu::log::trace!("consortium controller init: TIM2 clock enabled");

    // SAFETY: TIM2 is reserved for this demo firmware and clocked above; the
    // IRQ handler below forwards TIM2_IRQn to the HAL time driver.
    unsafe {
        consortium_hal_stm32mp2::timer_driver::init_timer(TIM2_BASE_NS, TIM2_CLOCK_HZ);
    }
    consortium_runtime_mcu::log::trace!(
        "consortium controller init: TIM2 timer driver initialized"
    );
    unsafe {
        consortium_hal_stm32mp2::hal::nvic::set_priority(TIM2_IRQ, 0x80);
    }
    consortium_runtime_mcu::log::trace!(
        "consortium controller init: TIM2 IRQ priority set to 0x80"
    );
    unsafe {
        consortium_hal_stm32mp2::hal::nvic::enable(TIM2_IRQ);
    }
    consortium_runtime_mcu::log::trace!("consortium controller init: TIM2 IRQ enabled/unmasked");
}
```

According to [`consortium_hal_stm32mp2::hal::nvic::enable`](https://github.com/consortium-rs/consortium/blob/82fff75b4c3469e48242c915bb89836199d270a8/crates/consortium-hal/src/nvic.rs#L34), it's just a wrapper for `NVIC::unmask`:

```rust
/// Enable (unmask) an interrupt.
///
/// # Safety
///
/// Enabling an interrupt may break mask-based critical sections and requires
/// that a handler for it is installed in the active vector table.
pub unsafe fn enable(irq: u16) {
    // SAFETY: Propagates the caller's critical-section/handler contract.
    unsafe { NVIC::unmask(Irq(irq)) }
}
```

The same setup, the same logic, why one stuck, one didn't?

Then we checked the IRQ table again, at [RM0457](https://www.st.com/resource/en/reference_manual/rm0457-stm32mp2325xx-advanced-armbased-3264bit-mpus-stmicroelectronics.pdf), page 1739 for `TIM2`:

![tim2_irq_stm32mp25](/figures/tim2_irq_mp25.png)

page 1741 for `IPCC1_RX`:

![ipcc_irq_stm32mp25](/figures/ipcc_irq_mp25.png)

There isn't any problems with exact IRQ numbers. What's going on?

## Skeptism 1: Already triggered before led to the recursion.

To validate this, we added an `unpend` command before unmasking:

```rust
::cortex_m::peripheral::NVIC::unpend(BellaInterrupt::IPCC1);
```

That's said, NVIC won't be triggered immediately after unmasking. However, it didn't work — still stucking after `unpend`, before `unmask` execution ends.

## Skeptism 2: EXTI-related.

Looking at the `IPCC1_RX`, I found there is `EXTI1` line, compared to `TIM2`. To examine it, we used `devmem2` to read exact blocks of those EXTI sections.

1. Read `EXTI1_C{m}IMR2`.

   Since `m=1` in A35; `m=2` in M33 inaccessible on A35, we read `EXTI1_C1IMR2` first.

   ![exti_cmimr2_stm32mp25](/figures/exti_cmimr2_mp25.png)

   ```sh
   root@stm32mp2-e3-cb-5f:~# devmem2 0x44220090 w
   /dev/mem opened.
   Memory mapped at address 0xffffab04b000.
   Read at address  0x44220090 (0xffffab04b090): 0x00003800
   ```

   Then we can know that

   ```js
   > 0x00003800 & (1 << 28)
   0
   ```

   Bit `28` (IM60) was `0`.

2. Read `EXTI1_RPR2`.

   ![exti_rpr3_stm32mp25](/figures/exti1_rpr2_mp25.png)

   ```
   root@stm32mp2-e3-cb-5f:~# devmem2 0x4422002c w
   /dev/mem opened.
   Memory mapped at address 0xffffbd3e2000.
   Read at address  0x4422002C (0xffffbd3e202c): 0x00000000
   ```

   Full zero.

3. Read `EXTI1_FPR2`.

   ![exti_fpr3_stm32mp25](/figures/exti1_fpr2_mp25.png)

   ```
   root@stm32mp2-e3-cb-5f:~# devmem2 0x44220030 w
   /dev/mem opened.
   Memory mapped at address 0xffffae16e000.
   Read at address  0x44220030 (0xffffae16e030): 0x00000000
   ```

   Full zero.

This didn't contribute: both `EXTI1_FPR2` and `EXTI1_RPR2` gave reserved bit for `IM60`. `EXTI` now shows the `IPCC1_RX` path masked and clear, so it directly nagivates to `NVIC` on-chip rather than missing direction. EXTI skeptism failed.

## Insane Skeptism 3: `cortex-m` crate is tricking us.

What if it's `cortex-m` crate calling issue? We'd write one MMIO on ourselves:

```rust
defmt::info!("before iser write");
unsafe {
    core::ptr::write_volatile(0xE000_E114 as *mut u32, 1u32 << 11);
}
defmt::info!("after iser write");
```

And the result is:

```
packet 10: 4 byte(s) [2f 0a 7a 00]
before iser write
drained 11 CDBG packet(s): decoded 11 defmt frame(s), 0 decode error(s)
root@stm32mp2-e3-cb-5f:~#
```

When trying to add some magic to write `0x0926` to `0x20064000` (IPC section), there isn't any indicators, either.

```
root@stm32mp2-e3-cb-5f:~# ./2607092332_uio_batch /dev/uio1 r 0x0
/dev/uio1 [window 0] offset 0x0 (4 bytes):
  0x0000: 43 50 49 43
As ASCII:
  "CPIC"
As 32-bit words:
  0x0000: 43495043
root@stm32mp2-e3-cb-5f:~# ./2607092332_uio_batch /dev/uio0 r 0x0
/dev/uio0 [window 0] offset 0x0 (4 bytes):
  0x0000: 47 42 44 43
As ASCII:
  "GBDC"
As 32-bit words:
  0x0000: 43444247
```

(ISER5 because `171 / 32 = 5`, bit `171 % 32 = 11`, replacing NVIC::unmask itself here, not adding alongside it, so we're testing the raw write in isolation from whatever the wrapper does.)

## Progress 2: IPCC Interrupt _Should_ be Generated

1. `IPCC_C1MR` word indicating the mask:

   ![ipcc_c1mr_mp25](/figures/ipcc_c1mr_mp25.png)

   ```
   /* (IPCC1) IPCC_C1MR */
   root@stm32mp2-e3-cb-5f:~# devmem2 0x40490004 w
   /dev/mem opened.
   Memory mapped at address 0xffff93a27000.
   Read at address  0x40490004 (0xffff93a27004): 0x0FFF0FFD
   ```

2. `IPCC_C1TOC2SR` word indicating the state:

   ![ipcc_c1toc2sr_1_mp25](/figures/ipcc_c1toc2sr_1_mp25.png)
   ![ipcc_c1toc2sr_2_mp25](/figures/ipcc_c1toc2sr_2_mp25.png)

   ```
   root@stm32mp2-e3-cb-5f:~# devmem2 0x4049000c w
   /dev/mem opened.
   Memory mapped at address 0xffffb493d000.
   Read at address  0x4049000C (0xffffb493d00c): 0x00000001
   ```

   So the interrupt _should_ be fired.

## Skeptism 3: Clock issue of IPCC.

Compared to `TIM2`, `IPCC` didn't have a place for handling `RCC`. That said, the peripheral _might_ didn't work for M33, just because M33 didn't power it up on its domain.

However, this claim is fragile. According to [the day before yesterday](jul-9-dts.md), we powered up `IPCC1` by assigning `clocks` in Linux `dts` — otherwise it would directly tell you `Bus error`.

To check if we can reset the `IPCC1` peripheral on M-core, we had to inquiry RCC: `RCC_IPCC1CFGR` (page 1172).

![rcc_ipcc1cfgr_mp25](/figures/rcc_ipcc1cfgr_mp25.png)

```
root@stm32mp2-e3-cb-5f:~# devmem2 0x44200570 w
/dev/mem opened.
Memory mapped at address 0xffff9510a000.
Read at address  0x44200570 (0xffff9510a570): 0x00000006
```

`IPCC1LPEN` is `true`, `IPCC1EN` is true, so the clock is enabled.

Then we need to explore which register controls the permission of `RCC_IPCC1CFGR` (page 1075):

![rcc_r87cidcfgr_mp25](/figures/rcc_r87cidcfgr_mp25.png)

Obviously, the `RCC_IPCC1CFGR` is managed by `RCC_R87CIDCFGR`.

On page 1088, the definition of `RCC_R87CIDCFGR` is:

![rcc_rxcidcfgr_mp25](/figures/rcc_rxcidcfgr_mp25.png)

```
root@stm32mp2-e3-cb-5f:~# devmem2 0x442002e8 w
/dev/mem opened.
Memory mapped at address 0xffffbb8fe000.
Read at address  0x442002E8 (0xffffbb8fe2e8): 0x00020003
```

`CFEN=1` (bit0), `SEM_EN=1` (bit1), `SEMWLC[7:0]=0x02` (bit17 of the full register → whitelist bit for compartment 1), SCID bits ignored since `SEM_EN=1`. Per the spec text, this governs write access to `RCC_IPCC1CFGR` itself — i.e., which CID may take the semaphore to enable/disable/reset the `IPCC1` clock — not access to the `IPCC` peripheral's own registers, and not anything in the CM33's core-private space.

Clock skeptism failed.

## Skeptism 4: Firmware Itself (`VTOR` table corruption)

To investiage this, a statement added:

```rust
    let vtor = unsafe { &*(cortex_m::peripheral::SCB::PTR) }.vtor().read();
    consortium_runtime_mcu::log::trace!("vtor value={}", vtor);
```

And the result is:

```
vtor value=2148533760
```

(= 0x8010_0000)

This matches the `m33-cube-fw` in the dts, which means that it's correct. This skeptism fails.

## Skeptism 5: Missing NVIC priority configuration

The MSP initialization from STM32CubeMX contains `HAL_NVIC_SetPriority` and `HAL_NVIC_EnableIRQ`.

```c

/**
  * @brief IPCC MSP Initialization
  * This function configures the hardware resources used in this example
  * @param hipcc: IPCC handle pointer
  * @retval None
  */
void HAL_IPCC_MspInit(IPCC_HandleTypeDef* hipcc)
{
  if(hipcc->Instance==IPCC1)
  {
    /* USER CODE BEGIN IPCC1_MspInit 0 */

    /* USER CODE END IPCC1_MspInit 0 */
    /* IPCC1 interrupt Init */
    HAL_NVIC_SetPriority(IPCC1_RX_IRQn, 1, 0);
    HAL_NVIC_EnableIRQ(IPCC1_RX_IRQn);
    /* USER CODE BEGIN IPCC1_MspInit 1 */

    /* USER CODE END IPCC1_MspInit 1 */

  }

}
```

Added:

```rust
    // SAFETY: interrupt numbers are emitted from the validated
    // Consortium hardware database for this visible core.
    unsafe {
        ::consortium_hal_stm32mp2::hal::nvic::set_priority(
            BellaInterrupt::IPCC1 as u16,
            0x80,
        );
    }
```

Still fails at unmask.

## Experiment: `TIM6`

To understand whether this stucking problem occurs only for IPCC or is universal, I found `TIM6`, which is also assigned to M33, to test about it:

![tim_risup_mp25](/figures/tim_risup_mp25.png)

### Cold Start: Stuck

Refer to `TIM2`, I added the following set-up code:

```rust
fn init_time_driver() {
    consortium_runtime_mcu::log::trace!("consortium controller init: time driver bring-up");
    // STM32MP257F CM33 non-secure CMSIS names this TIM2_BASE_NS and TIM2_IRQn.
    // The boot/runtime handoff must leave TIM2's kernel clock at this rate.
    // SAFETY: RCC_BASE_NS is the STM32MP257F non-secure RCC register block.
    let rcc = unsafe { consortium_hal_stm32mp2::pac::peripherals::rcc::RCC::from_ptr(RCC_BASE_NS) };
    consortium_runtime_mcu::log::trace!("consortium controller init: RCC register block at {:#010x}", RCC_BASE_NS as u32);
    rcc.TIM2CFGR().modify(|w| {
        w.set_TIM2RST(false);
        w.set_TIM2EN(true);
        w.set_TIM2LPEN(true);
    });
    consortium_runtime_mcu::log::trace!("consortium controller init: TIM2 clock enabled");

    consortium_runtime_mcu::log::trace!("test unmask tim6 irq start");
    unsafe {
        consortium_hal_stm32mp2::hal::nvic::enable(consortium_hal_stm32mp2::Interrupt::TIM6 as u16);
    }
    consortium_runtime_mcu::log::trace!("test unmask tim6 irq stop");
```

And the result is:

```
root@stm32mp2-e3-cb-5f:~# ./consortium-example-defmt-host 0
CDBG uio0: mapped_len=0x2000 capacity=0x2000 read_ptr=0x10 write_ptr=0x34 pending~=0x24 flags=Flags(0x0)
drained 4 CDBG packet(s), 20 payload byte(s) total
packet 0: 4 byte(s) [00 13 7e 00]
consortium controller init: time driver bring-up
packet 1: 8 byte(s) [0c 01 05 3a 20 44 67 00]
consortium controller init: RCC register block at 0x44200000
packet 2: 4 byte(s) [0f 02 7a 00]
consortium controller init: TIM2 clock enabled
packet 3: 4 byte(s) [17 03 7a 00]
test unmask tim6 irq start
drained 4 CDBG packet(s): decoded 4 defmt frame(s), 0 decode error(s)
```

That said, stuck again.

### Add Clock: Stuck

Adding the clock set-up code:

```rust
    rcc.TIM6CFGR().modify(|w| {
        w.set_TIM6RST(false);
        w.set_TIM6EN(true);
        w.set_TIM6LPEN(true);
    });
```

But the result didn't change:

```
packet 3: 4 byte(s) [17 03 7a 00]
test unmask tim6 irq start
packet 4: 4 byte(s) [18 04 7a 00]
test unmask tim6 irq start: rcc done
drained 5 CDBG packet(s): decoded 5 defmt frame(s), 0 decode error(s)
root@stm32mp2-e3-cb-5f:~#
```

### Add Handler: Stuck

Adding the following handler code:

```rust
#[interrupt]
fn TIM6() {
    unsafe { core::ptr::write_volatile(0x442b_0028 as *mut u32, 1u32 << 6); }
}
```

But the LED wasn't off, i.e., the interrupt logic wasn't entered.

## Weird Skeptism 6: Flat `NVIC/ISER` implementation limit at IRQ128 (ISER4+)

Based on TIM2 (105, works) vs TIM6 (128, hangs) vs IPCC1 (171, hangs), view ITCR number:

```rust
let ictr = unsafe { core::ptr::read_volatile(0xE000_E004 as *const u32) };
defmt::info!("ICTR={:#010x} intlinesnum={=u32}", ictr, ictr & 0xF);
```

Result:

```
ICTR=0x00000009 intlinesnum=9
```

`ICTR=0x9` → `INTLINESNUM=9` → implemented lines `= 32 × (9+1) = 320`, i.e. `ISER0..ISER9` are all real, backed hardware. `IRQ128` (`ISER4`) and `IRQ171` (`ISER5`) are both well within that — this cleanly rules out any "unimplemented ISER register" theory.

## Weird Skeptism 7: `AIRCR` unset.

`HAL_NVIC_SetPriorityGrouping()` writes `SCB->AIRCR` with the `VECTKEY` (`0x05FA` in the upper halfword) to set the priority grouping split. Our `init()` never touches AIRCR at all — priority grouping is left at whatever the boot chain last configured it to, or its architectural reset value.

```rust
unsafe {
    core::ptr::write_volatile(0xE000_ED0C as \*mut u32, (0x5FA << 16) | (3 << 8)); // AIRCR, VECTKEY + PRIGROUP=3
}
defmt::info!("aircr set");
```

Result:

```
packet 3: 4 byte(s) [14 03 7a 00]
consortium controller init: setting AIRCR (for testing)
packet 4: 4 byte(s) [08 04 7a 00]
consortium controller init: AIRCR set
packet 5: 4 byte(s) [19 05 7a 00]
test unmask tim6 irq start
packet 6: 4 byte(s) [1a 06 7a 00]
test unmask tim6 irq start: rcc done
drained 7 CDBG packet(s): decoded 7 defmt frame(s), 0 decode error(s)
root@stm32mp2-e3-cb-5f:~#
```

This didn't help, either.

## Skeptism 8: IPCC Doorbell didn't clean-up

Refer to the Embassy's `IPCC` implementation, we re-implemented the `no_std` waking mechanism:

```rust
pub unsafe fn notify_rx_occupied_interrupt<S: Side, const N: usize>(
    base: usize,
) {
    let ipcc = unsafe { Ipcc::<S, N>::new(base) };
    let occupied = ipcc.incoming_occupied_channels();

    ipcc.mask_rx_occupied_channels(occupied);

    for id in 0..IPCC_CHAN_COUNT {
        if occupied & (1 << id) != 0 {
            channel_waker::<N>(id).wake();
        }
    }
}

#[interrupt]
fn IPCC1_RX() {
    on_rx_occupied_interrupt::<Secondary, 1>()
}
```

But it still got stuck.

## Progress 2: `HardFault` v.s. `IRQ`

Again, even we use bare-metal access, it still didn't work:

```rust
#[interrupt]
fn IPCC1_RX() {
    unsafe {
        // Marker first.
        core::ptr::write_volatile(0x2006_4000 as *mut u32, 0x0926);

        // C2MR.CH0OM = 1: mask CM33 RX-occupied channel 0.
        let c2mr = 0x4049_0014 as *mut u32;
        let value = core::ptr::read_volatile(c2mr);
        core::ptr::write_volatile(c2mr, value | 1);
    }
}
```

It reads:

```
root@stm32mp2-e3-cb-5f:~# ./2607092332_uio_batch /dev/uio1 r 0x0
/dev/uio1 [window 0] offset 0x0 (4 bytes):
  0x0000: 43 50 49 43
As ASCII:
  "CPIC"
As 32-bit words:
  0x0000: 43495043
```

No `0x0926` at `0x2006_4000`, so the `#[interrupt]` didn't work entirely.

To literally identify which block it eventually entered, we splitted into two handling functions: `#[interrupt] fn IPCC_RX()` and `#[cortex_m_rt::execption] unsafe fn DefaultHandler(irqn: i16)`.

### `IPCC1_RX` Interrupt

```rust
#[consortium_hal_stm32mp2::interrupt]
fn IPCC1_RX() {
    unsafe {
        core::ptr::write_volatile(0x442b_0028 as *mut u32, 1u32 << 6);
        core::ptr::write_volatile(0x2006_4000 as *mut u32, 0x0926);
    }

    unsafe {
        consortium_ipc_doorbell_ipcc::notify_rx_occupied_interrupt::<
            consortium_ipc::Secondary,
            1,
        >(0x4049_0000);
    }

    consortium_runtime_mcu::log::trace!(
        "ipcc1_rx interrupt: doorbell RX occupied"
    );
}
```

### `DefaultHandler` Execption

```rust
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_irqn: i16) {
    let icsr = unsafe { core::ptr::read_volatile(0xE000_ED04 as *const u32) };
    let vectactive = icsr & 0x1ff;

    // Dedicated diagnostic output; preserve the existing diagnostics too.
    unsafe {
        core::ptr::write_volatile(0x2006_4000 as *mut u32, 0xD000_0000 | vectactive);
        core::ptr::write_volatile(0x2006_4010 as *mut i16, irqn);
        core::ptr::write_volatile(0x442b_0028 as *mut u32, 1u32 << 6);
    }

    loop {
        core::hint::spin_loop();
    }
}
```

Then LED is off, i.e., either `IPCC1_RX` or `DefaultHandler` is entered. Then we can extract the data:

```
root@stm32mp2-e3-cb-5f:~# devmem2 0x20064000 h
/dev/mem opened.
Memory mapped at address 0xffff8621c000.
Read at address  0x20064000 (0xffff8621c000): 0x00BB
root@stm32mp2-e3-cb-5f:~# devmem2 0x20064000 w
/dev/mem opened.
Memory mapped at address 0xffffb6342000.
Read at address  0x20064000 (0xffffb6342000): 0xD00000BB
root@stm32mp2-e3-cb-5f:~# devmem2 0x20064010 w
/dev/mem opened.
Memory mapped at address 0xffffb67ef000.
Read at address  0x20064010 (0xffffb67ef010): 0x98BE00AB
root@stm32mp2-e3-cb-5f:~# devmem2 0x20064010 h
/dev/mem opened.
Memory mapped at address 0xffff8c189000.
Read at address  0x20064010 (0xffff8c189010): 0x00AB
```

Given `0x00ab`, we can find:

```
VECTACTIVE = 0xBB = 187 = 16 + IRQ171
irqn       = 0xAB = 171
```

So IRQ171 is taken and DefaultHandler executes. No `IPCC` or `NVIC` ambiguity remains.

### Why it enters `DefaultHandler`?

Investigate the current `elf`:

```
VTOR                       0x80100600
IRQ171 slot                0x801008EC
ELF word at 0x801008EC     0x80100BB5
IPCC1_RX                   0x80100BB5
```

Read both side `vtor` to eliminate cache coherency issues:

- A-core:

  ```sh
  root@stm32mp2-e3-cb-5f:~# devmem2 0x801008ec w
  /dev/mem opened.
  Memory mapped at address 0xffffaa860000.
  Read at address  0x801008EC (0xffffaa8608ec): 0x80100BB5
  ```

- M-core:

  ```rust
  let vector = unsafe {
      core::ptr::read_volatile(0x8010_08ec as *const u32)
  };

  defmt::info!("cm33 irq171 vector={=u32:#010x}", vector);

  cortex_m::asm::dsb();
  cortex_m::asm::isb();

  cortex_m::peripheral::NVIC::pend(
      consortium_hal_stm32mp2::Interrupt::IPCC1_RX,
  );
  ```

  Result:

  ```
  packet 20: 9 byte(s) [36 14 b5 3a 0b 10 80 78 00]
  cm33 irq171 vector=0x80100bb5
  drained 21 CDBG packet(s): decoded 21 defmt frame(s), 0 decode error(s)
  ```

Collect what we had:

```
VTOR written/read:             0x80100600
base with bit 0x200 removed:   0x80100400
IRQ171 offset:                 0x2EC
candidate fetch address:       0x801006EC
word at 0x801006EC:            0x80100B75
DefaultHandler:                0x80100B75
```

Then we can find a problem: `0x80100400 + 0x2EC > 0x80100600`.

## Experiment: Whether Overflow Truncates Custom Interrupt Handler

The observed transition point can be tested exactly. Address `0x80100800` corresponds to exception number 128, which is external IRQ112:

```
IRQ111 I3C2 → vector at 0x801007FC, immediately below boundary
IRQ112 SPI1 → vector at 0x80100800, exactly at boundary
```

Then we can conduct experiments on `I3C2` and `SPI1`:

```rust
#[interrupt]
fn I3C2() {
    unsafe {
        core::ptr::write_volatile(0x2006_4014 as *mut u32, 0x1111_1111);
    }
}

#[interrupt]
fn SPI1() {
    unsafe {
        core::ptr::write_volatile(0x2006_4014 as *mut u32, 0x1121_1211);
    }
}

NVIC::unmask(Interrupt::I3C2);
NVIC::pend(Interrupt::I3C2);

NVIC::unmask(Interrupt::SPI1);
NVIC::pend(Interrupt::SPI1);
```

Then we got the result:

```
packet 20: 9 byte(s) [3b 14 c9 3a 0b 10 80 78 00]
cm33 irq171 vector=0x80100bc9
packet 21: 4 byte(s) [2a 15 7a 00]
test interrupt i3c2
packet 22: 4 byte(s) [2b 16 7a 00]
test interrupt i3c2 unmasked
packet 23: 4 byte(s) [2c 17 7a 00]
test interrupt spi1
packet 24: 4 byte(s) [2d 18 7a 00]
test interrupt spi1 unmasked
drained 25 CDBG packet(s): decoded 25 defmt frame(s), 0 decode error(s)
```

This confirms an exact boundary:

- `IRQ111` (`I3C2`, exception `127`, last word before `0x80100800`) executes correctly.
- `IRQ112` (`SPI1`, exception `128`, first word at `0x80100800`) enters the wrong path and sticks.

Try to extend the `NS_VECTOR_TBL`:

```link.x
NS_VECTOR_TBL: 0x80100000, length 0x800
FLASH:         0x80100800, length 0x7ff800
```

Then we can find the `.vector_table`:

```
.vector_table address:  0x80100800
.vector_table size:     0x51c
IRQ171 slot:            0x80100aec
IRQ171 value:           0x80100dc9 → IPCC1_RX
.text starts:           0x80100d1c
```

Then it worked.
