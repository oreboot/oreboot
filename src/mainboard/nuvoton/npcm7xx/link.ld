ENTRY(_start);

SECTIONS
{
    /* SRAM is mapped to 0xFFFD_0000 and is 128 Kb, but only 0xFFFD_6000 and
     * above seems to be available for us to use pre-DDR */
    /* We can also execute in place on flash, which gives us much more space
     * to store code. The flash is mapped at 0x80000000. Nuvoton has a 0x200
     * image header in the front, so that results in start code at 0x80000200
     */
    .bootblock 0x80000200:
    {
        KEEP(*(.bootblock.start));
    }
    .text :
    {
        KEEP(*(.text .text.*));
    }
    .rodata :
    {
        KEEP(*(.rodata .rodata.*));
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
	    	/* Unused exception related info that only wastes space */
		}
}
