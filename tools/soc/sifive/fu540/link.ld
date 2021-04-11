SECTIONS
{
    . = 0x80000000;
    .text :
    {
        KEEP(*(.text .text.*));
    }

    .stack_sizes (INFO) :
    {
        KEEP(*(.stack_sizes));
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*) }
}
