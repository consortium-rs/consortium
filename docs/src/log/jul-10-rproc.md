# RemoteProc, July 10, 2026

`remoteproc` executes the Cortex-M33 core in STM32MP257 and the Cortex-M7 core in i.MX 95 (MIMX9596). The conventional experience is:

```sh
# stop
echo stop > /sys/class/remoteproc/remoteproc0/state
# switch firmware
cp ~/cm33.elf /lib/firmware
echo cm33.elf > /sys/class/remoteproc/remoteproc0/firmware
# start
echo stop > /sys/class/remoteproc/remoteproc0/state
```

## Attempt 1: Resource Table Lacks

```
[  158.179437] remoteproc remoteproc1: stopped remote processor m33
[  194.588530] remoteproc remoteproc1: powering up m33
[  194.652612] remoteproc remoteproc1: Booting fw image cm33.elf, size 2026444
[  194.654000] remoteproc remoteproc1: no resource table found for this firmware
[  194.661328] remoteproc remoteproc1: bad phdr da 0x10000000 mem 0x800
[  194.667622] remoteproc remoteproc1: Failed to load program segments: -22
[  194.674620] remoteproc remoteproc1: Boot failed: -22
[  224.113632] remoteproc remoteproc1: powering up m33
[  224.180061] remoteproc remoteproc1: Booting fw image cm33.elf, size 2026444
[  224.181457] remoteproc remoteproc1: no resource table found for this firmware
[  224.188790] remoteproc remoteproc1: bad phdr da 0x10000000 mem 0x800
[  224.194972] remoteproc remoteproc1: Failed to load program segments: -22
[  224.202025] remoteproc remoteproc1: Boot failed: -22
```

### Firmware Anlaysis

We dumped two firmware, one pre-installed UCPD firmware, one custom firmware.

#### Official dump

```
Elf file type is EXEC (Executable file)
Entry point 0x801145e9
There are 4 program headers, starting at offset 52
Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  LOAD           0x001000 0x80100000 0x80100000 0x00540 0x00540 R   0x1000
  LOAD           0x001600 0x80100600 0x80100600 0x1a1f0 0x1a1f0 R E 0x1000
  LOAD           0x01c000 0x80a00000 0x80a00000 0x002d0 0x11220 RW  0x1000
  LOAD           0x01d000 0x81200000 0x81200000 0x00090 0x00090 R   0x1000
 Section to Segment mapping:
  Segment Sections...
   00     .isr_vectors
   01     .text .rodata .ARM
   02     .init_array .fini_array .data .bss ._user_heap_stack
   03     .resource_table
```

#### Our dump

```
Elf file type is EXEC (Executable file)
Entry point 0x60000801
There are 6 program headers, starting at offset 52
Program Headers:
  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align
  LOAD           0x000114 0x60000000 0x60000000 0x00800 0x00800 R   0x4
  LOAD           0x000914 0x60000800 0x60000800 0x0bda8 0x0bda8 R E 0x4
  LOAD           0x00c6c0 0x6000c5a8 0x6000c5a8 0x07bc8 0x07bc8 R   0x8
  LOAD           0x014288 0x20040000 0x60014170 0x00018 0x00018 RW  0x8
  LOAD           0x0142a0 0x20040018 0x20040018 0x00000 0x0020c RW  0x8
  GNU_STACK      0x000000 0x00000000 0x00000000 0x00000 0x00000 RW  0
 Section to Segment mapping:
  Segment Sections...
   00     .vector_table
   01     .text
   02     .rodata
   03     .data
   04     .bss
   05
```

It's clear that we didn't have `.resource_table` at least, according to the `rproc dmesg`.

### Add `ResourceTable`

It's straightforward to add resource table here because we don't use any OpenAMP- and RemoteProc-compatible vring and rpmsg.

```rust
#[repr(C, packed)]
struct ResourceTable {
    ver: u32,
    num: u32,
    reserved: [u32; 2],
}

#[link_section = ".resource_table"]
#[used]
static RESOURCE_TABLE: ResourceTable = ResourceTable {
    ver: 1,
    num: 0,
    reserved: [0, 0],
};
```

Then it worked according to `rproc`.

```
[ 8960.582799] remoteproc remoteproc0: stopped remote processor m33
[ 8965.854170] remoteproc remoteproc0: powering up m33
[ 8965.877304] remoteproc remoteproc0: Booting fw image cm33.elf, size 698292
[ 8965.878734] remoteproc remoteproc0: no resource table found for this firmware
[ 8965.886106] remoteproc remoteproc0: remote processor m33 is now up
```

## Attempt 2: Processor Up without GPIO Flip or Shared Memory Write

To check if M-core is alive, I wrote the logic to flip the GPIO once it enters main loop, and then write shared memory to load which step it entered.

According to [UM3385](https://www.st.com/resource/en/user_manual/um3385-discovery-kit-with-stm32mp257f-mpu-stmicroelectronics.pdf), there are `PH4`, `PH5`, `PH6`, and `PH7` on-board LEDs:

![led_dk_mp25](/figures/led_dk_mp25.png)

And `PH6` is granted to CM33:

![gpio_pgph_rif_mp25](/figures/gpio_pgph_rif_mp25.png)

According to the `hal`:

```rust
    #[inline]
    fn write_high(&self) {
        // BSRR low half is atomic bit-set; other pins are unaffected.
        self.port.BSRR().write_value(GPIO_BSRR(self.mask()));
    }

    #[inline]
    fn write_low(&self) {
        // BRR is atomic bit-reset; other pins are unaffected.
        self.port.BRR().write_value(GPIO_BRR(self.mask()));
    }
```

It's easy that we write the code to flip the GPIO to indicate the firmware is in panic or HardFault mode.

```rust

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    unsafe {
        core::ptr::write_volatile(0x442b_0028 as *mut u32, 1u32 << 6);
    }

    let mut current_addr = 0x2006_4000 as *mut u8;
    let end_addr = 0x2006_8000 as *mut u8;

    let prefix = b"[PANIC]";

    for &byte in prefix {
        unsafe {
            write_volatile(current_addr, byte as u8);
            current_addr = current_addr.add(1);
        }
    }

    if let Some(location) = info.location() {
        let file_bytes = location.file().as_bytes();
        let line_count = location.line();

        for &byte in file_bytes {
            unsafe {
                write_volatile(current_addr, byte as u8);
                current_addr = current_addr.add(1);
            }
        }

        if current_addr < end_addr {
            unsafe {
                write_volatile(current_addr, b':' as u8);
                current_addr = current_addr.add(1);
            }
        }

        let mut line_buf = [0u8; 10];
        let mut line_index = 0;
        let mut line = line_count;
        while line > 0 && line_index < line_buf.len() {
            line_buf[line_index] = b'0' + (line % 10) as u8;
            line /= 10;
            line_index += 1;
        }

        for i in (0..line_index).rev() {
            unsafe {
                write_volatile(current_addr, line_buf[i] as u8);
                current_addr = current_addr.add(1);
            }
        }
    }

    if current_addr < end_addr {
        unsafe {
            write_volatile(current_addr, b'\0' as u8);
        }
    }

    loop {
        compiler_fence(core::sync::atomic::Ordering::SeqCst);
        cortex_m::asm::wfi();
    }
}

#[cortex_m_rt::exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    cortex_m::interrupt::disable();

    unsafe {
        core::ptr::write_volatile(0x442b_0028 as *mut u32, 1u32 << 6);
    }

    let addr = 0x2006_4000 as *mut u32;

    unsafe {
        core::ptr::write_volatile(addr, b'H' as u32);
    }
    unsafe {
        core::ptr::write_volatile(addr.add(1), b'F' as u32);
    }
    unsafe {
        core::ptr::write_volatile(addr.add(2), b'\0' as u32);
    }

    let pc = ef.pc();
    let pc_bytes = pc.to_le_bytes();
    for (i, &byte) in pc_bytes.iter().enumerate() {
        unsafe {
            core::ptr::write_volatile(addr.add(3 + i), byte as u32);
        }
    }

    loop {
        compiler_fence(core::sync::atomic::Ordering::SeqCst);
        cortex_m::asm::wfi();
    }
}
```

But currently, the orange LED is still on, indicating either:

1. Running successfully — false because no shared memory touched.
2. Didn't enter the main at all.

## Attempt 3: Manual ASM

```asm
.section .text.entry
.global _start
_start:
    ldr r0, =0x442b0028
    mov r1, #0x40
    str r1, [r0]
b rust_main
```

But the LED was off again, indicating NOTHING ran.

## Attempt 4: Is the Power on?

Investiaging the `dts` we can find `st,syscfg-cm-state = <0x98 0x204 0x0c>`, which resolves to:

```
base  = 0x44210000   (from this PWR syscon node)
offset = 0x204
mask   = 0x0c         (bits [3:2])
```

This is the register `PWR_CPU2D2SR`, according to the reference manual page 934–935:

![pwr_cpu2d2sr_1_mp25](/figures/pwr_cpu2d2sr_1_mp25.png)
![pwr_cpu2d2sr_2_mp25](/figures/pwr_cpu2d2sr_2_mp25.png)

And had a script for automatic detection:

```sh
# 1. Before powering up
devmem 0x44210204 32

# 2. Trigger boot
echo start > /sys/class/remoteproc/remoteproc0/state
sleep 1
devmem 0x44210204 32

# 3. After it's "up" (per dmesg), wait a few seconds for any settle time
devmem 0x44210204 32

# 4. Then stop
echo stop > /sys/class/remoteproc/remoteproc0/state
devmem 0x44210204 32
```

And here's the result:

```
running
/dev/mem opened.
Memory mapped at address 0xffffb3674000.
Read at address  0x44210204 (0xffffb3674204): 0x00000001
/dev/mem opened.
Memory mapped at address 0xffff96ce2000.
Read at address  0x44210204 (0xffff96ce2204): 0x00000004
/dev/mem opened.
Memory mapped at address 0xffffa28e4000.
Read at address  0x44210204 (0xffffa28e4204): 0x00000004
/dev/mem opened.
Memory mapped at address 0xffff89f43000.
Read at address  0x44210204 (0xffff89f43204): 0x00000004
/dev/mem opened.
Memory mapped at address 0xffffa01f1000.
Read at address  0x44210204 (0xffffa01f1204): 0x00000001
```

Hence, the started value is `0x00000004` and the stopped value is `0x00000001` — it did power on.

## Attempt 5: Enable Dynamic Debug for Rproc

```sh
echo 'module stm32_rproc +p' > /sys/kernel/debug/dynamic_debug/control
echo start > /sys/class/remoteproc/remoteproc0/state
dmesg | tail -50
```

And the output is:

```
[ 175.077928] remoteproc remoteproc0: powering up m33
[ 175.194285] remoteproc remoteproc0: Booting fw image cm33.elf, size 2035168
[ 175.195647] stm32-rproc 0.m33: pa 0x0000000080100000 to da 80100000
[ 175.195664] stm32-rproc 0.m33: pa 0x0000000080a00000 to da 80a00000
[ 175.195675] stm32-rproc 0.m33: pa 0x0000000081200000 to da 81200000
[ 175.195686] stm32-rproc 0.m33: pa 0x00000000812f8000 to da 812f8000
[ 175.195697] stm32-rproc 0.m33: pa 0x00000000812f9000 to da 812f9000
[ 175.195707] stm32-rproc 0.m33: pa 0x00000000812fa000 to da 812fa000
[ 175.195717] stm32-rproc 0.m33: pa 0x00000000a060000 to da a060000
[ 175.200400] stm32-rproc 0.m33: map memory: 0x0000000080100000+800000
[ 175.200546] stm32-rproc 0.m33: map memory: 0x0000000080a00000+800000
[ 175.200560] stm32-rproc 0.m33: map memory: 0x0000000081200000+f8000
[ 175.200583] stm32-rproc 0.m33: map memory: 0x00000000812f8000+1000
[ 175.200601] stm32-rproc 0.m33: map memory: 0x00000000812f9000+1000
[ 175.200614] stm32-rproc 0.m33: map memory: 0x00000000a060000+20000
[ 175.200692] remoteproc remoteproc0: boot vector address = 0x0
[ 175.200832] rproc-virtio rproc-virtio.2.auto: assigned reserved memory node vdev0buffer@812fa000
[ 175.212945] virtio_rpmsg_bus virtio0: rpmsg host is online
[ 175.213021] rproc-virtio rproc-virtio.2.auto: registered virtio0 (type 7)
[ 175.224106] remoteproc remoteproc0: remote processor m33 is now up
```

Look at the line `[175.200692]`:

```
[ 175.200692] remoteproc remoteproc0: boot vector address = 0x0
```

Should boot vector address be `0x0`? If we read the elf, we can find:

```sh
readelf -h ./dist/firmware/cm33.elf | grep -i entry
  Entry point address:               0x80100e01
```

So it was the problem.

## Attempt 6: `.isr_vectors` Section as Hero

According to an [ST Community Forum](https://community.st.com/stm32-mpus-products-and-hardware-related-39/iac-exceptions-when-launching-m33-core-145981?postid=646273#post646273):

> The boot address is computed based on `.isr_vectors` section that should be defined in your Cortex-M33 firmware linker script.
>
> ```
>  /* The startup code into "NS_VECTOR_TBL" Ram type memory
>  * Do not update the ".isr_vectors" section name. the name
>  * is used by the Linux kernel to retrieve the address
>  * of the vector table*/
>  .isr_vectors :
>  {
>  . = ALIGN(4);
>  KEEP(*(.isr_vectors))
>  . = ALIGN(4);
>  } >NS_VECTOR_TBL
> ```

Add this to `memory.x`, and then the address is correct:

```
[  536.656497] remoteproc remoteproc0: boot vector address = 0x80100000
```

But still didn't work: no gpio flip, no volatile write, etc. Hence, we used `loadelf`:

```
Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .isr_vectors      PROGBITS        80100000 000134 000000 00   A  0   0  1
  [ 2] .vector_table     PROGBITS        80100600 000134 000800 00   A  0   0  4
```

This indicates an **empty** `.isr_vectors`.

## Attempt 7: `.isr_vectors` v.s. `.vector_table`

First we need to verify the `.vector_table` to be truly populated:

```sh
objcopy -O binary --only-section=.vector_table ./dist/firmware/cm33.elf vtor_orig.bin
xxd vtor_orig.bin | head -2
```

Then we dump the binary:

```
ethanwu@ethanwu:~/consortium/examples/melt-pot/stm32mp25$ xxd vtor_orig.bin | head -2
00000000: 0000 b080 010e 1080 f725 1080 2d48 1080  .........%..-H..
00000010: f725 1080 f725 1080 f725 1080 f725 1080  .%...%...%...%..
```

The original `.vector_table` was okay; perhaps renaming is enough:

```sh
arm-none-eabi-objcopy \
  --rename-section .vector_table=.isr_vectors \
  ./dist/firmware/cm33.elf ./dist/firmware/cm33_fixed.elf
```

Then it worked.
