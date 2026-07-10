/* Placeholder CM33 memory map for linking the demo firmware ELF.
 * On real hardware the remoteproc carveout addresses come from the device
 * tree; these regions only need to be valid (and disjoint from the SRAM2 IPC
 * region at 0x00060000) for the linker to place code and data. */
MEMORY
{
  FLASH : ORIGIN = 0x80100000, LENGTH = 8M
  RAM   : ORIGIN = 0x80A00000, LENGTH = 8M
}
