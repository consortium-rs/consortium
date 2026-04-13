/* STM32MP25 memory map (from RPI metadata):
 *   SYSRAM  0x20000000  256K  (M-core firmware code + stack)
 *   RETRAM  0x20040000  128K  (retained across stop mode)
 *   SRAM1   0x20060000   64K  (IPC shared region — accessible by both cores)
 *   LPSRAM1 0x200C0000    8K
 *   LPSRAM2 0x200C2000    8K
 *   LPSRAM3 0x200C4000   16K
 */

MEMORY {
  FLASH  (rx)  : ORIGIN = 0x20000000, LENGTH = 192K   /* firmware code in SYSRAM */
  RAM    (rwx) : ORIGIN = 0x20030000, LENGTH = 64K    /* M-core stack/data in SYSRAM tail */
  SHARED (rw)  : ORIGIN = 0x20060000, LENGTH = 64K    /* IPC: descriptor + slots in SRAM1 */
}

SECTIONS {
  .text : {
    *(.text .text.*)
  } > FLASH

  .data : {
    *(.data .data.*)
  } > RAM AT > FLASH

  .bss : {
    *(.bss .bss.*)
  } > RAM

  /DISCARD/ : {
    *(.note.GNU-stack)
  }
}
