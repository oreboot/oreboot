SECTIONS
{
    . = 0x80000000;
        .text :
        {
            KEEP(*(.text .text.*));
        }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
