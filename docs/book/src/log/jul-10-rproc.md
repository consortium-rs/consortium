# RemoteProc, July 10, 2026 (Powering On the M33)

`remoteproc` starts the Cortex-M33 core on STM32MP257 and the Cortex-M7 core on i.MX 95 (MIMX9596). The conventional workflow is:

```sh
# stop
echo stop > /sys/class/remoteproc/remoteproc0/state
# switch firmware
cp ~/cm33.elf /lib/firmware
echo cm33.elf > /sys/class/remoteproc/remoteproc0/firmware
# start
echo stop > /sys/class/remoteproc/remoteproc0/state
```

## Attempt 1: Missing Resource Table

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

### Firmware Analysis

We inspected two firmware images: the preinstalled UCPD firmware and our custom firmware.

#### Official Firmware

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

#### Custom Firmware

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

The program headers and `remoteproc` messages showed that our firmware did not contain a `.resource_table` section.

### Adding `ResourceTable`

Adding a resource table was straightforward because this firmware does not use any OpenAMP- or RemoteProc-compatible vrings or RPMsg channels.

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

After this change, `remoteproc` reported that the remote processor was up.

```
[ 8960.582799] remoteproc remoteproc0: stopped remote processor m33
[ 8965.854170] remoteproc remoteproc0: powering up m33
[ 8965.877304] remoteproc remoteproc0: Booting fw image cm33.elf, size 698292
[ 8965.878734] remoteproc remoteproc0: no resource table found for this firmware
[ 8965.886106] remoteproc remoteproc0: remote processor m33 is now up
```

## Attempt 2: Processor Up Without a GPIO Toggle or Shared-Memory Write

To determine whether the M-core was running, I added logic to toggle a GPIO upon entering the main loop and to write progress markers into shared memory.

According to [UM3385](https://www.st.com/resource/en/user_manual/um3385-discovery-kit-with-stm32mp257f-mpu-stmicroelectronics.pdf), the board has LEDs on `PH4`, `PH5`, `PH6`, and `PH7`:

![led_dk_mp25](/figures/led_dk_mp25.png)

`PH6` is assigned to the CM33:

![gpio_pgph_rif_mp25](/figures/gpio_pgph_rif_mp25.png)

The HAL implements the GPIO writes as follows:

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

We therefore added GPIO toggles to indicate whether the firmware entered its panic or HardFault handler.

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

The orange LED remained on, leaving two possibilities:

1. The firmware was running successfully. This was ruled out because shared memory remained untouched.
2. The firmware never entered `main`.

## Attempt 3: Manual Assembly

```asm
.section .text.entry
.global _start
_start:
    ldr r0, =0x442b0028
    mov r1, #0x40
    str r1, [r0]
b rust_main
```

The LED remained off, indicating that even this minimal entry sequence did not run.

## Attempt 4: Is the Core Powered On?

While investigating the DTS, we found `st,syscfg-cm-state = <0x98 0x204 0x0c>`, which resolves to:

```
base  = 0x44210000   (from this PWR syscon node)
offset = 0x204
mask   = 0x0c         (bits [3:2])
```

According to pages 934–935 of the reference manual, this is the `PWR_CPU2D2SR` register:

![pwr_cpu2d2sr_1_mp25](/figures/pwr_cpu2d2sr_1_mp25.png)
![pwr_cpu2d2sr_2_mp25](/figures/pwr_cpu2d2sr_2_mp25.png)

We used the following script to detect the state automatically:

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

The script produced the following result:

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

The running state was therefore `0x00000004`, while the stopped state was `0x00000001`. This confirmed that the core did power on.

## Attempt 5: Enabling Dynamic Debug for `remoteproc`

```sh
echo 'module stm32_rproc +p' > /sys/kernel/debug/dynamic_debug/control
echo start > /sys/class/remoteproc/remoteproc0/state
dmesg | tail -50
```

The resulting output was:

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

The key line was `[175.200692]`:

```
[ 175.200692] remoteproc remoteproc0: boot vector address = 0x0
```

The boot-vector address appeared as `0x0`. Inspecting the ELF showed:

```sh
readelf -h ./dist/firmware/cm33.elf | grep -i entry
  Entry point address:               0x80100e01
```

This identified the incorrect boot-vector address as the problem.

## Attempt 6: Adding the `.isr_vectors` Section

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

After adding this section to `memory.x`, the address was correct:

```
[  536.656497] remoteproc remoteproc0: boot vector address = 0x80100000
```

The firmware still did not run: there was no GPIO toggle or volatile write. We therefore inspected it with `loadelf`:

```
Section Headers:
  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            00000000 000000 000000 00      0   0  0
  [ 1] .isr_vectors      PROGBITS        80100000 000134 000000 00   A  0   0  1
  [ 2] .vector_table     PROGBITS        80100600 000134 000800 00   A  0   0  4
```

This output indicated that `.isr_vectors` was **empty**.

## Attempt 7: `.isr_vectors` vs. `.vector_table`

We first verified that `.vector_table` was populated:

```sh
objcopy -O binary --only-section=.vector_table ./dist/firmware/cm33.elf vtor_orig.bin
xxd vtor_orig.bin | head -2
```

We then dumped the binary:

```
ethanwu@ethanwu:~/consortium/examples/melt-pot/stm32mp25$ xxd vtor_orig.bin | head -2
00000000: 0000 b080 010e 1080 f725 1080 2d48 1080  .........%..-H..
00000010: f725 1080 f725 1080 f725 1080 f725 1080  .%...%...%...%..
```

The original `.vector_table` was valid, suggesting that renaming the section might be sufficient:

```sh
arm-none-eabi-objcopy \
  --rename-section .vector_table=.isr_vectors \
  ./dist/firmware/cm33.elf ./dist/firmware/cm33_fixed.elf
```

After renaming the section, the firmware ran successfully.
