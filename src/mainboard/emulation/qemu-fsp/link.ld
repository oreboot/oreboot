MEMORY
{
    /* FSP cannot be moved without recompiling or relocating. */
    fspblob   (rx) : ORIGIN = 0xFFF80000, LENGTH = 0x40000
    /* Bootblock must align with the end since it contains the reset vector. */
    bootblock (rx) : ORIGIN = 0xFFFC0000, LENGTH = 0x40000
}

SECTIONS
{
    .fspblob :
    {
        KEEP(*(.fspblob))
        . = ORIGIN(fspblob) + LENGTH(fspblob);
    } > fspblob = 0xFFFF
    . = 0xffff0000;
    .bootblock :
    {
        /* TODO: Everything is within 64KiB for real-mode jump. */
        . = ORIGIN(bootblock) + LENGTH(bootblock) - 0x10000;
        KEEP(*(.bootblock.boot))
        KEEP(*(.text .text.*))
        KEEP(*(.rodata .rodata.*))
        /* There should be no 'data' as such but we'll fix it later */
        KEEP(*(.data.*))
     }
    . = 0xffffff00;
    .reset . :
    {
        KEEP(*(.reset .reset.* .reset*))
    } > bootblock = 0xFFFF

    .stack_sizes (INFO) :
    {
        KEEP(*(.stack_sizes));
    }

    /DISCARD/ : { *(.comment) *(.note*) *(.eh_frame*)
        /* Unused exception related info that only wastes space */
    }
}
