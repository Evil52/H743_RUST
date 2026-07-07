MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
    /* AXI SRAM (D1). Chosen over DTCM because the DCMI DMA (D2 master)
       can reach it through the bus matrix; DTCM is CPU-only. */
    RAM   : ORIGIN = 0x24000000, LENGTH = 512K
}
