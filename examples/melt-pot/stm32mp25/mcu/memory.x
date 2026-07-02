/* Placeholder CM33 memory map for linking the demo firmware ELF.
 * On real hardware the remoteproc carveout addresses come from the device
 * tree; these regions only need to be valid (and disjoint from the SRAM2 IPC
 * region at 0x00060000) for the linker to place code and data. */
MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 512K
  RAM   : ORIGIN = 0x10080000, LENGTH = 256K
}
