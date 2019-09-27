ENTRY(_start);

SECTIONS
{
    /* SRAM is mapped to 0xFFFD_0000 and is 128 Kb, but only 0xFFFD_6000 and
     * above seems to be available for us to use pre-DDR */
    .bootblock 0xFFFD6000:
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
