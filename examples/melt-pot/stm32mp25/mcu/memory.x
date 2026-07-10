/* CM33 memory map aligned with STM32MP2 vendor LinkerScript.ld.
 * These addresses/sizes must match the device tree remoteproc carveouts. */
MEMORY
{
  NS_VECTOR_TBL (xrw) : ORIGIN = 0x80100000, LENGTH = 1536
  FLASH         (rx)  : ORIGIN = 0x80100600, LENGTH = 8387072
  VIRTIO_SHMEM  (xrw) : ORIGIN = 0x812F8000, LENGTH = 32K
  IPC_SHMEM_1   (xrw) : ORIGIN = 0x81200000, LENGTH = 992K
  RAM           (xrw) : ORIGIN = 0x80A00000, LENGTH = 1024K
}
